use anyhow::Result;
use chrono::{Duration, Local};
use crawler::storage::Storage;
use danmu_client::danmu::Clinet;
use dotenv::dotenv;
use duckdb::Connection;
use log::info;
use parse::Message;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    // 初始化 DuckDB
    let conn = Connection::open_in_memory()?;
    let mut current_time = Local::now().date_naive();
    let mut new_day = current_time + Duration::days(1);
    // 创建 Storage 实例
    let mut storage = Storage::new(&conn, current_time)?;

    let cookies = std::env::var("BILI_COOKIE")?;

    let roomid = 22747736;

    let client = Clinet::new(roomid, &cookies)?;
    let mut rx = client.listen().await?;

    info!("开始监听弹幕");

    while let Some(message) = rx.recv().await {
        current_time = Local::now().date_naive();
        if current_time == new_day {
            new_day = current_time + Duration::days(1);
            storage.flush()?;
            storage = Storage::new(&conn, current_time)?;
        }
        match message {
            Message::Danmu(msg) => {
                storage.create_danmu_message(msg)?;
            }
            Message::EnterRoom(_) => {}
            Message::OnlineCount(_) => {}
            Message::SuperChat(msg) => {
                storage.crate_super_chat_message(msg)?;
            }
            Message::Default => {}
        }
    }

    Ok(())
}
