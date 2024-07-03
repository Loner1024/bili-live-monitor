use anyhow::Result;
use duckdb::arrow::record_batch::RecordBatch;
use duckdb::arrow::util::pretty::print_batches;
use duckdb::Connection;
use std::thread::sleep;

fn main() -> Result<()> {
    // 初始化 DuckDB
    let conn = Connection::open_in_memory()?;

    // 创建表并插入数据
    // conn.execute(
    //     "CREATE TABLE data (
    //         uid BIGINT,
    //         username TEXT,
    //         msg TEXT,
    //         timestamp BIGINT,
    //     )",
    //     [],
    // )?;
    // let danmu = DanmuMessage {
    //     uid: 10000,
    //     username: "Alice".to_string(),
    //     msg: "哈哈哈哈哈哈哈哈哈哈哈哈[笑哭]哈哈哈哈哈哈哈哈哈哈哈哈[笑哭]哈哈哈哈哈哈哈哈哈哈哈哈[笑哭]哈哈哈哈哈哈哈哈哈哈哈哈[笑哭]".to_string(),
    //     timestamp: 1719764239,
    // };
    // for i in 0..50000 {
    //     println!("Inserting data... {}", i);
    //     conn.execute(
    //         "INSERT INTO data (uid, username, msg, timestamp) VALUES
    //         (?, ?, ?, ?)",
    //         params![danmu.uid, danmu.username, danmu.msg, danmu.timestamp]
    //     )?;
    // };
    //
    // // 导出数据到 Parquet 文件
    // conn.execute("CREATE TABLE merged_data AS SELECT * FROM existing_data UNION ALL SELECT * FROM new_data", [])?;
    // conn.execute("COPY data TO 'data.parquet' (FORMAT 'parquet')", [])?;
    //
    // println!("Data exported to data.parquet successfully.");
    conn.execute("SELECT * FROM 'data.parquet'", [])?;
    let res: Vec<RecordBatch> = conn
        .prepare("SELECT * FROM 'data.parquet' Limit 10")?
        .query_arrow([])?
        .collect();
    print_batches(&res)?;
    println!("Data imported from data.parquet successfully.");
    sleep(std::time::Duration::from_secs(60));
    Ok(())
}
