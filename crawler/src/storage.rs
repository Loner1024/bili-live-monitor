use anyhow::Result;
use dotenv::dotenv;
use duckdb::{params, Appender, Connection};
use log::debug;
use parse::{DanmuMessage, SuperChatMessage};
use std::env;
use std::sync::atomic;

pub struct Storage<'a> {
    conn: &'a Connection,
    danmu_message_buffer: Appender<'a>,
    danmu_message_buffer_size: atomic::AtomicI32,
    super_chat_message_buffer: Appender<'a>,
    super_chat_message_buffer_size: atomic::AtomicI32,
    bucket: String,
}

pub struct OssConfig {
    endpoint: String,
    region: String,
    key: String,
    secret: String,
}

impl OssConfig {
    pub fn new() -> Result<Self> {
        dotenv().ok();
        let endpoint = env::var("OSS_ENDPOINT")?;
        let region = env::var("OSS_REGION")?;
        let key = env::var("OSS_KEY")?;
        let secret = env::var("OSS_SECRET")?;
        Ok(Self {
            endpoint,
            region,
            key,
            secret,
        })
    }
}

impl<'a> Storage<'a> {
    fn new(conn: &'a Connection) -> Result<Self> {
        Self::init_table(conn)?;
        let oss_config = OssConfig::new()?;
        Self::init_oss(
            conn,
            oss_config.endpoint.as_str(),
            oss_config.region.as_str(),
            oss_config.key.as_str(),
            oss_config.secret.as_str(),
        )?;
        Ok(Self {
            conn,
            danmu_message_buffer: conn.appender("danmu")?,
            danmu_message_buffer_size: atomic::AtomicI32::new(0),
            super_chat_message_buffer: conn.appender("super_chat")?,
            super_chat_message_buffer_size: atomic::AtomicI32::new(0),
            bucket: "bili-data-1255746465".to_string(),
        })
    }

    fn init_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE super_chat (
                uid BIGINT,
                username TEXT,
                msg TEXT,
                timestamp BIGINT,
                worth FLOAT,
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE danmu (
                uid BIGINT,
                username TEXT,
                msg TEXT,
                timestamp BIGINT
            )",
            [],
        )?;
        Ok(())
    }

    fn init_oss(
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
    pub fn crate_super_chat_message(&mut self, message: SuperChatMessage) -> Result<()> {
        self.super_chat_message_buffer.append_row(params![
            message.uid,
            message.username,
            message.msg,
            message.timestamp,
            message.worth
        ])?;
        self.super_chat_message_buffer_size
            .fetch_add(1, atomic::Ordering::SeqCst);
        if self
            .super_chat_message_buffer_size
            .load(atomic::Ordering::SeqCst)
            >= 1000
        {
            self.super_chat_message_buffer.flush()?;
            self.super_chat_message_buffer_size
                .store(0, atomic::Ordering::SeqCst);
        }

        Ok(())
    }

    pub fn create_danmu_message(&mut self, message: DanmuMessage) -> Result<()> {
        self.danmu_message_buffer.append_row(params![
            message.uid,
            message.username,
            message.msg,
            message.timestamp
        ])?;
        self.danmu_message_buffer_size
            .fetch_add(1, atomic::Ordering::SeqCst);
        if self
            .danmu_message_buffer_size
            .load(atomic::Ordering::SeqCst)
            >= 1000
        {
            self.danmu_message_buffer.flush()?;
            self.danmu_message_buffer_size
                .store(0, atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    fn merge_data_and_persist(&self, table_name: &str) -> Result<()> {
        let persist_target = format!("s3://{}/{}.parquet", self.bucket, table_name);
        // check persist target exists
        if let Err(e) = self.conn.execute(
            &format!("SELECT COUNT(*) as count FROM '{persist_target}'"),
            [],
        ) {
            debug!("check persist target exists failed: {}", e);
        }
        // load existing data
        self.conn.execute(
            &format!("CREATE TABLE existing_data AS SELECT * FROM '{persist_target}'"),
            [],
        )?;
        // merge data
        self.conn.execute(
            &format!("CREATE TABLE merged_data AS SELECT * FROM existing_data UNION ALL SELECT * FROM {table_name}"), [],
        )?;
        self.conn
            .execute(&format!("COPY merged_data TO '{persist_target}'"), [])?;
        // clean
        self.conn.execute("DROP TABLE existing_data", [])?;
        self.conn.execute("DROP TABLE merged_data", [])?;
        self.conn
            .execute(&format!("DELETE FROM {table_name}"), [])?;
        Ok(())
    }

    pub fn persist(&mut self) -> Result<()> {
        // merge data

        self.conn
            .execute("COPY danmu TO 'data.parquet' (FORMAT 'parquet')", [])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn init() {
        pretty_env_logger::init();
    }

    #[test]
    fn test_storage_crate_danmu() {
        init();
        let conn = Connection::open_in_memory().unwrap();
        let now = Utc::now();
        let danmu = DanmuMessage {
            uid: 10000,
            username: "Alice".to_string(),
            msg: "Hello, Bilibili".to_string(),
            timestamp: now.timestamp_millis() as u64,
        };
        let mut storage = Storage::new(&conn).unwrap();
        for _ in 0..1000 {
            storage.create_danmu_message(danmu.clone()).unwrap();
        }
        conn.query_row("SELECT COUNT(*) as count FROM danmu", [], |row| {
            let count: i64 = row.get("count")?;
            assert_eq!(count, 1000);
            Ok(())
        })
        .unwrap();
        conn.query_row("SELECT * FROM danmu", [], |row| {
            let uid: i64 = row.get("uid")?;
            let username: String = row.get("username")?;
            let msg: String = row.get("msg")?;
            let timestamp: i64 = row.get("timestamp")?;
            assert_eq!(uid, 10000);
            assert_eq!(username, "Alice");
            assert_eq!(msg, "Hello, Bilibili");
            assert_eq!(timestamp, now.timestamp_millis());
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_storage_crate_super_chat() {
        init();
        let conn = Connection::open_in_memory().unwrap();
        let now = Utc::now();
        let super_chat = SuperChatMessage {
            uid: 10000,
            username: "Alice".to_string(),
            msg: "Hello, Bilibili".to_string(),
            timestamp: now.timestamp_millis() as u64,
            worth: 100.0,
        };
        let mut storage = Storage::new(&conn).unwrap();
        for _ in 0..1000 {
            storage
                .crate_super_chat_message(super_chat.clone())
                .unwrap();
        }
        conn.query_row("SELECT COUNT(*) as count FROM super_chat", [], |row| {
            let count: i64 = row.get("count")?;
            assert_eq!(count, 1000);
            Ok(())
        })
        .unwrap();
        conn.query_row("SELECT * FROM super_chat", [], |row| {
            let uid: i64 = row.get("uid")?;
            let username: String = row.get("username")?;
            let msg: String = row.get("msg")?;
            let timestamp: i64 = row.get("timestamp")?;
            let worth: f64 = row.get("worth")?;
            assert_eq!(uid, 10000);
            assert_eq!(username, "Alice");
            assert_eq!(msg, "Hello, Bilibili");
            assert_eq!(timestamp, now.timestamp_millis());
            assert_eq!(worth, 100.0);
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_merge_data_and_persist() {
        init();
        let conn = Connection::open_in_memory().unwrap();
        let now = Utc::now();
        let danmu = DanmuMessage {
            uid: 10000,
            username: "Alice".to_string(),
            msg: "Hello, Bilibili".to_string(),
            timestamp: now.timestamp_millis() as u64,
        };
        let mut storage = Storage::new(&conn).unwrap();
        for _ in 0..1000 {
            storage.create_danmu_message(danmu.clone()).unwrap();
        }
        storage.merge_data_and_persist("danmu").unwrap();
    }
}
