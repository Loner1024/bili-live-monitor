use crate::error::AppError;
use model::statistics;
use parse::{BlockUserMessage, Message};
use serde::{Deserialize, Serialize};
use utils::utils::MessageType;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: isize,
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct QueryRequest {
    pub timestamp: i64,
    pub username: Option<String>,
    pub message: Option<String>,
    pub message_type: Option<String>,
    pub limit: usize,
    pub offset: usize,
}

#[derive(Serialize, Debug)]
pub struct QueryResponse {
    pub code: isize,
    pub message: String,
    pub count: usize,
    pub data: Vec<QueryResponseData>,
}

#[derive(Serialize, Debug)]
pub struct QueryResponseData {
    pub uid: u64,
    pub username: String,
    pub message: String,
    pub message_type: String,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worth: Option<f64>,
}

pub fn message_vec_to_query_response_data_vec(
    message_vec: Vec<Message>,
) -> Result<Vec<QueryResponseData>, AppError> {
    let mut result = vec![];
    for message in message_vec {
        result.push(message.try_into()?)
    }
    Ok(result)
}

impl TryFrom<Message> for QueryResponseData {
    type Error = AppError;
    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Danmu(message) => Ok(QueryResponseData {
                uid: message.uid,
                username: message.username.clone(),
                message: message.msg.clone(),
                message_type: MessageType::Danmu.to_string(),
                timestamp: message.timestamp as i64,
                worth: None,
            }),
            Message::SuperChat(message) => Ok(QueryResponseData {
                uid: message.uid,
                username: message.username.clone(),
                message: message.msg.clone(),
                message_type: MessageType::SuperChat.to_string(),
                timestamp: message.timestamp as i64,
                worth: Some(message.worth),
            }),
            _ => Err(AppError::QueryError),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CheckerRequest {
    pub timestamp: i64,
    pub uid: u64,
}

#[derive(Serialize, Debug)]
pub struct CheckerResponse {
    pub code: isize,
    pub message: String,
    pub data: Vec<CheckerResponseData>,
}

#[derive(Serialize, Debug)]
pub struct CheckerResponseData {
    pub uid: u64,
    pub username: String,
    pub message: String,
    pub message_type: String,
    pub room_id: i64,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worth: Option<f64>,
}

pub fn message_to_checker_response_date(
    room_id: i64,
    message: &Message,
) -> Result<CheckerResponseData, AppError> {
    match message {
        Message::Danmu(message) => Ok(CheckerResponseData {
            uid: message.uid,
            username: message.username.clone(),
            message: message.msg.clone(),
            message_type: MessageType::Danmu.to_string(),
            room_id,
            timestamp: message.timestamp as i64,
            worth: None,
        }),
        Message::SuperChat(message) => Ok(CheckerResponseData {
            uid: message.uid,
            username: message.username.clone(),
            message: message.msg.clone(),
            message_type: MessageType::SuperChat.to_string(),
            room_id,
            timestamp: message.timestamp as i64,
            worth: Some(message.worth),
        }),
        _ => Err(AppError::QueryError),
    }
}

#[derive(Deserialize, Debug)]
pub struct QueryStatisticsRequest {
    pub timestamp: i64,
    pub room_id: i64,
}

#[derive(Serialize, Debug)]
pub struct QueryStatisticsResponse {
    pub code: isize,
    pub message: String,
    pub data: QueryStatisticsData,
}

#[derive(Serialize, Debug, Clone)]
pub struct QueryStatisticsData {
    pub today: statistics::StatisticsResult,
    pub yesterday: statistics::StatisticsResult,
}

#[derive(Deserialize, Debug)]
pub struct QueryBlockUserRequest {
    pub limit: usize,
    pub offset: usize,
}

#[derive(Serialize, Debug, Clone)]
pub struct QueryBlockerResponse {
    pub code: isize,
    pub message: String,
    pub count: isize,
    pub data: Vec<QueryBlockerResponseData>,
}

#[derive(Serialize, Debug, Clone)]
pub struct QueryBlockerResponseData {
    pub uid: u64,
    pub username: String,
    pub room_id: u64,
    pub operator: i16,
    pub timestamp: i64,
}

impl From<&BlockUserMessage> for QueryBlockerResponseData {
    fn from(value: &BlockUserMessage) -> Self {
        Self {
            uid: value.uid,
            username: value.username.clone(),
            room_id: value.room_id,
            operator: i16::from(value.clone().operator),
            timestamp: value.timestamp,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DanmuStatisticsRequest {
    pub room_id: i64,
    pub start: i64,
    pub end: i64,
}

#[derive(Serialize, Debug, Clone)]
pub struct DanmuStatisticsResponse {
    pub code: isize,
    pub message: String,
    pub data: Vec<statistics::StatisticsResult>,
}
