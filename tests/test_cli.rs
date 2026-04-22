use briefcase::cli;
use clap::Parser;

#[cfg(test)]
mod tests {
    use super::*;
    use briefcase::cli::config::ConfigArgs;

    #[test]
    fn test_cli_parsing() {
        // Test that CLI can parse basic commands
        let args = ["briefcase", "backup", "--dry-run"];
        let cli = cli::Cli::parse_from(args);
        assert!(matches!(cli.command, cli::Commands::Backup(_)));
    }

    #[test]
    fn test_cli_config_command() {
        let args = ["briefcase", "config", "validate"];
        let cli = cli::Cli::parse_from(args);
        assert!(matches!(cli.command, cli::Commands::Config(_)));
    }

    #[test]
    fn test_cli_sync_command() {
        let args = ["briefcase", "sync", "--dry-run"];
        let cli = cli::Cli::parse_from(args);
        assert!(matches!(cli.command, cli::Commands::Sync(_)));
    }

    #[test]
    fn test_cli_schedule_command() {
        let args = ["briefcase", "schedule", "start"];
        let cli = cli::Cli::parse_from(args);
        assert!(matches!(cli.command, cli::Commands::Schedule(_)));
    }

    #[tokio::test]
    async fn test_cli_config_validate_rejects_invalid_remote_config() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        std::fs::write(
            &config_path,
            r#"
[general]
password_hint = "hint"
password_hash = ""
encryption_key = ""
max_retention = 10

[source]
last_sync = "2026-04-22 14:42:18"

[source.firefox]
enabled = false
dir = "/tmp/firefox"
frequency = "Daily"

[source.folder]
enabled = false
dir = "/tmp/folder"
frequency = "Daily"

[remote]
remotes = "dropbox"
"#,
        )
        .unwrap();

        let result = briefcase::cli::config::run(ConfigArgs {
            action: "validate".to_string(),
            password: None,
            password_hint: None,
            file: Some(config_path),
            editor: None,
        })
        .await;

        assert!(result.is_err());
    }
}
