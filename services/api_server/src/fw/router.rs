use crate::presentation;
use crate::routes;
use axum::Router;
use rust_reborn_auth::infrastructure::jwt::{JwtConfig, JwtService};
use rust_reborn_auth::AuthState;
use rust_reborn_core::{PostgresProductRepository, ProductRepository};
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;

pub fn build_router(pool: sqlx::PgPool, auth_state: AuthState, jwt_config: JwtConfig) -> Router {
    let product_repo =
        Arc::new(PostgresProductRepository::new(pool.clone())) as Arc<dyn ProductRepository>;
    let jwt_service = Arc::new(JwtService::new(jwt_config));
    let openapi = presentation::build_openapi();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .nest("/api/auth", routes::auth_routes(auth_state.clone()))
        .nest(
            "/api/products",
            routes::product_routes(product_repo, jwt_service),
        )
}
