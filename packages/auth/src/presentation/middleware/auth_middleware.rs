use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use rust_reborn_contracts::AppError;

use crate::AuthState;

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Ambil Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("Missing Authorization header"))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized(
            "Invalid Authorization header format",
        ));
    }

    // Ekstrak token
    let token = &auth_header[7..];

    let user_id = state
        .auth_service
        .verify_token(token)
        .await
        .map_err(|_| AppError::unauthorized("Invalid or expired token"))?;

    request.extensions_mut().insert(user_id);

    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];

            if let Ok(user_id) = state.auth_service.verify_token(token).await {
                // Jika valid, simpan user_id
                request.extensions_mut().insert(user_id);
            }
        }
    }

    next.run(request).await
}
