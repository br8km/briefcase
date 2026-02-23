pub mod backup;
pub mod clean;
pub mod config;
pub mod crypto;
pub mod schedule;
pub mod sync;
pub mod uninstall;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "briefcase")]
#[command(about = "CLI application for backing up personal sensitive data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration
    Config(config::ConfigArgs),
    /// Perform backup
    Backup(backup::BackupArgs),
    /// Sync to remote storage
    Sync(sync::SyncArgs),
    /// Schedule operations
    Schedule(schedule::ScheduleArgs),
    /// Crypto operations
    Crypto(crypto::CryptoArgs),
    /// Clean data and logs
    Clean(clean::CleanArgs),
    /// Uninstall application
    Uninstall(uninstall::UninstallArgs),
}

pub async fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Config(args) => config::run(args).await,
        Commands::Backup(args) => backup::run(args).await,
        Commands::Sync(args) => sync::run(args).await,
        Commands::Schedule(args) => schedule::run(args).await,
        Commands::Crypto(args) => crypto::run(args).await,
        Commands::Clean(args) => clean::run(args).await,
        Commands::Uninstall(args) => uninstall::run(args).await,
    }
}
