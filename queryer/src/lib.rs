use anyhow::Result;
use duckdb::DuckdbConnectionManager;
use model::statistics;
use parse::{DanmuMessage, Message, SuperChatMessage};
use r2d2::Pool;
use utils::utils::{
    get_local_midnight, get_table_name, init_oss_with_pool, MessageType, OssConfig, Pagination,
};

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

    #[allow(clippy::too_many_arguments)]
    pub fn query(
        &self,
        room_id: i64,
        timestamp: i64,
        message_type: Option<MessageType>,
        uid: Option<u64>,
        username: Option<String>,
        message: Option<String>,
        pagination: Option<Pagination>,
    ) -> Result<Vec<Message>> {
        let conn = self.pool.get()?;
        let mut query_stmt = conn.prepare(&self.build_stmt(
            "*",
            room_id,
            timestamp,
            message_type,
            uid,
            username,
            message,
            pagination,
            Some("timestamp DESC".to_string()),
        )?)?;

        let mut rows = query_stmt.query([])?;

        let mut result = Vec::new();

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

    pub fn query_count(
        &self,
        room_id: i64,
        timestamp: i64,
        message_type: Option<MessageType>,
        uid: Option<u64>,
        username: Option<String>,
        message: Option<String>,
    ) -> Result<usize> {
        let conn = self.pool.get()?;
        let mut query_stmt = conn.prepare(&self.build_stmt(
            "count(*)",
            room_id,
            timestamp,
            message_type,
            uid,
            username,
            message,
            None,
            None,
        )?)?;

        let mut rows = query_stmt.query([])?;
        let count: usize = rows.next()?.unwrap().get(0)?;
        Ok(count)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_stmt(
        &self,
        col: &str,
        room_id: i64,
        timestamp: i64,
        message_type: Option<MessageType>,
        uid: Option<u64>,
        username: Option<String>,
        message: Option<String>,
        pagination: Option<Pagination>,
        order: Option<String>,
    ) -> Result<String> {
        let mut contidition = Vec::new();
        if let Some(username) = username {
            contidition.push(format!("username = '{}'", username));
        }
        if let Some(message) = message {
            contidition.push(format!("msg LIKE '%{}%'", message));
        }
        if let Some(message_type) = message_type {
            contidition.push(format!("msg_type = '{}'", i8::from(message_type)));
        }
        if let Some(uid) = uid {
            contidition.push(format!("uid = {}", uid));
        }
        let where_clause = if contidition.is_empty() {
            String::from("")
        } else {
            format!("WHERE {}", contidition.join(" AND "))
        };
        let table = get_table_name(&self.bucket, room_id, timestamp)?;

        let pagination_clause = match pagination {
            None => String::from(""),
            Some(pagination) => {
                format!("LIMIT {} OFFSET {}", pagination.limit, pagination.offset)
            }
        };
        let order_clause = match order {
            None => String::from(""),
            Some(order) => {
                format!("ORDER BY {}", order)
            }
        };
        Ok(format!(
            "SELECT {} FROM '{}' {} {} {}",
            col, table, where_clause, order_clause, pagination_clause
        ))
    }

    pub fn query_statistics_data(
        &self,
        room_id: i64,
        timestamp: i64,
    ) -> Result<statistics::StatisticsResult> {
        let timestamp = get_local_midnight(timestamp)?;
        let table = statistics::StatisticsScope::Day.remote_table_name(&self.bucket, room_id);
        let conn = self.pool.get()?;
        let result = conn.query_row(
            &format!("SELECT * FROM '{}' WHERE timestamp = ?", table),
            [timestamp],
            |row| {
                Ok(statistics::StatisticsResult {
                    danmu_total: row.get("danmu_total")?,
                    danmu_people: row.get("danmu_people")?,
                    super_chat_total: row.get("super_chat_total")?,
                    super_chat_worth: row.get("super_chat_worth")?,
                })
            },
        )?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    #[ignore]
    fn test_order() {
        pretty_env_logger::init();
        dotenv().ok().unwrap();
        let manager = DuckdbConnectionManager::memory().unwrap();
        let pool = Pool::new(manager).unwrap();
        let query = Query::new(pool).unwrap();
        let room_id = 21533102;
        let timestamp = 1721153475;
        let result = query
            .query(
                room_id,
                timestamp,
                Some(MessageType::Danmu),
                None,
                None,
                None,
                Some(Pagination {
                    limit: 10,
                    offset: 0,
                }),
            )
            .unwrap();
        for message in result {
            println!("{:?}", message);
        }
    }

    #[test]
    #[ignore]
    fn test_query() {
        pretty_env_logger::init();
        dotenv().ok().unwrap();
        let manager = DuckdbConnectionManager::memory().unwrap();
        let pool = Pool::new(manager).unwrap();
        let query = Query::new(pool).unwrap();
        let room_id = 22747736;
        let timestamp = 1720973747;
        let result = query
            .query(
                room_id,
                timestamp,
                Some(MessageType::SuperChat),
                None,
                None,
                None,
                None,
            )
            .unwrap();
        for message in result {
            println!("{:?}", message);
        }
    }
}
