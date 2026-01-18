use serde::Deserialize;
use anyhow::Result;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub(crate) token: String,
}

pub async fn load_config() -> Result<Config> {
    let config_str = tokio::fs::read_to_string("config.toml")
        .await?;
    let conf = toml::from_str(&config_str)?;
    Ok(conf)
}