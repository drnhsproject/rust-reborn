use crate::config::config::AppConfig;
use rust_reborn_auth::AuthState;
use sqlx::PgPool;

pub fn build_auth_state(pool: &PgPool, config: &AppConfig) -> AuthState {
    AuthState::new(pool.clone(), config.jwt.clone())
}
