use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen: String,
    pub log_level: String,

    #[serde(default)]
    pub rules: Vec<String>,
}

pub fn load_config() -> anyhow::Result<Config> {
    let content = fs::read_to_string("config.yaml")?;

    let cfg: Config = serde_yaml::from_str(&content)?;

    Ok(cfg)
}
