use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Rate Limited")]
    RateLimited,

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg.as_str()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg.as_str()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg.as_str()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg.as_str()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg.as_str()),
            AppError::ValidationError(msg) => (StatusCode::UNPROCESSABLE_ENTITY, "validation_error", msg.as_str()),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "rate_limited", "Too many requests"),
            AppError::InternalError(msg) => {
                error!("Internal Server Error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal Server Error")
            },
            AppError::DatabaseError(msg) => {
                error!("Database Error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", "Internal Server Error")
            },
        };

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
