// Configuration setup module
// Handles initial setup and configuration creation

use crate::config::{Config, GeneralConfig, SourceConfig, RemoteConfig, SourceType, RemoteType, BackupFrequency};
use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};

use log::info;

// Mock function pointers for testing
static mut GET_USER_INPUT: Option<Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>> = None;
static mut GET_PASSWORD_INPUT: Option<Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>> = None;
static mut GET_RETENTION_INPUT: Option<Box<dyn Fn() -> Result<u8, Box<dyn std::error::Error>>>> = None;

/// Run the setup process
pub fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting configuration setup");
    
    // Get config file path
    let config_path = get_config_path()?;
    
    // Check if config already exists
    if config_path.exists() {
        info!("Configuration file already exists at {:?}", config_path);
        return Ok(());
    }
    
    // Create config directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
        info!("Created config directory at {:?}", parent);
    }
    
    // Get user input for configuration
    let config = create_config_from_input()?;
    
    // Save configuration
    save_config(&config, &config_path)?;
    
    info!("Configuration setup completed successfully");
    println!("✅ Configuration created successfully!");
    println!("📁 Config file: {:?}", config_path);
    
    Ok(())
}

/// Get the configuration file path
pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().ok_or("Could not determine home directory")?;
    let config_dir = home_dir.join(".config").join("briefcase");
    Ok(config_dir.join("briefcase.toml"))
}

/// Create configuration from user input
fn create_config_from_input() -> Result<Config, Box<dyn std::error::Error>> {
    println!("🔐 Briefcase Configuration Setup");
    println!("================================");
    
    // Get password hint
    let password_hint = get_user_input("Password hint (to help remember your password): ")?;
    
    // Get password and generate key
    let password = get_password_input("Enter your master password (min 12 chars): ")?;
    let password_key = generate_password_key(&password)?;
    
    // Get max retention
    let max_retention = get_retention_input()?;
    
    // Create general config
    let general_config = GeneralConfig {
        password_hint,
        password_key: Some(password_key),
        max_retention,
        http_proxy: None,
        https_proxy: None,
        no_proxy: Some("localhost".to_string()),
        log_level: "info".to_string(),
    };
    
    // Create default config
    let mut config = Config {
        general: general_config,
        sources: Vec::new(),
        remotes: Vec::new(),
    };
    
    // Add default Firefox source (disabled)
    config.sources.push(SourceConfig {
        name: "firefox".to_string(),
        source_type: SourceType::Firefox,
        enabled: false,
        dir: PathBuf::from("/path/to/firefox/profile"),
        frequency: BackupFrequency::Daily,
        last_backup: None,
        backup_count: 0,
    });
    
    // Add default folder source (disabled)
    config.sources.push(SourceConfig {
        name: "documents".to_string(),
        source_type: SourceType::Folder,
        enabled: false,
        dir: PathBuf::from("/path/to/sensitive/folder"),
        frequency: BackupFrequency::Daily,
        last_backup: None,
        backup_count: 0,
    });
    
    // Add default cloud remotes (disabled)
    config.remotes.push(RemoteConfig {
        name: "dropbox".to_string(),
        remote_type: RemoteType::Dropbox,
        enabled: false,
        api_key: None,
        username: None,
        ipaddr: None,
        port: None,
        http_proxy: None,
        https_proxy: None,
        last_sync: None,
    });
    
    config.remotes.push(RemoteConfig {
        name: "ssh".to_string(),
        remote_type: RemoteType::SSH,
        enabled: false,
        api_key: None,
        username: None,
        ipaddr: None,
        port: Some(22),
        http_proxy: None,
        https_proxy: None,
        last_sync: None,
    });
    
    Ok(config)
}

/// Get user input
fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if mock function is set
    unsafe {
        if let Some(func) = &GET_USER_INPUT {
            return func();
        }
    }
    
    // Normal input
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Get password input (hidden)
fn get_password_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Check if mock function is set
    unsafe {
        if let Some(func) = &GET_PASSWORD_INPUT {
            return func();
        }
    }
    
    // Normal input
    use rpassword::read_password;
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let password = read_password()?;
    
    if password.len() < 12 {
        return Err("Password must be at least 12 characters long".into());
    }
    
    Ok(password)
}

/// Get retention input
fn get_retention_input() -> Result<u8, Box<dyn std::error::Error>> {
    // Check if mock function is set
    unsafe {
        if let Some(func) = &GET_RETENTION_INPUT {
            return func();
        }
    }
    
    // Normal input
    loop {
        let input = get_user_input("Maximum backup retention (1-10, default 10): ")?;
        
        if input.is_empty() {
            return Ok(10);
        }
        
        match input.parse::<u8>() {
            Ok(value) if value >= 1 && value <= 10 => return Ok(value),
            _ => println!("Please enter a number between 1 and 10"),
        }
    }
}

/// Generate password key (placeholder - will be implemented properly)
fn generate_password_key(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    // This is a placeholder - real implementation will use proper key derivation
    // For MVP, we'll use a simple hash to demonstrate the concept
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    
    Ok(format!("{:x}", result))
}

/// Save configuration to file
fn save_config(config: &Config, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(config)?;
    fs::write(path, toml_string)?;
    
    info!("Configuration saved to {:?}", path);
    Ok(())
}

/// Mock support functions for testing
pub fn set_mock_user_input(func: Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>) {
    unsafe {
        GET_USER_INPUT = Some(func);
    }
}

pub fn set_mock_password_input(func: Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>) {
    unsafe {
        GET_PASSWORD_INPUT = Some(func);
    }
}

pub fn set_mock_retention_input(func: Box<dyn Fn() -> Result<u8, Box<dyn std::error::Error>>>) {
    unsafe {
        GET_RETENTION_INPUT = Some(func);
    }
}

pub fn clear_mocks() {
    unsafe {
        GET_USER_INPUT = None;
        GET_PASSWORD_INPUT = None;
        GET_RETENTION_INPUT = None;
    }
}