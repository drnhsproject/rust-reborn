use crate::domain::value_objects::{Email, HashedPassword};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub username: String,
    pub password: HashedPassword,
    pub full_name: Option<String>,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        email: Email,
        username: String,
        password: HashedPassword,
        full_name: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            username,
            password,
            full_name,
            is_verified: false,
            is_active: true,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_login(&self) -> bool {
        self.is_active
    }

    pub fn verify(&mut self) {
        self.is_verified = true;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn update_last_login(&mut self) {
        self.last_login_at = Some(Utc::now());
    }
}
