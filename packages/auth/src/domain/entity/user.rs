use crate::domain::value_objects::{Email, HashedPassword};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub code: String,
    pub email: Email,
    pub username: String,
    pub password: HashedPassword,
    pub full_name: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub activation_key: Option<String>,
    pub reset_key: Option<String>,
    pub reset_key_expires_at: Option<DateTime<Utc>>,
    pub reset_date: Option<DateTime<Utc>>,
    pub status: i32,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(
        code: String,
        email: Email,
        username: String,
        password: HashedPassword,
        full_name: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            code,
            email,
            username,
            password,
            full_name,
            is_verified: false,
            is_active: true,
            activation_key: None,
            reset_key: None,
            reset_key_expires_at: None,
            reset_date: None,
            status: 1,
            created_by: None,
            updated_by: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            last_login_at: None,
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
