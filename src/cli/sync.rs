use crate::config;
use crate::models::backup_file::BackupFile;
use crate::models::backup_file::SourceType;
use crate::sync::service::SyncService;
use anyhow::Result;
use clap::Args;
use std::fs;
use std::path::PathBuf;

/// Find all backup files in the data directory
fn find_backup_files(data_dir: &PathBuf) -> Result<Vec<BackupFile>> {
    let mut backup_files = Vec::new();

    if !data_dir.exists() {
        return Ok(backup_files);
    }

    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(extension) = path.extension() {
            if extension == "7z" {
                // Parse filename to determine source type
                let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                let source_type = if filename.starts_with("Firefox_") {
                    SourceType::Firefox
                } else if filename.starts_with("Folder_") {
                    SourceType::Folder
                } else {
                    continue; // Skip files with unknown format
                };

                let mut backup_file = BackupFile::new(path, source_type);
                if let Ok(metadata) = entry.metadata() {
                    backup_file.size = metadata.len();
                }

                backup_files.push(backup_file);
            }
        }
    }

    Ok(backup_files)
}

#[derive(Args)]
pub struct SyncArgs {
    /// Dry run
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run(args: SyncArgs) -> Result<()> {
    let config_path = config::get_config_path()?;
    let config = config::load_config(&config_path)?;

    let service = SyncService::new(config.clone());

    // Validate remotes
    service.validate_remotes()?;

    // Get actual backup files from the data directory
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("briefcase")
        .join("data");

    let backup_files = find_backup_files(&data_dir)?;

    service.sync_backups(&backup_files, args.dry_run).await?;

    Ok(())
}
