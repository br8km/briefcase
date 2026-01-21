# Changelog

All notable changes to Briefcase will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Briefcase backup application
- Secure AES-256-GCM encryption with PBKDF2
- Firefox profile data backup (bookmarks, passwords, settings)
- Custom folder backup support
- 7Zip compression for efficient storage
- Cloud synchronization via rclone (Dropbox, OneDrive, iCloud, SFTP)
- Automated scheduling (hourly, daily, weekly)
- Background daemon for automated backups
- Comprehensive CLI interface with subcommands
- Configuration validation and management
- Daily log rotation with structured logging
- Retention policy enforcement
- Dry-run mode for safe operations
- Cross-platform support (Linux, macOS, Windows planned)

### Technical Details
- Built with Rust 1.75+ using tokio async runtime
- 37 comprehensive unit and integration tests
- Full TDD development with specify framework
- Code quality enforced with clippy and rustfmt
- CI/CD pipeline with GitHub Actions
- Comprehensive documentation and examples

---

## [1.0.0] - 2026-01-21

### Added
- Complete implementation of all user stories:
  - US1: Local backup with encryption and compression
  - US2: Cloud synchronization capabilities
  - US3: Automated scheduling and daemon mode
  - US4: Polish, monitoring, and comprehensive testing

### Technical
- Production-ready codebase with 37 passing tests
- Clean architecture following Rust best practices
- Full documentation and CI/CD setup
- Ready for community adoption and contributions

---

## Types of changes
- `Added` for new features
- `Changed` for changes in existing functionality
- `Deprecated` for soon-to-be removed features
- `Removed` for now removed features
- `Fixed` for any bug fixes
- `Security` in case of vulnerabilities

## Versioning
We use [SemVer](https://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/br8km/briefcase/tags).

---

*Changelog generated with ❤️ using the specify framework*