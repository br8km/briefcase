// Personal Data Backup System - Briefcase
// MVP Implementation: Configuration Setup (User Story 1)

use clap::{Parser, Subcommand};
use log::{info, LevelFilter};
use env_logger::Builder;
use anyhow::Context;


mod backup;
mod config;
mod crypto;
mod models;
mod logging;

/// Personal Data Backup System CLI
#[derive(Parser, Debug)]
#[command(name = "briefcase")]
#[command(version = "0.1.0")]
#[command(about = "Secure backup system for personal sensitive data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize configuration
    Setup,
    
    /// Perform backup operations
    Backup {
        /// Backup all enabled sources
        #[arg(short, long)]
        all: bool,
        
        /// Backup Firefox profile
        #[arg(long)]
        firefox: bool,
        
        /// Backup specific folder source
        #[arg(long)]
        folder: Option<String>,
        
        /// Force backup even if recent
        #[arg(long)]
        force: bool,
        
        /// Show what would be backed up (dry run)
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Synchronize backups to remote storage
    Sync {
        /// Sync to all enabled remotes
        #[arg(short, long)]
        all: bool,
        
        /// Sync to Dropbox
        #[arg(long)]
        dropbox: bool,
        
        /// Sync to OneDrive
        #[arg(long)]
        onedrive: bool,
        
        /// Sync to SSH server
        #[arg(long)]
        ssh: bool,
        
        /// Show what would be synced (dry run)
        #[arg(long)]
        dry_run: bool,
        
        /// Retry previously failed syncs
        #[arg(long)]
        retry_failed: bool,
    },
    
    /// Cryptographic operations
    Crypto {
        #[command(subcommand)]
        crypto_command: CryptoCommands,
    },
    
    /// System service management
    Service {
        #[command(subcommand)]
        service_command: ServiceCommands,
    },
    
    /// Configuration management
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
    
    /// Retention policy management
    Retention {
        #[command(subcommand)]
        retention_command: RetentionCommands,
    },
    
    /// Logging operations
    Log {
        #[command(subcommand)]
        log_command: LogCommands,
    },
}

#[derive(Subcommand, Debug)]
enum CryptoCommands {
    /// Generate password key
    GenerateKey {
        /// Password for key generation
        #[arg(short, long)]
        password: String,
    },
    
    /// Verify password key
    VerifyKey {
        /// Password for verification
        #[arg(short, long)]
        password: String,
        
        /// Key to verify
        #[arg(short, long)]
        key: String,
    },
    
    /// Show password hash for backup file
    ShowHash {
        /// Password for hash generation
        #[arg(short, long)]
        password: String,
        
        /// Backup filename
        #[arg(long)]
        filename: String,
    },
}

#[derive(Subcommand, Debug)]
enum ServiceCommands {
    /// Install system service
    Install {
        /// Schedule (daily, weekly)
        #[arg(long, default_value = "daily")]
        schedule: String,
        
        /// Install as user service
        #[arg(long)]
        user: bool,
        
        /// Install as system service
        #[arg(long)]
        system: bool,
    },
    
    /// Uninstall system service
    Uninstall,
    
    /// Start service
    Start,
    
    /// Stop service
    Stop,
    
    /// Restart service
    Restart,
    
    /// Show service status
    Status,
}

#[derive(Subcommand, Debug)]
enum ConfigCommands {
    /// Show current configuration
    Show {
        /// Show full configuration including sensitive data
        #[arg(long)]
        full: bool,
    },
    
    /// Edit configuration
    Edit,
    
    /// Validate configuration
    Validate,
}

#[derive(Subcommand, Debug)]
enum RetentionCommands {
    /// Apply retention policy (clean up old backups)
    Apply {
        /// Dry run - show what would be removed
        #[arg(long)]
        dry_run: bool,
    },
    
    /// Show retention status
    Status,
}

#[derive(Subcommand, Debug)]
enum LogCommands {
    /// Show logs
    Show {
        /// Filter by log level
        #[arg(long)]
        level: Option<String>,
        
        /// Limit number of entries
        #[arg(long, default_value = "100")]
        limit: usize,
        
        /// Follow log output
        #[arg(long)]
        follow: bool,
        
        /// Show logs since time
        #[arg(long)]
        since: Option<String>,
    },
    
    /// Clear logs
    Clear {
        /// Force clear without confirmation
        #[arg(long)]
        force: bool,
    },
}

// Backup helper functions
fn backup_all_sources(config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting backup of all enabled sources");
    
    let enabled_sources: Vec<_> = config.sources.iter()
        .filter(|s| s.enabled)
        .collect();
    
    if enabled_sources.is_empty() {
        println!("📋 No enabled sources found for backup");
        return Ok(());
    }
    
    println!("🔄 Backing up {} enabled sources:", enabled_sources.len());
    
    for source in enabled_sources {
        println!("- Backing up: {} ({:?})", source.name, source.source_type);
        
        match source.source_type {
            config::SourceType::Firefox => {
                let _operation = backup::firefox::perform_firefox_backup(config, &source.name)?;
                println!("  ✅ Completed: Backup operation created");
            }
            _ => {
                println!("  ℹ️  Source type backup not implemented in MVP");
            }
        }
    }
    
    Ok(())
}

fn backup_firefox(config: &config::Config, _force: bool, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Firefox backup process");
    
    // Find Firefox source
    let firefox_source = config.sources.iter()
        .find(|s| matches!(s.source_type, config::SourceType::Firefox))
        .with_context(|| "No Firefox source configured")?;
    
    if dry_run {
        println!("📋 Dry run: Would back up Firefox profile from {:?}", firefox_source.dir);
        println!("  - Profile path: {:?}", firefox_source.dir);
        println!("  - Last backup: {:?}", firefox_source.last_backup);
        println!("  - Backup count: {}", firefox_source.backup_count);
        return Ok(());
    }
    
    let operation = backup::firefox::perform_firefox_backup(config, &firefox_source.name)?;
    
    println!("✅ Firefox backup completed successfully!");
    println!("  - File size: {} bytes", operation.file_size);
    println!("  - Status: {:?}", operation.status);
    if let Some(completed_at) = operation.completed_at {
        println!("  - Duration: {:?}", completed_at.signed_duration_since(operation.started_at));
    }
    
    Ok(())
}

fn backup_folder(config: &config::Config, folder_name: &str, _force: bool, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting folder backup: {}", folder_name);
    
    // Find folder source
    let folder_source = config.sources.iter()
        .find(|s| s.name == folder_name)
        .with_context(|| "Folder source not found")?;
    
    if dry_run {
        println!("📋 Dry run: Would back up folder from {:?}", folder_source.dir);
        return Ok(());
    }
    
    println!("ℹ️  Folder backup not implemented in MVP");
    
    Ok(())
}

fn show_backup_status(config: &config::Config) -> Result<(), Box<dyn std::error::Error>> {
    info!("Showing backup status");
    
    println!("📊 Backup Status");
    println!("================");
    
    for source in &config.sources {
        println!("\n📁 Source: {} ({:?})", source.name, source.source_type);
        println!("  Status: {}", if source.enabled { "Enabled" } else { "Disabled" });
        println!("  Location: {:?}", source.dir);
        println!("  Frequency: {:?}", source.frequency);
        println!("  Last backup: {:?}", source.last_backup);
        println!("  Backup count: {}", source.backup_count);
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let cli = Cli::parse();
    
    let log_level = if cli.verbose {
        LevelFilter::Debug
    } else if cli.quiet {
        LevelFilter::Error
    } else {
        LevelFilter::Info
    };
    
    Builder::new()
        .filter_level(log_level)
        .init();
    
    info!("Starting Briefcase Personal Data Backup System");
    info!("Version: 0.1.0 (MVP - Configuration Setup)");
    
    match &cli.command {
        Commands::Setup => {
            info!("Running setup command");
            config::setup::run_setup()?;
        },
        Commands::Backup { all, firefox, folder, force, dry_run } => {
            info!("Running backup command");
            
            // Load configuration
            let config_path = config::setup::get_config_path()?;
            if !config_path.exists() {
                eprintln!("❌ Error: Configuration not found. Please run 'briefcase setup' first.");
                return Ok(());
            }
            
            let config_content = std::fs::read_to_string(&config_path)?;
            let config: config::Config = toml::from_str(&config_content)?;
            
            // Handle different backup scenarios
            if *all {
                info!("Backing up all enabled sources");
                backup_all_sources(&config)?;
            } else if *firefox {
                info!("Backing up Firefox profile");
                backup_firefox(&config, *force, *dry_run)?;
            } else if let Some(folder_name) = folder {
                info!("Backing up specific folder: {}", folder_name);
                backup_folder(&config, &folder_name, *force, *dry_run)?;
            } else {
                // Default: show backup status
                show_backup_status(&config)?;
            }
        },
        Commands::Sync { .. } => {
            eprintln!("Sync functionality not yet implemented in MVP");
            eprintln!("This will be available in User Stories 4-5 (Cloud/SSH Sync)");
        },
        Commands::Crypto { .. } => {
            eprintln!("Crypto functionality not yet implemented in MVP");
            eprintln!("This will be available in User Story 6 (CLI Utilities)");
        },
        Commands::Service { .. } => {
            eprintln!("Service functionality not yet implemented in MVP");
            eprintln!("This will be available in User Story 7 (System Service)");
        },
        Commands::Config { .. } => {
            eprintln!("Config functionality not yet implemented in MVP");
            eprintln!("This will be available in User Story 1 (Configuration)");
        },
        Commands::Retention { .. } => {
            eprintln!("Retention functionality not yet implemented in MVP");
            eprintln!("This will be available in User Story 8 (Retention Policy)");
        },
        Commands::Log { .. } => {
            eprintln!("Log functionality not yet implemented in MVP");
            eprintln!("This will be available in Polish phase");
        },
    }
    
    info!("Operation completed");
    Ok(())
}
