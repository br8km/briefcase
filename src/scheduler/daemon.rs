use crate::backup::service::BackupService;
use crate::models::backup_file::{BackupFile, SourceType};
use crate::models::config::Config;
use crate::scheduler::service::SchedulerService;
use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
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
        let _ = std::fs::create_dir_all(&data_dir);

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
        let legacy_last_backup = config.source.last_backup;

        let force = self.force_backup;

        if config.source.firefox.enabled {
            let last_backup = config.source.firefox.last_backup.or(legacy_last_backup);
            if force
                || SchedulerService::is_backup_due(last_backup, config.source.firefox.frequency)
            {
                info!("Firefox backup is due, starting backup");
                if let Err(e) = self.run_backup(SourceType::Firefox).await {
                    error!("Firefox backup failed: {}", e);
                }
            } else {
                info!("Firefox backup not due yet");
            }
        }

        if config.source.folder.enabled {
            let last_backup = config.source.folder.last_backup.or(legacy_last_backup);
            if force || SchedulerService::is_backup_due(last_backup, config.source.folder.frequency)
            {
                info!("Folder backup is due, starting backup");
                if let Err(e) = self.run_backup(SourceType::Folder).await {
                    error!("Folder backup failed: {}", e);
                }
            } else {
                info!("Folder backup not due yet");
            }
        }
    }

    async fn run_backup(&self, source_type: SourceType) -> anyhow::Result<()> {
        info!("Running scheduled backup for {}", source_name(source_type));

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
            .perform_source_backup_with_key(source_type, &encryption_key)
            .await?;
        info!("Created {} backup files", backup_files.len());

        let config = self.config.lock().await.clone();
        if let Err(e) = crate::config::save_current_config(&config) {
            error!(
                "Failed to persist {} last_backup time: {}",
                source_name(source_type),
                e
            );
        }

        if has_remotes {
            self.run_sync(&backup_files).await?;
        }

        info!(
            "Scheduled backup completed for {}",
            source_name(source_type)
        );
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
            let synced_remotes = service
                .sync_backups(backup_files, &self.data_dir, false)
                .await?;
            let persisted_config = {
                let mut config = self.config.lock().await;
                let sync_time = Local::now();
                for remote_key in synced_remotes {
                    if let Some(remote) = config.remote.remotes.get_mut(&remote_key) {
                        remote.last_sync = Some(sync_time);
                    }
                }
                config.clone()
            };
            if let Err(e) = crate::config::save_current_config(&persisted_config) {
                error!("Failed to persist remote last_sync times: {}", e);
            }
            info!("Automated sync completed");
        }

        Ok(())
    }
}

fn source_name(source_type: SourceType) -> &'static str {
    match source_type {
        SourceType::Firefox => "firefox",
        SourceType::Folder => "folder",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use base64::engine::general_purpose;
    use chrono::{Duration as ChronoDuration, Local};
    use std::sync::{Mutex as StdMutex, OnceLock};

    fn env_lock() -> &'static StdMutex<()> {
        static LOCK: OnceLock<StdMutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| StdMutex::new(()))
    }

    fn configure_test_env(base_dir: &std::path::Path) {
        std::env::set_var("XDG_CONFIG_HOME", base_dir.join("config-home"));
        std::env::set_var("XDG_DATA_HOME", base_dir.join("data-home"));
    }

    #[tokio::test]
    async fn test_check_and_run_backups_only_runs_due_source() {
        let _guard = env_lock().lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        configure_test_env(temp_dir.path());

        let firefox_dir = temp_dir.path().join("firefox_profile");
        std::fs::create_dir_all(&firefox_dir).unwrap();
        std::fs::write(firefox_dir.join("places.sqlite"), "mock firefox data").unwrap();

        let folder_dir = temp_dir.path().join("sensitive_data");
        std::fs::create_dir_all(&folder_dir).unwrap();
        std::fs::write(folder_dir.join("secret.txt"), "sensitive information").unwrap();

        let mut config = Config::default();
        config.general.encryption_key = general_purpose::STANDARD.encode([7u8; 32]);
        config.source.firefox.enabled = true;
        config.source.firefox.dir = firefox_dir;
        config.source.firefox.frequency = crate::models::config::Frequency::Hourly;
        let initial_firefox_backup = Local::now() - ChronoDuration::hours(2);
        config.source.firefox.last_backup = Some(initial_firefox_backup);
        config.source.folder.enabled = true;
        config.source.folder.dir = folder_dir;
        config.source.folder.frequency = crate::models::config::Frequency::Daily;
        let initial_folder_backup = Local::now() - ChronoDuration::hours(1);
        config.source.folder.last_backup = Some(initial_folder_backup);

        let config_path = config::get_config_path().unwrap();
        config::save_config(&config, &config_path).unwrap();

        let daemon = Daemon::new(config, false);
        daemon.check_and_run_backups().await;

        let updated = daemon.config.lock().await.clone();
        assert!(updated.source.firefox.last_backup.is_some());
        assert!(updated.source.folder.last_backup.is_some());
        assert!(updated.source.firefox.last_backup.unwrap() > initial_firefox_backup);
        assert_eq!(
            updated.source.folder.last_backup.unwrap(),
            initial_folder_backup
        );

        let data_dir = temp_dir
            .path()
            .join("data-home")
            .join("briefcase")
            .join("data");
        let firefox_count = std::fs::read_dir(&data_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Firefox_") && name.ends_with(".7z"))
            })
            .count();
        let folder_count = std::fs::read_dir(&data_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Folder_") && name.ends_with(".7z"))
            })
            .count();

        assert_eq!(firefox_count, 1);
        assert_eq!(folder_count, 0);
    }
}
