use crate::config;
use crate::sync::service::SyncService;
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct SyncArgs {
    /// Dry run
    #[arg(long)]
    pub dry_run: bool,

    /// Specific providers to sync (dropbox, onedrive, icloud, sftp)
    #[arg(long, value_delimiter = ',')]
    pub providers: Vec<String>,
}

pub async fn run(args: SyncArgs) -> Result<()> {
    let config_path = config::get_config_path()?;
    let config = config::load_config(&config_path)?;

    let service = SyncService::new(config.clone());

    // Validate remotes
    service.validate_remotes()?;

    // For now, assume we have backup files to sync
    // In real implementation, would get list of recent backups
    let backup_files = Vec::new(); // TODO: Get actual backup files

    service.sync_backups(&backup_files, args.dry_run).await?;

    Ok(())
}
