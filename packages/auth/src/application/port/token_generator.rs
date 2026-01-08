use crate::domain::entity::User;
use rust_reborn_contracts::Result;

pub trait TokenGenerator: Send + Sync {
    fn generate(&self, user: &User) -> Result<String>;
}
