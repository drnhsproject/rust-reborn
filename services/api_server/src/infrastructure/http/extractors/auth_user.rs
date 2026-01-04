use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use rust_reborn_contracts::AppError;
use sqlx::types::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct AuthUser(pub Uuid);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts.extensions.get::<Uuid>().copied().ok_or_else(|| {
            AppError::unauthorized("Authentication required. Please login first.")
        })?;

        Ok(AuthUser(user_id))
    }
}

#[derive(Debug, Clone)]
pub struct OptionalAuthUser(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts.extensions.get::<Uuid>().copied();

        Ok(OptionalAuthUser(user_id))
    }
}
