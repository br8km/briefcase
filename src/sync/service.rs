use crate::models::backup_file::BackupFile;
use crate::models::config::Config;
use crate::sync::rclone;
use anyhow::Result;
use log::{error, info};

pub struct SyncService {
    config: Config,
}

impl SyncService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn sync_backups(&self, backup_files: &[BackupFile], dry_run: bool) -> Result<()> {
        for remote_provider in self.config.remote.remotes.values() {
            if remote_provider.enabled {
                let remote_path = format!("{}:", remote_provider.name);
                self.sync_to_provider(backup_files, &remote_path, dry_run)
                    .await?;
            }
        }

        Ok(())
    }

    async fn sync_to_provider(
        &self,
        backup_files: &[BackupFile],
        remote_base: &str,
        dry_run: bool,
    ) -> Result<()> {
        // For sftp, create briefcase in 'upload' subfolder due to Chroot Jail
        let briefcase_dir = if remote_base.starts_with("sftp:") {
            format!("{}/upload/briefcase", remote_base.trim_end_matches('/'))
        } else {
            format!("{}/briefcase", remote_base.trim_end_matches('/'))
        };

        if dry_run {
            info!("Would create briefcase directory: {}", briefcase_dir);
        } else {
            match rclone::mkdir_remote(&briefcase_dir).await {
                Ok(_) => info!("Ensured briefcase directory exists: {}", briefcase_dir),
                Err(e) => error!(
                    "Failed to create briefcase directory {}: {}",
                    briefcase_dir, e
                ),
            }
        }

        for file in backup_files {
            let remote_path = format!(
                "{}/{}",
                briefcase_dir,
                file.path.file_name().unwrap().to_string_lossy()
            );

            info!("Syncing {} to {}", file.path.display(), remote_path);

            match rclone::sync_to_remote(&file.path, &remote_path, dry_run).await {
                Ok(_) => info!("Successfully synced {}", file.path.display()),
                Err(e) => error!("Failed to sync {}: {}", file.path.display(), e),
            }
        }

        Ok(())
    }

    pub async fn validate_remotes(&self) -> Result<()> {
        // Test connections to enabled remotes
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
