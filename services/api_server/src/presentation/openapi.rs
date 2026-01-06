use rust_reborn_core::features::product::presentation::ProductApiDoc;
use rust_reborn_auth::AuthApiDoc;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Rust-Reborn API",
        version = "1.0.0",
        description = "A modern Rust Framework with DDD and Clean Architecture",
    ),
    nest(
        (path = "/api/auth", api = AuthApiDoc),
        (path = "/api", api = ProductApiDoc)
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn build_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}