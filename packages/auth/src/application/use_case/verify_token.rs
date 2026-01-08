use crate::infrastructure::jwt::JwtService;
use rust_reborn_contracts::Result;
use std::sync::Arc;

pub struct VerifyTokenUseCase {
    jwt_service: Arc<JwtService>,
}

impl VerifyTokenUseCase {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }

    pub async fn execute(&self, token: &str) -> Result<i64> {
        self.jwt_service.verify_token(token)
    }
}
