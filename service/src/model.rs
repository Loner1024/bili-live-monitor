use serde::{Deserialize, Serialize};

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
