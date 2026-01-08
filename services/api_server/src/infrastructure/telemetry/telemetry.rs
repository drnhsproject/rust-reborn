use tracing::subscriber::set_global_default;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_telemetry(app_name: &str, environment: &str) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        if environment == "production" {
            EnvFilter::new("info")
        } else {
            EnvFilter::new("debug")
        }
    });

    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_level(true);

    let subscriber = Registry::default().with(env_filter).with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");
    tracing::info!("🔍 Telemetry initialized for {}", app_name);
}
