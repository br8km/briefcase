use briefcase::cli::backup::BackupArgs;

#[cfg(test)]
mod tests {
    use super::*;

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
}
