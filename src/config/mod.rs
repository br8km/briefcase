// Configuration module for Briefcase
// Handles all configuration management and setup

pub mod setup;
pub mod manager;
pub mod validator;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Main configuration structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: GeneralConfig,
    pub sources: Vec<SourceConfig>,
    pub remotes: Vec<RemoteConfig>,
}

/// General configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub password_hint: String,
    pub password_key: Option<String>, // Will be encrypted at rest
    pub max_retention: u8, // 1-10
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub log_level: String,
}

/// Source configuration
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

/// Remote configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteConfig {
    pub name: String,
    pub remote_type: RemoteType,
    pub enabled: bool,
    pub api_key: Option<String>, // Will be encrypted at rest
    pub username: Option<String>,
    pub ipaddr: Option<String>,
    pub port: Option<u16>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub last_sync: Option<DateTime<Utc>>,
}

/// Source types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SourceType {
    Firefox,
    Folder,
}

/// Remote types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RemoteType {
    Dropbox,
    OneDrive,
    SSH,
}

/// Backup frequency
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupFrequency {
    Daily,
    Weekly,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig::default(),
            sources: Vec::new(),
            remotes: Vec::new(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            password_hint: "What is your favorite color?".to_string(),
            password_key: None,
            max_retention: 10,
            http_proxy: None,
            https_proxy: None,
            no_proxy: Some("localhost".to_string()),
            log_level: "info".to_string(),
        }
    }
}