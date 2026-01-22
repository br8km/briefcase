use briefcase::cli;
use clap::Parser;

#[cfg(test)]
mod tests {
    use super::*;

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
}
