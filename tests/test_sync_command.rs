use briefcase::cli::sync::SyncArgs;
use briefcase::config;
use briefcase::models::config::Config;
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
mod tests {
    use super::*;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn configure_test_env(base_dir: &std::path::Path) {
        std::env::set_var("XDG_CONFIG_HOME", base_dir.join("config-home"));
        std::env::set_var("XDG_DATA_HOME", base_dir.join("data-home"));
    }

    #[test]
    fn test_sync_args_valid() {
        let args = SyncArgs { dry_run: false };
        assert!(!args.dry_run);
    }

    #[test]
    fn test_sync_args_with_dry_run() {
        let args = SyncArgs { dry_run: true };
        assert!(args.dry_run);
    }

    #[tokio::test]
    async fn test_sync_command_persists_last_sync() {
        let _guard = env_lock().lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        configure_test_env(temp_dir.path());

        let mut config = Config::default();
        config.source.last_sync = None;

        let config_path = config::get_config_path().unwrap();
        config::save_config(&config, &config_path).unwrap();

        let data_dir = temp_dir
            .path()
            .join("data-home")
            .join("briefcase")
            .join("data");
        std::fs::create_dir_all(&data_dir).unwrap();
        std::fs::write(
            data_dir.join("Folder_2026-04-22_10-00-00.7z"),
            b"mock backup",
        )
        .unwrap();

        briefcase::cli::sync::run(SyncArgs { dry_run: false })
            .await
            .unwrap();

        let updated = config::load_config(&config_path).unwrap();
        assert!(updated.source.last_sync.is_some());
    }
}
