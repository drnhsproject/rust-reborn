use crate::application::password_hasher::PasswordHasher;
use crate::domain::UserRepository;
use crate::infrastructure::jwt::JwtService;
use crate::infrastructure::password::PasswordService;
use crate::{AuthResponse, LoginRequest, TokenResponse, User};
use rust_reborn_contracts::{AppError, Result};
use std::sync::Arc;

pub struct LoginUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    token_provider: Arc<JwtService>,
    password_service: Arc<PasswordService>,
}

impl LoginUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        token_provider: Arc<JwtService>,
        password_service: Arc<PasswordService>,
    ) -> Self {
        Self {
            user_repo,
            token_provider,
            password_service,
        }
    }

    pub async fn execute(&self, req: LoginRequest) -> Result<AuthResponse> {
        let user = self
            .find_user_by_username_or_email(&req.username)
            .await?
            .ok_or_else(|| AppError::unauthorized("invalid credentials"))?;

        if !user.can_login() {
            return Err(AppError::forbidden("account is not active or verified"));
        }

        if !self
            .password_service
            .verify(&req.password, user.password.value())?
        {
            return Err(AppError::unauthorized("invalid credentials"));
        }

        let mut user = user;
        user.update_last_login();
        let user = self.user_repo.update(&user).await?;

        let token = self.token_provider.generate_token(&user)?;

        Ok(AuthResponse {
            user: user.into(),
            token: TokenResponse {
                access_token: token,
                token_type: "Bearer".to_string(),
                expires_in: 86400,
                refresh_token: None,
            },
        })
    }

    async fn find_user_by_username_or_email(
        &self,
        input: &str,
    ) -> rust_reborn_contracts::Result<Option<User>> {
        // Try email first
        if let Some(user) = self.user_repo.find_by_email(input).await? {
            return Ok(Some(user));
        }

        // Try username
        self.user_repo.find_by_username(input).await
    }
}
