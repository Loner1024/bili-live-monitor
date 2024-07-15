use crate::error::AppError;
use crate::model::{QueryRequest, QueryResponse, QueryResponseData};
use axum::extract;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::Json;
use parse::Message;
use queryer::Query;
use std::sync::Arc;
use tracing::info;
use utils::utils::{MessageType, Pagination};

pub async fn query(
    extract::State(storage): extract::State<Arc<Query>>,
    room_id: Result<extract::Path<i64>, PathRejection>,
    req: Result<extract::Query<QueryRequest>, QueryRejection>,
) -> Result<Json<QueryResponse>, AppError> {
    let room_id = match room_id {
        Ok(id) => id.0,
        Err(e) => {
            info!("parse room id error: {}", e);
            return Err(AppError::ParamError(format!("parse room id error: {}", e)));
        }
    };
    let req = match req {
        Ok(req) => req.0,
        Err(e) => {
            info!("parse query request error: {}", e);
            return Err(AppError::ParamError(format!(
                "parse query request error: {}",
                e
            )));
        }
    };
    let message_type: MessageType = req.message_type.into();
    let count = match storage.query_count(
        room_id,
        req.timestamp,
        Some(message_type),
        req.username.clone(),
        req.message.clone(),
    ) {
        Ok(count) => count,
        Err(e) => {
            info!("query from db error: {}", e);
            return Err(AppError::QueryError);
        }
    };
    if count == 0 {
        return Ok(Json(QueryResponse {
            code: 0,
            message: "success".to_string(),
            count: 0,
            data: vec![],
        }));
    };
    let query_result = match storage.query(
        room_id,
        req.timestamp,
        Some(message_type),
        req.username,
        req.message,
        Some(Pagination {
            limit: req.limit,
            offset: req.offset,
        }),
    ) {
        Ok(res) => res
            .iter()
            .map(|message| match message {
                Message::Danmu(message) => QueryResponseData {
                    uid: message.uid,
                    username: message.username.clone(),
                    message: message.msg.clone(),
                    message_type: MessageType::Danmu.to_string(),
                    timestamp: message.timestamp as i64,
                    worth: None,
                },
                Message::SuperChat(message) => QueryResponseData {
                    uid: message.uid,
                    username: message.username.clone(),
                    message: message.msg.clone(),
                    message_type: MessageType::Danmu.to_string(),
                    timestamp: message.timestamp as i64,
                    worth: Some(message.worth),
                },
                _ => todo!(),
            })
            .collect(),
        Err(e) => {
            info!("query from db error: {}", e);
            return Err(AppError::QueryError);
        }
    };

    Ok(Json(QueryResponse {
        code: 0,
        message: "success".to_string(),
        count,
        data: query_result,
    }))
}
