// Crypto models
// Defines cryptography-related data structures

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionResult {
    pub encrypted_data: Vec<u8>,
    pub algorithm: String,
    pub key_used: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DecryptionResult {
    pub decrypted_data: Vec<u8>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyPair {
    pub public_key: String,
    pub private_key: String,
    pub algorithm: String,
}