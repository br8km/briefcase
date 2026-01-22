use crate::config;
use crate::scheduler::daemon::Daemon;
use anyhow::Result;
use clap::{Args, Subcommand};
use std::fs;
use std::path::PathBuf;

fn get_pid_file() -> PathBuf {
    dirs::runtime_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("briefcase_daemon.pid")
}

fn is_daemon_running() -> bool {
    let pid_file = get_pid_file();
    if !pid_file.exists() {
        return false;
    }

    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            // On Unix systems, we can check if the process exists
            #[cfg(unix)]
            {
                use std::process::Command;
                // Simple check - try to send signal 0 (no-op signal)
                Command::new("kill")
                    .arg("-0")
                    .arg(pid.to_string())
                    .status()
                    .is_ok()
            }
            #[cfg(not(unix))]
            {
                // On non-Unix systems, just check if PID file exists
                true
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn stop_daemon() -> Result<()> {
    let pid_file = get_pid_file();
    if !pid_file.exists() {
        println!("Daemon is not running");
        return Ok(());
    }

    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            #[cfg(unix)]
            {
                use std::process::Command;
                if Command::new("kill").arg(pid.to_string()).status().is_ok() {
                    println!("Daemon stopped successfully");
                    let _ = fs::remove_file(&pid_file);
                } else {
                    println!("Failed to stop daemon");
                }
            }
            #[cfg(not(unix))]
            {
                println!("Daemon stop not implemented on this platform");
            }
        }
    }

    Ok(())
}

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

            // Create PID file
            let pid_file = get_pid_file();
            if let Ok(pid) = std::process::id().to_string().parse::<u32>() {
                let _ = fs::write(&pid_file, pid.to_string());
            }

            // In real implementation, would daemonize the process
            // For now, just run in foreground
            let result = daemon.run().await;

            // Clean up PID file on exit
            let _ = fs::remove_file(&pid_file);

            result?;
        }
        ScheduleCommands::Stop => {
            stop_daemon()?;
        }
        ScheduleCommands::Status => {
            if is_daemon_running() {
                println!("Daemon is running");
            } else {
                println!("Daemon is not running");
            }
        }
    }

    Ok(())
}
