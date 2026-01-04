use axum::Router;
use rust_reborn_auth::AuthState;

use rust_reborn_core::ProductRepository;
use std::sync::Arc;

pub fn auth_routes(state: AuthState) -> Router {
    rust_reborn_auth::create_routes(state)
}

pub fn product_routes(repo: Arc<dyn ProductRepository>, auth_state: AuthState) -> Router {
    rust_reborn_core::create_product_routes(repo, auth_state)
}

// Future: media routes
// pub fn media_routes(state: MediaState) -> Router {
//     rust_reborn_media::create_routes(state)
// }
