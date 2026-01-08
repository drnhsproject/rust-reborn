use crate::config::config::AppConfig;
use rust_reborn_auth::AuthState;
use sqlx::PgPool;
use rust_reborn_auth::infrastructure::jwt::JwtConfig;

pub fn build_auth_state(pool: &PgPool, config: &AppConfig) -> AuthState {
    AuthState::new(pool.clone(), config.jwt.clone())
}

pub fn build_jwt_config(config: &AppConfig) -> JwtConfig {
    JwtConfig {
        secret: config.jwt.secret.clone(),
        expiration_hours: config.jwt.expiration_hours,
        refresh_expiration_days: config.jwt.refresh_expiration_days,
    }
}
