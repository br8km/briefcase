// Hash generation module
// Handles PasswordHash generation

use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HashGenError {
    #[error("Hash generation failed: {0}")]
    GenerationError(String),
}

/// Generate PasswordHash from PasswordKey and timestamp
pub fn generate_password_hash(password_key: &str, timestamp: &str) -> Result<String, HashGenError> {
    let mut hasher = Sha256::new();
    hasher.update(password_key.as_bytes());
    hasher.update(timestamp.as_bytes());
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}