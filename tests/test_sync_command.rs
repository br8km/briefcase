use briefcase::cli::sync::SyncArgs;
use briefcase::config;
use briefcase::models::config::Config;
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn configure_test_env(base_dir: &std::path::Path) {
        std::env::set_var("XDG_CONFIG_HOME", base_dir.join("config-home"));
        std::env::set_var("XDG_DATA_HOME", base_dir.join("data-home"));
    }

    fn configure_mock_rclone(base_dir: &std::path::Path) -> std::ffi::OsString {
        let bin_dir = base_dir.join("bin");
        std::fs::create_dir_all(&bin_dir).unwrap();

        let rclone_path = bin_dir.join("rclone");
        std::fs::write(&rclone_path, "#!/bin/sh\nexit 0\n").unwrap();

        let mut permissions = std::fs::metadata(&rclone_path).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(&rclone_path, permissions).unwrap();

        let old_path = std::env::var_os("PATH").unwrap_or_default();
        let mut paths = vec![bin_dir];
        paths.extend(std::env::split_paths(&old_path));
        let new_path = std::env::join_paths(paths).unwrap();
        std::env::set_var("PATH", &new_path);

        old_path
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
    async fn test_sync_command_does_not_update_remote_last_sync_without_successful_remote() {
        let _guard = env_lock().lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        configure_test_env(temp_dir.path());

        let mut config = Config::default();
        config.remote.remotes.get_mut("dropbox").unwrap().enabled = false;
        config.remote.remotes.get_mut("dropbox").unwrap().last_sync = None;

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
        assert!(updated
            .remote
            .remotes
            .get("dropbox")
            .unwrap()
            .last_sync
            .is_none());
    }

    #[tokio::test]
    async fn test_sync_command_updates_successful_remote_last_sync() {
        let _guard = env_lock().lock().unwrap();
        let temp_dir = tempfile::tempdir().unwrap();
        configure_test_env(temp_dir.path());
        let old_path = configure_mock_rclone(temp_dir.path());

        let mut config = Config::default();
        config.remote.remotes.get_mut("dropbox").unwrap().enabled = true;
        config.remote.remotes.get_mut("dropbox").unwrap().last_sync = None;

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

        let result = briefcase::cli::sync::run(SyncArgs { dry_run: false }).await;
        std::env::set_var("PATH", &old_path);

        result.unwrap();

        let updated = config::load_config(&config_path).unwrap();
        assert!(updated
            .remote
            .remotes
            .get("dropbox")
            .unwrap()
            .last_sync
            .is_some());
    }
}
