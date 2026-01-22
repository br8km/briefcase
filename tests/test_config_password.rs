use briefcase::config::{load_config, save_config, verify_password};
use briefcase::models::config::Config;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");

        // Test password hashing during config init simulation
        let test_password = "mySecretPassword123";
        let test_hint = "My secret hint";

        // Generate a proper Argon2 hash like the CLI does
        use argon2::password_hash::{rand_core::OsRng, SaltString};
        use argon2::{Argon2, PasswordHasher};

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(test_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        let mut config = Config::default();
        config.general.password_hash = password_hash;
        config.general.password_hint = test_hint.to_string();

        save_config(&config, &config_path).unwrap();

        // Load and verify the config
        let loaded_config = load_config(&config_path).unwrap();
        assert_eq!(loaded_config.general.password_hint, test_hint);

        // Test password verification
        assert!(verify_password(&loaded_config, test_password).unwrap());
        assert!(!verify_password(&loaded_config, "wrongpassword").unwrap());

        // Test with empty password key
        let empty_config = Config::default();
        assert!(!verify_password(&empty_config, test_password).unwrap());
    }
}
