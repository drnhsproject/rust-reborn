use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::get,
    Json, Router,
};
use rust_reborn_auth::AuthState;
use rust_reborn_contracts::config::AppConfig;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod routes;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting Rust-Reborn API Server");

    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");

    // Create database connection pool
    tracing::info!("🔌 Connecting to database at {}...", config.database.url);

    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database.url)
        .await?;

    tracing::info!("✅ Database connected");

    // Test database connection
    let result: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;
    tracing::info!("✅ Database query test passed: {}", result.0);

    // Initialize Auth State
    let auth_state = AuthState::new(pool.clone(), config.clone());

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Build router
    let db_routes = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/db-check", get(db_check))
        .with_state(pool);

    let app = Router::new()
        .merge(db_routes)
        .nest("/api/auth", routes::auth_routes(auth_state.clone()))
        .nest("/api", routes::product_routes(auth_state))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("🎧 Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\n🦀 Rust-Reborn API Server is running!");
    println!("   → Address: http://{}", addr);
    println!("   → Health Check: http://{}/api/health", addr);
    println!("\n   → Press Ctrl+C to stop\n");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "API is healthy"
    }))
}

async fn db_check(
    axum::extract::State(pool): axum::extract::State<sqlx::PgPool>,
) -> Json<serde_json::Value> {
    match sqlx::query_as::<_, (i32,)>("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => Json(json!({
            "status": "ok",
            "message": "Database is connected"
        })),
        Err(e) => Json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    }
}
