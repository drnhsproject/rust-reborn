use crate::domain::{entities::User, repositories::UserRepository, value_objects::{Email, HashedPassword}};
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
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = query!(
            r#"
            SELECT
                id,
                email,
                username,
                full_name,
                password_hash,
                is_verified,
                created_at,
                last_login_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            full_name: r.full_name,
            password: HashedPassword::new(r.password_hash),
            is_verified: r.is_verified,
            created_at: r.created_at,
            updated_at: r.created_at, 
            is_active: true,
            last_login_at: r.last_login_at,
        }))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let row = query!(
            r#"
            SELECT
                id,
                email,
                username,
                full_name,
                password_hash,
                is_verified,
                created_at,
                updated_at,
                is_active,
                last_login_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            full_name: r.full_name,
            password: HashedPassword::new(r.password_hash),
            is_verified: r.is_verified,
            created_at: r.created_at,
            updated_at: r.updated_at,
            is_active: r.is_active,
            last_login_at: r.last_login_at,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let row = query!(
            r#"
            SELECT
                id,
                email,
                username,
                full_name,
                password_hash,
                is_verified,
                created_at,
                updated_at,
                is_active,
                last_login_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            full_name: r.full_name,
            password: HashedPassword::new(r.password_hash),
            is_verified: r.is_verified,
            created_at: r.created_at,
            updated_at: r.updated_at,
            is_active: r.is_active,
            last_login_at: r.last_login_at,
        }))
    }

    async fn update(&self, user: &User) -> Result<User> {
        query!(
            r#"
            UPDATE users
            SET
                full_name = $1,
                password_hash = $2,
                is_verified = $3,
                last_login_at = $4
            WHERE id = $5
            "#,
            user.full_name,
            user.password.value(),
            user.is_verified,
            user.last_login_at,
            user.id,
        )
        .execute(&self.pool)
        .await?;

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
