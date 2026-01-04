use async_trait::async_trait;
use rust_reborn_contracts::Result;
use uuid::Uuid;

use crate::features::product::domain::Product;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Product>>;
    async fn update(&self, product: &Product) -> Result<Product>;
    async fn save(&self, product: Product) -> Result<()>;
}
