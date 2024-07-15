use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use duckdb::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use duckdb::{Connection, DuckdbConnectionManager};
use r2d2::PooledConnection;
use std::env;
use std::fmt::{Display, Formatter};

impl From<MessageType> for i8 {
    fn from(message_type: MessageType) -> Self {
        match message_type {
            MessageType::Danmu => 1,
            MessageType::SuperChat => 2,
        }
    }
}

impl FromSql for MessageType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match i8::column_result(value)? {
            1 => Ok(MessageType::Danmu),
            2 => Ok(MessageType::SuperChat),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MessageType {
    Danmu,
    SuperChat,
}

impl Into<MessageType> for Option<String> {
    fn into(self) -> MessageType {
        match self {
            None => MessageType::Danmu,
            Some(s) => {
                if s == "super_chat" {
                    MessageType::SuperChat
                } else {
                    MessageType::Danmu
                }
            }
        }
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MessageType::Danmu => "danmu",
                MessageType::SuperChat => "super_chat",
            }
        )
    }
}

// 获取表名
pub fn get_table_name(bucket: &str, room_id: i64, timestamp: i64) -> Result<String> {
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(anyhow!("Invalid timestamp"))?
        .with_timezone(&Local)
        .format("%Y-%m-%d");
    Ok(format!(
        "s3://{}/{}/{}/danmu.parquet",
        bucket, datetime, room_id
    ))
}

pub fn is_new_day(old_timestamp: i64, new_timestamp: i64) -> Result<bool> {
    let old_day = get_local_midnight(old_timestamp)?;
    let new_day = get_local_midnight(new_timestamp)?;
    println!("old_day: {}, new_day: {}", old_day, new_day);

    Ok(new_day > old_day)
}

fn get_local_midnight(timestamp: i64) -> Result<i64> {
    let hour = 3600;
    let naive = DateTime::from_timestamp(timestamp, 0)
        .ok_or(anyhow!("Invalid timestamp"))?
        .with_timezone(&Local)
        .naive_local()
        .date()
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow!("Invalid timestamp"))?;
    let tz = FixedOffset::east_opt(8 * hour).ok_or(anyhow!("Invalid timezone"))?;
    let utc = naive
        .and_local_timezone(tz)
        .single()
        .ok_or(anyhow!("Invalid timestamp"))?;

    Ok(utc.timestamp())
}

pub struct OssConfig {
    pub endpoint: String,
    pub region: String,
    pub key: String,
    pub secret: String,
    pub bucket: String,
}

pub fn init_oss_with_pool(
    conn: &PooledConnection<DuckdbConnectionManager>,
    endpoint: &str,
    region: &str,
    key: &str,
    secret: &str,
) -> Result<()> {
    let stmt = format!(
        "CREATE SECRET (
                TYPE S3,
                Endpoint '{endpoint}',
                KEY_ID '{key}',
                SECRET '{secret}',
                REGION '{region}'
            );",
    );
    conn.execute(&stmt, [])?;
    Ok(())
}

pub fn init_oss_with_conn(
    conn: &Connection,
    endpoint: &str,
    region: &str,
    key: &str,
    secret: &str,
) -> Result<()> {
    let stmt = format!(
        "CREATE SECRET (
                TYPE S3,
                Endpoint '{endpoint}',
                KEY_ID '{key}',
                SECRET '{secret}',
                REGION '{region}'
            );",
    );
    conn.execute(&stmt, [])?;
    Ok(())
}

impl OssConfig {
    pub fn new() -> Result<Self> {
        let endpoint = env::var("OSS_ENDPOINT")?;
        let region = env::var("OSS_REGION")?;
        let key = env::var("OSS_KEY")?;
        let secret = env::var("OSS_SECRET")?;
        let bucket = env::var("OSS_BUCKET")?;
        Ok(Self {
            endpoint,
            region,
            key,
            secret,
            bucket,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_local_midnight() {
        let timestamp = 1720796606; // 2024-07-12 23:03:26
        let midnight = 1720713600; // 2024-07-12 00:00:00
        assert_eq!(get_local_midnight(timestamp).unwrap(), midnight);
        let timestamp = 1720800026; // 2024-07-13 00:00:26
        let midnight = 1720800000; // 2024-07-13 00:00:00
        assert_eq!(get_local_midnight(timestamp).unwrap(), midnight);
        let timestamp = 1720843226; // 2024-07-13 12:00:26
        let midnight = 1720800000; // 2024-07-13 00:00:00
        assert_eq!(get_local_midnight(timestamp).unwrap(), midnight);
        let timestamp = 1720886399; // 2024-07-13 23:59:59
        let midnight = 1720800000; // 2024-07-13 00:00:00
        assert_eq!(get_local_midnight(timestamp).unwrap(), midnight);
    }

    #[test]
    fn test_is_new_day() {
        let old_timestamp = 1720796606; // 2024-07-12 23:03:26
        let new_timestamp = 1720800026; // 2024-07-13 00:00:26
        assert!(is_new_day(old_timestamp, new_timestamp).unwrap());

        let old_timestamp = 1720796606; // 2024-07-12 23:03:26
        let new_timestamp = 1720796672; // 2024-07-12 23:04:32
        assert!(!is_new_day(old_timestamp, new_timestamp).unwrap());

        let old_timestamp = 1720886399; // 2024-07-13 23:59:59
        let new_timestamp = 1720972799; // 2024-07-14 23:59:59
        assert!(is_new_day(old_timestamp, new_timestamp).unwrap());
    }

    #[test]
    fn test_get_table_name() {
        let bucket_name = "bilibili";
        let room_id = 123456789;
        let timestamp = 1720973747;

        let table_name = get_table_name(bucket_name, room_id, timestamp).unwrap();
        assert_eq!(
            table_name,
            "s3://bilibili/2024-07-15/123456789/danmu.parquet"
        );
    }
}
