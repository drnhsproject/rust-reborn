use axum::Router;

use crate::fw::{
    auth::build_auth_state, db::build_db_pool, load_config::load_config, router::build_router,
};

pub struct App {
    pub router: Router,
    pub address: String,
}

pub async fn build_app() -> anyhow::Result<App> {
    let config = load_config()?;
    let pool = build_db_pool(&config).await?;
    let auth_state = build_auth_state(&pool, &config);

    let router = build_router(pool.clone(), auth_state);

    let address = format!("{}:{}", config.server.host, config.server.port);

    Ok(App { router, address })
}
