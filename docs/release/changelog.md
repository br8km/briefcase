# Changelog

All notable changes to Briefcase are recorded here. Historical entries describe releases at the time they were made; current runtime behavior is defined by the source-aligned documentation and implementation traceability report.

## Unreleased

## [1.1.1] - 2026-07-15

### Added

- Reorganized documentation into getting-started, user-guide, reference, operations, development, and release sections.
- Added implementation traceability and decision records for the sensitive-data-backup feature.
- Added per-remote `remote.<name>.last_sync` timestamp tracking to the config file for successful sync operations.
- Added command-level tests covering `briefcase backup` and `briefcase sync` config persistence.
- Added `briefcase version` and Unix/Linux-only `briefcase install` commands.

### Changed

- Documented Argon2 key derivation, external AES-256-GCM encryption, plain-text logging, and verified Linux/Unix platform scope.
- `source.<name>.last_backup` and `remote.<name>.last_sync` serialize in concise local timestamp format such as `2026-04-22 14:37:05`.
- Removed `rustfmt.toml`; formatting follows stable rustfmt defaults.

### Fixed

- `briefcase backup` persists `source.<name>.last_backup` after each successful manual backup.
- Scheduled backups update in-memory and persisted `last_backup` state consistently.
- Successful sync operations persist `remote.<name>.last_sync` per successful remote, while dry runs leave timestamps unchanged.
- Retention applies `general.max_retention` independently to each backup source.
- Scheduled backups run only the due source.

## [1.1.0] - 2026-02-23

### Added

- Added `briefcase clean` to clean data and logs directories.
- Added `briefcase uninstall` to uninstall the application, with optional config deletion.
- Added local timezone support for logs and backup filenames.
- Implemented retention based on `max_retention`.

### Changed

- Log timestamps use local timezone instead of UTC.
- Backup filenames use local timezone.
- Log filenames use local month format (`YYYY-MM.log`).

### Fixed

- Fixed uninstall behavior when the user keeps the config file.
- Removed dead code including obsolete cleanup, scheduler, and sync helpers.

### Removed

- Removed the unused `src/clean.rs` library module.
- Removed the unused `next_backup_time()` and `sync_to_remote()` functions.

## [1.0.2] - Unreleased

### Changed

- Improved sync performance by using one rclone folder sync instead of a per-file loop.
- rclone now handles incremental transfer of new and modified files.

## [1.0.1] - 2026-02-22

### Fixed

- Fixed scheduled backups by checking configured frequency and the last backup time.
- Added `last_backup` timestamp tracking.
- Fixed a logging function signature warning in clippy.

## [1.0.0] - 2026-02-16

### Added

- Added local Firefox profile and custom folder backups.
- Added AES-256-GCM encryption with Argon2 key derivation.
- Added 7z compression and rclone synchronization for Dropbox, OneDrive, iCloud, and SFTP.
- Added hourly, daily, and weekly scheduling.
- Added CLI configuration, recovery, cleanup, uninstall, and monthly log output.
- Added SFTP chroot support by routing backups to `/upload`.

### Changed

- Replaced librclone RPC with `std::process::Command` for rclone integration.
- Added detached daemon operation.

### Fixed

- Fixed rclone RPC and local path issues.
- Fixed recovery without the original config file.
- Fixed daemon process blocking and multiple-instance handling.
- Fixed build warnings and deprecated APIs.

### Removed

- Removed the librclone dependency and other unused dependencies.
- Removed the unused web UI and unimplemented features.

### Technical Details

- Built with Rust 1.75+ and tokio.
- Includes unit, integration, CLI, scheduler, sync, and restore tests.

## Types of changes

- `Added` for new features.
- `Changed` for changes to existing behavior.
- `Deprecated` for features planned for removal.
- `Removed` for removed behavior.
- `Fixed` for bug fixes.
- `Security` for security-related changes.

## Versioning

Briefcase follows Semantic Versioning.
