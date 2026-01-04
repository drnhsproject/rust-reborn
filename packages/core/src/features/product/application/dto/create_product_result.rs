use crate::features::product::domain::Product;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CreateProductResult {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
}

impl From<Product> for CreateProductResult {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
        }
    }
}
