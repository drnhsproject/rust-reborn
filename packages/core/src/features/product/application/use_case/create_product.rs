use crate::features::product::application::CreateProductCommand;
use crate::features::product::domain::{Product, ProductRepository};
use rust_reborn_contracts::Result;
use uuid::Uuid;

pub async fn create_product(
    service: &dyn ProductRepository,
    cmd: CreateProductCommand,
) -> Result<Product> {
    let product = Product::new(Uuid::new_v4(), cmd.name, cmd.description, cmd.price)?;

    service.save(product.clone()).await?;

    Ok(product)
}
