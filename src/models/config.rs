use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub source: SourceConfig,
    pub remote: RemoteConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub password_hint: String,
    pub password_hash: String,  // Argon2 hash for verification
    pub encryption_key: String, // Derived key for AES encryption/decryption
    pub max_retention: u32,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub firefox: FirefoxSource,
    pub folder: FolderSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirefoxSource {
    pub enabled: bool,
    pub dir: PathBuf,
    pub frequency: Frequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderSource {
    pub enabled: bool,
    pub dir: PathBuf,
    pub frequency: Frequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Frequency {
    Hourly,
    Daily,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub dropbox: Option<DropboxConfig>,
    pub onedrive: Option<OneDriveConfig>,
    pub icloud: Option<ICloudConfig>,
    pub sftp: Option<SFTPConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropboxConfig {
    pub enabled: bool,
    pub app_key: String,
    pub app_secret: String,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneDriveConfig {
    pub enabled: bool,
    pub client_id: String,
    pub client_secret: String,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICloudConfig {
    pub enabled: bool,
    pub apple_id: String,
    pub client_id: String,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFTPConfig {
    pub enabled: bool,
    pub username: String,
    pub ipaddr: String,
    pub port: u16,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub socks_proxy: Option<String>,
    pub no_proxy: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                password_hint: "What is your favorite color?".to_string(),
                password_hash: String::new(),
                encryption_key: String::new(),
                max_retention: 10,
                http_proxy: None,
                https_proxy: None,
                socks_proxy: None,
                no_proxy: Some("localhost,127.0.0.1".to_string()),
            },
            source: SourceConfig {
                firefox: FirefoxSource {
                    enabled: false,
                    dir: PathBuf::from("/path/to/firefox/profile"),
                    frequency: Frequency::Daily,
                },
                folder: FolderSource {
                    enabled: false,
                    dir: PathBuf::from("/path/to/sensitive/folder"),
                    frequency: Frequency::Daily,
                },
            },
            remote: RemoteConfig {
                dropbox: Some(DropboxConfig {
                    enabled: false,
                    app_key: String::new(),
                    app_secret: String::new(),
                    http_proxy: None,
                    https_proxy: None,
                    socks_proxy: None,
                    no_proxy: Some("localhost,127.0.0.1".to_string()),
                }),
                onedrive: Some(OneDriveConfig {
                    enabled: false,
                    client_id: String::new(),
                    client_secret: String::new(),
                    http_proxy: None,
                    https_proxy: None,
                    socks_proxy: None,
                    no_proxy: Some("localhost,127.0.0.1".to_string()),
                }),
                icloud: Some(ICloudConfig {
                    enabled: false,
                    apple_id: String::new(),
                    client_id: String::new(),
                    http_proxy: None,
                    https_proxy: None,
                    socks_proxy: None,
                    no_proxy: Some("localhost,127.0.0.1".to_string()),
                }),
                sftp: Some(SFTPConfig {
                    enabled: false,
                    username: String::new(),
                    ipaddr: String::new(),
                    port: 22,
                    http_proxy: None,
                    https_proxy: None,
                    socks_proxy: None,
                    no_proxy: Some("localhost,127.0.0.1".to_string()),
                }),
            },
        }
    }
}
