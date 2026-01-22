use aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use anyhow::Result;
use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordVerifier};
use rand::RngCore;
use std::fs;
use std::path::Path;

pub fn derive_key(password_key: &str, salt: &[u8; 32]) -> Result<[u8; 32]> {
    let salt_string =
        SaltString::encode_b64(salt).map_err(|e| anyhow::anyhow!("Salt encoding failed: {}", e))?;
    let argon2 = Argon2::default();
    let mut key = [0u8; 32];

    argon2
        .hash_password_into(
            password_key.as_bytes(),
            salt_string.as_str().as_bytes(),
            &mut key,
        )
        .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;

    Ok(key)
}

pub fn encrypt_file_with_derived_key(
    input_path: &Path,
    output_path: &Path,
    encryption_key: &[u8; 32],
) -> Result<()> {
    let input_data = fs::read(input_path)?;
    let mut salt = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut salt);
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(encryption_key);
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

pub fn decrypt_file_with_derived_key(
    input_path: &Path,
    output_path: &Path,
    derived_key: &[u8; 32],
) -> Result<()> {
    let encrypted_data = fs::read(input_path)?;
    if encrypted_data.len() < 44 {
        return Err(anyhow::anyhow!("Invalid encrypted data"));
    }

    let _salt = &encrypted_data[0..32];
    let nonce = &encrypted_data[32..44];
    let ciphertext = &encrypted_data[44..];

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(derived_key);
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

pub fn encrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    // For backward compatibility - derive key from password
    let salt_bytes: [u8; 32] = [
        98, 114, 105, 101, 102, 99, 97, 115, 101, 95, 101, 110, 99, 114, 121, 112, 116, 105, 111,
        110, 95, 115, 97, 108, 116, 95, 49, 50, 51, 52, 53, 54,
    ]; // "briefcase_encryption_salt_123456" as bytes, padded to 32
    let derived_key = derive_key(password, &salt_bytes)?;
    encrypt_file_with_derived_key(input_path, output_path, &derived_key)
}

pub fn decrypt_file(input_path: &Path, output_path: &Path, password: &str) -> Result<()> {
    // For backward compatibility - derive key from password
    let salt_bytes: [u8; 32] = [
        98, 114, 105, 101, 102, 99, 97, 115, 101, 95, 101, 110, 99, 114, 121, 112, 116, 105, 111,
        110, 95, 115, 97, 108, 116, 95, 49, 50, 51, 52, 53, 54,
    ]; // "briefcase_encryption_salt_123456" as bytes, padded to 32
    let derived_key = derive_key(password, &salt_bytes)?;
    decrypt_file_with_derived_key(input_path, output_path, &derived_key)
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;

    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
