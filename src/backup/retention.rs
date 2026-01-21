use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn enforce_retention(data_dir: &PathBuf, max_retention: u32) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(data_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("7z"))
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
