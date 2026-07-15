# Sensitive Data Backup

This document consolidates the product requirements, design notes, data model, CLI contracts, implementation status, and delivery history for Briefcase's sensitive-data-backup feature.

## Status and scope

The feature is implemented as a Rust CLI for local-first backups of Firefox profile data and configured sensitive folders. Local archives may be synchronized through rclone, and scheduled backups can run through the daemon. Linux/Unix is the verified runtime scope; macOS and Windows remain unverified.

The requirements below preserve the original product intent. The status tables distinguish that intent from behavior currently present in the source.

## User outcomes

### Manual local backup — P1

Users can back up Firefox bookmarks and selected profile files, plus a configured sensitive folder, into locally encrypted archives without requiring remote storage.

Acceptance expectations:

- Enabled sources are staged, compressed, and encrypted into dated `Firefox_<timestamp>.7z` or `Folder_<timestamp>.7z` files.
- Each successfully completed source persists its own `last_backup` timestamp.
- Retention removes the oldest archives for that source type without affecting the other source type.
- Invalid enabled paths produce a backup error.

### Remote sync — P2

Users can synchronize local encrypted archives to configured Dropbox, OneDrive, iCloud Drive, or SFTP rclone remotes.

Acceptance expectations:

- One folder-level sync runs per enabled remote.
- Successful non-dry-run syncs persist that remote's `last_sync` timestamp.
- Dry runs do not upload data or update timestamps.
- Remote credentials remain in rclone configuration.

### Automated scheduling — P3

Users can run a background daemon with hourly, daily, or weekly source frequencies. Each source is evaluated independently and only a due source is backed up.

### Recovery and maintenance — P3

Users can decrypt and extract archives, inspect operational logs, clean data and logs, and uninstall the application.

## Functional requirements

| ID | Requirement | Current status |
| --- | --- | --- |
| FR-001 | Initialize a default configuration and handle existing files safely. | Partial: initialization writes the file but does not warn before overwriting. |
| FR-002 | Edit and validate configuration values. | Implemented for retention, enabled source paths, and enabled remote names. |
| FR-003 | Accept a password and password hint during initialization. | Implemented. |
| FR-004 | Derive and store password credentials during initialization. | Implemented with Argon2 password verification and a stored derived AES key. |
| FR-005 | Export Firefox bookmarks and saved-password files to staging. | Implemented for bookmarks, `logins.json`, `key4.db`, and `prefs.js`. |
| FR-006 | Copy configured sensitive folders to staging. | Implemented recursively. |
| FR-007 | Compress and encrypt dated archives in the data directory. | Implemented as unencrypted 7z compression followed by AES-256-GCM encryption. |
| FR-007a | Persist the successful source's `last_backup`. | Implemented. |
| FR-008 | Enforce retention per source type. | Implemented. |
| FR-009/010 | Sync cloud and SFTP remotes. | Implemented through rclone folder sync. |
| FR-011 | Support sync dry runs. | Implemented. |
| FR-011a | Use one incremental folder sync per remote. | Implemented. |
| FR-011b | Persist successful non-dry-run `last_sync` values. | Implemented per remote. |
| FR-012 | Provide configurable JSON logging. | Not implemented; logging is plain text through `env_logger`. |
| FR-013 | Rotate logs by month and 10 MiB size with a maximum of three files. | Partial: monthly filenames exist; size rotation and file-count limits do not. |
| FR-014 | Remove temporary files after operations. | Partial: staging and temporary archive files are cleaned, but there is no separate post-sync cleanup phase. |
| FR-015 | Delete logs on request. | Implemented by `clean`. |
| FR-016 | Validate passwords for verification and recovery. | Partial: `config verify` validates the hash; recovery derives a key from the password. |
| FR-017 | Decrypt and restore original files and folders. | Implemented. |
| FR-018 | Run scheduled backups and execute only due sources. | Implemented. |
| FR-019 | Support hourly, daily, and weekly source frequencies. | Implemented. |
| FR-020 | Enforce configurable retention from 1 to 10 per source. | Implemented. |

## Edge cases

The intended behavior includes graceful handling for locked or incomplete Firefox profiles, missing source paths, incorrect passwords, network failures, full remotes, concurrent backup attempts, insufficient staging space, and corrupted Firefox data.

The current implementation specifically validates source existence, rejects invalid decryption authentication, enforces the 32 MiB staging limit for Firefox, and cleans staging directories on drop. It does not currently implement explicit concurrency locking, retry policy, folder-size enforcement, or partial continuation after a Firefox failure.

## Success criteria

The original measurable goals are:

- Complete a full 32 MiB backup in under five minutes.
- Achieve a 95% backup success rate over 30 days.
- Recover encrypted data with the correct password in all test cases.
- Complete a valid 32 MiB remote sync in under ten minutes.
- Execute scheduled backups with 98% reliability.
- Provide enough operational detail to diagnose 90% of failure scenarios.

These are product targets, not claims that have been measured in the current repository.

## Domain model

### Configuration

The TOML configuration contains `general`, `source.firefox`, `source.folder`, and flattened `remote.<name>` tables. Sources contain enabled state, directory, frequency, and `last_backup`. Remotes contain rclone name, enabled state, and `last_sync`. Timestamps serialize as local `YYYY-MM-DD HH:MM:SS` values.

### Backup archive

An archive records a path, local creation time, byte size, source type, and a reserved hash field. The runtime currently leaves the hash empty. Archive filenames use the source prefix and local timestamp.

### Temporary staging

Each backup creates a unique operating-system temporary directory with a 32 MiB limit. The limit is currently checked for Firefox staging only. The directory is removed when its owner is dropped.

### Logging

Runtime logging uses the `log` facade and `env_logger`, with `Info` as the default filter, stderr output, and monthly plain-text files under the platform data/log directory.

## Runtime design decisions

The original design selected Clap and tokio for an async CLI, TOML and serde for typed human-editable configuration, sevenz-rust for 7z compression, and rclone process invocation for remote synchronization. Custom argument parsing, JSON configuration, gzip/zstd archives, librclone RPC, and per-file sync were rejected for ergonomics, format, or operational complexity.

### Encryption

Briefcase compresses source data into an unencrypted 7z archive and then encrypts the archive bytes with AES-256-GCM. Argon2 derives the configured encryption key and verifies passwords. The derived 32-byte key is stored in configuration and reused for normal backups. The inner 7z archive is not password-protected. The encrypted writer prefixes a random salt, but the current reader does not consume that value; the per-file random nonce and GCM authentication are used by decryption.

PBKDF2, password-protected 7z archives, unique per-session encryption keys, and calculated SHA-256 backup hashes are not current runtime behavior.

### Synchronization

The sync boundary invokes the `rclone` executable through tokio process APIs. It uses one folder-level `rclone sync` operation per enabled remote. SFTP targets are placed under `/upload/briefcase`; other remotes use `/briefcase`.

### Scheduling

The daemon checks once per hour. A source is due when it has no timestamp or when its elapsed time exceeds its configured frequency. Detached operation uses Unix `fork`; stopping the daemon is not implemented on non-Unix platforms.

## CLI contracts

| Command | Arguments or actions |
| --- | --- |
| `config` | Actions: `init`, `edit`, `validate`, `show`, `verify`; initialization takes password and hint; verification takes password. |
| `backup` | Optional `--dry-run`. |
| `sync` | Optional `--dry-run`. |
| `crypto` | Actions: `validate`, `decrypt`; decryption requires `--input` and `--output`. Recovery prompts for a password when no usable config key exists. |
| `schedule` | `start`, `stop`, `status`; start supports `--detach` and `--force`. |
| `version` | Prints the package version. |
| `install` | Unix/Linux only; optional `--path` destination directory; installs the binary only. |
| `clean` | Optional `--force`; removes all data and log entries. |
| `uninstall` | Optional `--all`; removes application data and the current binary, with optional config removal. |

## Delivery history

The historical work was organized into these completed planning phases:

1. Setup and CLI foundation.
2. Configuration, models, validation, and logging foundation.
3. Manual local backup: Firefox export, folder copy, compression, encryption, retention, and tests.
4. Remote sync: rclone integration, dry runs, folder sync, cleanup, and tests.
5. Scheduling: daemon, source frequencies, due-source selection, and tests.
6. Recovery and maintenance: decryption, restore, cleanup, uninstall, and tests.
7. Polish: documentation, security hardening, performance work, and timestamp/retention corrections.

The original task checklist claimed some capabilities that are absent or partial in the current source, especially JSON/size-based logging, backup hashes, folder-size enforcement, and cross-platform daemon support. This document is the consolidated replacement for that checklist and its supporting spec files.

## Specification review

The original requirements checklist found the feature complete and testable, with no unresolved clarification markers, measurable outcomes, defined acceptance scenarios, identified edge cases, and bounded scope. Those quality checks remain useful for the product intent, but the implementation-status table above is the authority for what is actually available.

## Source references

- CLI: `src/cli/`
- Configuration: `src/config.rs`, `src/models/config.rs`
- Backup: `src/backup/`
- Encryption: `src/crypto/encrypt.rs`
- Sync: `src/sync/`
- Scheduling: `src/scheduler/`
- Logging: `src/logging.rs`
- Tests: `tests/`
