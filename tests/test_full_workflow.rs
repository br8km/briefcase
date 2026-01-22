use briefcase::backup::service::BackupService;
use briefcase::sync::service::SyncService;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_full_backup_workflow() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        std::fs::create_dir(&backup_dir).unwrap();

        // Create config with Firefox source
        let mut config = briefcase::models::config::Config::default();
        config.source.firefox.enabled = true;
        config.source.firefox.dir = temp_dir.path().join("firefox_profile");
        std::fs::create_dir(&config.source.firefox.dir).unwrap();

        // Create mock Firefox data
        let places_sqlite = config.source.firefox.dir.join("places.sqlite");
        std::fs::write(&places_sqlite, "mock firefox bookmarks").unwrap();

        // Perform backup
        let backup_service = BackupService::new(
            std::sync::Arc::new(tokio::sync::Mutex::new(config.clone())),
            backup_dir.clone(),
        );
        let backup_files = backup_service.perform_backup("testpassword").await.unwrap();
        assert_eq!(backup_files.len(), 1);

        // Check sync service can be created
        let sync_service = SyncService::new(config);
        sync_service.validate_remotes().unwrap();

        // Files should exist
        for file in backup_files {
            assert!(file.path.exists());
        }
    }
}
