use crate::config::config::AppConfig;

pub fn load_config() -> anyhow::Result<AppConfig> {
    AppConfig::load().map_err(|e| anyhow::anyhow!(e))
}
