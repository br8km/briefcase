use briefcase::crypto::encrypt::{decrypt_file, encrypt_file};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_file() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.txt");
        let encrypted_path = temp_dir.path().join("encrypted.7z");
        let decrypted_path = temp_dir.path().join("decrypted.txt");

        let test_data = b"Hello, world! This is test data.";
        fs::write(&input_path, test_data).unwrap();

        // Encrypt
        encrypt_file(&input_path, &encrypted_path, "testpassword").unwrap();

        // Check encrypted file exists and is different
        assert!(encrypted_path.exists());
        let encrypted_data = fs::read(&encrypted_path).unwrap();
        assert_ne!(encrypted_data, test_data);

        // Decrypt
        decrypt_file(&encrypted_path, &decrypted_path, "testpassword").unwrap();

        // Check decrypted matches original
        let decrypted_data = fs::read(&decrypted_path).unwrap();
        assert_eq!(decrypted_data, test_data);
    }

    #[test]
    fn test_encrypt_wrong_password() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path().join("input.txt");
        let encrypted_path = temp_dir.path().join("encrypted.7z");
        let decrypted_path = temp_dir.path().join("decrypted.txt");

        let test_data = b"Secret data";
        fs::write(&input_path, test_data).unwrap();

        encrypt_file(&input_path, &encrypted_path, "correctpassword").unwrap();

        // Try to decrypt with wrong password
        let result = decrypt_file(&encrypted_path, &decrypted_path, "wrongpassword");
        assert!(result.is_err());
    }
}
