use crate::config::config::AppConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn build_db_pool(config: &AppConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database.url)
        .await?;

    Ok(pool)
}
