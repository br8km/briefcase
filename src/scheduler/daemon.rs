use crate::backup::service::BackupService;
use crate::models::config::Config;
use crate::models::config::Frequency;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};
use tracing::{error, info};

pub struct Daemon {
    config: Arc<Mutex<Config>>,
    backup_service: BackupService,
}

impl Daemon {
    pub fn new(config: Config) -> Self {
        let config = Arc::new(Mutex::new(config.clone()));
        let backup_service = BackupService::new(config.clone(), PathBuf::from("/tmp/backup"));

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
        let now = chrono::Utc::now();

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

        // Would need to get password securely, perhaps from keyring
        let password = "dummy_password"; // TODO: Secure password retrieval

        let backup_files = self.backup_service.perform_backup(password).await?;

        // Sync if enabled
        // TODO: Integrate sync

        info!("Scheduled backup completed for {}", source);
        Ok(())
    }
}
