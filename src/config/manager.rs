// Configuration manager module
// Handles configuration file operations

use crate::config::{Config, GeneralConfig};
use std::path::PathBuf;
use std::fs;

use log::info;

/// Load configuration from file
pub fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(config)?;
    fs::write(path, toml_string)?;
    info!("Configuration saved to {:?}", path);
    Ok(())
}

/// Create default configuration
pub fn create_default_config() -> Config {
    Config {
        general: GeneralConfig::default(),
        sources: Vec::new(),
        remotes: Vec::new(),
    }
}