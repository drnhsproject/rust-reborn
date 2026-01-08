use crate::domain::UserRepository;
use crate::UserResponse;
use rust_reborn_contracts::AppError;
use std::sync::Arc;

pub struct GetUserDetailUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl GetUserDetailUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn execute(&self, user_id: i64) -> rust_reborn_contracts::Result<UserResponse> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("user not found"))?;

        Ok(user.into())
    }
}
