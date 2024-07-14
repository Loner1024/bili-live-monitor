use anyhow::Result;
use duckdb::Connection;
use parse::{DanmuMessage, Message, SuperChatMessage};
use utils::utils::{get_table_name, init_oss, MessageType, OssConfig};

pub struct Query<'a> {
    conn: &'a Connection,
    bucket: String,
}

impl<'a> Query<'a> {
    pub fn new(conn: &'a Connection) -> Result<Self> {
        let oss_config = OssConfig::new()?;
        init_oss(
            conn,
            oss_config.endpoint.as_str(),
            oss_config.region.as_str(),
            oss_config.key.as_str(),
            oss_config.secret.as_str(),
        )?;
        Ok(Self {
            conn,
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

        let mut query_stmt = self
            .conn
            .prepare(&format!("SELECT * FROM '{}' {}", table, where_clause))?;

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
        let conn = Connection::open_in_memory().unwrap();
        let query = Query::new(&conn).unwrap();
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
