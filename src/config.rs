use crate::errors::ConfigurationError;
use config::Config as ConfigRs;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::errors;

pub const CONFIG_PATH: &str = "~/.config/vpnsky.toml";

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub logs: LogsConfig,
    pub settings: SettingsConfig,
    pub secrets: SecretsConfig,
}

#[derive(Debug, Deserialize)]
pub struct LogsConfig {
    pub to_file: bool,
    pub path: PathBuf,
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct SettingsConfig {
    pub address: String,
    pub mtu: i32,
}

#[derive(Debug, Deserialize)]
pub struct SecretsConfig {
    pub path: String,
}

pub fn load_options() -> Result<Config, ConfigurationError> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let path = CONFIG_PATH.replacen("~", &home, 1);
    let settings = match ConfigRs::builder()
        .add_source(config::File::with_name(&path))
        .add_source(config::Environment::with_prefix("VPNSKY"))
        .build()
    {
        Ok(r) => r,
        Err(e) => return Err(errors::ConfigurationError::CouldNotLoadConfig(e).into()),
    };

    match settings.try_deserialize() {
        Ok(r) => return Ok(r),
        Err(_) => return Err(ConfigurationError::CouldNotDeserializeConfig).into(),
    }
}

pub fn get_options() -> &'static Config {
    CONFIG.get().expect("options not initialized")
}
