use rust_reborn_contracts::{AppError, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: Uuid, name: String, description: String, price: f64) -> Result<Self> {
        if price <= 0.0 {
            return Err(AppError::bad_request("Price must be greater than 0"));
        }

        Ok(Self {
            id,
            name,
            description,
            price,
        })
    }

    pub fn price(&self) -> f64 {
        self.price
    }
}
