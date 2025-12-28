use crate::domain::entities::User;
use rust_reborn_core::Result;

pub trait TokenGenerator: Send + Sync {
    fn generate(&self, user: &User) -> Result<String>;
}
