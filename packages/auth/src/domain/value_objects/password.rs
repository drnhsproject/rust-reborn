use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn new(value: String) -> Result<Self, validator::ValidationError> {
        rust_reborn_contracts::validation::custom::validate_password_strength(&value)?;
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for HashedPassword {
    fn from(s: String) -> Self {
        Self(s)
    }
}
