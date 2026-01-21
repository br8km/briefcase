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
```

2. Edit configuration if needed:
```bash
./target/release/briefcase config edit
# Modify ~/.config/briefcase/briefcase.toml
```

3. Validate configuration:
```bash
./target/release/briefcase config validate
```

## Basic Usage

### Perform Backup
```bash
./target/release/briefcase backup --password yourpassword
# Backs up enabled sources (Firefox + folder)
```

### Sync to Remote
```bash
./target/release/briefcase sync
# Syncs recent backups to enabled remote providers
```

### Decrypt and Restore
```bash
./target/release/briefcase crypto decrypt --password yourpassword --input Firefox_2026-01-19_12-00-00.7z --output ./restored/
```

## Configuration Example

Edit `~/.config/briefcase/briefcase.toml`:

```toml
[general]
PasswordHint = "Your hint"
PasswordKey = "your_password_hash"
MaxRetention = 10

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

## Logs

View application logs at `~/.local/briefcase/log/<%Y-%m>.log` (monthly rotation, e.g., `2026-01.log`)