use crate::RegisterResponse;
use crate::application::dto::{
    AuthResponse, LoginRequest, RegisterRequest, TokenResponse, UserResponse,
};
use crate::application::port::password_hasher::PasswordHasher;
use crate::domain::entities::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::value_objects::{Email, HashedPassword, Password};
use crate::infrastructure::{jwt::JwtService, password::PasswordService};
use rust_reborn_contracts::{AppError, Result};
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthService<R: UserRepository> {
    user_repository: Arc<R>,
    jwt_service: Arc<JwtService>,
    password_service: Arc<PasswordService>,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(
        user_repository: Arc<R>,
        jwt_service: Arc<JwtService>,
        password_service: Arc<PasswordService>,
    ) -> Self {
        Self {
            user_repository,
            jwt_service,
            password_service,
        }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse> {
        self.check_email_availability(&request.email).await?;
        self.check_username_availability(&request.username).await?;

        // Validate password strength
        let _ = Password::new(request.password.clone())
            .map_err(|e| AppError::bad_request(format!("Invalid password: {}", e)))?;

        let hashed = self.password_service.hash(&request.password)?;
        let hashed_password = HashedPassword::new(hashed);

        let email_obj = Email::new(request.email)
            .map_err(|e| AppError::bad_request(format!("Invalid email: {}", e)))?;

        let user = User::new(
            email_obj,
            request.username,
            hashed_password,
            request.full_name,
        );

        self.user_repository.save(user.clone()).await?;

        Ok(RegisterResponse {
            id: user.id,
            email: user.email.value().to_string(),
            username: user.username,
            full_name: user.full_name,
            is_verified: user.is_verified,
            created_at: user.created_at,
        })
    }

    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse> {
        let user = self
            .find_user_by_username_or_email(&request.username)
            .await?
            .ok_or_else(|| AppError::unauthorized("Invalid credentials"))?;

        if !user.can_login() {
            return Err(AppError::forbidden("Account is not active or verified"));
        }

        if !self
            .password_service
            .verify(&request.password, user.password.value())?
        {
            return Err(AppError::unauthorized("Invalid credentials"));
        }

        let mut user = user;
        user.update_last_login();
        let user = self.user_repository.update(&user).await?;

        let token = self.jwt_service.generate_token(&user)?;

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

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<UserResponse> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("user not found"))?;

        Ok(user.into())
    }

    pub async fn change_password(
        &self,
        user_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<()> {
        let mut user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        if !self
            .password_service
            .verify(&old_password, user.password.value())?
        {
            return Err(AppError::unauthorized("Invalid current password"));
        }

        let _ = Password::new(new_password.clone())
            .map_err(|e| AppError::bad_request(format!("Invalid password: {}", e)))?;

        let new_password_hash = self.password_service.hash(&new_password)?;
        user.password = HashedPassword::new(new_password_hash);
        user.updated_at = chrono::Utc::now();

        self.user_repository.update(&user).await?;

        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<Uuid> {
        self.jwt_service.verify_token(token)
    }

    async fn check_email_availability(&self, email: &str) -> Result<()> {
        if let Some(_) = self.user_repository.find_by_email(email).await? {
            return Err(AppError::conflict("Email already registered"));
        }
        Ok(())
    }

    async fn check_username_availability(&self, username: &str) -> Result<()> {
        if let Some(_) = self.user_repository.find_by_username(username).await? {
            return Err(AppError::conflict("Username already taken"));
        }
        Ok(())
    }

    async fn find_user_by_username_or_email(&self, input: &str) -> Result<Option<User>> {
        // Try email first
        if let Some(user) = self.user_repository.find_by_email(input).await? {
            return Ok(Some(user));
        }

        // Try username
        self.user_repository.find_by_username(input).await
    }
}
