use anyhow::{anyhow, Result};
use chrono::Utc;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Read;
use std::str;

#[derive(Debug, Clone)]
pub struct DanmuMessage {
    pub uid: u64,
    pub username: String,
    pub msg: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct EnterRoomMessage {
    pub uid: u64,
    pub username: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct OnlineCountMessage {
    pub count: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct SuperChatMessage {
    pub uid: u64,
    pub username: String,
    pub msg: String,
    pub timestamp: String,
    pub worth: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Danmu(DanmuMessage),
    EnterRoom(EnterRoomMessage),
    OnlineCount(OnlineCountMessage),
    SuperChat(SuperChatMessage),
    Default,
}

impl TryFrom<&[u8]> for Message {
    type Error = anyhow::Error;

    fn try_from(data: &[u8]) -> std::result::Result<Self, Self::Error> {
        let s = str::from_utf8(data).map_err(|e| anyhow!("utf8 error: {}", e))?;
        let bili_message = serde_json::from_str::<BiliMessage>(&s)?;
        match bili_message.cmd.as_ref().unwrap().as_str() {
            "DANMU_MSG" => bili_message.get_danmu_mesage(),
            "INTERACT_WORD" => bili_message.get_enter_room(),
            "SUPER_CHAT_MESSAGE" => bili_message.get_super_chat(),
            "ONLINE_RANK_COUNT" => bili_message.get_online_count(),

            // ignore
            "WATCHED_CHANGE"
            | "ENTRY_EFFECT"
            | "DM_INTERACTION"
            | "WIDGET_BANNER"
            | "ONLINE_RANK_V2"
            | "NOTICE_MSG"
            | "LIKE_INFO_V3_CLICK"
            | "STOP_LIVE_ROOM_LIST"
            | "RECOMMEND_CARD"
            | "LIKE_INFO_V3_UPDATE" => Ok(Message::Default),

            _ => {
                debug!("Unsupported message: {}", s);
                Ok(Message::Default)
            }
        }
    }
}

fn parse_brotli_packet(_header: Header, packet: &[u8]) -> Result<Vec<Message>> {
    let mut result = Vec::new();
    let packet = brotli_decode(&packet[16..])?;

    let mut offset = 0;
    let mut chunks = Vec::new();
    loop {
        let header = parse_header(&packet[offset..offset + 16]);
        if offset + 16 > packet.len() || offset + header.total_size as usize > packet.len() {
            break;
        }
        let body = &packet[offset + 16..offset + header.total_size as usize];
        chunks.push((header.clone(), body));
        offset += header.total_size as usize;
        if offset >= packet.len() {
            break;
        }
    }

    for (header, body) in chunks {
        match Message::try_from(body) {
            Ok(message) => result.push(message),
            Err(e) => {
                error!(
                    "Failed to parse message: {}, header: {:?},",
                    e,
                    header.clone()
                );
            }
        }
    }
    Ok(result)
}

fn parse_command_packet(packet: &[u8]) -> Result<Message> {
    Message::try_from(&packet[16..])
}

pub fn parse_message(header: Header, origin_data: &[u8]) -> Result<Vec<Message>> {
    // debug!("{:?}", header);
    // for i in 0..origin_data.len() {
    //     print!("{:02X}", origin_data[i])
    // }
    // println!();

    // 3 is heartbeat packet
    if header.msg_type == 3 {
        return Ok(vec![]);
    }
    return match header.protocol {
        1 | 0 => Ok(vec![parse_command_packet(origin_data)?]),
        3 => parse_brotli_packet(header, origin_data),
        _ => Err(anyhow!("Unsupported protocol")),
    };
}

fn brotli_decode(data: &[u8]) -> Result<Vec<u8>> {
    let mut reader = brotli::Decompressor::new(data, 4096);

    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    Ok(buf)
}

#[derive(Debug, Clone)]
pub struct Header {
    pub total_size: u32,
    pub head_size: usize,
    pub protocol: u16,
    pub msg_type: u32,
    pub seq_id: u32,
}

pub fn parse_header(data: &[u8]) -> Header {
    let total_size = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
    let head_size = u16::from_be_bytes([data[4], data[5]]) as usize;
    let protocol = u16::from_be_bytes([data[6], data[7]]);
    let msg_type = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);
    let seq_id = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
    Header {
        total_size,
        head_size,
        protocol,
        msg_type,
        seq_id,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub uid: u64, // 此处 UID 或许可以填 0, B 站游客可看到弹幕流
    pub roomid: u64,
    pub protover: i32,
    pub buvid: String,    // cookies 中的 buvid
    pub platform: String, // web
    pub r#type: i32,      // 2
    pub key: String, // 从 https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo?id={room_id}&type=0 获取
}

pub fn build_auth_packet(certificate: &Certificate) -> Vec<u8> {
    let body = serde_json::to_vec(certificate).unwrap();
    build_packet(1, 7, &body)
}

pub fn build_hearbeat_packet() -> Vec<u8> {
    build_packet(1, 2, &[])
}

pub fn build_packet(protocol: u16, msg_type: u32, body: &[u8]) -> Vec<u8> {
    let total_size = body.len() as u32 + 16;
    let mut packet = Vec::with_capacity(total_size as usize);
    packet.extend_from_slice(&total_size.to_be_bytes()); // total size
    packet.extend_from_slice(&(16 as u16).to_be_bytes()); // head size
    packet.extend_from_slice(&protocol.to_be_bytes()); // protocol
    packet.extend_from_slice(&msg_type.to_be_bytes()); // msg type
    packet.extend_from_slice(&(1 as u32).to_be_bytes()); // seq id
    packet.extend_from_slice(body); // body
    packet
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliMessage {
    pub cmd: Option<String>,
    pub dm_v2: Option<String>,
    pub info: Option<Vec<Value>>,
    pub data: Option<BiliMessageData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiliMessageData {
    pub data: Option<String>,
    pub dmscore: Option<i64>,
    pub id: Option<i64>,
    pub status: Option<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub uinfo: Option<Uinfo>,
    pub timestamp: Option<u64>,
    pub online_count: Option<u64>,
    pub send_time: Option<u64>,
    pub message: Option<String>,
    pub price: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uinfo {
    pub uid: u64,
    pub base: UinfoBase,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UinfoBase {
    pub name: String,
}

impl BiliMessage {
    fn get_danmu_mesage(self) -> Result<Message> {
        if self.cmd != Some("DANMU_MSG".to_string()) {
            return Err(anyhow!("Not a Danmu message"));
        }
        let danmu = self.info.as_ref().ok_or(anyhow!("Failed to get info"))?;
        let uid = danmu[2][0].as_u64().ok_or(anyhow!("Failed to get uid"))?;
        let username = danmu[2][1]
            .as_str()
            .ok_or(anyhow!("Failed to get username"))?;
        let msg = danmu[1].as_str().ok_or(anyhow!("Failed to get msg"))?;
        let timestamp = danmu[0][4]
            .as_u64()
            .ok_or(anyhow!("Failed to get timestamp"))?;
        Ok(Message::Danmu(DanmuMessage {
            uid,
            username: username.to_string(),
            msg: msg.to_string(),
            timestamp,
        }))
    }

    fn get_enter_room(self) -> Result<Message> {
        if self.cmd != Some("INTERACT_WORD".to_string()) {
            return Err(anyhow!("Not a enter room message"));
        }
        let data = self.data.ok_or(anyhow!("Failed to get data"))?;
        let user_info = data.uinfo.ok_or(anyhow!("Failed to get uinfo"))?;
        let timestamp = data.timestamp.ok_or(anyhow!("Failed to get timestamp"))?;

        Ok(Message::EnterRoom(EnterRoomMessage {
            uid: user_info.uid,
            username: user_info.base.name,
            timestamp,
        }))
    }

    fn get_online_count(self) -> Result<Message> {
        if self.cmd != Some("ONLINE_RANK_COUNT".to_string()) {
            return Err(anyhow!("Not a online count message"));
        }
        let data = self.data.ok_or(anyhow!("Failed to get data"))?;
        Ok(Message::OnlineCount(OnlineCountMessage {
            count: data
                .online_count
                .ok_or(anyhow!("Failed to get online count"))?,
            timestamp: Utc::now().timestamp_millis() as u64,
        }))
    }

    fn get_super_chat(self) -> Result<Message> {
        if self.cmd != Some("SUPER_CHAT_MESSAGE".to_string()) {
            return Err(anyhow!("Not a super chat message"));
        }
        let data = self.data.ok_or(anyhow!("Failed to get data"))?;
        let user_info = data.uinfo.ok_or(anyhow!("Failed to get uinfo"))?;
        Ok(Message::SuperChat(SuperChatMessage {
            uid: user_info.uid,
            username: user_info.base.name,
            msg: data.message.ok_or(anyhow!("Failed to get data"))?,
            timestamp: data
                .send_time
                .ok_or(anyhow!("Failed to get send_time"))?
                .to_string(),
            worth: data.price.ok_or(anyhow!("Failed to get worth"))? as u32,
        }))
    }
}
