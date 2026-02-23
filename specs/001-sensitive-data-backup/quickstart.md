# Quickstart Guide: Sensitive Data Backup

**Date**: 2026-01-19
**Feature**: Sensitive Data Backup CLI

## Prerequisites

- Linux operating system (x86_64 or ARM64)
- Rust 1.75+ installed
- Firefox browser (optional, for bookmarks/passwords backup)

## Installation

```bash
# Clone and build
git clone <repository>
cd briefcase
cargo build --release
```

## First-Time Setup

1. Initialize configuration:
```bash
./target/release/briefcase config init
# Enter password when prompted
# Enter password hint
# text_editor is auto-set to "vi" on Linux/Mac, "notepad" on Windows
```

2. Edit configuration if needed:
```bash
./target/release/briefcase config edit
# Opens config file in your configured text_editor
# Or use: ./target/release/briefcase config edit --editor nano
```

3. Validate configuration:
```bash
./target/release/briefcase config validate
```

## Basic Usage

**Note:** After initial setup, backup and sync operations are completely password-free. The system uses your configured password key automatically.

### Perform Backup
```bash
./target/release/briefcase backup
# Backs up all enabled sources from configuration
```

### Preview Backup (Dry Run)
```bash
./target/release/briefcase backup --dry-run
# Shows what would be backed up without actually doing it
```

### Sync to Remote
```bash
./target/release/briefcase sync
# Syncs recent backups to enabled remote providers
```

### Decrypt and Restore
```bash
./target/release/briefcase crypto decrypt --input Firefox_2026-01-19_12-00-00.7z --output ./restored/
# Uses configured password key automatically
```

## Configuration Example

Edit `~/.config/briefcase/briefcase.toml`:

```toml
[general]
PasswordHint = "Your hint"
PasswordKey = "sha256_hash_of_your_password"
MaxRetention = 10
TextEditor = "vi"  # or "notepad" on Windows

[source.firefox]
enabled = true
dir = "/home/user/.mozilla/firefox/profile"
frequency = "daily"

[source.folder]
enabled = true
dir = "/home/user/sensitive-data"
frequency = "hourly"

[remote.dropbox]
enabled = true
app_key = "your_app_key"
app_secret = "your_app_secret"
```

## Testing

```bash
# Run all tests
cargo test

# Run integration tests
cargo test --test integration
```

## Troubleshooting

- **Backup fails**: Check source directories exist and are readable
- **Sync fails**: Verify remote credentials and network connectivity
- **Decryption fails**: Ensure correct password and file integrity
- **OneDrive sync fails with "nameAlreadyExists"**: Run `rclone config`, select OneDrive, choose "Edit advanced config", set `expose_onenote_files` to `true`

## Logs

View application logs at `~/.local/briefcase/log/<%Y-%m>.log` (monthly rotation, e.g., `2026-01.log`)