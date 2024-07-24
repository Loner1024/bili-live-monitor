use anyhow::{anyhow, Result};
use base64::Engine;
use chrono::Utc;
use crawler::storage::Storage;
use danmu_client::danmu::Client;
use duckdb::Connection;
use log::{debug, error, info};
use parse::Message;
use std::process::exit;
use std::time::Duration;
use tokio::signal;
use tokio::sync::watch;
use tokio::task::LocalSet;
use tokio::time::sleep;
use utils::utils::{get_rooms, is_new_day};

#[tokio::main]
async fn main() -> Result<()> {
    // dotenv::dotenv().ok();
    pretty_env_logger::init_timed();
    let cookies = String::from_utf8(
        base64::engine::general_purpose::STANDARD.decode(std::env::var("BILI_COOKIE")?)?,
    )?;
    // let cookies = std::env::var("BILI_COOKIE")?;
    debug!("{}", cookies);

    let room_ids = get_rooms(); // 传入多个 room_id
    info!("获取到 {} 个 room {:?}", room_ids.len(), room_ids);

    let local_set = LocalSet::new();

    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let main_shutdown_tx = shutdown_tx.clone();

    let shutdown_signal = tokio::spawn(async move {
        let sigint = signal::ctrl_c();
        let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;

        tokio::select! {
            _ = sigint => {
                info!("Received SIGINT, shutting down...");
            },
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down...");
            },
        }
        let _ = main_shutdown_tx.send(());
        Result::<(), anyhow::Error>::Ok(())
    });
    info!("启动监听信号");

    local_set
        .run_until(async move {
            let mut tasks = Vec::new();

            for room_id in room_ids {
                info!("开始启动 room_id: {}", room_id);
                let cookies = cookies.clone();
                let conn = match Connection::open_in_memory() {
                    Ok(conn) => conn,
                    Err(e) => {
                        error!("Error opening connection: {:?}", e);
                        return;
                    }
                };
                let shutdown_rx = shutdown_rx.clone();
                let err_shutdown_tx = shutdown_tx.clone();
                let task = tokio::task::spawn_local(async move {
                    if let Err(e) = process_room(room_id, cookies, conn, shutdown_rx).await {
                        error!("Error processing room {}: {:?}", room_id, e);
                        let _ = err_shutdown_tx.send(());
                        sleep(Duration::from_secs(30)).await;
                        exit(0);
                    }
                });
                tasks.push(task);
            }

            // 等待所有任务完成
            futures::future::join_all(tasks).await;
        })
        .await;

    // 等待终止信号任务完成
    shutdown_signal.await??;

    Ok(())
}

async fn process_room(
    room_id: i64,
    cookies: String,
    conn: Connection,
    mut shutdown_rx: watch::Receiver<()>,
) -> Result<()> {
    let mut start_time = Utc::now();

    // 创建 Storage 实例
    let mut storage = Storage::new(&conn, room_id, start_time.timestamp())?;

    let client = Client::new(room_id as u64, &cookies)?;
    let mut rx = client.listen().await?;

    info!("开始监听 room_id: {}", room_id);

    loop {
        tokio::select! {
            message = rx.recv() => {
                if let Some(message) = message {
                    let now = Utc::now();
                    if is_new_day(start_time.timestamp(), now.timestamp())? {
                        start_time = now;
                        storage.switch_new_date(now.timestamp())?;
                        info!("切换到新的日期: {}", now);
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
            },
            _ = shutdown_rx.changed() => {
                // 收到 shutdown 信号，退出循环
                info!("收到 shutdown 信号，停止监听 room_id: {}", room_id);
                break;
            },
        }
    }

    // 执行清理工作
    info!("开始清理 room_id: {}", room_id);
    storage
        .flush()
        .map_err(|e| anyhow!("清理 room {} 出错: {}", room_id, e))?;
    info!("清理 room {} 完成, Bye!", room_id);

    Ok(())
}
