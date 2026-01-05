use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub message: Option<String>,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { 
            success: true,
            message: None,
            data 
        }
    }

    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

pub fn ok<T: Serialize>(data: T) -> impl IntoResponse {
    ApiResponse::ok(data)
}

pub fn created<T: Serialize>(data: T) -> impl IntoResponse {
    (StatusCode::CREATED, Json(ApiResponse::ok(data)))
}

pub fn created_with_message<T: Serialize>(
    data: T,
    msg: impl Into<String>,
) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(ApiResponse::ok(data).with_message(msg)),
    )
}

pub fn success_with_message<T: Serialize>(
    data: T,
    msg: impl Into<String>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::ok(data).with_message(msg)),
    )
}

/// Return 204 NO CONTENT
pub fn no_content() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

// ============================================
// Message Response (for simple messages)
// ============================================

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Return message response
pub fn message(msg: impl Into<String>) -> impl IntoResponse {
    ok(MessageResponse::new(msg))
}
