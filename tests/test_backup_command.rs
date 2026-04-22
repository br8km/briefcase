use briefcase::cli::backup::BackupArgs;
use briefcase::config;
use briefcase::models::config::Config;
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose, Engine as _};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn configure_test_env(base_dir: &std::path::Path) {
        std::env::set_var("XDG_CONFIG_HOME", base_dir.join("config-home"));
        std::env::set_var("XDG_DATA_HOME", base_dir.join("data-home"));
    }

    #[test]
    fn test_backup_args_dry_run() {
        let args = BackupArgs { dry_run: true };
        assert!(args.dry_run);
    }

    #[test]
    fn test_backup_args_default() {
        let args = BackupArgs { dry_run: false };
        assert!(!args.dry_run);
    }

    #[tokio::test]
    async fn test_backup_command_persists_last_backup() {
        let _guard = env_lock().lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        configure_test_env(temp_dir.path());

        let source_dir = temp_dir.path().join("source");
        std::fs::create_dir_all(&source_dir).unwrap();
        std::fs::write(source_dir.join("secret.txt"), "classified").unwrap();

        let data_dir = temp_dir
            .path()
            .join("data-home")
            .join("briefcase")
            .join("data");
        std::fs::create_dir_all(&data_dir).unwrap();

        let mut config = Config::default();
        config.general.encryption_key = general_purpose::STANDARD.encode([7u8; 32]);
        config.source.folder.enabled = true;
        config.source.folder.dir = source_dir;

        let config_path = config::get_config_path().unwrap();
        config::save_config(&config, &config_path).unwrap();

        briefcase::cli::backup::run(BackupArgs { dry_run: false })
            .await
            .unwrap();

        let updated = config::load_config(&config_path).unwrap();
        assert!(updated.source.last_backup.is_some());
    }
}
