use crate::error::AppError;
use crate::model::{
    message_to_checker_response_date, message_vec_to_query_response_data_vec, CheckerRequest,
    CheckerResponse, QueryBlockUserRequest, QueryBlockerResponse, QueryRequest, QueryResponse,
    QueryStatisticsData, QueryStatisticsRequest, QueryStatisticsResponse,
};
use crate::AppState;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::Json;
use chrono::Duration;
use tracing::{debug, info};
use utils::utils::{get_local_midnight, get_rooms, MessageType, Pagination};

pub async fn query(
    State(state): State<AppState>,
    room_id: Result<Path<i64>, PathRejection>,
    req: Result<Query<QueryRequest>, QueryRejection>,
) -> Result<Json<QueryResponse>, AppError> {
    let storage = state.queryer;
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
    State(state): State<AppState>,
    req: Result<Query<CheckerRequest>, QueryRejection>,
) -> Result<Json<CheckerResponse>, AppError> {
    let storage = state.queryer;
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
    result.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    Ok(Json(CheckerResponse {
        code: 0,
        message: "success".to_string(),
        data: result,
    }))
}

pub async fn query_statistics(
    State(state): State<AppState>,
    req: Result<Query<QueryStatisticsRequest>, QueryRejection>,
) -> Result<Json<QueryStatisticsResponse>, AppError> {
    let (storage, cache) = (state.queryer, state.statistics_cache);
    let req = extract_req(req)?;

    let data = match cache
        .get(&(
            req.room_id,
            get_local_midnight(req.timestamp).map_err(|_| AppError::QueryError)?,
        ))
        .await
    {
        Some(data) => data,
        None => {
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
            let data = QueryStatisticsData { today, yesterday };
            cache
                .insert(
                    (
                        req.room_id,
                        get_local_midnight(req.timestamp).map_err(|_| AppError::QueryError)?,
                    ),
                    data.clone(),
                )
                .await;
            data
        }
    };

    debug!("today: {:?}, yesterday: {:?}", data.today, data.yesterday);
    Ok(Json(QueryStatisticsResponse {
        code: 0,
        message: "success".to_string(),
        data,
    }))
}

pub async fn query_block_user(
    State(state): State<AppState>,
    req: Result<Query<QueryBlockUserRequest>, QueryRejection>,
) -> Result<Json<QueryBlockerResponse>, AppError> {
    let params = extract_req(req)?;

    let response = match state
        .block_user_cache
        .get(&(params.limit, params.offset))
        .await
    {
        Some(resp) => resp,
        None => {
            let count = match state.queryer.query_block_user_count() {
                Ok(count) => count,
                Err(e) => {
                    info!("query from db error: {}", e);
                    return Err(AppError::QueryError);
                }
            };
            let result = match state.queryer.query_block_user_data(Some(Pagination {
                limit: params.limit,
                offset: params.offset,
            })) {
                Ok(data) => data.iter().map(|x| x.into()).collect(),
                Err(e) => {
                    info!("query from db error: {}", e);
                    return Err(AppError::QueryError);
                }
            };
            let response = QueryBlockerResponse {
                code: 0,
                message: "success".to_string(),
                count: count as isize,
                data: result,
            };
            state
                .block_user_cache
                .insert((params.limit, params.offset), response.clone())
                .await;
            response
        }
    };
    Ok(Json(response))
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
