use crate::domain::entity::User;
use async_trait::async_trait;
use rust_reborn_contracts::Result;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: i64) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn update(&self, user: &User) -> Result<User>;
    async fn save(&self, user: &mut User) -> Result<()>;
}
