# Briefcase

A CLI application for backing up small-size personal sensitive data on Linux.

## Features

- **Secure Backup**: Encrypt and backup Firefox bookmarks/passwords and custom folders
- **Remote Sync**: Sync backups to Dropbox, OneDrive, iCloud, or SFTP
- **Automated Scheduling**: Run backups automatically as a background daemon
- **Data Management**: Decrypt and restore backed up data
- **Comprehensive Logging**: Detailed JSON logs with monthly rotation

## Installation

1. Install Rust 1.75+
2. Clone and build:
   ```bash
   git clone <repository>
   cd briefcase
   cargo build --release
   ```

## Quick Start

1. Initialize configuration:
   ```bash
   ./target/release/briefcase config init
   ```

2. Perform backup:
   ```bash
   ./target/release/briefcase backup --password yourpassword
   ```

3. Sync to remote:
   ```bash
   ./target/release/briefcase sync
   ```

## Configuration

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

## Commands

- `briefcase config init|edit|validate|show` - Manage configuration
- `briefcase backup --password <pwd>` - Perform backup
- `briefcase sync [--dry-run]` - Sync to remote storage
- `briefcase crypto validate|decrypt --password <pwd>` - Crypto operations
- `briefcase schedule start|stop|status` - Manage automated backups

## Security

- AES-256 encryption for all data
- PBKDF2 key derivation
- Secure password handling
- No plaintext storage of sensitive data

## Architecture

- **Language**: Rust
- **Encryption**: AES-256-GCM
- **Compression**: 7Zip
- **Remote Sync**: Rclone
- **Logging**: Tracing with JSON output

## Development

```bash
# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Build release
cargo build --release
```

## License

MIT OR Apache-2.0