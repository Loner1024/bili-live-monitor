use anyhow::Result;
use chrono::Utc;
use crawler::storage::Storage;
use danmu_client::danmu::Clinet;
use dotenv::dotenv;
use duckdb::Connection;
use log::info;
use parse::Message;
use utils::utils::is_new_day;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init_timed();

    // 初始化 DuckDB
    let conn = Connection::open_in_memory()?;
    let mut start_time = Utc::now();
    let room_id = 22747736;
    // 创建 Storage 实例
    let mut storage = Storage::new(&conn, room_id, start_time.timestamp())?;

    let cookies = std::env::var("BILI_COOKIE")?;


    let client = Clinet::new(room_id as u64, &cookies)?;
    let mut rx = client.listen().await?;

    info!("开始监听弹幕");

    while let Some(message) = rx.recv().await {
        let now = Utc::now();
        if is_new_day(start_time.timestamp(), now.timestamp())? {
            start_time = now;
            storage.switch_new_date(now.timestamp())?;
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
