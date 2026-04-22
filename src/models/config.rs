use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::path::PathBuf;

const LAST_BACKUP_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    #[serde(
        default,
        skip_serializing,
        deserialize_with = "deserialize_optional_local_datetime"
    )]
    pub last_backup: Option<DateTime<Local>>,
    #[serde(
        default,
        serialize_with = "serialize_optional_local_datetime",
        deserialize_with = "deserialize_optional_local_datetime"
    )]
    pub last_sync: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirefoxSource {
    pub enabled: bool,
    pub dir: PathBuf,
    pub frequency: Frequency,
    #[serde(
        default,
        serialize_with = "serialize_optional_local_datetime",
        deserialize_with = "deserialize_optional_local_datetime"
    )]
    pub last_backup: Option<DateTime<Local>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderSource {
    pub enabled: bool,
    pub dir: PathBuf,
    pub frequency: Frequency,
    #[serde(
        default,
        serialize_with = "serialize_optional_local_datetime",
        deserialize_with = "deserialize_optional_local_datetime"
    )]
    pub last_backup: Option<DateTime<Local>>,
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
                    last_backup: None,
                },
                folder: FolderSource {
                    enabled: false,
                    dir: PathBuf::from("/path/to/sensitive/folder"),
                    frequency: Frequency::Daily,
                    last_backup: None,
                },
                last_backup: None,
                last_sync: None,
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

fn serialize_optional_local_datetime<S>(
    value: &Option<DateTime<Local>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(timestamp) => serializer.serialize_some(
            &timestamp
                .naive_local()
                .format(LAST_BACKUP_FORMAT)
                .to_string(),
        ),
        None => serializer.serialize_none(),
    }
}

fn deserialize_optional_local_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<String>::deserialize(deserializer)?;
    let Some(value) = value else {
        return Ok(None);
    };

    let naive = NaiveDateTime::parse_from_str(&value, LAST_BACKUP_FORMAT)
        .map_err(serde::de::Error::custom)?;

    Local
        .from_local_datetime(&naive)
        .single()
        .map(Some)
        .ok_or_else(|| serde::de::Error::custom("invalid local timestamp"))
}
