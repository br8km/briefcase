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

    #[test]
    fn test_sync_service_validate_remotes_no_remotes() {
        let config = Config::default();
        let service = SyncService::new(config);
        assert!(service.validate_remotes().is_ok());
    }

    // TODO: Add tests for actual sync when rclone is configured
    // For now, dry run tests would require mock rclone
}
