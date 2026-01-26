use briefcase::config::validate_config;
use briefcase::models::config::{Config, RemoteProvider};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dropbox_config_valid() {
        let mut config = Config::default();
        config.remote.remotes.insert(
            "dropbox".to_string(),
            RemoteProvider {
                name: "dropbox".to_string(),
                enabled: true,
            },
        );
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_onedrive_config_valid() {
        let mut config = Config::default();
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
    fn test_validate_iclouddrive_config_valid() {
        let mut config = Config::default();
        config.remote.remotes.insert(
            "iclouddrive".to_string(),
            RemoteProvider {
                name: "iclouddrive".to_string(),
                enabled: true,
            },
        );
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_sftp_config_valid() {
        let mut config = Config::default();
        config.remote.remotes.insert(
            "sftp".to_string(),
            RemoteProvider {
                name: "sftp".to_string(),
                enabled: true,
            },
        );
        assert!(validate_config(&config).is_ok());
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
}
