use crate::model::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("unknown error")]
    Unknown,
    #[error("param error: {0}")]
    ParamError(String),
    #[error("query error")]
    QueryError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            _ => (StatusCode::OK, self.to_string()),
        };

        let body = Json(ErrorResponse {
            code: -1,
            message: error_message.to_string(),
        });

        (status, body).into_response()
    }
}
