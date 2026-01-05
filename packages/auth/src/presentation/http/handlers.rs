use crate::application::dto::{LoginRequest, RegisterRequest};
use crate::AuthState;
use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use rust_reborn_contracts::{
    common::response::created_with_message, 
    common::success_with_message, 
    AppError, 
    Result
};
use validator::Validate;

pub async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    payload.validate()?;

    // Call use case
    let response = state.auth_service.register(payload).await?;

    Ok(created_with_message(
        response,
        "your account registered successfully",
    ))
}

pub async fn login(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    payload.validate()?;

    // Call use case
    let response = state.auth_service.login(payload).await?;

    Ok(Json(response))
}

pub async fn get_current_user(
    State(state): State<AuthState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("missing Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized("invalid Authorization header"));
    }

    let token = &auth_header[7..];
    let user_id = state.auth_service.verify_token(token).await?;

    let user = state.auth_service.get_user_by_id(user_id).await?;

    Ok(success_with_message(
        user,
        "current user fetched successfully",
    ))
}

pub async fn logout() -> Result<impl IntoResponse> {
    // In a stateless JWT system, logout is typically handled client-side
    // by removing the token from storage.
    // For server-side logout, you'd need to implement token blacklisting.
    Ok(Json(serde_json::json!({
        "message": "logged out successfully"
    })))
}
