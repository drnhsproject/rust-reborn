pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub use application::dto::*;
pub use domain::entity::User;
pub use presentation::auth_routes_handler;
pub use presentation::{AuthApiDoc};
pub use presentation::middleware::{auth_middleware, optional_auth_middleware};

use infrastructure::jwt::JwtConfig;
use infrastructure::jwt::JwtService;
use infrastructure::password::PasswordService;
use infrastructure::repository::PostgresUserRepository;
use sqlx::PgPool;
use std::sync::Arc;
use rust_reborn_contracts::common::{CodeGenerator, UuidV7CodeGenerator};
use crate::application::{
    register_user::RegisterUserUseCase,
    login_user::LoginUserUseCase
};
use crate::application::get_user_detail::GetUserDetailUseCase;
use crate::application::verify_token::VerifyTokenUseCase;

#[derive(Clone)]
pub struct AuthState {
    pub register_user_use_case: Arc<RegisterUserUseCase>,
    pub login_user_use_case: Arc<LoginUserUseCase>,
    pub get_user_detail_use_case: Arc<GetUserDetailUseCase>,
    pub verify_token_use_case: Arc<VerifyTokenUseCase>,
}

impl AuthState {
    pub fn new(pool: PgPool, jwt_config: JwtConfig) -> Self {
        // ===== Infrastructure (ONCE) =====
        let user_repo = Arc::new(PostgresUserRepository::new(pool));
        let password_service = Arc::new(PasswordService::new());
        let jwt_service = Arc::new(JwtService::new(jwt_config));
        let code_generator: Arc<dyn CodeGenerator> =
            Arc::new(UuidV7CodeGenerator);

        let register_user_use_case = Arc::new(RegisterUserUseCase::new(
            user_repo.clone(),
            password_service.clone(),
            code_generator.clone(),
        ));

        let login_user_use_case = Arc::new(LoginUserUseCase::new(
            user_repo.clone(),
            jwt_service.clone(),
            password_service.clone(),
        ));
        
        let get_user_detail_use_case = Arc::new(GetUserDetailUseCase::new(
            user_repo.clone(),
        ));
        
        let verify_token_use_case = Arc::new(VerifyTokenUseCase::new(
            jwt_service.clone(),
        ));

        Self {
            get_user_detail_use_case,
            register_user_use_case,
            login_user_use_case,
            verify_token_use_case,
        }
    }
}
