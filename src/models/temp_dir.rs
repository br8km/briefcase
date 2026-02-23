use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempDir {
    pub path: PathBuf,
    pub created_at: DateTime<Local>,
    pub size_limit: u64,
}

impl TempDir {
    pub fn new(size_limit: u64) -> std::io::Result<Self> {
        let temp_path =
            std::env::temp_dir().join(format!("briefcase_{}", Local::now().timestamp()));
        std::fs::create_dir_all(&temp_path)?;

        Ok(Self {
            path: temp_path,
            created_at: Local::now(),
            size_limit,
        })
    }

    pub fn size(&self) -> std::io::Result<u64> {
        fn calculate_size(path: &PathBuf) -> std::io::Result<u64> {
            let mut size = 0u64;
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let metadata = entry.metadata()?;
                if metadata.is_dir() {
                    size += calculate_size(&entry.path())?;
                } else {
                    size += metadata.len();
                }
            }
            Ok(size)
        }
        calculate_size(&self.path)
    }

    pub fn is_within_limit(&self) -> std::io::Result<bool> {
        Ok(self.size()? <= self.size_limit)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        if self.path.exists() {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
}
