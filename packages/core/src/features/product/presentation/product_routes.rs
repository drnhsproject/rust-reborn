use axum::{middleware, routing::post, Router};
use rust_reborn_auth::auth_middleware;

use crate::features::product::presentation::product_controller::create_product_handler;

use crate::features::product::domain::ProductRepository;
use std::sync::Arc;

pub fn create_product_routes<S>(
    repo: Arc<dyn ProductRepository>,
    auth_state: rust_reborn_auth::AuthState,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/products",
            post(create_product_handler).layer(middleware::from_fn_with_state(
                auth_state.clone(),
                auth_middleware,
            )),
        )
        .with_state(repo)
}
