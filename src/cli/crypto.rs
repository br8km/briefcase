use crate::backup::compress;
use crate::config;
use crate::crypto::encrypt;
use anyhow::Result;
use clap::Args;

use base64::{engine::general_purpose, Engine as _};
use std::path::Path;

#[derive(Args)]
pub struct CryptoArgs {
    /// Action: validate, decrypt
    action: String,
    /// Input file for decrypt
    #[arg(short, long)]
    input: Option<String>,
    /// Output directory for decrypt
    #[arg(short, long)]
    output: Option<String>,
}

pub async fn run(args: CryptoArgs) -> Result<()> {
    match args.action.as_str() {
        "validate" => {
            let config_path = config::get_config_path()?;
            let config = config::load_config(&config_path)?;

            if config.general.password_hash.is_empty() {
                println!("Config not initialized - no password hash found");
                return Err(anyhow::anyhow!("Config not initialized"));
            } else {
                println!("Config validation successful - password hash present");
            }
        }
        "decrypt" => {
            let input = args
                .input
                .ok_or_else(|| anyhow::anyhow!("Input file required"))?;
            let output = args
                .output
                .ok_or_else(|| anyhow::anyhow!("Output directory required"))?;

            let input_path = Path::new(&input);
            let output_path = Path::new(&output);

            // Create temp file for decrypted 7Zip
            let temp_7z_path = output_path.with_extension("temp.7z");

            // Try to get encryption key from config first
            let encryption_key_bytes = match config::get_config_path() {
                Ok(config_path) => match config::load_config(&config_path) {
                    Ok(config) if !config.general.encryption_key.is_empty() => {
                        // Use derived key from config
                        Some(
                            general_purpose::STANDARD
                                .decode(&config.general.encryption_key)
                                .map_err(|e| {
                                    anyhow::anyhow!("Failed to decode encryption key: {}", e)
                                })?,
                        )
                    }
                    _ => None,
                },
                _ => None,
            };

            let encryption_key_bytes = match encryption_key_bytes {
                Some(key_bytes) => key_bytes,
                None => {
                    // No config available, prompt for password and derive key
                    eprintln!("No config found. Please enter your password to decrypt:");
                    let password = rpassword::read_password()
                        .map_err(|e| anyhow::anyhow!("Failed to read password: {}", e))?;

                    let salt_bytes: [u8; 32] = [
                        98, 114, 105, 101, 102, 99, 97, 115, 101, 95, 101, 110, 99, 114, 121, 112,
                        116, 105, 111, 110, 95, 115, 97, 108, 116, 95, 49, 50, 51, 52, 53, 54,
                    ]; // "briefcase_encryption_salt_12345678" as bytes, padded to 32
                    encrypt::derive_key(&password, &salt_bytes)
                        .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?
                        .to_vec()
                }
            };

            let mut encryption_key = [0u8; 32];
            encryption_key.copy_from_slice(&encryption_key_bytes);

            // Decrypt AES-encrypted file using derived key
            encrypt::decrypt_file_with_derived_key(input_path, &temp_7z_path, &encryption_key)?;

            // Extract 7Zip archive (no password needed for archives created by compress_directory)
            compress::extract_archive(&temp_7z_path, output_path)?;

            // Clean up temp file
            if temp_7z_path.exists() {
                std::fs::remove_file(&temp_7z_path)?;
            }

            println!("Decryption and extraction completed to: {}", output);
        }
        _ => {
            println!("Invalid action. Use validate or decrypt");
        }
    }

    Ok(())
}
