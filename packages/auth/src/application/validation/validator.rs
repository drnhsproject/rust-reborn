use rust_reborn_contracts::error::{AppError, Result};
use validator::Validate;

pub fn validate<T: Validate>(data: &T) -> Result<()> {
    data.validate().map_err(|e| AppError::ValidationError(e))
}
