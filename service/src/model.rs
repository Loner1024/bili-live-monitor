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
}

#[derive(Serialize, Debug)]
pub struct QueryResponse {
    pub code: isize,
    pub message: String,
    pub data: Vec<QueryResponseData>,
}

#[derive(Serialize, Debug)]
pub struct QueryResponseData {
    pub uid: u64,
    pub username: String,
    pub message: String,
    pub message_type: String,
    pub timestamp: i64,
    pub worth: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::utils::MessageType;

    #[test]
    fn test_deserialize_query_request() {
        let req = r#"{
        "timestamp":11111,
        "username":"name",
        "message":"message"
    }"#;
        let data = serde_json::from_str::<QueryRequest>(req).unwrap();
        assert_eq!(data.timestamp, 11111);
        assert_eq!(data.username, Some("name".to_string()));
        assert_eq!(data.message, Some("message".to_string()));
        assert_eq!(
            MessageType::from(data.message_type.into()),
            MessageType::Danmu
        );
    }
}
