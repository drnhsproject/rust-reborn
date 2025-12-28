use crate::application::dto::{LoginRequest, RegisterRequest};
use crate::AuthState;
use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use rust_reborn_core::{utils::response::created, validation::validate, AppError, Result};

pub async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    validate(&payload)?;

    // Call use case
    let response = state.auth_service.register(payload).await?;

    Ok(created(response))
}

pub async fn login(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    validate(&payload)?;

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
        .ok_or_else(|| AppError::unauthorized("Missing Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized("Invalid Authorization header"));
    }

    let token = &auth_header[7..];
    let user_id = state.auth_service.verify_token(token).await?;

    let user = state.auth_service.get_user_by_id(user_id).await?;

    Ok(Json(user))
}

pub async fn logout() -> Result<impl IntoResponse> {
    // In a stateless JWT system, logout is typically handled client-side
    // by removing the token from storage.
    // For server-side logout, you'd need to implement token blacklisting.
    Ok(Json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}
