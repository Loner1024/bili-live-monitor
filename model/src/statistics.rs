use serde::Serialize;
use utils::utils::get_format_date;

#[derive(Debug, Serialize, Clone)]
pub struct StatisticsResult {
    pub timestamp: i64,
    pub danmu_total: u64,      // 总弹幕数量
    pub danmu_people: u64,     // 总弹幕人数
    pub super_chat_total: u64, // 总SC数量
    pub super_chat_worth: u64, // 总SC人数
}

#[derive(Copy, Clone)]
pub enum StatisticsScope {
    Day,
}

impl StatisticsScope {
    pub fn remote_table_name(self, bucket: &str, room_id: i64, timestamp: i64) -> String {
        match self {
            StatisticsScope::Day => {
                format!(
                    "s3://{}/statistics/{}/{}_{}.parquet",
                    bucket,
                    room_id,
                    "day",
                    get_format_date(timestamp).unwrap(),
                )
            }
        }
    }

    pub fn local_table_name(&self, room_id: i64) -> String {
        match self {
            StatisticsScope::Day => {
                format!("{}_{}", "day", room_id)
            }
        }
    }
}
