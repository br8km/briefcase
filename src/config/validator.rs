// Configuration validator module
// Handles configuration validation

use crate::config::Config;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid retention value: {0} (must be 1-10)")]
    InvalidRetention(u8),
    
    #[error("Invalid source directory: {0}")]
    InvalidSourceDirectory(PathBuf),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Validate configuration
pub fn validate_config(config: &Config) -> Result<(), ValidationError> {
    // Validate retention
    if config.general.max_retention < 1 || config.general.max_retention > 10 {
        return Err(ValidationError::InvalidRetention(config.general.max_retention));
    }
    
    // Validate sources
    for source in &config.sources {
        if source.enabled && !source.dir.exists() {
            return Err(ValidationError::InvalidSourceDirectory(source.dir.clone()));
        }
    }
    
    Ok(())
}

/// Validate password strength
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 12 {
        return Err(ValidationError::InvalidConfig("Password must be at least 12 characters".to_string()));
    }
    
    Ok(())
}