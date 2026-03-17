//! Error Handling
//! Custom error types and conversions

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Model error: {0}")]
    ModelError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ModelError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ApiError(msg) => (StatusCode::BAD_GATEWAY, msg),
            AppError::CacheError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded".to_string()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::ConfigError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }));

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<redis::RedisError> for AppError {
    fn from(err: redis::RedisError) -> Self {
        AppError::CacheError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ApiError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(err: tokio::task::JoinError) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl From<candle_core::Error> for AppError {
    fn from(err: candle_core::Error) -> Self {
        AppError::ModelError(err.to_string())
    }
}

impl From<tokenizers::Error> for AppError {
    fn from(err: tokenizers::Error) -> Self {
        AppError::ModelError(err.to_string())
    }
}

/// Result type alias with AppError
pub type AppResult<T> = Result<T, AppError>;

/// Helper function to handle not found
pub fn not_found(resource: &str) -> AppError {
    AppError::NotFound(format!("{} not found", resource))
}

/// Helper function to handle bad request
pub fn bad_request(message: &str) -> AppError {
    AppError::ValidationError(message.to_string())
}

/// Helper function to handle unauthorized
pub fn unauthorized(message: &str) -> AppError {
    AppError::Unauthorized(message.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let err = anyhow::anyhow!("test error");
        let app_err: AppError = err.into();
        assert!(matches!(app_err, AppError::InternalServerError(_)));
    }

    #[test]
    fn test_error_response() {
        let err = AppError::NotFound("model".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
