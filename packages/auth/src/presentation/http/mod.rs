pub mod handlers;
use crate::AuthState;
use axum::{
    routing::{get, post},
    Router,
};
use handlers::{get_current_user, login, logout, register};

pub fn create_routes(state: AuthState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_current_user))
        .route("/logout", post(logout))
        .with_state(state)
}
