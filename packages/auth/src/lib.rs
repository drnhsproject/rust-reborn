pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use application::dto::*;
pub use domain::entities::User;
pub use presentation::auth_routes_handler;
pub use presentation::{AuthApiDoc};
pub use presentation::middleware::{auth_middleware, optional_auth_middleware};

use application::services::AuthService;
use infrastructure::jwt::JwtConfig;
use infrastructure::jwt::JwtService;
use infrastructure::password::PasswordService;
use infrastructure::repositories::PostgresUserRepository;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub auth_service: Arc<AuthService<PostgresUserRepository>>,
}

impl AuthState {
    pub fn new(pool: PgPool, jwt_config: JwtConfig) -> Self {
        let user_repository = PostgresUserRepository::new(pool);
        let jwt_service = JwtService::new(jwt_config);
        let password_service = PasswordService::new();

        let auth_service = Arc::new(AuthService::new(
            Arc::new(user_repository),
            Arc::new(jwt_service),
            Arc::new(password_service),
        ));

        Self { auth_service }
    }
}
