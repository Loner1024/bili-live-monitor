use anyhow::{anyhow, Result};
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};
use duckdb::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use duckdb::{Connection, DuckdbConnectionManager};
use r2d2::PooledConnection;
use std::env;
use std::fmt::{Display, Formatter};

pub fn get_rooms() -> Vec<i64> {
    vec![22747736, 21533102, 23649609]
}

#[derive(Default)]
pub struct Pagination {
    pub limit: usize,
    pub offset: usize,
}

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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MessageType {
    Danmu,
    SuperChat,
}

impl From<Option<String>> for MessageType {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => match v.as_str() {
                "danmu" => MessageType::Danmu,
                "super_chat" => MessageType::SuperChat,
                _ => MessageType::Danmu,
            },
            None => MessageType::Danmu,
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

pub fn remote_block_user_table_name(bucket: &str) -> String {
    format!("s3://{bucket}/block/block_user.parquet")
}

// 获取表名
pub fn get_table_name(bucket: &str, room_id: i64, timestamp: i64) -> Result<String> {
    Ok(format!(
        "s3://{}/{}/{}/danmu.parquet",
        bucket,
        get_format_date(timestamp)?,
        room_id
    ))
}

pub fn get_format_date(timestamp: i64) -> Result<String> {
    Ok(Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(anyhow!("Invalid timestamp"))?
        .with_timezone(&Local)
        .format("%Y-%m-%d")
        .to_string())
}

pub fn is_new_day(old_timestamp: i64, new_timestamp: i64) -> Result<bool> {
    let old_day = get_local_midnight(old_timestamp)?;
    let new_day = get_local_midnight(new_timestamp)?;

    Ok(new_day > old_day)
}

pub fn get_local_midnight(timestamp: i64) -> Result<i64> {
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

pub fn get_every_day_with_start_end(start: i64, end: i64) -> Result<Vec<i64>> {
    let mut start = get_local_midnight(start)?;
    let end = get_local_midnight(end)?;
    let mut result = vec![];
    while start < end {
        result.push(start);
        start += 24 * 3600;
    }
    result.push(end);
    Ok(result)
}

#[derive(Clone)]
pub struct OssConfig {
    pub endpoint: String,
    pub region: String,
    pub key: String,
    pub secret: String,
    pub bucket: String,
}

impl OssConfig {}

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

impl OssConfig {
    pub fn new() -> Result<Self> {
        let endpoint = env::var("OSS_ENDPOINT").map_err(|_| anyhow!("OSS_ENDPOINT must be set"))?;
        let region = env::var("OSS_REGION").map_err(|_| anyhow!("OSS_REGION must be set"))?;
        let key = env::var("OSS_KEY").map_err(|_| anyhow!("OSS_KEY must be set"))?;
        let secret = env::var("OSS_SECRET").map_err(|_| anyhow!("OSS_SECRET must be set"))?;
        let bucket = env::var("OSS_BUCKET").map_err(|_| anyhow!("OSS_BUCKET must be set"))?;
        Ok(Self {
            endpoint,
            region,
            key,
            secret,
            bucket,
        })
    }

    pub fn init_oss_with_conn(self, conn: &Connection) -> Result<()> {
        let stmt = format!(
            "CREATE SECRET (
                TYPE S3,
                Endpoint '{}',
                KEY_ID '{}',
                SECRET '{}',
                REGION '{}'
            );",
            self.endpoint, self.key, self.secret, self.region,
        );
        conn.execute(&stmt, [])?;

        Ok(())
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
