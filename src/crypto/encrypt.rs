use aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use anyhow::Result;
use pbkdf2::pbkdf2;
use rand::RngCore;
use std::fs;
use std::path::Path;

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::<hmac::Hmac<sha2::Sha256>>(password.as_bytes(), salt, 10000, &mut key);
    key
}

pub fn encrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    let input_data = fs::read(input_path)?;
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    let key = derive_key(password, &salt);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let payload = Payload {
        msg: input_data.as_ref(),
        aad: &[],
    };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|e| anyhow::anyhow!("Encryption error: {}", e))?;

    // Prepend salt and nonce to ciphertext
    let mut encrypted_data = salt.to_vec();
    encrypted_data.extend_from_slice(&nonce_bytes);
    encrypted_data.extend(ciphertext);

    fs::write(output_path, encrypted_data)?;
    Ok(())
}

pub fn decrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    let encrypted_data = fs::read(input_path)?;
    if encrypted_data.len() < 44 {
        // salt(32) + nonce(12)
        return Err(anyhow::anyhow!("Invalid encrypted data"));
    }

    let salt = &encrypted_data[0..32];
    let nonce = &encrypted_data[32..44];
    let ciphertext = &encrypted_data[44..];

    let key = derive_key(password, salt);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(nonce);
    let payload = Payload {
        msg: ciphertext,
        aad: &[],
    };
    let plaintext = cipher
        .decrypt(nonce, payload)
        .map_err(|e| anyhow::anyhow!("Decryption error: {}", e))?;

    fs::write(output_path, plaintext)?;
    Ok(())
}
