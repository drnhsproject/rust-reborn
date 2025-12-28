use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use rust_reborn_contracts::AppError;

use crate::AuthState;

/// Middleware untuk memverifikasi JWT token
/// Middleware ini akan memeriksa Authorization header dan memverifikasi token
/// Jika valid, user_id akan disimpan di request extensions
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

    // Pastikan format Bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::unauthorized(
            "Invalid Authorization header format",
        ));
    }

    // Ekstrak token
    let token = &auth_header[7..];

    // Verifikasi token dan dapatkan user_id
    let user_id = state
        .auth_service
        .verify_token(token)
        .await
        .map_err(|_| AppError::unauthorized("Invalid or expired token"))?;

    // Simpan user_id di request extensions agar bisa diakses di handler
    request.extensions_mut().insert(user_id);

    // Lanjutkan ke handler berikutnya
    Ok(next.run(request).await)
}

/// Optional auth middleware - tidak akan error jika token tidak ada
/// Berguna untuk endpoint yang bisa diakses dengan atau tanpa login
pub async fn optional_auth_middleware(
    State(state): State<AuthState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Coba ambil Authorization header
    if let Some(auth_header) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        // Jika ada dan formatnya benar
        if auth_header.starts_with("Bearer ") {
            let token = &auth_header[7..];

            // Coba verifikasi token
            if let Ok(user_id) = state.auth_service.verify_token(token).await {
                // Jika valid, simpan user_id
                request.extensions_mut().insert(user_id);
            }
        }
    }

    // Lanjutkan ke handler (dengan atau tanpa user_id)
    next.run(request).await
}
