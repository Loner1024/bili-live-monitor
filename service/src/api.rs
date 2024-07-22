use crate::error::AppError;
use crate::model::{
    message_to_checker_response_date, message_vec_to_query_response_data_vec, CheckerRequest,
    CheckerResponse, QueryRequest, QueryResponse, QueryStatisticsData, QueryStatisticsRequest,
    QueryStatisticsResponse,
};
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::Duration;
use queryer::Queryer;
use std::sync::Arc;
use tracing::{info, debug};
use utils::utils::{get_rooms, MessageType, Pagination};

pub async fn query(
    State(storage): State<Arc<Queryer>>,
    room_id: Result<Path<i64>, PathRejection>,
    req: Result<Query<QueryRequest>, QueryRejection>,
) -> Result<Json<QueryResponse>, AppError> {
    let room_id = match room_id {
        Ok(id) => id.0,
        Err(e) => {
            info!("parse room id error: {}", e);
            return Err(AppError::ParamError(format!("parse room id error: {}", e)));
        }
    };

    let req = extract_req(req)?;
    let message_type: MessageType = req.message_type.into();
    let count = match storage.query_count(
        room_id,
        req.timestamp,
        Some(message_type),
        None,
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
        None,
        req.username,
        req.message,
        Some(Pagination {
            limit: req.limit,
            offset: req.offset,
        }),
    ) {
        Ok(res) => message_vec_to_query_response_data_vec(res)?,
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

pub async fn checker(
    State(storage): State<Arc<Queryer>>,
    req: Result<Query<CheckerRequest>, QueryRejection>,
) -> Result<Json<CheckerResponse>, AppError> {
    let req = extract_req(req)?;
    let mut result = vec![];
    for room in get_rooms() {
        match storage.query(room, req.timestamp, None, Some(req.uid), None, None, None) {
            Ok(data) => {
                for message in data {
                    result.push(message_to_checker_response_date(room, &message)?);
                }
            }
            Err(e) => {
                info!("query from db error: {}", e);
                return Err(AppError::QueryError);
            }
        };
    }

    Ok(Json(CheckerResponse {
        code: 0,
        message: "success".to_string(),
        data: result,
    }))
}

pub async fn query_statistics(
    State(storage): State<Arc<Queryer>>,
    req: Result<Query<QueryStatisticsRequest>, QueryRejection>,
) -> Result<Json<QueryStatisticsResponse>, AppError> {
    let req = extract_req(req)?;
    let today = match storage.query_statistics_data(req.room_id, req.timestamp) {
        Ok(data) => data,
        Err(e) => {
            info!("query from db error: {}", e);
            return Err(AppError::QueryError);
        }
    };
    let yesterday = match storage
        .query_statistics_data(req.room_id, req.timestamp - Duration::days(1).num_seconds())
    {
        Ok(data) => data,
        Err(e) => {
            info!("query from db error: {}", e);
            return Err(AppError::QueryError);
        }
    };
    debug!("today: {:?}, yesterday: {:?}", today, yesterday);
    Ok(Json(QueryStatisticsResponse {
        code: 0,
        message: "success".to_string(),
        data: QueryStatisticsData { today, yesterday },
    }))
}

fn extract_req<T>(req: Result<Query<T>, QueryRejection>) -> Result<T, AppError> {
    match req {
        Ok(req) => Ok(req.0),
        Err(e) => {
            info!("parse query request error: {}", e);
            Err(AppError::ParamError(format!(
                "parse query request error: {}",
                e
            )))
        }
    }
}
