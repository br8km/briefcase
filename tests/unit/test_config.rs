use briefcase::config::{get_config_path, load_config, save_config, validate_config, Config};
use briefcase::models::config::{Frequency, RemoteProvider};
use std::collections::HashMap;
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
        // Check that default remotes are configured but disabled
        assert_eq!(config.remote.remotes.len(), 4);
        assert!(!config.remote.remotes.get("dropbox").unwrap().enabled);
        assert!(!config.remote.remotes.get("onedrive").unwrap().enabled);
        assert!(!config.remote.remotes.get("iclouddrive").unwrap().enabled);
        assert!(!config.remote.remotes.get("sftp").unwrap().enabled);
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
    fn test_validate_remote_with_empty_name() {
        let mut config = Config::default();
        config.remote.remotes.insert(
            "test".to_string(),
            RemoteProvider {
                name: "".to_string(),
                enabled: true,
            },
        );
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_multiple_enabled_remotes() {
        let mut config = Config::default();
        config.remote.remotes.insert(
            "dropbox".to_string(),
            RemoteProvider {
                name: "dropbox".to_string(),
                enabled: true,
            },
        );
        config.remote.remotes.insert(
            "onedrive".to_string(),
            RemoteProvider {
                name: "onedrive".to_string(),
                enabled: true,
            },
        );
        assert!(validate_config(&config).is_ok());
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
