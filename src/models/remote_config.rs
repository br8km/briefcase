use serde::{Deserialize, Serialize};

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
