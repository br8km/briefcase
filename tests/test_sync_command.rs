use briefcase::cli::sync::SyncArgs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_args_valid() {
        let args = SyncArgs {
            dry_run: false,
            providers: vec![],
        };
        assert!(!args.dry_run);
        assert!(args.providers.is_empty());
    }

    #[test]
    fn test_sync_args_with_dry_run() {
        let args = SyncArgs {
            dry_run: true,
            providers: vec![],
        };
        assert!(args.dry_run);
    }

    #[test]
    fn test_sync_args_with_providers() {
        let args = SyncArgs {
            dry_run: false,
            providers: vec!["dropbox".to_string(), "onedrive".to_string()],
        };
        assert_eq!(args.providers, vec!["dropbox", "onedrive"]);
    }
}
