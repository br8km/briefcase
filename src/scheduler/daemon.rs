use crate::backup::service::BackupService;
use crate::models::backup_file::BackupFile;
use crate::models::config::Config;
use crate::scheduler::service::SchedulerService;
use base64::{engine::general_purpose, Engine as _};
use log::{error, info};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

pub struct Daemon {
    config: Arc<Mutex<Config>>,
    backup_service: BackupService,
    data_dir: PathBuf,
    force_backup: bool,
}

impl Daemon {
    pub fn new(config: Config, force_backup: bool) -> Self {
        let config = Arc::new(Mutex::new(config.clone()));

        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("briefcase")
            .join("data");

        let backup_service = BackupService::new(config.clone(), data_dir.clone());

        Self {
            config,
            backup_service,
            data_dir,
            force_backup,
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
        let last_backup = config.source.last_backup;

        let force = self.force_backup;

        if config.source.firefox.enabled {
            if force || SchedulerService::is_backup_due(last_backup, config.source.firefox.frequency) {
                info!("Firefox backup is due, starting backup");
                if let Err(e) = self.run_backup("firefox").await {
                    error!("Firefox backup failed: {}", e);
                }
            } else {
                info!("Firefox backup not due yet");
            }
        }

        if config.source.folder.enabled {
            if force || SchedulerService::is_backup_due(last_backup, config.source.folder.frequency) {
                info!("Folder backup is due, starting backup");
                if let Err(e) = self.run_backup("folder").await {
                    error!("Folder backup failed: {}", e);
                }
            } else {
                info!("Folder backup not due yet");
            }
        }
    }

    async fn run_backup(&self, source: &str) -> anyhow::Result<()> {
        info!("Running scheduled backup for {}", source);

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

        let has_remotes = config.remote.remotes.values().any(|remote| remote.enabled);
        drop(config);

        let backup_files = self
            .backup_service
            .perform_backup_with_key(&encryption_key)
            .await?;
        info!("Created {} backup files", backup_files.len());

        if has_remotes {
            self.run_sync(&backup_files).await?;
        }

        if let Err(e) = crate::config::update_last_backup() {
            error!("Failed to update last_backup time: {}", e);
        }

        info!("Scheduled backup completed for {}", source);
        Ok(())
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
            service
                .sync_backups(backup_files, &self.data_dir, false)
                .await?;
            info!("Automated sync completed");
        }

        Ok(())
    }
}
