use briefcase::config::validate_config;
use briefcase::models::config::{DropboxConfig, ICloudConfig, OneDriveConfig, SFTPConfig};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dropbox_config_valid() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.dropbox = Some(DropboxConfig {
            enabled: true,
            app_key: "key".to_string(),
            app_secret: "secret".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_dropbox_config_missing_secret() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.dropbox = Some(DropboxConfig {
            enabled: true,
            app_key: "key".to_string(),
            app_secret: "".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }

    #[test]
    fn test_validate_onedrive_config_valid() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.onedrive = Some(OneDriveConfig {
            enabled: true,
            client_id: "id".to_string(),
            client_secret: "secret".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_icloud_config_valid() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.icloud = Some(ICloudConfig {
            enabled: true,
            apple_id: "apple".to_string(),
            client_id: "client".to_string(),
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_sftp_config_valid() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.sftp = Some(SFTPConfig {
            enabled: true,
            username: "user".to_string(),
            ipaddr: "127.0.0.1".to_string(),
            port: 22,
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_validate_sftp_config_missing_username() {
        let mut config = briefcase::models::config::Config::default();
        config.remote.sftp = Some(SFTPConfig {
            enabled: true,
            username: "".to_string(),
            ipaddr: "127.0.0.1".to_string(),
            port: 22,
            http_proxy: None,
            https_proxy: None,
            socks_proxy: None,
            no_proxy: None,
        });
        assert!(validate_config(&config).is_err());
    }
}
