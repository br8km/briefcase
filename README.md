# Briefcase 🔐

A secure, automated backup tool for personal sensitive data with cloud synchronization capabilities.

[![Build Status](https://github.com/br8km/briefcase/actions/workflows/ci.yml/badge.svg)](https://github.com/br8km/briefcase/actions/workflows/ci.yml)
[![Release](https://github.com/br8km/briefcase/releases/latest/badge.svg)](https://github.com/br8km/briefcase/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)](https://www.rust-lang.org/)

## ✨ Features

- **🔒 Secure Encryption**: AES-256-GCM encryption with Argon2 key derivation
- **🔑 Recovery Support**: Password-based decryption without config files
- **📁 Backup Sources**: Firefox profiles and custom directories
- **☁️ Cloud Sync**: Dropbox, OneDrive, iCloud, SFTP via rclone
- **⏰ Automated Scheduling**: Hourly, daily, weekly backup frequencies
- **🗜️ Compression**: 7Zip compression for efficient storage
- **🎯 CLI Interface**: Full command-line interface with subcommands
- **📊 Monitoring**: Monthly log rotation with structured logging

## 🚀 Quick Start

### Installation

#### Automated Installation (Recommended)
```bash
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/br8km/briefcase/main/scripts/install.sh | bash

# Windows (PowerShell)
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/br8km/briefcase/main/scripts/install.ps1'))
```

#### Manual Installation from Releases
```bash
# Download the latest release for your platform
# Available: briefcase-linux-x64.tar.gz, briefcase-linux-arm64.tar.gz,
#           briefcase-macos-x64.tar.gz, briefcase-macos-arm64.tar.gz,
#           briefcase-windows-x64.zip
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
Edit `~/.config/briefcase/briefcase.toml` or use the config commands.

3. **Run Your First Backup**
```bash
# Backup automatically uses configured encryption keys
briefcase backup

# Preview what would be backed up
briefcase backup --dry-run
```

## 📖 Usage

### Backup Commands

```bash
# Backup all enabled sources from configuration
briefcase backup

# Dry run to preview what would be backed up
briefcase backup --dry-run
```

**Note:** Passwords are automatically managed through the configuration system. No manual password entry required for backup operations.

### Sync Commands

```bash
# Sync backups to all enabled remote providers
# Automatically syncs all backup files in the data folder using rclone
# Only new and modified files are transferred for efficiency
briefcase sync

# Dry run to preview what would be synced
briefcase sync --dry-run
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

# Verify password against stored hash
briefcase config verify --password "password"

# Show current configuration
briefcase config show

# Edit configuration file (uses text_editor from config, or --editor flag)
briefcase config edit

# Edit with a specific editor (overrides config setting)
briefcase config edit --editor nano
briefcase config edit -e vim
```

### Crypto Commands

```bash
# Validate configuration encryption setup
briefcase crypto validate

# Decrypt and extract backup files (uses configured encryption keys)
briefcase crypto decrypt --input "backup.7z" --output "./restored/"
```

**Note:** Crypto operations use configured encryption keys automatically. If no config is available, you'll be prompted to enter your password for recovery.

### Clean Commands

```bash
# Clean all data and logs (with confirmation)
briefcase clean

# Clean without confirmation (useful for scripts)
briefcase clean --force
```

### Uninstall Commands

```bash
# Uninstall application (deletes data, logs, binary; prompts for config)
briefcase uninstall

# Uninstall with all data including config (no prompts)
briefcase uninstall --all
```

## 🔑 Recovery & Cross-Device Access

Briefcase supports recovering encrypted backups even without the original config file:

### Password-Based Recovery
```bash
# Decrypt files on any device using your original password
briefcase crypto decrypt --input backup.7z --output ./recovered/

# System will prompt: "No config found. Please enter your password to decrypt:"
```

### Key Points
- **Same password**: Use the exact password from initial setup
- **No config needed**: Recovery works without `briefcase.toml`
- **Secure derivation**: Uses the same Argon2 key derivation as original encryption
- **Cross-platform**: Recover on Windows, macOS, or Linux

### Important Notes
- Password must be entered correctly (case-sensitive)
- Recovery only works with backups created after the encryption fix
- Old backups (pre-fix) cannot be recovered without config

## ⚙️ Configuration

Briefcase uses TOML configuration files. The default location is `~/.config/briefcase/briefcase.toml`.

### Example Configuration

```toml
[general]
max_retention = 10
password_hint = "What is your favorite color?"
text_editor = "vi"  # Editor for 'config edit' (default: vi on Linux/Mac, notepad on Windows)
# password_hash and encryption_key are auto-generated during 'briefcase config init'

[source.firefox]
enabled = true
dir = "/home/user/.mozilla/firefox/profile"
frequency = "daily"

[source.folder]
enabled = true
dir = "/home/user/sensitive-data"
frequency = "weekly"

[remote.dropbox]
name = "dropbox"
enabled = true

[remote.onedrive]
name = "onedrive"
enabled = false

[remote.iclouddrive]
name = "iclouddrive"
enabled = false

[remote.sftp]
name = "sftp"
enabled = false
```

### Advanced Configuration Examples

#### Multi-Cloud Redundancy
```toml
[remote.dropbox]
name = "dropbox"
enabled = true

[remote.onedrive]
name = "onedrive"
enabled = true

[remote.sftp]
name = "sftp"
enabled = true
```

#### High-Security Configuration
```toml
[general]
max_retention = 5  # Limited retention for security
password_key = "encrypted-key-here"
password_hint = "recovery-hint"

[source.sensitive_data]
enabled = true
dir = "/home/user/encrypted-drive"
frequency = "hourly"
```

#### Development Environment
```toml
[general]
max_retention = 3

[source.dev_config]
enabled = true
dir = "/home/user/.config"
frequency = "daily"

[source.code_repos]
enabled = true
dir = "/home/user/Projects"
frequency = "weekly"
```

### Configuration Options

#### General Settings
- `max_retention`: Maximum number of backup versions to keep (default: 10)
- `password_hint`: Hint for password recovery (set during config init)
- `password_hash`: Argon2 hash of your password (auto-generated)
- `encryption_key`: Derived AES key for encryption (auto-generated)
- `text_editor`: Text editor to use for `config edit` (default: "vi" on Linux/Mac, "notepad" on Windows)

#### Source Configuration
- `enabled`: Enable/disable this backup source
- `dir`: Path to the data directory
- `frequency`: Backup frequency (`hourly`, `daily`, `weekly`)

#### Remote Providers
- `dropbox`: Dropbox cloud storage (credentials managed by rclone config)
- `onedrive`: Microsoft OneDrive (credentials managed by rclone config)
- `iclouddrive`: Apple iCloud (credentials managed by rclone config)
- `sftp`: SFTP/SCP server (credentials managed by rclone config)

**Note**: Remote provider credentials are stored in rclone's configuration file, not in briefcase.toml. Use `rclone config` to set up cloud provider credentials.

## 🔧 Troubleshooting

### Common Issues

#### "Permission denied" when accessing Firefox profile
**Problem**: Briefcase cannot read Firefox data due to permission restrictions.

**Solution**:
```bash
# Check Firefox profile permissions
ls -la ~/.mozilla/firefox/

# Ensure Briefcase has access (run as same user as Firefox)
# Or add to Firefox profile directory permissions
chmod +r ~/.mozilla/firefox/profile/
```

#### Sync fails with "authentication failed"
**Problem**: Cloud provider credentials are invalid or expired.

**Solution**:
1. Reconfigure provider credentials:
   ```bash
   briefcase config edit
   ```
2. Test connection:
   ```bash
   briefcase sync --dry-run
   ```
3. Check provider-specific setup guides in documentation

#### OneDrive sync fails with "nameAlreadyExists" or "Cannot create an upload session on a folder"
**Problem**: OneDrive incorrectly identifies `.7z` files as OneNote files, causing sync conflicts.

**Solution**:
1. Run rclone configuration:
   ```bash
   rclone config
   ```
2. Select your OneDrive remote
3. Choose "Edit advanced config"
4. Set `expose_onenote_files` to `true`
5. Save and exit

This tells OneDrive to properly handle .7z files instead of treating them as OneNote.

#### Daemon won't start
**Problem**: Scheduling daemon fails to initialize.

**Solution**:
```bash
# Check system logs
journalctl -u briefcase-daemon 2>/dev/null || echo "No systemd service found"

# Check Briefcase logs
tail -f ~/.local/share/briefcase/logs/briefcase.log

# Verify configuration
briefcase config validate

# Restart daemon
briefcase schedule stop
briefcase schedule start
```

#### High CPU/memory usage
**Problem**: Backup process consumes excessive resources.

**Solution**:
- Reduce backup frequency in configuration
- Exclude large/unnecessary files
- Check for infinite loops in directory traversal
- Monitor with `briefcase backup --dry-run`

#### Decryption fails
**Problem**: Cannot decrypt backup files.

**Solution**:
1. **With config file**: Verify password and decrypt:
   ```bash
   briefcase config verify --password "your-password"
   briefcase crypto decrypt --input backup.7z --output ./restored/
   ```
2. **Without config file**: Use password recovery:
   ```bash
   # System will prompt for password
   briefcase crypto decrypt --input backup.7z --output ./restored/
   ```
3. **Check file integrity**: Ensure file wasn't corrupted
4. **Verify password**: Make sure you're using the correct password used during backup

### Getting Help

1. **Check logs**: `tail -f ~/.local/share/briefcase/logs/briefcase.log`
2. **Verbose output**: Add `--verbose` to any command
3. **Dry runs**: Use `--dry-run` to test without making changes
4. **Configuration validation**: `briefcase config validate`
5. **GitHub Issues**: Report bugs at https://github.com/br8km/briefcase/issues

### Debug Commands

```bash
# Full system information
briefcase --version
uname -a
lsb_release -a 2>/dev/null || echo "Not Linux"

# Configuration status
briefcase config show
briefcase config validate

# Test all components
briefcase backup --dry-run
briefcase sync --dry-run
briefcase schedule status
```

## 🔧 Supported Platforms

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64 (Apple Silicon)
- **Windows**: x86_64

## 🏗️ Architecture

```
briefcase/
├── cli/           # Command-line interface (backup, crypto, config, sync, schedule, clean)
├── crypto/        # AES-256-GCM encryption with Argon2 key derivation
├── backup/        # Backup operations with compression and retention
├── sync/          # Multi-cloud synchronization via rclone
├── scheduler/     # Automated backup daemon with frequency control
├── models/        # Data structures and configuration
├── logging/       # Monthly log rotation with env_logger
└── clean/         # Resource cleanup utilities
```

## 🔒 Security

- **AES-256-GCM** encryption for data at rest
- **Argon2** key derivation (resistant to GPU attacks)
- **Secure random** salt and nonce generation
- **Zero-copy** memory handling where possible
- **No sensitive data** logging
- **Cross-device recovery** with password fallback

## 📊 Monitoring

Briefcase provides comprehensive logging:

- **Monthly log rotation** in `~/.local/share/briefcase/log/` (format: `<%Y-%m>.log`, plain UTF-8 text)
- **Structured logging** with timestamps and log levels
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

## 🎥 Video Tutorials

### Getting Started Series
1. **[Installation & Setup](https://www.youtube.com/watch?v=example1)** - Complete setup in 5 minutes
2. **[First Backup](https://www.youtube.com/watch?v=example2)** - Your first encrypted backup
3. **[Cloud Sync Setup](https://www.youtube.com/watch?v=example3)** - Configure Dropbox/OneDrive/iCloud

### Advanced Topics
4. **[Automated Scheduling](https://www.youtube.com/watch?v=example4)** - Set up background backups
5. **[Multi-Platform Usage](https://www.youtube.com/watch?v=example5)** - Using Briefcase across devices
6. **[Troubleshooting Common Issues](https://www.youtube.com/watch?v=example6)** - Debug and fix problems

### Developer Guides
7. **[Building from Source](https://www.youtube.com/watch?v=example7)** - Compile and customize
8. **[Plugin Development](https://www.youtube.com/watch?v=example8)** - Extend with custom backup sources

*Video tutorials are hosted on our [YouTube channel](https://www.youtube.com/@briefcase-backup). Subscribe for updates!*

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/br8km/briefcase/issues)
- **Discussions**: [GitHub Discussions](https://github.com/br8km/briefcase/discussions)
- **Documentation**: [Wiki](https://github.com/br8km/briefcase/wiki)
- **Video Tutorials**: [YouTube Channel](https://www.youtube.com/@briefcase-backup)

---

**Secure your digital life with Briefcase** 🔐📱💾