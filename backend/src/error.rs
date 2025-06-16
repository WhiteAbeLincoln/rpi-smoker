use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono;
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[cfg(all(feature = "rpi-hardware", target_os = "linux"))]
    #[error("Hardware error: {0}")]
    Hardware(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

impl AppError {
    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Create a new bad request error
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    /// Create a new not found error
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Create a new validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::Config(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Configuration error: {}", msg),
            ),
            AppError::Io(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
            AppError::Json(err) => (StatusCode::BAD_REQUEST, format!("JSON error: {}", err)),
            #[cfg(all(feature = "rpi-hardware", target_os = "linux"))]
            AppError::Hardware(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Hardware error: {}", err),
            ),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

        (status, body).into_response()
    }
}
