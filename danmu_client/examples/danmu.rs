use chrono::{DateTime, Local};
use danmu_client::danmu::Client;
use dotenv::dotenv;
use owo_colors::OwoColorize;
use parse::Message;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();
    let cookies = std::env::var("BILI_COOKIE").unwrap();

    let roomid = 22747736;

    let client = Client::new(roomid, &cookies)?;
    let mut rx = client.listen().await?;

    while let Some(message) = rx.recv().await {
        print_danmu(message);
    }

    Ok(())
}

fn print_danmu(message: Message) {
    match message {
        Message::Danmu(message) => {
            let datetime_local = timestamp_to_local_time(message.timestamp);
            println!(
                "[{}] - {}: {}",
                datetime_local.format("%H:%M:%S").bright_yellow(),
                message.username.purple(),
                message.msg.green()
            );
        }
        Message::EnterRoom(enter_room) => {
            let datetime_local = timestamp_to_local_time(enter_room.timestamp * 1000);
            println!(
                "[{}] - {} 进入房间",
                datetime_local.format("%H:%M:%S").bright_yellow(),
                enter_room.username.purple()
            );
        }
        Message::OnlineCount(online_count) => {
            let datetime_local = timestamp_to_local_time(online_count.timestamp);
            println!(
                "[{}] - 当前在线人数: {}",
                datetime_local.format("%H:%M:%S").bright_yellow(),
                online_count.count
            );
        }
        _ => {}
    }
}

fn timestamp_to_local_time(timestamp: u64) -> DateTime<Local> {
    let datetime_local;
    if let Some(datetime_utc) = DateTime::from_timestamp_millis(timestamp as i64) {
        datetime_local = datetime_utc.with_timezone(&Local);
    } else {
        datetime_local = Local::now();
    }
    datetime_local
}
