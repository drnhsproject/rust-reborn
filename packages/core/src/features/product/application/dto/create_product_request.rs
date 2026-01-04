use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateProductRequest {
    #[validate(length(min = 3))]
    pub name: String,
    pub description: String,
    pub price: f64,
}
