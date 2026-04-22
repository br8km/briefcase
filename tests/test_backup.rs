use briefcase::backup::service::BackupService;
use briefcase::models::config::Config;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::Mutex;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_backup_workflow_firefox() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        std::fs::create_dir(&backup_dir).unwrap();

        let mut config = Config::default();
        config.source.firefox.enabled = true;
        config.source.firefox.dir = temp_dir.path().join("firefox_profile");
        std::fs::create_dir(&config.source.firefox.dir).unwrap();

        // Create mock Firefox data
        let places_sqlite = config.source.firefox.dir.join("places.sqlite");
        std::fs::write(&places_sqlite, "mock firefox data").unwrap();

        let service = BackupService::new(Arc::new(Mutex::new(config)), backup_dir.clone());
        let result = service.perform_backup("testpassword").await;
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].path.exists());
        assert!(files[0].filename().starts_with("Firefox_"));
    }

    #[tokio::test]
    async fn test_backup_workflow_folder() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        std::fs::create_dir(&backup_dir).unwrap();

        let mut config = Config::default();
        config.source.folder.enabled = true;
        config.source.folder.dir = temp_dir.path().join("sensitive_data");
        std::fs::create_dir(&config.source.folder.dir).unwrap();

        // Create mock data
        let test_file = config.source.folder.dir.join("secret.txt");
        std::fs::write(&test_file, "sensitive information").unwrap();

        let service = BackupService::new(Arc::new(Mutex::new(config)), backup_dir.clone());
        let result = service.perform_backup("testpassword").await;
        assert!(result.is_ok());

        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].path.exists());
        assert!(files[0].filename().starts_with("Folder_"));
    }

    #[tokio::test]
    async fn test_backup_retention_is_applied_per_source() {
        let temp_dir = tempdir().unwrap();
        let backup_dir = temp_dir.path().join("backups");
        std::fs::create_dir(&backup_dir).unwrap();

        for file_name in [
            "Firefox_2026-04-22_08-00-00.7z",
            "Firefox_2026-04-22_09-00-00.7z",
            "Folder_2026-04-22_08-00-00.7z",
            "Folder_2026-04-22_09-00-00.7z",
        ] {
            std::fs::write(backup_dir.join(file_name), b"old backup").unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }

        let mut config = Config::default();
        config.general.max_retention = 2;
        config.source.firefox.enabled = true;
        config.source.firefox.dir = temp_dir.path().join("firefox_profile");
        std::fs::create_dir(&config.source.firefox.dir).unwrap();
        std::fs::write(
            config.source.firefox.dir.join("places.sqlite"),
            "mock firefox data",
        )
        .unwrap();

        config.source.folder.enabled = true;
        config.source.folder.dir = temp_dir.path().join("sensitive_data");
        std::fs::create_dir(&config.source.folder.dir).unwrap();
        std::fs::write(
            config.source.folder.dir.join("secret.txt"),
            "sensitive information",
        )
        .unwrap();

        let service = BackupService::new(Arc::new(Mutex::new(config)), backup_dir.clone());
        let result = service.perform_backup("testpassword").await;
        assert!(result.is_ok());

        let firefox_count = std::fs::read_dir(&backup_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Firefox_") && name.ends_with(".7z"))
            })
            .count();
        let folder_count = std::fs::read_dir(&backup_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Folder_") && name.ends_with(".7z"))
            })
            .count();

        assert_eq!(firefox_count, 2);
        assert_eq!(folder_count, 2);
    }
}
