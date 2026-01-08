use axum::{middleware, Router};
use rust_reborn_auth::infrastructure::jwt::JwtService;
use rust_reborn_auth::{auth_middleware, AuthState};
use rust_reborn_core::{product_routes_handler, ProductRepository};
use std::sync::Arc;

pub fn auth_routes(state: AuthState) -> Router {
    rust_reborn_auth::auth_routes_handler(state)
}

pub fn product_routes(repo: Arc<dyn ProductRepository>, jwt: Arc<JwtService>) -> Router {
    product_routes_handler(repo).layer(middleware::from_fn_with_state(jwt, auth_middleware))
}

// Future: media routes
// pub fn media_routes(state: MediaState) -> Router {
//     rust_reborn_media::create_routes(state)
// }
