# CLI Contracts: Personal Data Backup System

**Feature**: 001-backup-system
**Created**: 2026-01-15
**Status**: Completed

## CLI Interface Specification

### Command Structure

```bash
briefcase [OPTIONS] <COMMAND> [ARGS]
```

### Global Options

```bash
--config <PATH>    # Custom config file path (default: $HOME/.config/briefcase.toml)
--verbose          # Enable verbose logging
--quiet            # Suppress all output except errors
--version          # Show version information
--help             # Show help message
```

### Commands

#### 1. `setup` - Initialize configuration

**Purpose**: Create or update configuration file

**Usage**:
```bash
briefcase setup [OPTIONS]
```

**Options**:
```bash
--password-hint <TEXT>    # Hint for remembering password
--max-retention <NUM>     # Max backup versions (1-10, default: 10)
--force                   # Overwrite existing config
```

**Input**:
- Interactive password prompt (hidden input)
- Optional configuration values

**Output**:
- Success: "Configuration created/updated successfully"
- Error: Detailed error message

**Exit Codes**:
- `0`: Success
- `1`: Validation error
- `2`: File system error
- `3`: Encryption error

**Contract**:
```json
{
  "command": "setup",
  "inputs": {
    "password": "string (hidden)",
    "password_hint": "string (optional)",
    "max_retention": "integer (1-10, optional)",
    "force": "boolean (optional)"
  },
  "outputs": {
    "success": "boolean",
    "message": "string",
    "config_path": "string"
  },
  "errors": ["validation", "filesystem", "encryption"]
}
```

---

#### 2. `backup` - Perform backup operations

**Purpose**: Execute backup of configured sources

**Usage**:
```bash
briefcase backup [OPTIONS] [SOURCE]
```

**Options**:
```bash
--all               # Backup all enabled sources
--firefox           # Backup Firefox profile
--folder <NAME>     # Backup specific folder source
--force             # Force backup even if recent
--dry-run           # Show what would be backed up
```

**Input**:
- Source specification (all, firefox, or folder name)
- Current configuration

**Output**:
- Success: Summary of backup operations
- Error: Detailed error messages

**Exit Codes**:
- `0`: Success
- `1`: No sources configured
- `2`: Source unavailable
- `3`: Encryption error
- `4`: Storage error

**Contract**:
```json
{
  "command": "backup",
  "inputs": {
    "source": "string (all|firefox|folder_name)",
    "force": "boolean (optional)",
    "dry_run": "boolean (optional)"
  },
  "outputs": {
    "success": "boolean",
    "operations": [
      {
        "source": "string",
        "status": "string",
        "file_path": "string",
        "file_size": "integer",
        "duration_ms": "integer"
      }
    ],
    "errors": ["string"]
  },
  "errors": ["no_sources", "source_unavailable", "encryption", "storage"]
}
```

---

#### 3. `sync` - Synchronize backups to remote storage

**Purpose**: Upload encrypted backups to cloud/SSH storage

**Usage**:
```bash
briefcase sync [OPTIONS] [REMOTE]
```

**Options**:
```bash
--all               # Sync to all enabled remotes
--dropbox           # Sync to Dropbox
--onedrive          # Sync to OneDrive
--ssh               # Sync to SSH server
--dry-run           # Show what would be synced
--retry-failed      # Retry previously failed syncs
```

**Input**:
- Remote specification (all or specific remote)
- Current backups in temp directory

**Output**:
- Success: Summary of sync operations
- Error: Detailed error messages

**Exit Codes**:
- `0`: Success
- `1`: No remotes configured
- `2`: No backups available
- `3`: Network error
- `4`: Authentication error

**Contract**:
```json
{
  "command": "sync",
  "inputs": {
    "remote": "string (all|dropbox|onedrive|ssh)",
    "dry_run": "boolean (optional)",
    "retry_failed": "boolean (optional)"
  },
  "outputs": {
    "success": "boolean",
    "operations": [
      {
        "remote": "string",
        "backup_file": "string",
        "status": "string",
        "bytes_transferred": "integer",
        "duration_ms": "integer"
      }
    ],
    "errors": ["string"]
  },
  "errors": ["no_remotes", "no_backups", "network", "authentication"]
}
```

---

#### 4. `crypto` - Cryptographic operations

**Purpose**: Manage encryption keys and hashes

**Subcommands**:

##### `crypto generate-key`

**Usage**:
```bash
briefcase crypto generate-key --password <PASSWORD>
```

**Input**: User password

**Output**: Generated PasswordKey

**Contract**:
```json
{
  "command": "crypto generate-key",
  "inputs": {
    "password": "string"
  },
  "outputs": {
    "password_key": "string (encrypted)"
  },
  "errors": ["weak_password", "encryption_error"]
}
```

##### `crypto verify-key`

**Usage**:
```bash
briefcase crypto verify-key --password <PASSWORD> --key <KEY>
```

**Input**: Password and key to verify

**Output**: Verification result

**Contract**:
```json
{
  "command": "crypto verify-key",
  "inputs": {
    "password": "string",
    "key": "string"
  },
  "outputs": {
    "valid": "boolean",
    "message": "string"
  },
  "errors": ["invalid_key", "decryption_error"]
}
```

##### `crypto show-hash`

**Usage**:
```bash
briefcase crypto show-hash --password <PASSWORD> --filename <FILENAME>
```

**Input**: Password and backup filename

**Output**: PasswordHash for the file

**Contract**:
```json
{
  "command": "crypto show-hash",
  "inputs": {
    "password": "string",
    "filename": "string"
  },
  "outputs": {
    "password_hash": "string"
  },
  "errors": ["invalid_filename", "hash_error"]
}
```

---

#### 5. `service` - System service management

**Purpose**: Manage background service

**Subcommands**:

##### `service install`

**Usage**:
```bash
briefcase service install [OPTIONS]
```

**Options**:
```bash
--schedule <SCHEDULE>  # daily|weekly (default: daily)
--user                 # Install as user service
--system               # Install as system service
```

**Contract**:
```json
{
  "command": "service install",
  "inputs": {
    "schedule": "string (daily|weekly)",
    "service_type": "string (user|system)"
  },
  "outputs": {
    "success": "boolean",
    "service_name": "string",
    "message": "string"
  },
  "errors": ["permission_denied", "service_exists", "config_error"]
}
```

##### `service uninstall`

**Usage**:
```bash
briefcase service uninstall
```

**Contract**:
```json
{
  "command": "service uninstall",
  "outputs": {
    "success": "boolean",
    "message": "string"
  },
  "errors": ["service_not_found", "permission_denied"]
}
```

##### `service start/stop/restart/status`

**Usage**:
```bash
briefcase service <action>
```

**Contract**:
```json
{
  "command": "service <action>",
  "outputs": {
    "success": "boolean",
    "status": "string",
    "message": "string"
  },
  "errors": ["service_not_found", "operation_failed"]
}
```

---

#### 6. `log` - Logging operations

**Purpose**: View and manage logs

**Subcommands**:

##### `log show`

**Usage**:
```bash
briefcase log show [OPTIONS]
```

**Options**:
```bash
--level <LEVEL>     # Filter by log level (error, warn, info, debug)
--limit <NUM>       # Limit number of entries (default: 100)
--follow            # Follow log output
--since <TIME>      # Show logs since time
```

**Contract**:
```json
{
  "command": "log show",
  "inputs": {
    "level": "string (optional)",
    "limit": "integer (optional)",
    "follow": "boolean (optional)",
    "since": "string (optional)"
  },
  "outputs": {
    "entries": [
      {
        "timestamp": "string",
        "level": "string",
        "module": "string",
        "message": "string"
      }
    ]
  },
  "errors": ["log_file_not_found", "read_error"]
}
```

##### `log clear`

**Usage**:
```bash
briefcase log clear [--force]
```

**Contract**:
```json
{
  "command": "log clear",
  "inputs": {
    "force": "boolean (optional)"
  },
  "outputs": {
    "success": "boolean",
    "cleared_bytes": "integer",
    "message": "string"
  },
  "errors": ["not_confirmed", "write_error"]
}
```

---

#### 7. `config` - Configuration management

**Purpose**: View and modify configuration

**Subcommands**:

##### `config show`

**Usage**:
```bash
briefcase config show [--full]
```

**Contract**:
```json
{
  "command": "config show",
  "inputs": {
    "full": "boolean (optional)"
  },
  "outputs": {
    "config": "object",
    "sensitive_fields_redacted": "boolean"
  },
  "errors": ["config_not_found", "parse_error"]
}
```

##### `config edit`

**Usage**:
```bash
briefcase config edit
```

**Contract**:
```json
{
  "command": "config edit",
  "outputs": {
    "success": "boolean",
    "editor_used": "string",
    "message": "string"
  },
  "errors": ["config_not_found", "validation_error", "save_error"]
}
```

##### `config validate`

**Usage**:
```bash
briefcase config validate
```

**Contract**:
```json
{
  "command": "config validate",
  "outputs": {
    "valid": "boolean",
    "errors": ["string"],
    "warnings": ["string"]
  }
}
```

---

#### 8. `retention` - Retention policy management

**Purpose**: Manage backup retention

**Subcommands**:

##### `retention apply`

**Usage**:
```bash
briefcase retention apply [--dry-run]
```

**Contract**:
```json
{
  "command": "retention apply",
  "inputs": {
    "dry_run": "boolean (optional)"
  },
  "outputs": {
    "success": "boolean",
    "removed_backups": [
      {
        "source": "string",
        "file_path": "string",
        "file_size": "integer",
        "timestamp": "string"
      }
    ],
    "current_count": "integer",
    "message": "string"
  },
  "errors": ["no_backups", "delete_error"]
}
```

##### `retention status`

**Usage**:
```bash
briefcase retention status
```

**Contract**:
```json
{
  "command": "retention status",
  "outputs": {
    "sources": [
      {
        "source": "string",
        "max_retention": "integer",
        "current_backups": "integer",
        "oldest_backup": "string (optional)",
        "needs_cleanup": "boolean"
      }
    ]
  },
  "errors": ["no_sources"]
}
```

---

## Error Handling Contract

### Standard Error Format

```json
{
  "error": {
    "code": "string",
    "message": "string",
    "details": "string (optional)",
    "suggestion": "string (optional)"
  }
}
```

### Error Codes

| Code | Meaning | Recovery |
|------|---------|----------|
| `config_error` | Configuration invalid | Fix config file |
| `auth_error` | Authentication failed | Check credentials |
| `encryption_error` | Encryption/decryption failed | Verify password |
| `network_error` | Network operation failed | Check connection |
| `storage_error` | Storage operation failed | Check disk space |
| `validation_error` | Input validation failed | Correct input |
| `not_found` | Resource not found | Verify resource exists |
| `permission_error` | Permission denied | Run with proper rights |
| `timeout_error` | Operation timed out | Retry or check system |

### Error Handling Requirements

1. **User-friendly messages**: Clear, actionable error messages
2. **Detailed logging**: Full error context in logs
3. **Exit codes**: Appropriate exit codes for scripting
4. **Recovery suggestions**: When possible, suggest solutions
5. **Sensitive data**: Never include sensitive data in errors

## Input Validation Contract

### Validation Rules

1. **Paths**: Must be absolute, exist, and be accessible
2. **Passwords**: Minimum length, complexity requirements
3. **Configuration**: Schema validation against expected structure
4. **Network**: Valid URLs, ports, and credentials
5. **Retention**: Values between 1-10

### Validation Errors

```json
{
  "validation_errors": [
    {
      "field": "string",
      "error": "string",
      "rejected_value": "string (optional)",
      "acceptable_values": "string (optional)"
    }
  ]
}
```

## Performance Contract

### Performance Guarantees

| Operation | Max Duration | Notes |
|-----------|--------------|-------|
| Configuration | 5s | Includes encryption setup |
| Firefox Backup | 30s | For 1-10MB data |
| Folder Backup | 60s | For 10MB data |
| Encryption | 10s | For 10MB data |
| Compression | 15s | For 10MB data |
| CLI Commands | 1s | Response time |
| Service Start | 3s | Normal conditions |

### Resource Limits

| Resource | Limit | Notes |
|----------|-------|-------|
| Memory | 100MB | During normal operation |
| CPU | 1 core | Single-threaded operations |
| Disk | Configurable | Respects retention policy |
| Network | Unlimited | Depends on remote storage |

## Security Contract

### Security Requirements

1. **Data at Rest**: AES-256 encryption for all sensitive data
2. **Data in Transit**: TLS 1.2+ for cloud, SSH for server transfers
3. **Memory**: Zeroize sensitive data after use
4. **Authentication**: Strong password requirements
5. **Logging**: No sensitive data in logs
6. **Configuration**: Encrypted credentials

### Security Guarantees

- No plaintext storage of passwords or keys
- All network communications encrypted
- Sensitive memory zeroized after use
- Comprehensive audit logging
- Regular security updates

## Contract Compliance

All CLI contracts comply with:
- ✅ Constitution Principle 1: Test-Driven Development
- ✅ Constitution Principle 2: Code Safety
- ✅ Constitution Principle 3: Robust Error Handling
- ✅ Constitution Principle 4: Data Integrity and Safety
- ✅ Constitution Principle 5: Modularity and Organization
- ✅ Constitution Principle 6: Detailed Logging

These contracts define the complete CLI interface and can be used for:
- Implementation guidance
- Test case generation
- Documentation
- API compatibility verification