use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::get,
    Json, Router,
};
use rust_reborn_auth::AuthState;
use rust_reborn_core::config::{AppConfig, DatabaseConfig, JwtConfig, MediaConfig, ServerConfig};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting Rust-Reborn API Server");

    // Load configuration manually from env to be safe
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://admin:password@localhost:5432/rust_reborn".to_string());

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "supersecretkey".to_string());

    let config = AppConfig {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 3000,
            environment: "development".to_string(),
        },
        database: DatabaseConfig {
            url: database_url.clone(),
            max_connections: 5,
            min_connections: 1,
        },
        jwt: JwtConfig {
            secret: jwt_secret,
            expiration_hours: 24,
            refresh_expiration_days: 7,
        },
        media: MediaConfig {
            upload_dir: "uploads".to_string(),
            max_file_size: 10 * 1024 * 1024,
            allowed_extensions: vec!["jpg".to_string(), "png".to_string()],
        },
    };

    // Create database connection pool
    tracing::info!("🔌 Connecting to database at {}...", database_url);

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
    // Build router
    let db_routes = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/db-check", get(db_check))
        .with_state(pool);

    let app = Router::new()
        .merge(db_routes)
        .nest("/api/auth", rust_reborn_auth::create_routes(auth_state))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("🎧 Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\n🦀 Rust-Reborn API Server is running!");
    println!("   → Address: http://{}", addr);
    println!("   → Health Check: http://{}/api/health", addr);
    println!("   → Auth Register: POST http://{}/api/auth/register", addr);
    println!("   → Auth Login: POST http://{}/api/auth/login", addr);
    println!("   → Press Ctrl+C to stop\n");

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
