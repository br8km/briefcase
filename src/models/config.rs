use chrono::{DateTime, Local};
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
    pub text_editor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub firefox: FirefoxSource,
    pub folder: FolderSource,
    pub last_backup: Option<DateTime<Local>>,
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

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub remotes: HashMap<String, RemoteProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteProvider {
    pub name: String,
    pub enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                password_hint: "What is your favorite color?".to_string(),
                password_hash: String::new(),
                encryption_key: String::new(),
                max_retention: 10,
                text_editor: None,
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
                last_backup: None,
            },
            remote: RemoteConfig {
                remotes: {
                    let mut remotes = HashMap::new();
                    remotes.insert(
                        "dropbox".to_string(),
                        RemoteProvider {
                            name: "dropbox".to_string(),
                            enabled: false,
                        },
                    );
                    remotes.insert(
                        "onedrive".to_string(),
                        RemoteProvider {
                            name: "onedrive".to_string(),
                            enabled: false,
                        },
                    );
                    remotes.insert(
                        "iclouddrive".to_string(),
                        RemoteProvider {
                            name: "iclouddrive".to_string(),
                            enabled: false,
                        },
                    );
                    remotes.insert(
                        "sftp".to_string(),
                        RemoteProvider {
                            name: "sftp".to_string(),
                            enabled: false,
                        },
                    );
                    remotes
                },
            },
        }
    }
}
