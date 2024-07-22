#[derive(Debug)]
pub struct StatisticsResult {
    pub danmu_total: u64,      // 总弹幕数量
    pub danmu_people: u64,     // 总弹幕人数
    pub super_chat_total: u64, // 总SC数量
    pub super_chat_worth: u64, // 总SC人数
}

#[derive(Copy, Clone)]
pub enum StatisticsScope {
    Day,
    Week,
}

impl StatisticsScope {
    pub fn remote_table_name(self, bucket: &str, room_id: i64) -> String {
        match self {
            StatisticsScope::Day => {
                format!("s3://{}/statistics/{}/{}.parquet", bucket, room_id, "day")
            }
            StatisticsScope::Week => {
                format!("s3://{}/statistics/{}/{}.parquet", bucket, room_id, "week")
            }
        }
    }

    pub fn local_table_name(&self, room_id: i64) -> String {
        match self {
            StatisticsScope::Day => {
                format!("{}_{}", "day", room_id)
            }
            StatisticsScope::Week => {
                format!("{}_{}", "week", room_id)
            }
        }
    }
}
