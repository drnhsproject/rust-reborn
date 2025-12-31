use crate::domain::{entities::User, repositories::UserRepository};
use async_trait::async_trait;
use rust_reborn_contracts::Result;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresUserRepository {
    #[allow(dead_code)]
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, _email: &str) -> Result<Option<User>> {
        // TODO: Implement actual DB query
        Ok(None)
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>> {
        // TODO: Implement actual DB query
        Ok(None)
    }

    async fn find_by_username(&self, _username: &str) -> Result<Option<User>> {
        // TODO: Implement actual DB query
        Ok(None)
    }

    async fn update(&self, user: &User) -> Result<User> {
        // TODO: Implement actual DB update
        Ok(user.clone())
    }

    async fn save(&self, user: User) -> Result<()> {
        query!(
            r#"
            INSERT INTO users (
                id,
                email,
                username,
                full_name,
                password_hash,
                is_verified,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user.id,
            user.email.value(),          // Email VO → &str
            user.username,
            user.full_name,
            user.password.value(),       // HashedPassword VO → &str
            user.is_verified,
            user.created_at,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
