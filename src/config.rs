use crate::errors::ConfigurationError;
use config::Config as ConfigRs;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::errors;

pub const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml");
// pub const ALT_CONFIG_PATH: &str = "~/.config/vpnsky.toml";

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
    let settings = match ConfigRs::builder()
        .add_source(config::File::with_name(CONFIG_PATH))
        // .add_source(config::File::with_name(ALT_CONFIG_PATH))
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
