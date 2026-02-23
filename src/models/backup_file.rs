use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupFile {
    pub path: PathBuf,
    pub datetime: DateTime<Local>,
    pub size: u64,
    pub source_type: SourceType,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceType {
    Firefox,
    Folder,
}

impl BackupFile {
    pub fn new(path: PathBuf, source_type: SourceType) -> Self {
        Self {
            path,
            datetime: Local::now(),
            size: 0,
            source_type,
            hash: String::new(),
        }
    }

    pub fn filename(&self) -> String {
        let source = match self.source_type {
            SourceType::Firefox => "Firefox",
            SourceType::Folder => "Folder",
        };
        format!(
            "{}_{}.7z",
            source,
            self.datetime.format("%Y-%m-%d_%H-%M-%S")
        )
    }
}
