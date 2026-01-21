# Briefcase 🔐

A secure, automated backup tool for personal sensitive data with cloud synchronization capabilities.

[![Build Status](https://github.com/br8km/briefcase/actions/workflows/ci.yml/badge.svg)](https://github.com/br8km/briefcase/actions/workflows/ci.yml)
[![Release](https://github.com/br8km/briefcase/releases/latest/badge.svg)](https://github.com/br8km/briefcase/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org/)

## ✨ Features

- **🔒 Secure Encryption**: AES-256-GCM encryption with PBKDF2 key derivation
- **📁 Multiple Sources**: Firefox profile data, custom folders
- **☁️ Cloud Sync**: Dropbox, OneDrive, iCloud, SFTP support via rclone
- **⏰ Automated Scheduling**: Hourly, daily, weekly backup frequencies
- **🗜️ Compression**: 7Zip compression for efficient storage
- **🎯 CLI Interface**: Full command-line interface with subcommands
- **📊 Monitoring**: Comprehensive logging and backup tracking

## 🚀 Quick Start

### Installation

#### From Releases (Recommended)
```bash
# Download the latest release for your platform
curl -L https://github.com/br8km/briefcase/releases/latest/download/briefcase-linux-x64.tar.gz | tar xz
sudo mv briefcase /usr/local/bin/
```

#### From Source
```bash
# Requires Rust 1.75+
git clone https://github.com/br8km/briefcase.git
cd briefcase
cargo build --release
sudo cp target/release/briefcase /usr/local/bin/
```

### First Time Setup

1. **Initialize Configuration**
```bash
briefcase config init --password "your-strong-password" --password-hint "hint"
```

2. **Configure Backup Sources**
Edit `~/.config/briefcase.toml` or use the config commands.

3. **Run Your First Backup**
```bash
briefcase backup --password "your-password"
```

## 📖 Usage

### Backup Commands

```bash
# Backup all configured sources
briefcase backup --password "your-password"

# Backup specific sources only
briefcase backup --password "your-password" --sources firefox folder

# Dry run to preview what would be backed up
briefcase backup --password "your-password" --dry-run
```

### Sync Commands

```bash
# Sync backups to all configured remote providers
briefcase sync --dry-run

# Sync to specific providers only
briefcase sync --providers dropbox onedrive
```

### Schedule Commands

```bash
# Start automated backup daemon
briefcase schedule start

# Stop the daemon
briefcase schedule stop

# Check daemon status
briefcase schedule status
```

### Configuration Commands

```bash
# Initialize new configuration
briefcase config init --password "password" --password-hint "hint"

# Validate current configuration
briefcase config validate

# Show current configuration
briefcase config show

# Edit configuration file
briefcase config edit
```

## ⚙️ Configuration

Briefcase uses TOML configuration files. The default location is `~/.config/briefcase.toml`.

### Example Configuration

```toml
[general]
max_retention = 10
password_key = "your-encrypted-key"
password_hint = "your-password-hint"

[source.firefox]
enabled = true
dir = "/home/user/.mozilla/firefox/profile"
frequency = "daily"

[source.folder]
enabled = true
dir = "/home/user/sensitive-data"
frequency = "weekly"

[remote.dropbox]
enabled = true
app_key = "your-dropbox-app-key"
app_secret = "your-dropbox-app-secret"

[remote.onedrive]
enabled = false
client_id = "your-onedrive-client-id"
client_secret = "your-onedrive-client-secret"

[remote.icloud]
enabled = false
apple_id = "your-apple-id"
client_id = "your-icloud-client-id"

[remote.sftp]
enabled = false
username = "remote-user"
ipaddr = "remote-server.com"
port = 22
```

### Configuration Options

#### General Settings
- `max_retention`: Maximum number of backup versions to keep (default: 10)
- `password_key`: Encrypted password key for automation
- `password_hint`: Hint for password recovery

#### Source Configuration
- `enabled`: Enable/disable this backup source
- `dir`: Path to the data directory
- `frequency`: Backup frequency (`hourly`, `daily`, `weekly`)

#### Remote Providers
- `dropbox`: Dropbox cloud storage
- `onedrive`: Microsoft OneDrive
- `icloud`: Apple iCloud
- `sftp`: SFTP/SCP server

## 🔧 Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64 (Apple Silicon)
- **Windows**: x86_64 (coming soon)

## 🏗️ Architecture

```
briefcase/
├── cli/           # Command-line interface
├── crypto/        # Encryption/decryption
├── backup/        # Backup operations
├── sync/          # Remote synchronization
├── scheduler/     # Automated scheduling
├── models/        # Data structures
├── logging/       # Logging utilities
└── clean/         # Cleanup utilities
```

## 🔒 Security

- **AES-256-GCM** encryption for data at rest
- **PBKDF2** key derivation with high iteration count
- **Secure random** salt and nonce generation
- **Zero-copy** memory handling where possible
- **No sensitive data** logging

## 📊 Monitoring

Briefcase provides comprehensive logging:

- **Daily log rotation** in `~/.local/share/briefcase/logs/`
- **Structured logging** with JSON format option
- **Backup success/failure** tracking
- **Performance metrics** and timing information

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- Uses [rclone](https://rclone.org/) for cloud synchronization
- Inspired by the need for secure, automated personal data backup

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/br8km/briefcase/issues)
- **Discussions**: [GitHub Discussions](https://github.com/br8km/briefcase/discussions)
- **Documentation**: [Wiki](https://github.com/br8km/briefcase/wiki)

---

**Secure your digital life with Briefcase** 🔐📱💾