use anyhow::Result;
use chrono::Utc;
use crawler::storage::Storage;
use danmu_client::danmu::Client;
use duckdb::Connection;
use log::{debug, error, info};
use parse::Message;
use tokio::task::LocalSet;
use utils::utils::is_new_day;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let cookies = std::env::var("BILI_COOKIE")?;
    debug!("{}", cookies);

    let room_ids = vec![22747736, 21533102, 23649609]; // 传入多个 room_id

    let local_set = LocalSet::new();

    local_set
        .run_until(async move {
            let mut tasks = Vec::new();

            for room_id in room_ids {
                let cookies = cookies.clone();
                let conn = match Connection::open_in_memory() {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("Error opening connection: {:?}", e);
                        return;
                    }
                };
                let task = tokio::task::spawn_local(async move {
                    if let Err(e) = process_room(room_id, cookies, conn).await {
                        error!("Error processing room {}: {:?}", room_id, e);
                    }
                });
                tasks.push(task);
            }

            futures::future::join_all(tasks).await;
        })
        .await;

    Ok(())
}

async fn process_room(room_id: i64, cookies: String, conn: Connection) -> Result<()> {
    let mut start_time = Utc::now();

    // 创建 Storage 实例
    let mut storage = Storage::new(&conn, room_id, start_time.timestamp())?;

    let client = Client::new(room_id as u64, &cookies)?;
    let mut rx = client.listen().await?;

    info!("开始监听 room_id: {}", room_id);

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
                storage.create_super_chat_message(msg)?;
            }
            Message::Default => {}
        }
    }

    Ok(())
}
