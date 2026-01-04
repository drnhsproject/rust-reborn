use crate::features::product::{
    application::{
        create_product::create_product,
        dto::{
            create_product_command::CreateProductCommand,
            create_product_request::CreateProductRequest,
            create_product_result::CreateProductResult,
        },
    },
    domain::ProductRepository,
};
use axum::{extract::State, response::IntoResponse, Json};
use rust_reborn_contracts::{common::response::created, validation::validate, Result};
use std::sync::Arc;

pub async fn create_product_handler(
    State(repo): State<Arc<dyn ProductRepository>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    validate(&req)?;

    let command = CreateProductCommand {
        name: req.name,
        description: req.description,
        price: req.price,
    };

    let product = create_product(repo.as_ref(), command).await?;

    Ok(created(CreateProductResult::from(product)))
}
