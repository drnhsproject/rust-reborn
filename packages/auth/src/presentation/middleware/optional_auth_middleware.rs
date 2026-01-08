use crate::application::auth_context::AuthContext;
use crate::infrastructure::jwt::JwtService;
use crate::presentation::request_auth_context::RequestAuthContext;
use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn optional_auth_middleware(
    State(jwt): State<Arc<JwtService>>,
    mut request: Request,
    next: Next,
) -> Response {
    let ctx: Arc<dyn AuthContext> = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header_value| header_value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .and_then(|token| jwt.verify_token(token).ok())
        .map(RequestAuthContext::authenticated)
        .map(|ctx| Arc::new(ctx) as Arc<dyn AuthContext>)
        .unwrap_or_else(|| Arc::new(RequestAuthContext::anonymous()));

    request.extensions_mut().insert(ctx);

    next.run(request).await
}
