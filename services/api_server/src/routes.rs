use axum::Router;
use rust_reborn_auth::AuthState;

pub fn auth_routes(state: AuthState) -> Router {
    rust_reborn_auth::create_routes(state)
}

// Future: media routes
// pub fn media_routes(state: MediaState) -> Router {
//     rust_reborn_media::create_routes(state)
// }