use axum::{
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rust_reborn_auth::auth_middleware;
use rust_reborn_contracts::{
    common::response::created, validation::validate, AuthUser, OptionalAuthUser, Result,
};
use uuid::Uuid;

use crate::product::{CreateProductRequest, ProductResponse};

/// Handler untuk membuat product - HARUS LOGIN
/// Menggunakan AuthUser extractor yang otomatis memverifikasi user sudah login
pub async fn create_product(
    AuthUser(user_id): AuthUser,  // Ini akan otomatis reject jika user tidak login
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse> {
    // Validate input
    validate(&payload)?;

    // Simulasi create product (dalam real app, ini akan save ke database)
    let product = ProductResponse {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        price: payload.price,
        created_by: user_id, // User ID dari token yang sudah diverifikasi
    };

    Ok(created(product))
}

/// Handler untuk list products - TIDAK PERLU LOGIN
/// Menggunakan OptionalAuthUser untuk mendukung user yang login maupun tidak
pub async fn list_products(
    OptionalAuthUser(user_id): OptionalAuthUser,
) -> Result<impl IntoResponse> {
    // Simulasi list products
    let products = vec![
        ProductResponse {
            id: Uuid::new_v4(),
            name: "Product 1".to_string(),
            description: "Description 1".to_string(),
            price: 100.0,
            created_by: Uuid::new_v4(),
        },
        ProductResponse {
            id: Uuid::new_v4(),
            name: "Product 2".to_string(),
            description: "Description 2".to_string(),
            price: 200.0,
            created_by: Uuid::new_v4(),
        },
    ];

    // Jika user login, bisa kasih info tambahan
    if let Some(user_id) = user_id {
        tracing::info!("Authenticated user {} is viewing products", user_id);
    } else {
        tracing::info!("Anonymous user is viewing products");
    }

    Ok(Json(products))
}

/// Handler untuk get product by id - TIDAK PERLU LOGIN
pub async fn get_product() -> Result<impl IntoResponse> {
    // Simulasi get product
    let product = ProductResponse {
        id: Uuid::new_v4(),
        name: "Sample Product".to_string(),
        description: "Sample Description".to_string(),
        price: 150.0,
        created_by: Uuid::new_v4(),
    };

    Ok(Json(product))
}

/// Membuat routes untuk product
/// Mendemonstrasikan penggunaan middleware auth untuk protected routes
pub fn create_product_routes<S>(auth_state: rust_reborn_auth::AuthState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        // Public routes - tidak perlu login
        .route("/products", get(list_products))
        .route("/products/{id}", get(get_product))
        // Protected routes - harus login
        // Menggunakan layer middleware untuk memverifikasi token
        .route(
            "/products",
            post(create_product)
                .layer(middleware::from_fn_with_state(
                    auth_state.clone(),
                    auth_middleware,
                )),
        )
}
