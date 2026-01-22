use crate::backup::service::BackupService;
use crate::config;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use log::info;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Args)]
pub struct BackupArgs {
    /// Perform dry run without actual backup
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn run(args: BackupArgs) -> Result<()> {
    let config_path = config::get_config_path()?;
    let config = config::load_config(&config_path)?;

    if args.dry_run {
        info!("Starting dry run backup");
        println!("Dry run: Would backup enabled sources");
        return Ok(());
    }

    // Use encryption_key from config as the derived key
    if config.general.encryption_key.is_empty() {
        return Err(anyhow::anyhow!(
            "Config not initialized. Run 'briefcase config init' first."
        ));
    }

    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("briefcase")
        .join("data");

    let config_arc = Arc::new(Mutex::new(config));
    let service = BackupService::new(config_arc.clone(), data_dir);

    // Get encryption_key from the Arc<Mutex<Config>> and decode from base64
    let encryption_key_bytes = {
        let config_lock = config_arc.lock().await;
        general_purpose::STANDARD
            .decode(&config_lock.general.encryption_key)
            .map_err(|e| anyhow::anyhow!("Failed to decode encryption key: {}", e))?
    };

    let mut encryption_key = [0u8; 32];
    encryption_key.copy_from_slice(&encryption_key_bytes);

    let backup_files = service.perform_backup_with_key(&encryption_key).await?;

    for file in backup_files {
        println!("Created backup: {:?}", file.path);
    }

    Ok(())
}
