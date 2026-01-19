// Backup models
// Defines backup-related data structures

use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupOperation {
    pub id: Uuid,
    pub source_type: String,
    pub source_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: BackupStatus,
    pub file_path: PathBuf,
    pub file_size: u64,
    pub password_hash: String,
    pub encryption_algorithm: String,
    pub compression_algorithm: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FirefoxProfileData {
    pub profile_path: PathBuf,
    pub bookmarks: Vec<Bookmark>,
    pub passwords: Vec<SavedPassword>,
    pub export_timestamp: DateTime<Utc>,
    pub firefox_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub date_added: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub tags: Vec<String>,
    pub folder: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SavedPassword {
    pub id: String,
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub date_created: DateTime<Utc>,
    pub date_last_used: DateTime<Utc>,
}