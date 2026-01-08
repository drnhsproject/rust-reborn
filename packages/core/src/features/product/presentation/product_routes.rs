use axum::{routing::post, Router};
use crate::features::product::presentation::product_controller::create_product_handler;
use crate::features::product::domain::ProductRepository;
use std::sync::Arc;

pub fn product_routes_handler(
    repo: Arc<dyn ProductRepository>,
) -> Router {
    Router::new()
        .route("/", post(create_product_handler))
        .with_state(repo)
}

