use crate::error::{AppError, Result};
use validator::Validate;

pub fn validate<T: Validate>(data: &T) -> Result<()> {
    data.validate().map_err(|e| AppError::ValidationError(e))
}

pub mod custom {
    use validator::ValidationError;

    pub fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
        if password.len() < 8 {
            return Err(ValidationError::new("password_too_short"));
        }
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        
        if !(has_upper && has_lower && has_digit && has_special) {
            return Err(ValidationError::new("password_too_weak"));
        }
        Ok(())
    }
}
