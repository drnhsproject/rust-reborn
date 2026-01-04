pub mod config;
pub mod fw;
pub mod infrastructure;
pub mod presentation;
pub mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("🚀 Starting Rust-Reborn API Server");
    let app = fw::build_app().await?;
    let addr = app.address;
    tracing::info!("🎧 Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("\n🦀 Rust-Reborn API Server is running!");
    println!("   → Address: http://{}", addr);
    println!("   → Api Url: http://{}/api", addr);
    println!("\n   → Press Ctrl+C to stop\n");

    axum::serve(listener, app.router).await?;

    Ok(())
}
