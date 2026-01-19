// Key generation module
// Handles PasswordKey generation

use sha2::{Sha256, Digest};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyGenError {
    #[error("Key generation failed: {0}")]
    GenerationError(String),
}

/// Generate PasswordKey from password
pub fn generate_password_key(password: &str) -> Result<String, KeyGenError> {
    if password.len() < 12 {
        return Err(KeyGenError::GenerationError("Password too short".to_string()));
    }
    
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    
    Ok(format!("{:x}", result))
}