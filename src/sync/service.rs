use crate::models::backup_file::BackupFile;
use crate::models::config::Config;
use crate::sync::rclone;
use anyhow::Result;
use log::{error, info};
use std::path::Path;

pub struct SyncService {
    config: Config,
}

impl SyncService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn sync_backups(
        &self,
        _backup_files: &[BackupFile],
        data_dir: &Path,
        dry_run: bool,
    ) -> Result<()> {
        for remote_provider in self.config.remote.remotes.values() {
            if remote_provider.enabled {
                let remote_path = format!("{}:", remote_provider.name);
                self.sync_folder_to_provider(data_dir, &remote_path, dry_run)
                    .await?;
            }
        }

        Ok(())
    }

    async fn sync_folder_to_provider(
        &self,
        data_dir: &Path,
        remote_base: &str,
        dry_run: bool,
    ) -> Result<()> {
        let briefcase_dir = if remote_base.starts_with("sftp:") {
            format!("{}/upload/briefcase", remote_base.trim_end_matches('/'))
        } else {
            format!("{}/briefcase", remote_base.trim_end_matches('/'))
        };

        if dry_run {
            info!(
                "Would sync folder {} to {}",
                data_dir.display(),
                briefcase_dir
            );
        } else {
            match rclone::mkdir_remote(&briefcase_dir).await {
                Ok(_) => info!("Ensured briefcase directory exists: {}", briefcase_dir),
                Err(e) => error!(
                    "Failed to create briefcase directory {}: {}",
                    briefcase_dir, e
                ),
            }
        }

        info!("Syncing folder {} to {}", data_dir.display(), briefcase_dir);

        match rclone::sync_folder_to_remote(data_dir, &briefcase_dir, dry_run).await {
            Ok(_) => info!("Successfully synced folder {}", data_dir.display()),
            Err(e) => error!("Failed to sync folder {}: {}", data_dir.display(), e),
        }

        Ok(())
    }

    pub async fn validate_remotes(&self) -> Result<()> {
        for remote_provider in self.config.remote.remotes.values() {
            if remote_provider.enabled {
                let remote_path = format!("{}:", remote_provider.name);
                if !rclone::test_remote_connection(&remote_path).await? {
                    return Err(anyhow::anyhow!(
                        "{} connection failed",
                        remote_provider.name
                    ));
                }
            }
        }

        Ok(())
    }
}
