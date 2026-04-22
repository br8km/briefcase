use crate::models::backup_file::SourceType;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn enforce_retention(
    data_dir: &PathBuf,
    source_type: &SourceType,
    max_retention: u32,
) -> Result<()> {
    let source_prefix = match source_type {
        SourceType::Firefox => "Firefox_",
        SourceType::Folder => "Folder_",
    };

    let mut entries: Vec<_> = fs::read_dir(data_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("7z"))
        .filter(|e| {
            e.path()
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(source_prefix))
        })
        .collect();

    // Sort by modified time, oldest first
    entries.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());

    if entries.len() > max_retention as usize {
        let to_remove = entries.len() - max_retention as usize;
        for entry in entries.into_iter().take(to_remove) {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tempfile::tempdir;

    #[test]
    fn test_enforce_retention_counts_each_source_separately() {
        let temp_dir = tempdir().unwrap();
        let data_dir = temp_dir.path().to_path_buf();

        for file_name in [
            "Firefox_2026-04-22_10-00-00.7z",
            "Firefox_2026-04-22_10-05-00.7z",
            "Firefox_2026-04-22_10-10-00.7z",
            "Folder_2026-04-22_10-00-00.7z",
            "Folder_2026-04-22_10-05-00.7z",
            "Folder_2026-04-22_10-10-00.7z",
        ] {
            fs::write(data_dir.join(file_name), b"backup").unwrap();
            thread::sleep(Duration::from_millis(20));
        }

        enforce_retention(&data_dir, &SourceType::Firefox, 2).unwrap();

        assert!(!data_dir.join("Firefox_2026-04-22_10-00-00.7z").exists());
        assert!(data_dir.join("Firefox_2026-04-22_10-05-00.7z").exists());
        assert!(data_dir.join("Firefox_2026-04-22_10-10-00.7z").exists());

        assert!(data_dir.join("Folder_2026-04-22_10-00-00.7z").exists());
        assert!(data_dir.join("Folder_2026-04-22_10-05-00.7z").exists());
        assert!(data_dir.join("Folder_2026-04-22_10-10-00.7z").exists());
    }
}
