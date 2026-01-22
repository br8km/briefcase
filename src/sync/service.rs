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
        if let Some(dropbox) = &self.config.remote.dropbox {
            if dropbox.enabled {
                self.sync_to_provider(backup_files, "dropbox:", dry_run)
                    .await?;
            }
        }

        if let Some(onedrive) = &self.config.remote.onedrive {
            if onedrive.enabled {
                self.sync_to_provider(backup_files, "onedrive:", dry_run)
                    .await?;
            }
        }

        if let Some(icloud) = &self.config.remote.icloud {
            if icloud.enabled {
                self.sync_to_provider(backup_files, "icloud:", dry_run)
                    .await?;
            }
        }

        if let Some(sftp) = &self.config.remote.sftp {
            if sftp.enabled {
                let remote_path = format!(
                    "sftp:{}@{}:{}/backup",
                    sftp.username, sftp.ipaddr, sftp.port
                );
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
        for file in backup_files {
            let remote_path = format!(
                "{}{}",
                remote_base,
                file.path.file_name().unwrap().to_string_lossy()
            );

            info!("Syncing {} to {}", file.path.display(), remote_path);

            match rclone::sync_to_remote(&file.path, &remote_path, dry_run) {
                Ok(_) => info!("Successfully synced {}", file.path.display()),
                Err(e) => error!("Failed to sync {}: {}", file.path.display(), e),
            }
        }

        Ok(())
    }

    pub fn validate_remotes(&self) -> Result<()> {
        // Test connections to enabled remotes
        if let Some(dropbox) = &self.config.remote.dropbox {
            if dropbox.enabled && !rclone::test_remote_connection("dropbox:")? {
                return Err(anyhow::anyhow!("Dropbox connection failed"));
            }
        }

        if let Some(onedrive) = &self.config.remote.onedrive {
            if onedrive.enabled && !rclone::test_remote_connection("onedrive:")? {
                return Err(anyhow::anyhow!("OneDrive connection failed"));
            }
        }

        if let Some(icloud) = &self.config.remote.icloud {
            if icloud.enabled && !rclone::test_remote_connection("icloud:")? {
                return Err(anyhow::anyhow!("iCloud connection failed"));
            }
        }

        Ok(())
    }
}
