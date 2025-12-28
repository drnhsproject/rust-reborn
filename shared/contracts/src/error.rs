use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    ValidationError(validator::ValidationErrors),
    DatabaseError(String),
}

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self { Self::Internal(msg.into()) }
    pub fn not_found(msg: impl Into<String>) -> Self { Self::NotFound(msg.into()) }
    pub fn bad_request(msg: impl Into<String>) -> Self { Self::BadRequest(msg.into()) }
    pub fn unauthorized(msg: impl Into<String>) -> Self { Self::Unauthorized(msg.into()) }
    pub fn forbidden(msg: impl Into<String>) -> Self { Self::Forbidden(msg.into()) }
    pub fn conflict(msg: impl Into<String>) -> Self { Self::Conflict(msg.into()) }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            Self::ValidationError(e) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", e.to_string()),
            Self::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", msg),
        };

        (status, Json(ErrorResponse { error: error_type.to_string(), message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(err.to_string())
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::ValidationError(err)
    }
}
