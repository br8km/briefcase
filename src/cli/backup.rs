use crate::backup::service::BackupService;
use crate::config;
use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Args)]
pub struct BackupArgs {
    /// User password for encryption/decryption
    #[arg(long)]
    pub password: String,

    /// Perform dry run without actual backup
    #[arg(long)]
    pub dry_run: bool,

    /// Specific sources to backup (firefox, folder)
    #[arg(long, value_delimiter = ',')]
    pub sources: Vec<String>,
}

pub async fn run(args: BackupArgs) -> Result<()> {
    let config_path = config::get_config_path()?;
    let config = config::load_config(&config_path)?;

    if args.dry_run {
        println!("Dry run: Would backup enabled sources");
        return Ok(());
    }

    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("briefcase")
        .join("data");

    let config = Arc::new(Mutex::new(config));
    let service = BackupService::new(config, data_dir);
    let backup_files = service.perform_backup(&args.password).await?;

    for file in backup_files {
        println!("Created backup: {:?}", file.path);
    }

    Ok(())
}
