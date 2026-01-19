// Logging module for Briefcase
// Handles all logging functionality

use log::{LevelFilter, SetLoggerError};

use env_logger::Builder;
use std::path::PathBuf;


/// Initialize logging system
pub fn init_logging(level: LevelFilter, log_file: Option<PathBuf>) -> Result<(), SetLoggerError> {
    let mut builder = Builder::new();
    
    // Set log level
    builder.filter_level(level);
    
    // If log file is specified, add file logging
    if let Some(file_path) = log_file {
        match std::fs::File::create(file_path) {
            Ok(file) => {
                builder.target(env_logger::Target::Pipe(Box::new(file)));
            }
            Err(e) => {
                eprintln!("Failed to create log file: {}", e);
                panic!("Failed to create log file: {}", e);
            }
        }
    }
    
    // Initialize logger
    builder.try_init()?;
    Ok(())
}

/// Get default log file path
pub fn get_default_log_path() -> Option<PathBuf> {
    dirs::home_dir().map(|home| {
        home.join(".cache").join("briefcase").join("log").join("briefcase.log")
    })
}

/// Ensure log directory exists
pub fn ensure_log_directory() -> Result<(), std::io::Error> {
    if let Some(log_path) = get_default_log_path() {
        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}