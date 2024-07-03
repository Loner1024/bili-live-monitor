use anyhow::Result;
use duckdb::Connection;

fn main() -> Result<()> {
    // 初始化 DuckDB
    let conn = Connection::open_in_memory()?;

    Ok(())
}
