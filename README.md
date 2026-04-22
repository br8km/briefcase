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

Prefer to start from the repository example at `config.example.toml` if you want a ready-made template before initializing your local config.

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

## 📚 Documentation

- [Usage Guide](docs/USAGE.md)
- [Contributing Guide](docs/CONTRIBUTING.md)
- [Changelog](docs/CHANGELOG.md)
- [Release Process](docs/RELEASE.md)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run the test suite: `cargo test`
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

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
