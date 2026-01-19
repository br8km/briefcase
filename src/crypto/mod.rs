// Cryptography module for Briefcase
// Handles encryption, decryption, and key management

pub mod keygen;
pub mod hashgen;

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm
};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::generic_array::GenericArray;
use thiserror::Error;

/// Cryptographic error types
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    
    #[error("Invalid key length: {0}")]
    InvalidKeyLength(usize),
    
    #[error("Key derivation failed: {0}")]
    KeyDerivationError(String),
}

/// Encrypt data using AES-256-GCM
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Validate key length
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength(key.len()));
    }
    
    // Convert key to GenericArray
    let key_array = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(key_array);
    
    // Generate random nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Encrypt data
    match cipher.encrypt(&nonce, data) {
        Ok(ciphertext) => {
            // Combine nonce and ciphertext
            let mut result = nonce.to_vec();
            result.extend(ciphertext);
            Ok(result)
        },
        Err(e) => Err(CryptoError::EncryptionError(e.to_string())),
    }
}

/// Decrypt data using AES-256-GCM
pub fn decrypt(encrypted_data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    // Extract nonce (first 12 bytes) and ciphertext
    if encrypted_data.len() < 12 {
        return Err(CryptoError::DecryptionError("Data too short".to_string()));
    }
    
    let nonce = GenericArray::from_slice(&encrypted_data[0..12]);
    let ciphertext = &encrypted_data[12..];
    
    // For now, we'll need the key to be passed in - this is a placeholder
    // In real implementation, key would come from secure storage
    let dummy_key = b"0123456789abcdef0123456789abcdef"; // 32-byte dummy key
    let key_array = GenericArray::from_slice(dummy_key);
    let cipher = Aes256Gcm::new(key_array);
    
    match cipher.decrypt(nonce, ciphertext) {
        Ok(plaintext) => Ok(plaintext),
        Err(e) => Err(CryptoError::DecryptionError(e.to_string())),
    }
}

/// Generate PasswordKey from user password
pub fn generate_password_key(password: &str) -> Result<String, CryptoError> {
    // Use PBKDF2 for key derivation
    use pbkdf2::pbkdf2_hmac;
    use sha2::Sha256;
    
    // Generate random salt
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    
    // Derive key
    let mut key = [0u8; 32]; // 32 bytes for AES-256
    pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, 100000, &mut key);
    
    // Combine salt and key for storage
    let mut result = salt.to_vec();
    result.extend(&key);
    
    Ok(hex::encode(result))
}

/// Generate PasswordHash from PasswordKey and timestamp
pub fn generate_password_hash(password_key: &str, timestamp: &str) -> Result<String, CryptoError> {
    use sha2::{Sha256, Digest};
    
    // Decode the password key (which contains salt + derived key)
    let decoded_key = hex::decode(password_key)
        .map_err(|e| CryptoError::KeyDerivationError(e.to_string()))?;
    
    // Extract the derived key part (after 16-byte salt)
    if decoded_key.len() < 48 { // 16 (salt) + 32 (key)
        return Err(CryptoError::KeyDerivationError("Invalid key format".to_string()));
    }
    
    let derived_key = &decoded_key[16..48];
    
    // Create hash from key + timestamp
    let mut hasher = Sha256::new();
    hasher.update(derived_key);
    hasher.update(timestamp.as_bytes());
    
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}