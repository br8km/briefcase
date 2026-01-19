// Configuration models
// Defines all configuration-related data structures

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub sources: Vec<SourceConfig>,
    pub remotes: Vec<RemoteConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub password_hint: String,
    pub password_key: Option<String>,
    pub max_retention: u8,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub log_level: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceConfig {
    pub name: String,
    pub source_type: SourceType,
    pub enabled: bool,
    pub dir: PathBuf,
    pub frequency: BackupFrequency,
    pub last_backup: Option<DateTime<Utc>>,
    pub backup_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteConfig {
    pub name: String,
    pub remote_type: RemoteType,
    pub enabled: bool,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub ipaddr: Option<String>,
    pub port: Option<u16>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub last_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SourceType {
    Firefox,
    Folder,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RemoteType {
    Dropbox,
    OneDrive,
    SSH,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupFrequency {
    Daily,
    Weekly,
}