use briefcase::backup::service::BackupService;
use briefcase::sync::service::SyncService;
use rusqlite::Connection;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_places_db(path: &std::path::Path) {
        let connection = Connection::open(path).unwrap();
        connection
            .execute_batch(
                "CREATE TABLE moz_places (
                    id INTEGER PRIMARY KEY,
                    url TEXT
                );
                CREATE TABLE moz_bookmarks (
                    id INTEGER PRIMARY KEY,
                    parent INTEGER,
                    position INTEGER,
                    title TEXT,
                    type INTEGER,
                    fk INTEGER
                );
                INSERT INTO moz_bookmarks (id, parent, position, title, type, fk)
                VALUES (1, 0, 0, 'root', 2, NULL);
                INSERT INTO moz_bookmarks (id, parent, position, title, type, fk)
                VALUES (2, 1, 0, 'menu', 2, NULL);
                INSERT INTO moz_places (id, url)
                VALUES (1, 'https://example.com');
                INSERT INTO moz_bookmarks (id, parent, position, title, type, fk)
                VALUES (3, 2, 0, 'Example', 1, 1);",
            )
            .unwrap();
    }

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
        create_places_db(&places_sqlite);

        // Perform backup
        let config = std::sync::Arc::new(tokio::sync::Mutex::new(config));
        let backup_service = BackupService::new(config.clone(), backup_dir.clone());
        let backup_files = backup_service.perform_backup("testpassword").await.unwrap();
        assert_eq!(backup_files.len(), 1);
        assert!(config.lock().await.source.firefox.last_backup.is_some());

        // Check sync service can be created
        let sync_service = SyncService::new(config.lock().await.clone());
        sync_service.validate_remotes().await.unwrap();

        // Files should exist
        for file in backup_files {
            assert!(file.path.exists());
        }
    }
}
