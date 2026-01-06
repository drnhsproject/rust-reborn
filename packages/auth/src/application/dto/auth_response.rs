use super::token_response::TokenResponse;
use super::user_response::UserResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: TokenResponse,
}
