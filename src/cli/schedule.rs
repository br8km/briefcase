use crate::config;
use crate::scheduler::daemon::Daemon;
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ScheduleArgs {
    #[command(subcommand)]
    pub command: ScheduleCommands,
}

#[derive(Subcommand)]
pub enum ScheduleCommands {
    /// Start the backup daemon
    Start,
    /// Stop the backup daemon
    Stop,
    /// Show daemon status
    Status,
}

pub async fn run(args: ScheduleArgs) -> Result<()> {
    match args.command {
        ScheduleCommands::Start => {
            println!("Starting backup daemon...");

            let config_path = config::get_config_path()?;
            let config = config::load_config(&config_path)?;

            let daemon = Daemon::new(config);

            // In real implementation, would daemonize the process
            // For now, just run in foreground
            daemon.run().await?;
        }
        ScheduleCommands::Stop => {
            println!("Stopping backup daemon...");
            // TODO: Send signal to running daemon
        }
        ScheduleCommands::Status => {
            println!("Daemon status: Not implemented");
            // TODO: Check if daemon is running
        }
    }

    Ok(())
}
