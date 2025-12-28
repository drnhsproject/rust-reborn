use super::token_response::TokenResponse;
use super::user_response::UserResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: TokenResponse,
}
