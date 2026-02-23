use crate::config;
use crate::scheduler::daemon::Daemon;
use anyhow::Result;
use clap::{Args, Subcommand};
use nix::unistd::{fork, ForkResult};
use std::fs;
use std::path::PathBuf;
use tokio::runtime::Runtime;

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
    Start(StartArgs),
    /// Stop the backup daemon
    Stop,
    /// Show daemon status
    Status,
}

#[derive(Args)]
pub struct StartArgs {
    /// Detach the daemon to run in background
    #[arg(long)]
    pub detach: bool,
}

pub async fn run(args: ScheduleArgs) -> Result<()> {
    match args.command {
        ScheduleCommands::Start(start_args) => {
            // Check if daemon is already running
            if is_daemon_running() {
                println!("Daemon is already running. Use 'stop' to stop it first.");
                return Ok(());
            }

            println!("Starting backup daemon...");

            let config_path = config::get_config_path()?;
            let config = config::load_config(&config_path)?;

            let daemon = Daemon::new(config);

            if start_args.detach {
                println!("Daemon will run in background");
                match unsafe { fork() } {
                    Ok(ForkResult::Parent { .. }) => {
                        // Parent process exits immediately
                        return Ok(());
                    }
                    Ok(ForkResult::Child) => {
                        // Child continues as daemon
                        // Create PID file in child
                        let pid_file = get_pid_file();
                        let pid = std::process::id().to_string();
                        let _ = fs::write(&pid_file, pid);

                        // Spawn a new thread with its own runtime to run the daemon
                        let child_thread = std::thread::spawn(move || {
                            let rt = Runtime::new().unwrap();
                            let result = rt.block_on(async { daemon.run().await });
                            if let Err(e) = result {
                                eprintln!("Daemon error: {}", e);
                            }
                        });

                        // The main thread of the child process can exit
                        // The daemon runs in the spawned thread
                        child_thread.join().unwrap();

                        // Clean up PID file on exit (in thread, but for simplicity)
                        let _ = fs::remove_file(&pid_file);
                    }
                    Err(_) => {
                        return Err(anyhow::anyhow!("Failed to fork daemon process"));
                    }
                }
            } else {
                println!("Daemon will run in foreground (use --detach to background it)");
                // Create PID file
                let pid_file = get_pid_file();
                if let Ok(pid) = std::process::id().to_string().parse::<u32>() {
                    let _ = fs::write(&pid_file, pid.to_string());
                }

                // Run in foreground
                let result = daemon.run().await;

                // Clean up PID file on exit
                let _ = fs::remove_file(&pid_file);

                result?;
            }
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
