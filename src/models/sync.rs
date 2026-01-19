// Sync models
// Defines synchronization-related data structures

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncOperation {
    pub id: Uuid,
    pub backup_id: Uuid,
    pub remote_type: String,
    pub remote_id: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub bytes_transferred: u64,
    pub error: Option<String>,
    pub retry_count: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RetryPending,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloudSync {
    pub remote_type: String,
    pub api_key: Option<String>,
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SSHConfig {
    pub hostname: String,
    pub username: String,
    pub port: u16,
    pub key_path: Option<String>,
    pub last_sync: Option<DateTime<Utc>>,
}