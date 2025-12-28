use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 10 }

impl PaginationParams {
    pub fn offset(&self) -> u32 { (self.page - 1) * self.per_page }
    pub fn limit(&self) -> u32 { self.per_page }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub current_page: u32,
    pub per_page: u32,
    pub total_items: u64,
    pub total_pages: u32,
}
