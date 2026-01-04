use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
}
