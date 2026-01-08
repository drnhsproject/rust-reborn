use crate::application::dto::{RegisterRequest, RegisterResponse};
use crate::application::password_hasher::PasswordHasher;
use crate::domain::entity::User;
use crate::domain::repository::user_repository::UserRepository;
use crate::domain::value_objects::{Email, HashedPassword, Password};
use rust_reborn_contracts::common::CodeGenerator;
use rust_reborn_contracts::{AppError, Result};
use std::sync::Arc;

pub struct RegisterUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    code_generator: Arc<dyn CodeGenerator>,
}

impl RegisterUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        code_generator: Arc<dyn CodeGenerator>,
    ) -> Self {
        Self {
            user_repo,
            password_hasher,
            code_generator,
        }
    }

    pub async fn execute(&self, req: RegisterRequest) -> Result<RegisterResponse> {
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AppError::conflict("email already registered"));
        }

        if self
            .user_repo
            .find_by_username(&req.username)
            .await?
            .is_some()
        {
            return Err(AppError::conflict("username already taken"));
        }

        Password::new(req.password.clone()).map_err(|e| AppError::bad_request(e.to_string()))?;

        let email_obj = Email::new(req.email)
            .map_err(|e| AppError::bad_request(format!("invalid email: {}", e)))?;

        let code = self.code_generator.generate("usr");
        let hashed = self.password_hasher.hash(&req.password)?;
        let hashed_password = HashedPassword::new(hashed);

        let mut user = User::new(
            code,
            email_obj,
            req.username,
            hashed_password,
            req.full_name,
        );

        self.user_repo.save(&mut user).await?;

        Ok(RegisterResponse {
            id: user.id.expect("user must be persisted"),
            email: user.email.value().to_string(),
            username: user.username,
            full_name: user.full_name,
            is_verified: user.is_verified,
            created_at: user.created_at,
        })
    }
}
