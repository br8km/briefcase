use crate::backup::service::BackupService;
use crate::models::backup_file::BackupFile;
use crate::models::config::Config;
use base64::{engine::general_purpose, Engine as _};
use log::{error, info};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

pub struct Daemon {
    config: Arc<Mutex<Config>>,
    backup_service: BackupService,
}

impl Daemon {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(Mutex::new(config.clone()));

        // Use the same data directory as other backup operations
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("briefcase")
            .join("data");

        let backup_service = BackupService::new(config.clone(), data_dir);

        Self {
            config,
            backup_service,
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        info!("Starting backup daemon");

        loop {
            // Check if any backups are due
            self.check_and_run_backups().await;

            // Sleep for 1 hour
            time::sleep(Duration::from_secs(3600)).await;
        }
    }

    async fn check_and_run_backups(&self) {
        let config: Config = self.config.lock().await.clone();
        let _now = chrono::Utc::now();

        // Firefox backup
        if config.source.firefox.enabled {
            if let Err(e) = self.run_backup("firefox").await {
                error!("Firefox backup failed: {}", e);
            }
        }

        // Folder backup
        if config.source.folder.enabled {
            if let Err(e) = self.run_backup("folder").await {
                error!("Folder backup failed: {}", e);
            }
        }
    }

    async fn run_backup(&self, source: &str) -> anyhow::Result<()> {
        info!("Running scheduled backup for {}", source);

        // Get encryption_key from config for automated backups
        let config = self.config.lock().await;
        if config.general.encryption_key.is_empty() {
            return Err(anyhow::anyhow!(
                "Config not initialized - no encryption key found for automated backup"
            ));
        }

        let encryption_key_bytes = general_purpose::STANDARD
            .decode(&config.general.encryption_key)
            .map_err(|e| anyhow::anyhow!("Failed to decode encryption key: {}", e))?;
        let mut encryption_key = [0u8; 32];
        encryption_key.copy_from_slice(&encryption_key_bytes);

        let backup_files = self
            .backup_service
            .perform_backup_with_key(&encryption_key)
            .await?;
        info!("Created {} backup files", backup_files.len());

        // Sync if any remote providers are enabled
        if self.has_enabled_remotes(&config) {
            drop(config); // Release lock before calling run_sync
            self.run_sync(&backup_files).await?;
        }

        info!("Scheduled backup completed for {}", source);
        Ok(())
    }

    fn has_enabled_remotes(&self, config: &Config) -> bool {
        config.remote.remotes.values().any(|remote| remote.enabled)
    }

    async fn run_sync(&self, backup_files: &[BackupFile]) -> anyhow::Result<()> {
        use crate::sync::service::SyncService;

        let config = self.config.lock().await.clone();
        let service = SyncService::new(config);

        if service.validate_remotes().await.is_ok() {
            info!(
                "Starting automated sync of {} backup files",
                backup_files.len()
            );
            service.sync_backups(backup_files, false).await?;
            info!("Automated sync completed");
        }

        Ok(())
    }
}
