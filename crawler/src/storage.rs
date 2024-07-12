use anyhow::Result;
use duckdb::{params, Appender, Connection};
use log::{debug, info};
use parse::{DanmuMessage, SuperChatMessage};
use std::env;
use std::sync::atomic;
use utils::utils::{get_table_name, MessageType};

pub struct Storage<'a> {
    conn: &'a Connection,
    danmu_message_buffer: Appender<'a>,
    danmu_message_buffer_size: atomic::AtomicI32,
    super_chat_message_buffer: Appender<'a>,
    super_chat_message_buffer_size: atomic::AtomicI32,
    bucket: String,
    timestamp: i64,
    room_id: i64,
}

pub struct OssConfig {
    endpoint: String,
    region: String,
    key: String,
    secret: String,
    bucket: String,
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

impl<'a> Storage<'a> {
    pub fn new(conn: &'a Connection, room_id: i64, timestamp: i64) -> Result<Self> {
        let oss_config = OssConfig::new()?;
        Self::init_oss(
            conn,
            oss_config.endpoint.as_str(),
            oss_config.region.as_str(),
            oss_config.key.as_str(),
            oss_config.secret.as_str(),
        )?;
        Self::init_table(conn, &oss_config.bucket, room_id, timestamp)?;
        Ok(Self {
            conn,
            danmu_message_buffer: conn.appender("danmu")?,
            danmu_message_buffer_size: atomic::AtomicI32::new(0),
            super_chat_message_buffer: conn.appender("super_chat")?,
            super_chat_message_buffer_size: atomic::AtomicI32::new(0),
            bucket: oss_config.bucket,
            room_id,
            timestamp,
        })
    }

    fn init_table(conn: &Connection, bucket: &str, room_id: i64, timestamp: i64) -> Result<()> {
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

        let super_chat_target = get_table_name(bucket, MessageType::SuperChat, room_id, timestamp)?;
        let danmu_target = get_table_name(bucket, MessageType::Danmu, room_id, timestamp)?;
        // check file exists
        if conn
            .execute(
                &format!("SELECT COUNT(*) as count FROM '{super_chat_target}'"),
                [],
            )
            .is_err()
        {
            conn.execute(&format!("COPY super_chat TO '{super_chat_target}'"), [])?;
        }
        if conn
            .execute(
                &format!("SELECT COUNT(*) as count FROM '{danmu_target}'"),
                [],
            )
            .is_err()
        {
            conn.execute(&format!("COPY danmu TO '{danmu_target}'"), [])?;
        }

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
            >= 10
        {
            self.flush()?;
        }
        Ok(())
    }

    pub fn create_danmu_message(&mut self, message: DanmuMessage) -> Result<()> {
        debug!(
            "receive danmu count: {}",
            self.danmu_message_buffer_size
                .load(atomic::Ordering::SeqCst)
        );
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
            >= 100
        {
            self.flush()?;
        }
        Ok(())
    }

    fn merge_data_and_persist(&self, persist_target: &str, local_table: MessageType) -> Result<()> {
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
            &format!("CREATE TABLE merged_data AS SELECT * FROM existing_data UNION ALL SELECT * FROM {local_table}"), [],
        )?;
        self.conn
            .execute(&format!("COPY merged_data TO '{persist_target}'"), [])?;
        // clean
        self.conn.execute("DROP TABLE existing_data", [])?;
        self.conn.execute("DROP TABLE merged_data", [])?;
        self.conn
            .execute(&format!("DELETE FROM {local_table}"), [])?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.super_chat_message_buffer.flush()?;
        self.danmu_message_buffer.flush()?;
        self.super_chat_message_buffer_size
            .store(0, atomic::Ordering::SeqCst);
        self.danmu_message_buffer_size
            .store(0, atomic::Ordering::SeqCst);

        let super_chat_target = get_table_name(&self.bucket, MessageType::SuperChat, self.room_id, self.timestamp)?;
        let danmu_target = get_table_name(&self.bucket, MessageType::Danmu, self.room_id, self.timestamp)?;

        self.merge_data_and_persist(&super_chat_target, MessageType::SuperChat)?;
        self.merge_data_and_persist(&danmu_target, MessageType::Danmu)?;
        info!("flush success");

        Ok(())
    }

    pub fn switch_new_date(&mut self, timestamp: i64) -> Result<()> {
        // flush exist data
        self.flush()?;
        // change timestamp
        self.timestamp = timestamp;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Local, Utc};
    use dotenv::dotenv;

    fn init() {
        pretty_env_logger::init();
        dotenv().ok();
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
            timestamp: now.timestamp() as u64,
        };
        let room_id = 22747736;
        let mut storage = Storage::new(&conn, room_id, now.timestamp()).unwrap();
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
            assert_eq!(timestamp, now.timestamp());
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
            timestamp: now.timestamp() as u64,
            worth: 100.0,
        };

        let room_id = 22747736;
        let mut storage = Storage::new(&conn, room_id, now.timestamp()).unwrap();
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
            assert_eq!(timestamp, now.timestamp());
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
        let room_id = 22747736;
        let mut storage = Storage::new(&conn, room_id, now.timestamp()).unwrap();
        for _ in 0..1000 {
            storage.create_danmu_message(danmu.clone()).unwrap();
        }
        let oss_config = OssConfig::new().unwrap();
        let danmu_target = get_table_name(&oss_config.bucket, MessageType::Danmu, room_id, now.timestamp()).unwrap();
        storage.merge_data_and_persist(&danmu_target, MessageType::Danmu).unwrap();
    }
}
