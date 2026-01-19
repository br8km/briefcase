# Research: Personal Data Backup System

**Feature**: 001-backup-system
**Created**: 2026-01-15
**Status**: Completed

## Technical Research Findings

### 1. Rust Cryptography Libraries

**Decision**: Use `aes-gcm` crate for AES-256 encryption

**Rationale**: 
- `aes-gcm` provides authenticated encryption (AEAD) which combines confidentiality and integrity
- Strong security track record with regular audits
- Good performance for small data sizes (1-10MB)
- Compatible with Rust's zero-copy principles

**Alternatives considered**:
- `openssl`: More complex, heavier dependency
- `ring`: Limited to specific algorithms, less flexible
- `rust-crypto`: Deprecated, not recommended for new projects

**Implementation notes**:
- Use 256-bit keys for maximum security
- Generate random nonces for each encryption operation
- Include authentication tag in encrypted output

---

### 2. 7zip Compression in Rust

**Decision**: Use `sevenz-rust` crate for 7zip compression

**Rationale**:
- Native Rust implementation, no external dependencies
- Good compression ratio for small files
- Supports password protection (though we'll use our own encryption)
- Active maintenance and good documentation

**Alternatives considered**:
- `zip`: Standard but lower compression ratio
- External `7za` binary: Requires system dependency, less portable
- `flate2`: Only supports deflate, not full 7zip format

**Implementation notes**:
- Use maximum compression level for small files
- Compress before encryption for better security
- Handle compression errors gracefully

---

### 3. Firefox Profile Data Export

**Decision**: Direct SQLite database access for Firefox data

**Rationale**:
- Firefox stores bookmarks in `places.sqlite` and passwords in `logins.json`
- Direct access is more reliable than Firefox API
- Works even when Firefox is not running
- Can handle running Firefox by copying files to temp location

**Alternatives considered**:
- Firefox API: Requires Firefox to be running, more complex
- Browser extensions: Not suitable for CLI tool

**Implementation notes**:
- Locate profile directory from standard Firefox locations
- Copy `places.sqlite` and `logins.json` to temp location if Firefox is running
- Use `rusqlite` crate for SQLite access
- Export bookmarks as JSON, passwords as encrypted JSON

---

### 4. Cloud Storage Synchronization with rclone

**Decision**: Use rclone as external process for cloud sync

**Rationale**:
- rclone supports Dropbox, OneDrive, and many other providers
- Mature, well-tested solution with excellent error handling
- Handles network retries, rate limiting, and partial transfers
- No need to implement individual provider APIs

**Alternatives considered**:
- Direct API integration: Would require separate implementations for each provider
- Custom solution: High development and maintenance cost

**Implementation notes**:
- Call rclone as subprocess with appropriate arguments
- Configure rclone programmatically or guide user through setup
- Handle rclone errors and provide meaningful messages
- Support proxy configuration for enterprise environments

---

### 5. SSH File Transfer

**Decision**: Use `ssh2` crate for SFTP/SCP operations

**Rationale**:
- Pure Rust implementation, no external dependencies
- Supports both SFTP and SCP protocols
- Good error handling and async support
- Compatible with passwordless SSH authentication

**Alternatives considered**:
- External `scp`/`sftp` commands: Requires system tools, less portable
- `libssh`: C library, more complex integration

**Implementation notes**:
- Support key-based authentication (passwordless)
- Implement retry logic for network failures
- Handle large file transfers efficiently
- Provide progress feedback for user

---

### 6. System Service Implementation

**Decision**: Use systemd service for Linux

**Rationale**:
- systemd is standard on modern Linux distributions
- Provides reliable process management and automatic restarts
- Supports scheduling via timers
- Easy to configure and monitor

**Alternatives considered**:
- cron: Less reliable, no process management
- Custom daemon: More complex, reinvents wheel

**Implementation notes**:
- Create systemd service file template
- Support both user and system service modes
- Implement graceful shutdown handling
- Provide status monitoring commands

---

### 7. Configuration Management

**Decision**: TOML format with `toml` and `serde` crates

**Rationale**:
- TOML is human-readable and easy to edit
- Good Rust ecosystem support
- Supports nested configuration structures
- Easy to validate and deserialize

**Alternatives considered**:
- JSON: Less human-friendly for configuration
- YAML: More complex parsing, security concerns
- Custom format: Reinvents wheel

**Implementation notes**:
- Store config at `$HOME/.config/briefcase.toml`
- Provide config validation on load
- Support environment variable overrides
- Include schema documentation

---

### 8. Logging Implementation

**Decision**: `log` + `env_logger` + custom file logging

**Rationale**:
- `log` facade provides flexible logging interface
- `env_logger` for development/debug logging
- Custom file logger for production use
- Supports different log levels and filters

**Alternatives considered**:
- `slog`: More complex, steeper learning curve
- `tracing`: Overkill for this application

**Implementation notes**:
- Log to `$HOME/.cache/briefcase/log/briefcase.log`
- Rotate logs based on size/date
- Include timestamps and log levels
- Sanitize sensitive data from logs

---

### 9. Error Handling Strategy

**Decision**: Custom error enum with `thiserror` and `anyhow`

**Rationale**:
- `thiserror` for defined error types
- `anyhow` for flexible error handling
- Provides good error context and chaining
- Supports custom error messages

**Alternatives considered**:
- String-based errors: Less type-safe
- Custom error trait: More complex

**Implementation notes**:
- Define comprehensive error enum
- Provide user-friendly error messages
- Include error codes for CLI use
- Log detailed error information

---

### 10. Testing Strategy

**Decision**: Comprehensive test suite with `cargo test`

**Rationale**:
- Unit tests for individual components
- Integration tests for module interactions
- End-to-end tests for complete workflows
- Contract tests for CLI and service interfaces

**Implementation notes**:
- Follow TDD approach as per constitution
- Test error cases and edge conditions
- Include performance benchmarks
- Automate test execution in CI/CD

---

## Architecture Decisions

### Modular Design

**Decision**: Functional module organization

**Rationale**:
- Clear separation of concerns
- Minimal inter-module dependencies
- Easy to test and maintain
- Supports constitution principle of modularity

**Modules**:
- `config`: Configuration management
- `crypto`: Encryption/decryption
- `backup`: Backup operations
- `sync`: Cloud/SSH synchronization
- `cli`: Command-line interface
- `service`: System service
- `logging`: Logging infrastructure

---

### Data Flow

**Decision**: Pipeline architecture for backup operations

**Rationale**:
- Clear data transformation steps
- Easy to add/remove processing steps
- Good error isolation
- Supports parallel processing where possible

**Pipeline stages**:
1. Data collection (Firefox/folder)
2. Temporary copying (if source is in use)
3. Compression (7zip)
4. Encryption (AES-256)
5. Storage (local temp)
6. Synchronization (cloud/SSH)
7. Retention cleanup

---

### Security Architecture

**Decision**: Defense-in-depth security approach

**Rationale**:
- Multiple layers of protection
- Fail-safe defaults
- Minimal attack surface
- Comprehensive audit logging

**Security layers**:
1. **Data at rest**: AES-256 encryption
2. **Data in transit**: TLS for cloud, SSH encryption
3. **Configuration**: Secure storage of credentials
4. **Memory**: Zeroize sensitive data after use
5. **Authentication**: Strong password requirements
6. **Audit**: Comprehensive logging

---

## Performance Optimization

### Memory Management

**Strategy**: Stream processing for large files

**Implementation**:
- Process data in chunks rather than loading entire files
- Use Rust's zero-copy principles where possible
- Implement proper buffer management
- Monitor and limit memory usage

### Parallel Processing

**Strategy**: Parallelize independent operations

**Implementation**:
- Use Rayon for parallel iteration
- Process multiple files concurrently
- Parallel compression of independent data
- Concurrent cloud uploads when possible

### Caching

**Strategy**: Minimal caching due to security concerns

**Implementation**:
- Cache configuration to avoid repeated file I/O
- Cache cloud provider authentication tokens
- Do NOT cache sensitive data or encryption keys

---

## Research Summary

All technical decisions have been made with consideration for:
- **Security**: Strong encryption, minimal attack surface
- **Reliability**: Comprehensive error handling, retry logic
- **Performance**: Efficient processing for 1-10MB files
- **Maintainability**: Modular design, clear interfaces
- **User Experience**: Clear feedback, good error messages
- **Constitution Compliance**: All principles addressed

The research phase is complete and ready for Phase 1 design.