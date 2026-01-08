use crate::domain::{entities::User, repositories::UserRepository, value_objects::{Email, HashedPassword}};
use async_trait::async_trait;
use rust_reborn_contracts::Result;
use sqlx::{query, PgPool};

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
                code,
                email,
                username,
                password,
                full_name,
                is_verified,
                is_active,
                activation_key,
                reset_key,
                reset_key_expires_at,
                reset_date,
                status,
                created_by,
                updated_by,
                created_at,
                updated_at,
                deleted_at,
                last_login_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: Some(r.id),
            code: r.code,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            password: HashedPassword::new(r.password),
            full_name: r.full_name,
            is_active: r.is_active,
            is_verified: r.is_verified,
            activation_key: r.activation_key,
            reset_key: r.reset_key,
            reset_key_expires_at: r.reset_key_expires_at,
            reset_date: r.reset_date,
            status: r.status,
            created_by: r.created_by,
            updated_by: r.updated_by,
            created_at: r.created_at,
            updated_at: r.created_at,
            deleted_at: r.deleted_at,
            last_login_at: r.last_login_at,
        }))
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>> {
        let row = query!(
            r#"
            SELECT
                id,
                code,
                email,
                username,
                password,
                full_name,
                is_verified,
                is_active,
                activation_key,
                reset_key,
                reset_key_expires_at,
                reset_date,
                status,
                created_by,
                updated_by,
                created_at,
                updated_at,
                deleted_at,
                last_login_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: Some(r.id),
            code: r.code,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            password: HashedPassword::new(r.password),
            full_name: r.full_name,
            is_active: r.is_active,
            is_verified: r.is_verified,
            activation_key: r.activation_key,
            reset_key: r.reset_key,
            reset_key_expires_at: r.reset_key_expires_at,
            reset_date: r.reset_date,
            status: r.status,
            created_by: r.created_by,
            updated_by: r.updated_by,
            created_at: r.created_at,
            updated_at: r.created_at,
            deleted_at: r.deleted_at,
            last_login_at: r.last_login_at,
        }))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let row = query!(
            r#"
            SELECT
                id,
                code,
                email,
                username,
                full_name,
                password,
                is_verified,
                is_active,
                activation_key,
                reset_key,
                reset_key_expires_at,
                reset_date,
                status,
                created_by,
                updated_by,
                created_at,
                updated_at,
                deleted_at,
                last_login_at
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: Some(r.id),
            code: r.code,
            email: Email::new(r.email).unwrap(),
            username: r.username,
            full_name: r.full_name,
            password: HashedPassword::new(r.password),
            is_active: r.is_active,
            is_verified: r.is_verified,
            activation_key: r.activation_key,
            reset_key: r.reset_key,
            reset_key_expires_at: r.reset_key_expires_at,
            reset_date: r.reset_date,
            status: r.status,
            created_by: r.created_by,
            updated_by: r.updated_by,
            created_at: r.created_at,
            updated_at: r.updated_at,
            deleted_at: r.deleted_at,
            last_login_at: r.last_login_at,
        }))
    }

    async fn update(&self, user: &User) -> Result<User> {
        query!(
            r#"
            UPDATE users
            SET
                full_name = $1,
                password = $2,
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

    async fn save(&self, user: &mut User) -> Result<()> {
        assert!(user.id.is_none(), "User already persisted");

        let id = sqlx::query!(
        r#"INSERT INTO users (
                code,
                email,
                username,
                full_name,
                password,
                is_verified,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            &user.code,
            user.email.value(),
            &user.username,
            user.full_name,
            user.password.value(),
            user.is_verified,
            user.created_at,
            user.updated_at,
        )
            .fetch_one(&self.pool)
            .await?
            .id;

        user.id = Some(id);
        Ok(())
    }
}
