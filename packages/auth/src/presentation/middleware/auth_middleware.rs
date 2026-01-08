use std::sync::Arc;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use rust_reborn_contracts::AppError;
use crate::application::auth_context::AuthContext;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::request_auth_context::RequestAuthContext;

pub async fn auth_middleware(
    State(jwt): State<Arc<JwtService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("missing Authorization header"))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| AppError::unauthorized("invalid Authorization header"))?;

    let user_id = jwt
        .verify_token(token)
        .map_err(|_| AppError::unauthorized("invalid or expired token"))?;

    let ctx: Arc<dyn AuthContext> =
        Arc::new(RequestAuthContext::authenticated(user_id));

    request.extensions_mut().insert(ctx);

    Ok(next.run(request).await)
}