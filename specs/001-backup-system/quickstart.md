# Quickstart Guide: Personal Data Backup System

**Feature**: 001-backup-system
**Created**: 2026-01-15
**Status**: Completed

## Installation

### Prerequisites

- Linux system (x86_64 or arm64)
- Rust 1.75+ (for building from source)
- rclone (for cloud synchronization)
- OpenSSL (for SSH operations)
- 50MB free disk space

### Install from Pre-built Binary

```bash
# Download latest release
wget https://github.com/yourorg/briefcase/releases/latest/download/briefcase-linux-x86_64.tar.gz

# Extract and install
tar -xzf briefcase-linux-x86_64.tar.gz
sudo mv briefcase /usr/local/bin/
sudo chmod +x /usr/local/bin/briefcase

# Verify installation
briefcase --version
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/yourorg/briefcase.git
cd briefcase

# Build release binary
cargo build --release

# Install
sudo cp target/release/briefcase /usr/local/bin/
sudo chmod +x /usr/local/bin/briefcase

# Verify installation
briefcase --version
```

## Initial Setup

### Step 1: Run Setup Wizard

```bash
briefcase setup
```

Follow the interactive prompts:
1. Enter a strong password (minimum 12 characters)
2. Provide a password hint (optional)
3. Set maximum retention (1-10, default: 10)
4. Configure proxy settings if needed

### Step 2: Configure Firefox Backup (Optional)

```bash
briefcase config edit
```

Add Firefox source configuration:
```toml
[source.firefox]
enabled = true
dir = "/home/youruser/.mozilla/firefox/xxxxxxxx.default-release/"
frequency = "daily"
```

### Step 3: Configure Folder Backup (Optional)

```bash
briefcase config edit
```

Add folder source configuration:
```toml
[source.documents]
enabled = true
source_type = "folder"
dir = "/home/youruser/Documents/Sensitive/"
frequency = "daily"
```

### Step 4: Configure Cloud Storage (Optional)

#### Dropbox

```toml
[remote.dropbox]
enabled = true
api_key = "your_dropbox_api_key"
```

#### OneDrive

```toml
[remote.onedrive]
enabled = true
api_key = "your_onedrive_api_key"
```

#### SSH Server

```toml
[remote.ssh]
enabled = true
username = "backupuser"
ipaddr = "192.168.1.100"
port = 22
```

## Basic Usage

### Perform Manual Backup

```bash
# Backup all enabled sources
briefcase backup --all

# Backup specific source
briefcase backup --firefox
briefcase backup --folder documents

# Force backup (ignore frequency)
briefcase backup --all --force
```

### Synchronize Backups

```bash
# Sync to all enabled remotes
briefcase sync --all

# Sync to specific remote
briefcase sync --dropbox
briefcase sync --ssh

# Check what would be synced (dry run)
briefcase sync --all --dry-run
```

### View Status

```bash
# Check backup status
briefcase retention status

# View logs
briefcase log show
briefcase log show --level error
briefcase log show --limit 20

# View configuration
briefcase config show
```

## Advanced Usage

### Cryptographic Operations

```bash
# Generate password key
briefcase crypto generate-key --password "yourpassword"

# Verify password key
briefcase crypto verify-key --password "yourpassword" --key "yourkey"

# Show password hash for backup file
briefcase crypto show-hash --password "yourpassword" --filename "backup_20260115.7z"
```

### Service Management

```bash
# Install as user service (daily backups)
briefcase service install --user --schedule daily

# Install as system service (weekly backups)
sudo briefcase service install --system --schedule weekly

# Service control
briefcase service start
briefcase service stop
briefcase service status
briefcase service restart

# Uninstall service
briefcase service uninstall
```

### Retention Management

```bash
# Check retention status
briefcase retention status

# Apply retention policy (clean up old backups)
briefcase retention apply

# Dry run - see what would be removed
briefcase retention apply --dry-run
```

## Configuration Management

### Edit Configuration

```bash
# Edit with default editor
briefcase config edit

# Edit with specific editor
EDITOR=nano briefcase config edit
```

### Validate Configuration

```bash
briefcase config validate
```

### Configuration File Location

- **Config**: `$HOME/.config/briefcase.toml`
- **Cache**: `$HOME/.cache/briefcase/`
- **Logs**: `$HOME/.cache/briefcase/log/briefcase.log`

## Troubleshooting

### Common Issues

#### "Configuration not found"

```bash
# Create default configuration
briefcase setup
```

#### "Firefox profile not found"

```bash
# Locate your Firefox profile
find ~/.mozilla/firefox -name "*.default*"

# Update configuration
briefcase config edit
```

#### "Permission denied"

```bash
# Check folder permissions
ls -la /path/to/source/folder

# Fix permissions
chmod +r /path/to/source/folder
```

#### "Cloud sync failed"

```bash
# Check rclone configuration
rclone config show

# Test rclone manually
rclone lsd dropbox:

# Check logs
briefcase log show --level error
```

### Debugging

```bash
# Enable verbose logging
briefcase --verbose backup --all

# Check detailed logs
briefcase log show --level debug

# Test encryption
briefcase crypto generate-key --password "test"
```

## Security Best Practices

### Password Management

- Use a strong password (12+ characters, mixed case, numbers, symbols)
- Never share your password
- Store password hint in secure location
- If you lose your password, backups cannot be restored

### Configuration Security

- Config file contains encrypted credentials
- Never commit config file to version control
- Use file system permissions to protect config:
  ```bash
  chmod 600 ~/.config/briefcase.toml
  ```

### Network Security

- Use HTTPS proxies when available
- Configure firewall to allow only necessary traffic
- Use SSH keys instead of passwords for SSH remotes

## Backup Verification

### Verify Backup Files

```bash
# List backup files
ls -la ~/.cache/briefcase/temp/

# Check file integrity
briefcase crypto show-hash --password "yourpassword" --filename "backup_20260115.7z"
```

### Test Restore Process

```bash
# 1. Copy backup file to safe location
cp ~/.cache/briefcase/temp/backup_20260115.7z ~/test_restore/

# 2. Decrypt manually (using openssl or similar)
# 3. Verify contents
# 4. Delete test files
rm -rf ~/test_restore/
```

## Performance Optimization

### Large Backups

```bash
# Increase compression level (slower but smaller files)
# Configure in advanced settings

# Schedule during off-peak hours
briefcase service install --schedule daily --time "02:00"
```

### Network Performance

```bash
# Limit bandwidth for cloud sync
# Configure in rclone settings

# Use proxy for better performance
briefcase config edit
```

## Uninstallation

```bash
# Stop service
briefcase service stop
briefcase service uninstall

# Remove binary
sudo rm /usr/local/bin/briefcase

# Remove configuration (optional)
rm -rf ~/.config/briefcase/
rm -rf ~/.cache/briefcase/

# Remove logs (optional)
rm -rf ~/.cache/briefcase/log/
```

## Support

### Get Help

```bash
# Show help for specific command
briefcase backup --help
briefcase sync --help

# Show version information
briefcase --version
```

### Report Issues

```bash
# Collect diagnostic information
briefcase log show --level debug > debug.log
briefcase config show --full > config.json

# Include in issue report:
# - Version number
# - Operating system
# - Configuration (redact sensitive data)
# - Log files
# - Steps to reproduce
```

## Examples

### Complete Workflow

```bash
# 1. Setup
briefcase setup

# 2. Configure sources
briefcase config edit

# 3. Test backup
briefcase backup --all --dry-run
briefcase backup --all

# 4. Test sync
briefcase sync --all --dry-run
briefcase sync --all

# 5. Install service
briefcase service install --user --schedule daily

# 6. Monitor
briefcase log show --follow
```

### Migration from Another System

```bash
# 1. Install briefcase
# 2. Configure sources to match old system
# 3. Perform initial backup
briefcase backup --all

# 4. Verify backups
ls -la ~/.cache/briefcase/temp/

# 5. Configure remotes
briefcase config edit

# 6. Sync to new locations
briefcase sync --all
```

## Configuration Examples

### Minimal Configuration

```toml
[general]
PasswordHint = "My favorite vacation spot"
PasswordKey = "encrypted_key_here"
MaxRetention = 5

[source.documents]
enabled = true
dir = "/home/user/Important/"
frequency = "daily"
```

### Full Configuration

```toml
[general]
PasswordHint = "My favorite vacation spot"
PasswordKey = "encrypted_key_here"
MaxRetention = 10
http_proxy = "http://proxy.example.com:8080"
https_proxy = "http://proxy.example.com:8080"
no_proxy = "localhost,127.0.0.1"

[source.firefox]
enabled = true
dir = "/home/user/.mozilla/firefox/xxxxxxxx.default-release/"
frequency = "daily"

[source.documents]
enabled = true
dir = "/home/user/Important/"
frequency = "weekly"

[source.projects]
enabled = true
dir = "/home/user/Projects/Secret/"
frequency = "weekly"

[remote.dropbox]
enabled = true
api_key = "encrypted_dropbox_key"
http_proxy = "http://proxy.example.com:8080"

[remote.ssh]
enabled = true
username = "backupuser"
ipaddr = "backup.example.com"
port = 22
```

## Quick Reference

| Command | Description |
|---------|-------------|
| `briefcase setup` | Initialize configuration |
| `briefcase backup --all` | Backup all sources |
| `briefcase sync --all` | Sync to all remotes |
| `briefcase service install` | Install background service |
| `briefcase log show` | View logs |
| `briefcase config edit` | Edit configuration |
| `briefcase crypto generate-key` | Generate encryption key |
| `briefcase retention apply` | Clean up old backups |

## Next Steps

1. **Test thoroughly**: Verify backups and restores work correctly
2. **Configure monitoring**: Set up log monitoring and alerts
3. **Document recovery process**: Create recovery procedures for your team
4. **Regular maintenance**: Check logs weekly, test restores monthly
5. **Update regularly**: Keep briefcase updated with latest security patches

This quickstart guide provides everything needed to get started with the Personal Data Backup System. For advanced configurations and troubleshooting, refer to the full documentation.