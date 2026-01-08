use std::sync::Arc;
use crate::application::password_hasher::PasswordHasher;
use crate::domain::{UserRepository, value_objects::Password};
use rust_reborn_contracts::{AppError, Result};
use crate::domain::value_objects::HashedPassword;

pub struct ChangePasswordUseCase {
    user_repo: Arc<dyn UserRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl ChangePasswordUseCase {
    pub async fn execute(
        &self,
        user_id: i64,
        old: String,
        new: String,
    ) -> Result<()> {
        let mut user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| AppError::not_found("user not found"))?;

        if !self.password_hasher.verify(&old, user.password.value())? {
            return Err(AppError::unauthorized("invalid password"));
        }

        Password::new(new.clone())
            .map_err(|e| AppError::bad_request(format!("invalid password: {}", e)))?;

        user.password = HashedPassword::new(self.password_hasher.hash(&new)?);
        user.updated_at = chrono::Utc::now();

        self.user_repo.update(&user).await?;
        Ok(())
    }
}
