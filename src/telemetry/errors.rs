use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Internal server error")]
    Internal,
}

impl axum::response::IntoResponse for TelemetryError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::Validation(msg) => (axum::http::StatusCode::BAD_REQUEST, msg),
            Self::Storage(msg) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg),
            Self::Internal => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = axum::Json(serde_json::json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
