use serde::Serialize;
use crate::UserResponse;

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
}