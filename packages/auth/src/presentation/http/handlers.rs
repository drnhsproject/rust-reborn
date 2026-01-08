use crate::application::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::AuthState;
use axum::{extract::State, http::HeaderMap, response::IntoResponse, Json};
use rust_reborn_contracts::{
    common::response::created_with_message, common::success_with_message, AppError, Result,
};
use validator::Validate;

pub async fn register(
    State(state): State<AuthState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse> {
    payload.validate()?;

    let response = state.register_user_use_case.execute(payload).await?;

    Ok(created_with_message(
        response,
        "your account registered successfully",
    ))
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "login success", body = AuthResponse),
        (status = 400, description = "Validation error")
    ),
    tag = "Authentication"
)]
pub async fn login(
    State(state): State<AuthState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse> {
    payload.validate()?;

    let response = state.login_user_use_case.execute(payload).await?;

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
    let user_id = state.verify_token_use_case.execute(token).await?;

    let user = state.get_user_detail_use_case.execute(user_id).await?;

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
