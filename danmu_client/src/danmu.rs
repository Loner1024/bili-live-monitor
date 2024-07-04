use anyhow::Result;
use cookie::Cookie;
use log::{debug, error, info};
use parse::{parse_message, Message};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::str;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::time;

// const HOST: &str = "broadcastlv.chat.bilibili.com";
// const PORT: u16 = 2243;

#[derive(Debug)]
pub struct Clinet {
    pub room_id: u64,
    pub cookies: HeaderMap,
    pub uid: u64,
    pub buvid: String,
}

impl Clinet {
    pub fn new(room_id: u64, cookies: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", cookies.parse()?);
        let cookies = Cookie::split_parse(cookies.to_string());
        let mut uid = 0;
        let mut buvid = String::new();
        for cookie in cookies {
            let cookie = cookie?;
            match cookie.name() {
                "DedeUserID" => {
                    uid = cookie.value().parse()?;
                }
                "buvid3" => {
                    buvid = cookie.value().to_string();
                }
                _ => {}
            }
        }
        if uid == 0 || buvid.is_empty() {
            return Err(anyhow::anyhow!("Failed to parse uid or buvid"));
        }

        Ok(Self {
            room_id,
            cookies: headers,
            uid,
            buvid,
        })
    }

    pub async fn listen(&self) -> Result<Receiver<Message>> {
        let room_info = self.get_danmu_info().await?;

        let certificate = parse::Certificate {
            uid: self.uid,
            roomid: self.room_id,
            protover: 3,
            buvid: self.buvid.clone(),
            platform: "web".to_string(),
            r#type: 2,
            key: room_info.data.token,
        };

        let mut stream = None;
        for host in room_info.data.host_list {
            if let Ok(conn) = TcpStream::connect((host.host, host.port)).await {
                stream = Some(conn);
                break;
            }
        }
        let (mut reader, mut writer) = stream
            .ok_or(anyhow::anyhow!("Failed to connect to Danmu server"))?
            .into_split();

        let auth_packet = parse::build_auth_packet(&certificate);

        writer.write_all(&auth_packet).await?;
        info!("Auth packet sent");
        let mut buffer = [0; 1024];

        let n = reader.read(&mut buffer).await?; // read auth resp
        if n == 0 {
            error!("Failed to read auth resp");
            return Err(anyhow::anyhow!("Failed to read auth resp"));
        }
        info!("Auth resp: {:?}", str::from_utf8(&buffer[16..n])?);

        tokio::spawn(async move {
            loop {
                let heartbeat_packet = parse::build_hearbeat_packet();
                if let Err(e) = writer.write_all(&heartbeat_packet).await {
                    error!("Failed to send heartbeat packet: {}", e);
                } else {
                    debug!("Heartbeat packet sent");
                }
                time::sleep(Duration::from_secs(30)).await;
            }
        });

        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            loop {
                let mut header_buffer = [0; 16];
                match reader.read_exact(&mut header_buffer).await {
                    Ok(_) => (),
                    Err(e) => {
                        error!("Failed to read header: {}", e);
                        continue;
                    }
                };

                let header = parse::parse_header(&header_buffer);

                let mut buffer = vec![0; header.total_size as usize - 16];
                match reader.read_exact(&mut buffer).await {
                    Ok(_) => header.total_size as usize,
                    Err(e) => {
                        error!("Failed to read message: {}", e);
                        continue;
                    }
                };
                let mut packet = Vec::new();
                packet.extend_from_slice(&header_buffer);
                packet.extend_from_slice(&buffer);
                match parse_message(header, &packet) {
                    Ok(messages) => {
                        for msg in messages {
                            if let Err(e) = tx.send(msg).await {
                                error!("Failed to send message: {}", e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse message: {}", e);
                        continue;
                    }
                };
            }
        });

        // tokio::try_join!(heart_handle, read_handle)?;

        Ok(rx)
    }

    async fn get_danmu_info(&self) -> Result<GetKeyResponse> {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(self.cookies.clone())
            .build()?;
        let url = format!(
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id={}&type=0",
            self.room_id
        );
        let resp = client
            .get(&url)
            .send()
            .await?
            .json::<GetKeyResponse>()
            .await?;
        Ok(resp)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKeyResponse {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: GetKeyData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKeyData {
    pub group: String,
    #[serde(rename = "business_id")]
    pub business_id: i64,
    #[serde(rename = "refresh_row_factor")]
    pub refresh_row_factor: f64,
    #[serde(rename = "refresh_rate")]
    pub refresh_rate: i64,
    #[serde(rename = "max_delay")]
    pub max_delay: i64,
    pub token: String,
    #[serde(rename = "host_list")]
    pub host_list: Vec<HostList>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostList {
    pub host: String,
    pub port: u16,
    #[serde(rename = "wss_port")]
    pub wss_port: i64,
    #[serde(rename = "ws_port")]
    pub ws_port: i64,
}
