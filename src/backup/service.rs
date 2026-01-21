use crate::backup::{compress, firefox, folder, retention};

use crate::models::backup_file::{BackupFile, SourceType};
use crate::models::config::Config;
use crate::models::temp_dir::TempDir;
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct BackupService {
    config: Arc<Mutex<Config>>,
    backup_dir: PathBuf,
}

impl BackupService {
    pub fn new(config: Arc<Mutex<Config>>, backup_dir: PathBuf) -> Self {
        Self { config, backup_dir }
    }

    pub async fn perform_backup(&self, password: &str) -> anyhow::Result<Vec<BackupFile>> {
        info!("Starting backup process");
        let mut backup_files = Vec::new();
        let temp_dir = TempDir::new(10 * 1024 * 1024)?; // 10MB limit
        let config = self.config.lock().await;

        // Firefox backup
        if config.source.firefox.enabled {
            info!("Backing up Firefox data");
            firefox::export_firefox_data(&config.source.firefox.dir, &temp_dir.path).await?;
            let backup_file = self
                .create_backup_file(&temp_dir.path, SourceType::Firefox, password)
                .await?;
            backup_files.push(backup_file);
        }

        // Folder backup
        if config.source.folder.enabled {
            info!("Backing up sensitive folder");
            folder::copy_sensitive_folder(&config.source.folder.dir, &temp_dir.path).await?;
            let backup_file = self
                .create_backup_file(&temp_dir.path, SourceType::Folder, password)
                .await?;
            backup_files.push(backup_file);
        }

        // Enforce retention
        let data_dir = self.get_data_dir();
        retention::enforce_retention(&data_dir, config.general.max_retention)?;

        // Cleanup temp
        drop(temp_dir);
        info!("Backup completed successfully");

        Ok(backup_files)
    }

    async fn create_backup_file(
        &self,
        temp_dir: &std::path::Path,
        source_type: SourceType,
        password: &str,
    ) -> anyhow::Result<BackupFile> {
        let datetime = Utc::now();
        let filename = format!(
            "{}_{}.7z",
            match source_type {
                SourceType::Firefox => "Firefox",
                SourceType::Folder => "Folder",
            },
            datetime.format("%Y-%m-%d_%H-%M-%S")
        );

        let data_dir = self.get_data_dir();
        std::fs::create_dir_all(&data_dir)?;
        let archive_path = data_dir.join(&filename);

        // Compress temp dir
        compress::compress_directory(temp_dir, &archive_path, Some(password))?;

        // Create backup file record
        let mut backup_file = BackupFile::new(archive_path, source_type);
        backup_file.datetime = datetime;
        backup_file.size = std::fs::metadata(&backup_file.path)?.len();
        // Hash would be calculated here

        Ok(backup_file)
    }

    fn get_data_dir(&self) -> PathBuf {
        self.backup_dir.clone()
    }
}
