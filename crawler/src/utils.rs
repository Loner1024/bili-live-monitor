use crate::storage::MessageType;
use anyhow::{anyhow, Result};
use chrono::{Local, TimeZone, Utc};

// 获取表名
fn get_table_name(message_type: MessageType, room_id: i64, timestamp: i64) -> Result<String> {
    // 将 Unix 时间戳转换为 UTC 时间
    // 将 UTC 时间转换为本地时间
    let datetime = Utc
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or(anyhow!("Invalid timestamp"))?
        .with_timezone(&Local).format("%Y-%m-%d");

    Ok(format!("{}/{}/{}", datetime, room_id, message_type))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_table_name() {
        let message_type = MessageType::Danmu;
        let room_id = 123456789;
        let timestamp = 1720526217; // 2023-05-10 12:00:00 UTC

        let table_name = get_table_name(message_type, room_id, timestamp).unwrap();
        assert_eq!(table_name, "2024-07-09/123456789/danmu");
    }
}
