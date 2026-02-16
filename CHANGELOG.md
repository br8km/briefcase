# Changelog

All notable changes to Briefcase will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [1.0.0] - 2026-02-16

### Added
- Complete implementation of all user stories:
  - US1: Local backup with encryption and compression
  - US2: Cloud synchronization capabilities
  - US3: Automated scheduling and daemon mode
  - US4: Polish, monitoring, and comprehensive testing
- Initial release of Briefcase backup application
- Secure AES-256-GCM encryption with Argon2 key derivation
- Firefox profile data backup (bookmarks, passwords, settings)
- Custom folder backup support
- 7Zip compression for efficient storage
- Cloud synchronization via rclone (Dropbox, OneDrive, iCloud, SFTP)
- Automated scheduling (hourly, daily, weekly)
- Background daemon for automated backups
- Comprehensive CLI interface with subcommands
- Configuration validation and management
- Monthly log rotation with structured logging (YYYY-MM.log format)
- Cross-device recovery with password-based decryption
- Optimized binary size with minimal dependencies
- SFTP Chroot Jail support by routing backups to /upload subdirectory

### Changed
- Migrated rclone backend from librclone RPC to std::process::Command for improved stability and error handling
- Enhanced daemon mode with --detach flag for background operation
- Implemented fork-based daemonization to run scheduler independently

### Fixed
- rclone RPC method "list" errors by replacing with direct Command calls
- Daemon process blocking shell (now detaches on --detach)
- Tokio runtime conflicts in forked daemon processes
- Multiple daemon instance prevention
- rclone local path issues (removed invalid "local:" prefixes)
- Critical encryption design flaw preventing recovery without config file
- Build warnings and deprecated API usage
- Logging system optimized for smaller executable size

### Removed
- Librclone crate dependency in favor of std::process::Command
- Unused dependencies (pbkdf2, hmac, sha2, regex, uuid, directories)
- Web UI module (not needed for CLI-focused tool)
- Unimplemented features from documentation

### Technical Details
- Built with Rust 1.75+ using tokio async runtime
- 37 comprehensive unit and integration tests
- Full TDD development with specify framework
- Code quality enforced with clippy and rustfmt
- CI/CD pipeline with GitHub Actions
- Comprehensive documentation and examples


## Types of changes
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities

## Versioning
We use [SemVer](https://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/br8km/briefcase/tags).


*Changelog generated with ❤️ using the specify framework*