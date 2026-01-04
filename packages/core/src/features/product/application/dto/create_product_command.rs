use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductCommand {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 1000,
        message = "Description must be between 1 and 1000 characters"
    ))]
    pub description: String,

    #[validate(range(min = 0.01, message = "Price must be greater than 0"))]
    pub price: f64,
}
