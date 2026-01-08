use crate::domain::entities::user::User;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            email: user.email.value().to_string(),
            username: user.username,
            full_name: user.full_name,
            is_verified: user.is_verified,
            created_at: user.created_at,
        }
    }
}
