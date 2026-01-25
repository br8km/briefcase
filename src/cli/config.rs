use crate::config;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use clap::Args;
use std::path::PathBuf;

#[derive(Args)]
pub struct ConfigArgs {
    /// Action: init, edit, validate, show, verify
    pub action: String,

    /// Password for key generation (required for init) or verification (required for verify)
    #[arg(long)]
    pub password: Option<String>,

    /// Password hint
    #[arg(long)]
    pub password_hint: Option<String>,

    /// Custom config file path
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

pub async fn run(args: ConfigArgs) -> Result<()> {
    let config_path = args
        .file
        .unwrap_or_else(|| config::get_config_path().unwrap());

    // Initialize directories when initializing config
    if args.action == "init" {
        crate::config::get_data_dir()?;
        crate::config::get_log_dir()?;
    }

    match args.action.as_str() {
        "init" => {
            let password = args
                .password
                .ok_or_else(|| anyhow::anyhow!("Password required for init"))?;
            let hint = args
                .password_hint
                .ok_or_else(|| anyhow::anyhow!("Password hint required for init"))?;

            // Generate password hash for verification and encryption key
            use crate::crypto::encrypt;
            use argon2::password_hash::{rand_core::OsRng, SaltString};
            use argon2::{Argon2, PasswordHasher};

            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            // Hash for verification
            let password_hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
                .to_string();

            // Derived key for encryption (fixed salt for consistency)
            let salt_bytes: [u8; 32] = [
                98, 114, 105, 101, 102, 99, 97, 115, 101, 95, 101, 110, 99, 114, 121, 112, 116,
                105, 111, 110, 95, 115, 97, 108, 116, 95, 49, 50, 51, 52, 53, 54,
            ]; // "briefcase_encryption_salt_12345678" as bytes, padded to 32
            let encryption_key_bytes = encrypt::derive_key(&password, &salt_bytes)
                .map_err(|e| anyhow::anyhow!("Key derivation failed: {}", e))?;
            let encryption_key = general_purpose::STANDARD.encode(encryption_key_bytes);

            let mut config = crate::models::config::Config::default();
            config.general.password_hash = password_hash;
            config.general.encryption_key = encryption_key;
            config.general.password_hint = hint;
            config::save_config(&config, &config_path)?;
            println!("Config initialized at {:?}", config_path);
        }
        "validate" => {
            let config = config::load_config(&config_path)?;
            config::validate_config(&config)?;
            println!("Config is valid");
        }
        "show" => {
            let config = config::load_config(&config_path)?;
            println!("{}", toml::to_string_pretty(&config)?);
        }
        "verify" => {
            let password = args
                .password
                .ok_or_else(|| anyhow::anyhow!("Password required for verify"))?;
            let config = config::load_config(&config_path)?;

            if config::verify_password(&config, &password)? {
                println!("Password verified successfully");
            } else {
                println!("Password verification failed");
            }
        }
        "edit" => {
            // Open editor
            println!("Edit config at {:?}", config_path);
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid action: {}", args.action));
        }
    }
    Ok(())
}
