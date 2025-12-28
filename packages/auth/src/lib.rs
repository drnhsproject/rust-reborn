pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use application::dto::*;
pub use domain::entities::User;
pub use presentation::create_routes;
pub use presentation::middleware::{auth_middleware, optional_auth_middleware};

// Re-export extractors from contracts for convenience
pub use rust_reborn_contracts::{AuthUser, OptionalAuthUser};


use application::services::AuthService;
use infrastructure::jwt::JwtService;
use infrastructure::password::PasswordService;
use infrastructure::repositories::PostgresUserRepository;
use rust_reborn_contracts::config::AppConfig;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub auth_service: Arc<AuthService<PostgresUserRepository>>,
}

impl AuthState {
    pub fn new(pool: PgPool, config: AppConfig) -> Self {
        let user_repository = PostgresUserRepository::new(pool);
        let jwt_service = JwtService::new(config.jwt.clone());
        let password_service = PasswordService::new();

        let auth_service = Arc::new(AuthService::new(
            Arc::new(user_repository),
            Arc::new(jwt_service),
            Arc::new(password_service),
        ));

        Self { auth_service }
    }
}
