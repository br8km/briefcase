use crate::models::config::Config;
use anyhow::{anyhow, Result};
use std::path::Path;

pub fn load_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    validate_config(&config)?;
    Ok(config)
}

pub fn save_config(config: &Config, path: &Path) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    std::fs::write(path, content)?;
    Ok(())
}

/// Verify a password against the stored password hash
pub fn verify_password(config: &Config, password: &str) -> Result<bool> {
    if config.general.password_hash.is_empty() {
        return Ok(false); // No password set
    }

    crate::crypto::encrypt::verify_password(password, &config.general.password_hash)
}

pub fn validate_config(config: &Config) -> Result<()> {
    // Validate general
    if config.general.max_retention == 0 || config.general.max_retention > 10 {
        return Err(anyhow!("max_retention must be between 1 and 10"));
    }

    // Validate sources
    if config.source.firefox.enabled && !config.source.firefox.dir.exists() {
        return Err(anyhow!(
            "Firefox directory does not exist: {:?}",
            config.source.firefox.dir
        ));
    }

    if config.source.folder.enabled && !config.source.folder.dir.exists() {
        return Err(anyhow!(
            "Folder directory does not exist: {:?}",
            config.source.folder.dir
        ));
    }

    // Validate remotes if enabled
    if let Some(dropbox) = &config.remote.dropbox {
        if dropbox.enabled && (dropbox.app_key.is_empty() || dropbox.app_secret.is_empty()) {
            return Err(anyhow!("Dropbox app_key and app_secret are required"));
        }
    }

    if let Some(onedrive) = &config.remote.onedrive {
        if onedrive.enabled && (onedrive.client_id.is_empty() || onedrive.client_secret.is_empty())
        {
            return Err(anyhow!("OneDrive client_id and client_secret are required"));
        }
    }

    if let Some(icloud) = &config.remote.icloud {
        if icloud.enabled && (icloud.apple_id.is_empty() || icloud.client_id.is_empty()) {
            return Err(anyhow!("iCloud apple_id and client_id are required"));
        }
    }

    if let Some(sftp) = &config.remote.sftp {
        if sftp.enabled && (sftp.username.is_empty() || sftp.ipaddr.is_empty()) {
            return Err(anyhow!("SFTP username and ipaddr are required"));
        }
    }

    Ok(())
}

pub fn get_config_path() -> Result<std::path::PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("Could not find config directory"))?
        .join("briefcase");

    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir.join("briefcase.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_default_config() {
        let config = Config::default();
        assert_eq!(config.general.max_retention, 10);
        assert!(!config.source.firefox.enabled);
        assert!(!config.source.folder.enabled);
    }

    #[test]
    fn test_validate_valid_config() {
        let mut config = Config::default();
        config.general.max_retention = 5;
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_invalid_retention() {
        let mut config = Config::default();
        config.general.max_retention = 15;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_missing_firefox_dir() {
        let mut config = Config::default();
        config.source.firefox.enabled = true;
        config.source.firefox.dir = PathBuf::from("/nonexistent");
        assert!(validate_config(&config).is_err());
    }
}
