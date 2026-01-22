use briefcase::backup::compress;
use briefcase::crypto::encrypt;
use std::fs;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_and_extract_directory() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let compressed_file = temp_dir.path().join("archive.7z");
        let extract_dir = temp_dir.path().join("extracted");

        // Create source directory with test files
        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("test1.txt"), "Hello World 1").unwrap();
        fs::write(source_dir.join("test2.txt"), "Hello World 2").unwrap();
        fs::create_dir(source_dir.join("subdir")).unwrap();
        fs::write(source_dir.join("subdir/test3.txt"), "Hello World 3").unwrap();

        // Test compression without password
        compress::compress_directory(&source_dir, &compressed_file, None).unwrap();
        assert!(compressed_file.exists());

        // Test extraction
        compress::extract_archive(&compressed_file, &extract_dir).unwrap();
        assert!(extract_dir.exists());
        assert!(extract_dir.join("test1.txt").exists());
        assert!(extract_dir.join("test2.txt").exists());
        assert!(extract_dir.join("subdir/test3.txt").exists());

        // Verify content
        let content1 = fs::read_to_string(extract_dir.join("test1.txt")).unwrap();
        let content2 = fs::read_to_string(extract_dir.join("test2.txt")).unwrap();
        let content3 = fs::read_to_string(extract_dir.join("subdir/test3.txt")).unwrap();

        assert_eq!(content1, "Hello World 1");
        assert_eq!(content2, "Hello World 2");
        assert_eq!(content3, "Hello World 3");
    }

    #[test]
    fn test_compress_and_extract_with_encryption() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let compressed_file = temp_dir.path().join("archive.7z");
        let encrypted_file = temp_dir.path().join("encrypted.7z");
        let decrypted_file = temp_dir.path().join("decrypted.7z");
        let extract_dir = temp_dir.path().join("extracted");
        let password = "testpassword123";

        // Create source directory
        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("secret.txt"), "This is secret data").unwrap();

        // Test compression (no password in 7Zip itself)
        compress::compress_directory(&source_dir, &compressed_file, None).unwrap();
        assert!(compressed_file.exists());

        // Encrypt the compressed file
        encrypt::encrypt_file(&compressed_file, &encrypted_file, password).unwrap();

        // Decrypt the file
        encrypt::decrypt_file(&encrypted_file, &decrypted_file, password).unwrap();

        // Test extraction of decrypted 7Zip
        compress::extract_archive(&decrypted_file, &extract_dir).unwrap();
        assert!(extract_dir.exists());
        assert!(extract_dir.join("secret.txt").exists());

        let content = fs::read_to_string(extract_dir.join("secret.txt")).unwrap();
        assert_eq!(content, "This is secret data");
    }

    // Note: Password protection for 7Zip archives is not implemented yet
    // Archives are protected by external AES encryption instead

    #[test]
    fn test_full_backup_restore_workflow() {
        let temp_dir = tempdir().unwrap();
        let source_dir = temp_dir.path().join("source");
        let encrypted_file = temp_dir.path().join("backup.7z");
        let restored_dir = temp_dir.path().join("restored");
        let password = "backuppassword";

        // Create source data
        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("bookmarks.html"), "<html>Bookmarks</html>").unwrap();
        fs::write(source_dir.join("passwords.txt"), "user:pass").unwrap();

        // Step 1: Compress directory
        let temp_compressed = temp_dir.path().join("temp.7z");
        compress::compress_directory(&source_dir, &temp_compressed, None).unwrap();

        // Step 2: Encrypt the compressed file
        encrypt::encrypt_file(&temp_compressed, &encrypted_file, password).unwrap();

        // Step 3: Restore workflow - decrypt and extract
        let temp_decrypted = temp_dir.path().join("temp_decrypted.7z");
        encrypt::decrypt_file(&encrypted_file, &temp_decrypted, password).unwrap();
        compress::extract_archive(&temp_decrypted, &restored_dir).unwrap();

        // Verify restoration
        assert!(restored_dir.exists());
        assert!(restored_dir.join("bookmarks.html").exists());
        assert!(restored_dir.join("passwords.txt").exists());

        let bookmarks = fs::read_to_string(restored_dir.join("bookmarks.html")).unwrap();
        let passwords = fs::read_to_string(restored_dir.join("passwords.txt")).unwrap();

        assert_eq!(bookmarks, "<html>Bookmarks</html>");
        assert_eq!(passwords, "user:pass");
    }
}
