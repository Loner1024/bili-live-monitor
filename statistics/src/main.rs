use anyhow::{anyhow, Result};
use chrono::{Duration, Local};
use duckdb::{params, Connection};
use log::{debug, info};
use model::statistics::{StatisticsResult, StatisticsScope};
use utils::utils::{get_local_midnight, get_rooms, get_table_name, MessageType, OssConfig};

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("please input scope"));
    }

    let scope = args[1].clone();
    if scope != "today" && scope != "yesterday" {
        return Err(anyhow!("scope must be today or yesterday"));
    }
    let rooms = get_rooms();
    let mut conn = Connection::open_in_memory()?;
    let mut stats = Statistics::new(&mut conn)?;
    info!("start statistics {}", scope);
    match scope.as_str() {
        "today" => {
            for room in rooms {
                stats.init_table(room)?;
                stats.statistics_today(room)?;
                info!("statistics room {} done", room);
            }
        }
        "yesterday" => {
            for room in rooms {
                stats.init_table(room)?;
                stats.statistics_yesterday(room)?;
                info!("statistics room {} done", room);
            }
        }
        "someday" => {
            let timestamp = args[2].clone().parse::<i64>()?;
            for room in rooms {
                stats.init_table(room)?;
                stats.statistics_day(timestamp, room)?;
                info!("statistics room {} done", room);
            }
        }
        _ => {}
    };
    info!("statistics {} done", scope);

    Ok(())
}

struct Statistics<'a> {
    bucket: String,
    conn: &'a mut Connection,
}

impl<'a> Statistics<'a> {
    fn new(conn: &'a mut Connection) -> Result<Self> {
        let oss_config = OssConfig::new()?;
        oss_config.clone().init_oss_with_conn(conn)?;
        Ok(Self {
            bucket: oss_config.bucket,
            conn,
        })
    }

    pub fn init_table(&self, room_id: i64) -> Result<()> {
        let tables = vec![StatisticsScope::Day, StatisticsScope::Week];
        for table in tables {
            // crate local table
            let local_table = table.local_table_name(room_id);
            // check remote table exist
            let remote_table_name = table.remote_table_name(&self.bucket, room_id);
            if self
                .conn
                .execute(
                    &format!("SELECT COUNT(*) as count FROM '{remote_table_name}'"),
                    [],
                )
                .is_err()
            {
                info!(
                    "[room: {}] remote {} table not exist, create it and crate local table",
                    room_id, remote_table_name
                );
                // create local table
                self.conn
                    .execute(self.get_create_ddl(local_table.as_str()).as_str(), [])?;
                self.conn
                    .execute(&format!("COPY {local_table} TO '{remote_table_name}'"), [])?;
            } else {
                // load remote data to local
                info!(
                    "[room: {}] remote {} table exist, load it to local table",
                    room_id, remote_table_name
                );
                self.conn.execute(
                    &format!("CREATE TABLE {local_table} AS SELECT * FROM '{remote_table_name}'"),
                    [],
                )?;
            }
        }
        Ok(())
    }

    pub fn statistics_today(&mut self, room_id: i64) -> Result<()> {
        let timestamp = Local::now().timestamp();
        self.statistics_day(timestamp, room_id)
    }

    pub fn statistics_yesterday(&mut self, room_id: i64) -> Result<()> {
        let timestamp = Local::now().timestamp() - Duration::days(1).num_seconds();
        self.statistics_day(timestamp, room_id)
    }

    fn statistics_day(&mut self, timestamp: i64, room_id: i64) -> Result<()> {
        let timestamp = get_local_midnight(timestamp)?;
        let data_table = get_table_name(&self.bucket, room_id, timestamp)?;
        let table = StatisticsScope::Day;
        let local_table = table.local_table_name(room_id);
        let remote_table = table.remote_table_name(&self.bucket, room_id);

        let result = self.conn.query_row(
            format!(
                "SELECT
                        '{}' AS timestamp,
                        COALESCE(SUM(worth), 0) AS super_chat_worth,
                        COALESCE(COUNT(DISTINCT uid), 0) AS danmu_people,
                        COALESCE(COUNT(CASE WHEN msg_type = {} THEN 1 END), 0) AS super_chat_total,
                        COALESCE(COUNT(*), 0) AS danmu_total
                    FROM
                        '{}'",
                local_table,
                i8::from(MessageType::SuperChat),
                data_table,
            )
            .as_str(),
            [],
            |row| {
                Ok(StatisticsResult {
                    danmu_total: row.get("danmu_total")?,
                    danmu_people: row.get("danmu_people")?,
                    super_chat_total: row.get("super_chat_total")?,
                    super_chat_worth: row.get("super_chat_worth")?,
                    timestamp: row.get("timestamp")?,
                })
            },
        )?;
        debug!("statistics result: {:?}", result);

        // start transaction
        let tx = self.conn.transaction()?;
        // update record
        let updated = tx.execute(
            format!(
                "UPDATE {}
         SET
            danmu_total = {},
            danmu_people = {},
            super_chat_total = {},
            super_chat_worth = {},
         WHERE
            timestamp = ?",
                local_table,
                result.danmu_total,
                result.danmu_people,
                result.super_chat_total,
                result.super_chat_worth
            )
            .as_str(),
            params![timestamp],
        )?;
        debug!("try update");
        // insert new record if not update
        if updated == 0 {
            tx.execute(
                format!("INSERT INTO {} (danmu_total, danmu_people, super_chat_total, super_chat_worth, timestamp)
                        VALUES ({}, {}, {}, {}, {})", local_table, result.danmu_total,result.danmu_people, result.super_chat_total, result.super_chat_worth, timestamp).as_str(),
                [],
            )?;
        }
        tx.commit()?;

        self.conn
            .execute(&format!("COPY {local_table} TO '{remote_table}'"), [])?;
        Ok(())
    }

    fn get_create_ddl(&self, table_name: &str) -> String {
        format!(
            "CREATE TABLE {} (
                timestamp BIGINT UNIQUE,
                danmu_total BIGINT,
                danmu_people BIGINT,
                super_chat_total BIGINT,
                super_chat_worth BIGINT,
            )",
            table_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remote_table_name() {
        let table_name = StatisticsScope::Day.remote_table_name("test", 1);
        assert_eq!(table_name, "s3://test/statistics/1/day.parquet");
        let table_name = StatisticsScope::Week.remote_table_name("test", 1);
        assert_eq!(table_name, "s3://test/statistics/1/week.parquet");
    }
}
