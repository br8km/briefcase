# briefcase Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-01-19

## Active Technologies

- Rust 1.75+ + tokio (async runtime), serde (serialization), aes-gcm (encryption), sevenz-rust (compression), rclone (sync), clap (CLI), toml (config), tracing (logging) (001-sensitive-data-backup)

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root with module declarations
├── cli/                 # Command-line interface modules
├── config.rs            # Configuration loading/validation
├── models/              # Data structures and types
├── backup/              # Backup functionality
├── crypto/              # Encryption/decryption operations
├── sync/                # Remote synchronization (rclone integration)
├── logging.rs           # Logging setup and utilities
├── scheduler/           # Scheduled backup operations
└── clean.rs             # Cleanup utilities

tests/
└── unit/                # Unit tests organized by module

specs/                   # Feature specifications and plans
.github/workflows/       # CI/CD pipelines
Cargo.toml               # Rust dependencies and metadata
rustfmt.toml            # Code formatting configuration
```

## Commands

### Build & Development
```bash
# Build in debug mode
cargo build

# Build optimized release binary
cargo build --release

# Run the application
cargo run

# Run with specific arguments
cargo run -- [args]
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests for a specific module
cargo test --lib config

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --ignore-tests

# Run integration tests only
cargo test --test '*'
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Lint code
cargo clippy

# Lint with fixes applied
cargo clippy --fix

# Check for security vulnerabilities
cargo audit

# Generate documentation
cargo doc --open
```

### Dependencies
```bash
# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated

# Add a new dependency
cargo add package_name

# Remove a dependency
cargo remove package_name
```

## Code Style Guidelines

### General Principles
- Follow Rust's official style guidelines (rustfmt defaults with project-specific overrides)
- Prefer explicit over implicit code
- Use meaningful names that describe intent and purpose
- Keep functions small and focused on a single responsibility
- Use early returns to reduce nesting
- Handle errors explicitly with `Result` and `?` operator

### Imports and Modules
```rust
// Group imports by standard library, external crates, then local modules
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::{Config, BackupFile};
use crate::backup::service::BackupService;

// Prefer importing specific items over glob imports
// Good: use std::collections::HashMap;
// Bad:  use std::collections::*;
```

### Naming Conventions
- **Types/Structs/Enums/Traits**: `PascalCase` (e.g., `BackupService`, `SourceType`)
- **Functions/Methods**: `snake_case` (e.g., `perform_backup`, `validate_config`)
- **Variables/Fields**: `snake_case` (e.g., `backup_files`, `temp_dir`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`)
- **Modules**: `snake_case` (e.g., `backup_service`, `crypto_utils`)

### Error Handling
```rust
// Use anyhow::Result for application-level errors
use anyhow::{anyhow, Result};

// Prefer specific error types for library boundaries
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackupError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Configuration error: {0}")]
    Config(String),
}

// Use ? operator for error propagation
pub fn process_backup(config: &Config) -> Result<()> {
    validate_config(config)?;
    perform_backup(config).await?;
    Ok(())
}

// Log errors with context
if let Err(e) = risky_operation() {
    error!("Failed to perform operation: {}", e);
    return Err(e);
}
```

### Async Code
```rust
// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<()> {
    // Async functions should be clearly marked
    let result = process_async().await?;
    Ok(())
}

// Prefer async trait methods over blocking operations
impl BackupService {
    pub async fn perform_backup(&self) -> Result<()> {
        // Async operations here
        tokio::fs::read_file(&path).await?;
        Ok(())
    }
}
```

### Types and Data Structures
```rust
// Use strong typing with meaningful names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub max_retention: usize,
    pub compression_level: CompressionLevel,
    pub encryption_enabled: bool,
}

// Prefer enums over booleans for state
#[derive(Debug, Clone)]
pub enum SourceType {
    Firefox,
    Folder,
    Custom(String),
}

// Use builder pattern for complex construction
impl BackupConfig {
    pub fn builder() -> BackupConfigBuilder {
        BackupConfigBuilder::default()
    }
}
```

### Logging
```rust
// Use tracing for structured logging
use tracing::{info, warn, error, debug};

// Log at appropriate levels with context
pub async fn perform_backup(&self) -> Result<()> {
    info!("Starting backup process for {} sources", source_count);
    debug!("Backup configuration: {:?}", config);

    if let Err(e) = backup_operation().await {
        error!("Backup failed: {}", e);
        return Err(e);
    }

    info!("Backup completed successfully");
    Ok(())
}
```

### Testing
```rust
// Unit tests in the same file as implementation
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_valid_config() {
        let config = Config::default();
        assert!(validate_config(&config).is_ok());
    }

    #[test]
    fn test_backup_with_temp_dir() {
        let temp_dir = tempdir().unwrap();
        // Test implementation
    }

    #[tokio::test]
    async fn test_async_backup_operation() {
        // Async test
        let result = perform_async_backup().await;
        assert!(result.is_ok());
    }
}

// Integration tests in tests/ directory
#[cfg(test)]
mod integration_tests {
    use briefcase::*;

    #[test]
    fn test_full_backup_workflow() {
        // Full integration test
    }
}
```

### Security Best Practices
- **Never log sensitive data** (passwords, keys, tokens)
- **Validate all inputs** before processing
- **Use secure defaults** for encryption and permissions
- **Handle secrets securely** using environment variables or secure storage
- **Validate file paths** to prevent directory traversal
- **Use constant-time operations** for cryptographic comparisons

### Configuration
```rust
// Use TOML for configuration files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub source: SourceConfig,
    pub remote: RemoteConfig,
}

// Provide sensible defaults
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                max_retention: 10,
                compression_level: CompressionLevel::Normal,
            },
            // ... other defaults
        }
    }
}
```

### File Organization
- Keep modules focused and cohesive
- Use clear separation of concerns
- Place related functionality together
- Use `mod.rs` files for module organization
- Keep test files alongside implementation or in dedicated test directories

### Performance Considerations
- Use async operations for I/O bound tasks
- Avoid unnecessary allocations in hot paths
- Use `Arc` for shared ownership in async contexts
- Prefer iterators over collections where possible
- Profile performance-critical code

### Documentation
```rust
/// Performs a complete backup operation for all enabled sources.
///
/// This function handles the entire backup workflow including:
/// - Data collection from configured sources
/// - Compression and encryption
/// - Retention policy enforcement
/// - Cleanup of temporary files
///
/// # Arguments
/// * `password` - Master password for encryption
///
/// # Returns
/// Returns a vector of created backup files on success
///
/// # Errors
/// Returns an error if backup fails at any stage
pub async fn perform_backup(&self, password: &str) -> Result<Vec<BackupFile>> {
    // Implementation
}
```

## Recent Changes

- 001-sensitive-data-backup: Added Rust 1.75+ + tokio (async runtime), serde (serialization), aes-gcm (encryption), sevenz-rust (compression), rclone (sync), clap (CLI), toml (config), tracing (logging)

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
