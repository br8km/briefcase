use briefcase::cli::sync::SyncArgs;

#[cfg(test)]
mod tests {
    use super::*;

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
}
