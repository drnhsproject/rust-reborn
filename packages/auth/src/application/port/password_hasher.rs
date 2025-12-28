use rust_reborn_core::Result;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, raw: &str) -> Result<String>;
    fn verify(&self, raw: &str, hashed: &str) -> Result<bool>;
}
