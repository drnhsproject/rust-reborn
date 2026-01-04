use crate::presentation;
use crate::routes;
use axum::Router;
use rust_reborn_auth::AuthState;
use rust_reborn_core::{PostgresProductRepository, ProductRepository};
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;

pub fn build_router(pool: sqlx::PgPool, auth_state: AuthState) -> Router {
    let product_repo =
        Arc::new(PostgresProductRepository::new(pool.clone())) as Arc<dyn ProductRepository>;

    let openapi = presentation::build_openapi();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .nest("/api/auth", routes::auth_routes(auth_state.clone()))
        .nest(
            "/api/products",
            routes::product_routes(product_repo, auth_state.clone()),
        )
}
