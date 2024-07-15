use anyhow::{Context, Result};
use duckdb::DuckdbConnectionManager;
use parse::{DanmuMessage, Message, SuperChatMessage};
use r2d2::{Pool, PooledConnection};
use tokio::task;
use utils::utils::{get_table_name, init_oss_with_pool, MessageType, OssConfig};

#[derive(Clone)]
pub struct Query {
    pool: Pool<DuckdbConnectionManager>,
    bucket: String,
}

impl Query {
    pub fn new(pool: Pool<DuckdbConnectionManager>) -> Result<Self> {
        let oss_config = OssConfig::new()?;
        init_oss_with_pool(
            &pool.get()?,
            oss_config.endpoint.as_str(),
            oss_config.region.as_str(),
            oss_config.key.as_str(),
            oss_config.secret.as_str(),
        )?;
        Ok(Self {
            pool,
            bucket: oss_config.bucket,
        })
    }

    pub fn query(
        &self,
        room_id: i64,
        timestamp: i64,
        message_type: Option<MessageType>,
        username: Option<String>,
        message: Option<String>,
    ) -> Result<Vec<Message>> {
        let conn = self.pool.get()?;
        let mut result = Vec::new();
        let mut contidition = Vec::new();
        if let Some(username) = username {
            contidition.push(format!("username = '{}'", username));
        }
        if let Some(message) = message {
            contidition.push(format!("message LIKE '%{}%'", message));
        }
        if let Some(message_type) = message_type {
            contidition.push(format!("msg_type = '{}'", i8::from(message_type)));
        }
        let where_clause = if contidition.is_empty() {
            String::from("")
        } else {
            format!("WHERE {}", contidition.join(" AND "))
        };
        let table = get_table_name(&self.bucket, room_id, timestamp)?;

        let mut query_stmt =
            conn.prepare(&format!("SELECT * FROM '{}' {}", table, where_clause))?;

        let mut rows = query_stmt.query([])?;

        while let Some(row) = rows.next()? {
            let message_type: MessageType = row.get("msg_type")?;
            let uid: u64 = row.get("uid")?;
            let username: String = row.get("username")?;
            let msg: String = row.get("msg")?;
            let timestamp: u64 = row.get("timestamp")?;
            let worth: f64 = row.get("worth")?;
            let message = match message_type {
                MessageType::Danmu => Message::Danmu(DanmuMessage {
                    uid,
                    username,
                    msg,
                    timestamp,
                }),
                MessageType::SuperChat => Message::SuperChat(SuperChatMessage {
                    uid,
                    username,
                    msg,
                    timestamp,
                    worth,
                }),
            };
            result.push(message);
        }

        Ok(result)
    }

    async fn get_conn(&self) -> Result<PooledConnection<DuckdbConnectionManager>> {
        let pool = self.pool.clone();
        task::spawn_blocking(move || pool.get())
            .await
            .context("Failed to spawn blocking task")?
            .context("Failed to get connection from pool")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    #[ignore]
    fn test_query() {
        pretty_env_logger::init();
        dotenv().ok().unwrap();
        let manager = DuckdbConnectionManager::memory().unwrap();
        let pool = Pool::new(manager)?;
        let query = Query::new(pool).unwrap();
        let room_id = 22747736;
        let timestamp = 1720973747;
        let result = query
            .query(room_id, timestamp, Some(MessageType::SuperChat), None, None)
            .unwrap();
        for message in result {
            println!("{:?}", message);
        }
    }
}
