use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_by: Uuid,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    
    #[validate(length(min = 1, max = 1000, message = "Description must be between 1 and 1000 characters"))]
    pub description: String,
    
    #[validate(range(min = 0.01, message = "Price must be greater than 0"))]
    pub price: f64,
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_by: Uuid,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            created_by: product.created_by,
        }
    }
}
