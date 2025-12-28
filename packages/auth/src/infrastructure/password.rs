use crate::application::port::password_hasher::PasswordHasher;
use async_trait::async_trait;
use bcrypt::{hash, verify, DEFAULT_COST};
use rust_reborn_core::{AppError, Result};

#[derive(Clone)]
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PasswordHasher for PasswordService {
    fn hash(&self, raw: &str) -> Result<String> {
        hash(raw, DEFAULT_COST)
            .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))
    }

    fn verify(&self, raw: &str, hashed: &str) -> Result<bool> {
        verify(raw, hashed)
            .map_err(|e| AppError::internal(format!("Failed to verify password: {}", e)))
    }
}
