use briefcase::config::{get_config_path, load_config, save_config, validate_config, Config};
use briefcase::models::config::{
    DropboxConfig, Frequency, ICloudConfig, OneDriveConfig, SFTPConfig,
};
use std::path::PathBuf;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_validate_invalid_retention_zero() {
        let mut config = Config::default();
        config.general.max_retention = 0;
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_invalid_retention_too_high() {
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

    #[test]
    fn test_validate_missing_folder_dir() {
        let mut config = Config::default();
        config.source.folder.enabled = true;
        config.source.folder.dir = PathBuf::from("/nonexistent");
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_dropbox_missing_credentials() {
        let mut config = Config::default();
        config.remote.dropbox = Some(DropboxConfig {
            enabled: true,
            app_key: String::new(),
            app_secret: "secret".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_onedrive_missing_credentials() {
        let mut config = Config::default();
        config.remote.onedrive = Some(OneDriveConfig {
            enabled: true,
            client_id: String::new(),
            client_secret: "secret".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_icloud_missing_credentials() {
        let mut config = Config::default();
        config.remote.icloud = Some(ICloudConfig {
            enabled: true,
            apple_id: String::new(),
            client_id: "client".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_sftp_missing_credentials() {
        let mut config = Config::default();
        config.remote.sftp = Some(SFTPConfig {
            enabled: true,
            username: String::new(),
            ipaddr: "127.0.0.1".to_string(),
            port: 22,
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_load_config_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");
        let config = load_config(&config_path).unwrap();
        assert_eq!(config.general.max_retention, 10); // default
    }

    #[test]
    fn test_load_config_valid_file() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let mut config = Config::default();
        config.general.max_retention = 5;
        save_config(&config, &config_path).unwrap();
        let loaded = load_config(&config_path).unwrap();
        assert_eq!(loaded.general.max_retention, 5);
    }

    #[test]
    fn test_load_config_invalid_toml() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("invalid.toml");
        std::fs::write(&config_path, "invalid toml content").unwrap();
        assert!(load_config(&config_path).is_err());
    }

    #[test]
    fn test_save_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let config = Config::default();
        save_config(&config, &config_path).unwrap();
        assert!(config_path.exists());
        let content = std::fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("max_retention = 10"));
    }

    #[test]
    fn test_get_config_path() {
        let path = get_config_path().unwrap();
        assert!(path.ends_with("briefcase.toml"));
    }
}
