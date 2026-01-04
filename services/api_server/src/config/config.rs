use config::{Config, ConfigError, Environment, File};
use rust_reborn_auth::infrastructure::jwt::JwtConfig;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub media: MediaConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MediaConfig {
    pub upload_dir: String,
    pub max_file_size: usize,
    pub allowed_extensions: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        tracing::info!("📝 Loading configuration for: {}", run_mode);

        let config = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        // Deserialize with variable substitution
        let mut app_config: AppConfig = config.try_deserialize()?;

        // Manual environment variable substitution for ${VAR} syntax
        app_config = Self::substitute_env_vars(app_config)?;

        // Validate
        app_config.validate()?;

        Ok(app_config)
    }

    /// Substitute ${VAR} or ${VAR:-default} in config values
    fn substitute_env_vars(mut config: AppConfig) -> Result<Self, ConfigError> {
        // Database URL
        config.database.url = Self::expand_env(&config.database.url)?;

        // JWT Secret
        config.jwt.secret = Self::expand_env(&config.jwt.secret)?;

        // Media upload dir
        config.media.upload_dir = Self::expand_env(&config.media.upload_dir)?;

        // Integer Overrides (Manual to support single underscore APP_ vars)
        if let Some(port) = Self::get_env_parse("APP_SERVER_PORT") {
            config.server.port = port;
        }
        if let Some(max) = Self::get_env_parse("APP_DATABASE_MAX_CONNECTIONS") {
            config.database.max_connections = max;
        }
        if let Some(min) = Self::get_env_parse("APP_DATABASE_MIN_CONNECTIONS") {
            config.database.min_connections = min;
        }
        if let Some(exp) = Self::get_env_parse("APP_JWT_EXPIRATION_HOURS") {
            config.jwt.expiration_hours = exp;
        }
        if let Some(ref_exp) = Self::get_env_parse("APP_JWT_REFRESH_EXPIRATION_DAYS") {
            config.jwt.refresh_expiration_days = ref_exp;
        }
        if let Some(size) = Self::get_env_parse("APP_MEDIA_MAX_FILE_SIZE") {
            config.media.max_file_size = size;
        }

        Ok(config)
    }

    fn get_env_parse<T: std::str::FromStr>(key: &str) -> Option<T> {
        env::var(key).ok().and_then(|v| v.parse().ok())
    }

    /// Expand ${VAR} or ${VAR:-default} syntax
    fn expand_env(value: &str) -> Result<String, ConfigError> {
        if !value.starts_with("${") || !value.ends_with("}") {
            // Not a variable reference, return as-is
            return Ok(value.to_string());
        }

        // Remove ${ and }
        let inner = &value[2..value.len() - 1];

        // Check for default value syntax: ${VAR:-default}
        if let Some(pos) = inner.find(":-") {
            let var_name = &inner[..pos];
            let default_value = &inner[pos + 2..];

            match env::var(var_name) {
                Ok(val) if !val.is_empty() => Ok(val),
                _ => Ok(default_value.to_string()),
            }
        } else {
            // No default value, variable is required
            env::var(inner).map_err(|_| {
                ConfigError::Message(format!(
                    "Required environment variable '{}' is not set",
                    inner
                ))
            })
        }
    }

    fn validate(&self) -> Result<(), ConfigError> {
        // Check if still contains placeholder
        if self.database.url.contains("${") {
            return Err(ConfigError::Message(
                "DATABASE_URL environment variable is required".into(),
            ));
        }

        if self.jwt.secret.contains("${") {
            return Err(ConfigError::Message(
                "JWT_SECRET environment variable is required".into(),
            ));
        }

        // Production checks
        if self.server.environment == "production" {
            if self.jwt.secret.len() < 32 {
                return Err(ConfigError::Message(
                    "JWT secret must be at least 32 characters in production".into(),
                ));
            }

            if self.database.url.contains("localhost") {
                tracing::warn!("⚠️  Using localhost database in production mode!");
            }
        }

        Ok(())
    }

    pub fn is_production(&self) -> bool {
        self.server.environment == "production"
    }

    pub fn is_development(&self) -> bool {
        self.server.environment == "development"
    }
}
