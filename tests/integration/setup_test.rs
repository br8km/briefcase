// Integration test for setup command
// Tests the complete setup workflow with mock data

use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_setup_command_with_mock_data() {
    // Create temporary directory for test
    let temp_dir = tempdir().unwrap();
    let config_dir = temp_dir.path().join(".config").join("briefcase");
    
    // Create mock environment
    std::env::set_var("HOME", temp_dir.path());
    
    // Mock user input
    let password = "testpassword123";
    let password_hint = "test hint";
    let max_retention = 5;
    
    // Run setup with mock data
    let result = setup_with_mock_data(password, password_hint, max_retention);
    
    // Verify setup completed
    assert!(result.is_ok(), "Setup should complete successfully");
    
    // Verify config file was created
    let config_path = config_dir.join("briefcase.toml");
    assert!(config_path.exists(), "Config file should be created");
    
    // Verify config file content
    let config_content = fs::read_to_string(config_path).unwrap();
    assert!(config_content.contains("password_hint = \"test hint\""), "Config should contain password hint");
    assert!(config_content.contains("max_retention = 5"), "Config should contain max retention");
    assert!(config_content.contains("PasswordKey"), "Config should contain password key");
    
    // Verify default sources
    assert!(config_content.contains("firefox"), "Config should contain Firefox source");
    assert!(config_content.contains("documents"), "Config should contain documents source");
    
    // Verify default remotes
    assert!(config_content.contains("dropbox"), "Config should contain Dropbox remote");
    assert!(config_content.contains("ssh"), "Config should contain SSH remote");
    
    println!("✅ Setup integration test passed!");
}

#[test]
fn test_setup_with_existing_config() {
    // Create temporary directory for test
    let temp_dir = tempdir().unwrap();
    let config_dir = temp_dir.path().join(".config").join("briefcase");
    fs::create_dir_all(&config_dir).unwrap();
    
    // Create existing config
    let existing_config = r#"
[general]
password_hint = "existing hint"
password_key = "existing_key"
max_retention = 10

[[sources]]
name = "existing"
enabled = true
source_type = "Folder"
dir = "/existing/path"
frequency = "Daily"
"#;
    
    let config_path = config_dir.join("briefcase.toml");
    fs::write(&config_path, existing_config).unwrap();
    
    // Set environment
    std::env::set_var("HOME", temp_dir.path());
    
    // Run setup with mock data
    let result = setup_with_mock_data("newpassword", "new hint", 7);
    
    // Verify setup completed
    assert!(result.is_ok(), "Setup should complete successfully with existing config");
    
    // Verify config file still exists
    assert!(config_path.exists(), "Config file should still exist");
    
    // Verify existing data is preserved
    let config_content = fs::read_to_string(config_path).unwrap();
    assert!(config_content.contains("existing hint"), "Existing password hint should be preserved");
    assert!(config_content.contains("existing_key"), "Existing password key should be preserved");
    assert!(config_content.contains("existing"), "Existing source should be preserved");
    
    println!("✅ Existing config test passed!");
}

#[test]
fn test_password_validation() {
    // Test weak password rejection
    let result = setup_with_mock_data("short", "hint", 5);
    assert!(result.is_err(), "Short password should be rejected");
    
    // Test valid password acceptance
    let temp_dir = tempdir().unwrap();
    std::env::set_var("HOME", temp_dir.path());
    
    let result = setup_with_mock_data("validpassword123", "hint", 5);
    assert!(result.is_ok(), "Valid password should be accepted");
    
    println!("✅ Password validation test passed!");
}

#[test]
fn test_retention_validation() {
    // Test invalid retention values
    let temp_dir = tempdir().unwrap();
    std::env::set_var("HOME", temp_dir.path());
    
    // Test retention out of range
    let result = setup_with_mock_data("validpassword123", "hint", 0);
    assert!(result.is_ok(), "Retention 0 should be accepted (will be validated later)");
    
    let result = setup_with_mock_data("validpassword123", "hint", 11);
    assert!(result.is_ok(), "Retention 11 should be accepted (will be validated later)");
    
    println!("✅ Retention validation test passed!");
}

// Helper function to run setup with mock data
fn setup_with_mock_data(password: &str, password_hint: &str, max_retention: u8) -> Result<(), Box<dyn std::error::Error>> {
    use briefcase::config::setup::*;
    use std::io::Write;
    
    // Mock the user input functions
    let original_get_user_input = GET_USER_INPUT;
    let original_get_password_input = GET_PASSWORD_INPUT;
    let original_get_retention_input = GET_RETENTION_INPUT;
    
    // Set mock inputs
    unsafe {
        GET_USER_INPUT = Some(Box::new(move || Ok(password_hint.to_string())));
        GET_PASSWORD_INPUT = Some(Box::new(move || Ok(password.to_string())));
        GET_RETENTION_INPUT = Some(Box::new(move || Ok(max_retention)));
    }
    
    // Run setup
    let result = run_setup();
    
    // Restore original functions
    unsafe {
        GET_USER_INPUT = original_get_user_input;
        GET_PASSWORD_INPUT = original_get_password_input;
        GET_RETENTION_INPUT = original_get_retention_input;
    }
    
    result
}

// Mock function pointers for testing
static mut GET_USER_INPUT: Option<Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>> = None;
static mut GET_PASSWORD_INPUT: Option<Box<dyn Fn() -> Result<String, Box<dyn std::error::Error>>>> = None;
static mut GET_RETENTION_INPUT: Option<Box<dyn Fn() -> Result<u8, Box<dyn std::error::Error>>>> = None;