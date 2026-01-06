use crate::AuthState;
use axum::{
    routing::{get, post},
    Router,
};
use crate::presentation::http::handlers::{get_current_user, login, logout, register};

pub fn auth_routes_handler(state: AuthState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_current_user))
        .route("/logout", post(logout))
        .with_state(state)
}