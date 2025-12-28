use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_type: String,
    pub expires_in: i64,
}

impl Token {
    pub fn new(access_token: String, expires_in: i64) -> Self {
        Self {
            access_token,
            refresh_token: None,
            token_type: "Bearer".to_string(),
            expires_in,
        }
    }

    pub fn with_refresh(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }
}
