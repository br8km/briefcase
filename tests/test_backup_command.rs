use briefcase::cli::backup::BackupArgs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_args_valid() {
        let args = BackupArgs {
            password: "secret".to_string(),
            dry_run: false,
            sources: vec![],
        };
        assert_eq!(args.password, "secret");
        assert!(!args.dry_run);
        assert!(args.sources.is_empty());
    }

    #[test]
    fn test_backup_args_with_dry_run() {
        let args = BackupArgs {
            password: "secret".to_string(),
            dry_run: true,
            sources: vec![],
        };
        assert!(args.dry_run);
    }

    #[test]
    fn test_backup_args_with_sources() {
        let args = BackupArgs {
            password: "secret".to_string(),
            dry_run: false,
            sources: vec!["firefox".to_string(), "folder".to_string()],
        };
        assert_eq!(args.sources, vec!["firefox", "folder"]);
    }
}
