use briefcase::models::backup_file::{BackupFile, SourceType};
use briefcase::models::config::Config;
use briefcase::sync::service::SyncService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_service_creation() {
        let config = Config::default();
        let _service = SyncService::new(config);
        // Just check it creates without error
        assert!(true);
    }

    #[tokio::test]
    async fn test_sync_service_validate_remotes_no_remotes() {
        let config = Config::default();
        let service = SyncService::new(config);
        assert!(service.validate_remotes().await.is_ok());
    }

    #[tokio::test]
    async fn test_actual_sync_with_configured_rclone() {
        let mut config = Config::default();
        // Assume first remote is configured, e.g., dropbox
        if let Some(remote) = config.remote.remotes.values_mut().next() {
            remote.enabled = true;
        }

        let service = SyncService::new(config.clone());
        // Only run if remotes are configured and connection succeeds
        if service.validate_remotes().await.is_ok() {
            // Create a temporary file to sync
            use tempfile::tempdir;
            let temp_dir = tempdir().unwrap();
            let temp_file_path = temp_dir.path().join("test_backup.7z");
            std::fs::write(&temp_file_path, "test content").unwrap();

            let backup_files = vec![BackupFile::new(temp_file_path.clone(), SourceType::Folder)];

            // Perform actual sync (not dry run)
            let result = service.sync_backups(&backup_files, false).await;
            assert!(
                result.is_ok(),
                "Sync should succeed when rclone is configured"
            );
        } else {
            // Skip if no remotes configured
            println!("Skipping actual sync test: No configured remotes");
        }
    }
}
