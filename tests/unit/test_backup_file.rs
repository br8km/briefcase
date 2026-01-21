use briefcase::models::backup_file::{BackupFile, SourceType};
use std::path::PathBuf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_file_new() {
        let path = PathBuf::from("/tmp/test.7z");
        let backup = BackupFile::new(path.clone(), SourceType::Firefox);
        assert_eq!(backup.path, path);
        assert_eq!(backup.source_type, SourceType::Firefox);
        assert_eq!(backup.size, 0);
        assert_eq!(backup.hash, String::new());
    }

    #[test]
    fn test_backup_file_filename_firefox() {
        let path = PathBuf::from("/tmp/test.7z");
        let mut backup = BackupFile::new(path, SourceType::Firefox);
        backup.datetime = chrono::DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        assert_eq!(backup.filename(), "Firefox_2023-01-01_12-00-00.7z");
    }

    #[test]
    fn test_backup_file_filename_folder() {
        let path = PathBuf::from("/tmp/test.7z");
        let mut backup = BackupFile::new(path, SourceType::Folder);
        backup.datetime = chrono::DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        assert_eq!(backup.filename(), "Folder_2023-01-01_12-00-00.7z");
    }
}
