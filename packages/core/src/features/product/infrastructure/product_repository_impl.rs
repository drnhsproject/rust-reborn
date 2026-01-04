use crate::features::product::domain::{Product, ProductRepository};
use async_trait::async_trait;
use rust_reborn_contracts::Result;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresProductRepository {
    #[allow(dead_code)]
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Product>> {
        // TODO: Implement actual DB query
        Ok(None)
    }

    async fn update(&self, _product: &Product) -> Result<Product> {
        Ok(_product.clone())
    }

    async fn save(&self, _product: Product) -> Result<()> {
        Ok(())
    }
}
