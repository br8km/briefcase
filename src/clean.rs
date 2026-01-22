use anyhow::Result;
use log::info;
use std::path::PathBuf;
use tokio::fs;

pub async fn clean_temp_files(temp_dir: &PathBuf) -> Result<()> {
    if temp_dir.exists() {
        info!("Cleaning temp directory: {}", temp_dir.display());
        fs::remove_dir_all(temp_dir).await?;
    }
    Ok(())
}

pub fn clean_old_logs(log_dir: &PathBuf, keep_files: usize) -> Result<()> {
    if !log_dir.exists() {
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(log_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("log"))
        .collect();

    // Sort by modified time, oldest first
    entries.sort_by_key(|e| e.metadata().unwrap().modified().unwrap());

    if entries.len() > keep_files {
        let to_remove = entries.len() - keep_files;
        for entry in entries.into_iter().take(to_remove) {
            std::fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}
