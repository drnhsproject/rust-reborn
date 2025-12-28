use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::AppError;

/// Extractor untuk mendapatkan authenticated user ID dari request
/// Gunakan ini di handler untuk mendapatkan user_id yang sudah diverifikasi oleh middleware
///
/// # Example
/// ```rust
/// async fn create_product(
///     AuthUser(user_id): AuthUser,
///     Json(payload): Json<CreateProductRequest>,
/// ) -> Result<impl IntoResponse> {
///     // user_id sudah terverifikasi dan bisa langsung digunakan
///     println!("User {} is creating a product", user_id);
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct AuthUser(pub Uuid);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Ambil user_id dari extensions yang sudah disimpan oleh middleware
        let user_id = parts
            .extensions
            .get::<Uuid>()
            .copied()
            .ok_or_else(|| {
                AppError::unauthorized("Authentication required. Please login first.")
            })?;

        Ok(AuthUser(user_id))
    }
}

/// Optional extractor - tidak akan error jika user tidak login
/// Berguna untuk endpoint yang bisa diakses dengan atau tanpa login
///
/// # Example
/// ```rust
/// async fn get_products(
///     OptionalAuthUser(user_id): OptionalAuthUser,
/// ) -> Result<impl IntoResponse> {
///     if let Some(user_id) = user_id {
///         println!("Authenticated user {} is viewing products", user_id);
///     } else {
///         println!("Anonymous user is viewing products");
///     }
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Coba ambil user_id dari extensions
        let user_id = parts.extensions.get::<Uuid>().copied();

        Ok(OptionalAuthUser(user_id))
    }
}
