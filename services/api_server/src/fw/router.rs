use axum::Router;
use std::sync::Arc;

use rust_reborn_auth::AuthState;
use rust_reborn_core::{PostgresProductRepository, ProductRepository};

use crate::routes;

pub fn build_router(pool: sqlx::PgPool, auth_state: AuthState) -> Router {
    let product_repo =
        Arc::new(PostgresProductRepository::new(pool.clone())) as Arc<dyn ProductRepository>;

    Router::new()
        .nest("/api/auth", routes::auth_routes(auth_state.clone()))
        .nest(
            "/api/products",
            routes::product_routes(product_repo, auth_state.clone()),
        )
}
