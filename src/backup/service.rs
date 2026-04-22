use crate::backup::{compress, firefox, folder, retention};

use crate::models::backup_file::{BackupFile, SourceType};
use crate::models::config::Config;
use crate::models::temp_dir::TempDir;
use chrono::Local;
use log::{info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BackupService {
    config: Arc<Mutex<Config>>,
    backup_dir: PathBuf,
}

impl BackupService {
    pub fn new(config: Arc<Mutex<Config>>, backup_dir: PathBuf) -> Self {
        Self { config, backup_dir }
    }

    pub async fn perform_backup(&self, password: &str) -> anyhow::Result<Vec<BackupFile>> {
        // Derive encryption key from password for backward compatibility
        let salt_bytes: [u8; 32] = [
            98, 114, 105, 101, 102, 99, 97, 115, 101, 95, 101, 110, 99, 114, 121, 112, 116, 105,
            111, 110, 95, 115, 97, 108, 116, 95, 49, 50, 51, 52, 53, 54,
        ]; // "briefcase_encryption_salt_12345678" as bytes, padded to 32
        let encryption_key = crate::crypto::encrypt::derive_key(password, &salt_bytes)?;

        self.perform_backup_with_key(&encryption_key).await
    }

    pub async fn perform_backup_with_key(
        &self,
        encryption_key: &[u8; 32],
    ) -> anyhow::Result<Vec<BackupFile>> {
        info!("Starting backup process");
        let mut backup_files = Vec::new();
        let config = self.config.lock().await;

        // Firefox backup
        if config.source.firefox.enabled {
            let backup_file = self
                .backup_source_with_key(SourceType::Firefox, encryption_key, &config)
                .await?;
            backup_files.push(backup_file);
        }

        // Folder backup
        if config.source.folder.enabled {
            let backup_file = self
                .backup_source_with_key(SourceType::Folder, encryption_key, &config)
                .await?;
            backup_files.push(backup_file);
        }

        // Enforce retention policy
        let max_retention = config.general.max_retention;
        drop(config); // Release lock before retention check
        for backup_file in &backup_files {
            if let Err(e) = retention::enforce_retention(
                &self.backup_dir,
                &backup_file.source_type,
                max_retention,
            ) {
                warn!(
                    "Failed to enforce retention policy for {} backups: {}",
                    match backup_file.source_type {
                        SourceType::Firefox => "Firefox",
                        SourceType::Folder => "Folder",
                    },
                    e
                );
            }
        }

        let mut config = self.config.lock().await;
        for backup_file in &backup_files {
            match backup_file.source_type {
                SourceType::Firefox => {
                    config.source.firefox.last_backup = Some(backup_file.datetime)
                }
                SourceType::Folder => config.source.folder.last_backup = Some(backup_file.datetime),
            }
        }
        drop(config);

        info!("Backup completed successfully");

        Ok(backup_files)
    }

    pub async fn perform_source_backup_with_key(
        &self,
        source_type: SourceType,
        encryption_key: &[u8; 32],
    ) -> anyhow::Result<Vec<BackupFile>> {
        info!("Starting backup process for {:?}", source_type);
        let config = self.config.lock().await;
        let backup_file = self
            .backup_source_with_key(source_type, encryption_key, &config)
            .await?;
        let max_retention = config.general.max_retention;
        drop(config);

        if let Err(e) =
            retention::enforce_retention(&self.backup_dir, &backup_file.source_type, max_retention)
        {
            warn!(
                "Failed to enforce retention policy for {} backups: {}",
                match backup_file.source_type {
                    SourceType::Firefox => "Firefox",
                    SourceType::Folder => "Folder",
                },
                e
            );
        }

        let mut config = self.config.lock().await;
        match backup_file.source_type {
            SourceType::Firefox => config.source.firefox.last_backup = Some(backup_file.datetime),
            SourceType::Folder => config.source.folder.last_backup = Some(backup_file.datetime),
        }
        drop(config);

        info!("Backup completed successfully");

        Ok(vec![backup_file])
    }

    async fn backup_source_with_key(
        &self,
        source_type: SourceType,
        encryption_key: &[u8; 32],
        config: &Config,
    ) -> anyhow::Result<BackupFile> {
        let temp_dir = TempDir::new(32 * 1024 * 1024)?;

        match source_type {
            SourceType::Firefox => {
                info!("Backing up Firefox data");
                firefox::export_firefox_data(&config.source.firefox.dir, &temp_dir.path).await?;

                if !temp_dir.is_within_limit()? {
                    return Err(anyhow::anyhow!(
                        "Firefox backup data exceeds size limit (32MB). Current size: {} bytes",
                        temp_dir.size()?
                    ));
                }
            }
            SourceType::Folder => {
                info!("Backing up sensitive folder");
                folder::copy_sensitive_folder(&config.source.folder.dir, &temp_dir.path).await?;
            }
        }

        self.create_backup_file_with_key(&temp_dir.path, source_type, encryption_key)
            .await
    }

    async fn create_backup_file_with_key(
        &self,
        temp_dir: &std::path::Path,
        source_type: SourceType,
        encryption_key: &[u8; 32],
    ) -> anyhow::Result<BackupFile> {
        let datetime = Local::now();
        let filename = format!(
            "{}_{}.7z",
            match source_type {
                SourceType::Firefox => "Firefox",
                SourceType::Folder => "Folder",
            },
            datetime.format("%Y-%m-%d_%H-%M-%S")
        );

        let data_dir = self.get_data_dir();

        // First create the compressed archive
        let temp_archive_path = data_dir.join(format!("{}.temp", filename));
        compress::compress_directory(temp_dir, &temp_archive_path, None)?;

        // Then encrypt the compressed archive
        let encrypted_archive_path = data_dir.join(&filename);
        crate::crypto::encrypt::encrypt_file_with_derived_key(
            &temp_archive_path,
            &encrypted_archive_path,
            encryption_key,
        )?;

        // Clean up the unencrypted temp file
        if temp_archive_path.exists() {
            std::fs::remove_file(&temp_archive_path)?;
        }

        // Create backup file record
        let mut backup_file = BackupFile::new(encrypted_archive_path, source_type);
        backup_file.datetime = datetime;
        backup_file.size = std::fs::metadata(&backup_file.path)?.len();
        // Hash would be calculated here

        Ok(backup_file)
    }

    fn get_data_dir(&self) -> PathBuf {
        self.backup_dir.clone()
    }
}
