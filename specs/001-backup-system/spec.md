# Feature Specification: Personal Data Backup System

**Feature Branch**: `001-backup-system`  
**Created**: 2026-01-15  
**Status**: Draft  
**Input**: User description: "Building a backing up application named as `briefcase` for personal sensitive data (small size 1~10 MB) security."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Configuration Setup (Priority: P1)

As a user, I want to configure the backup system with my password and preferences so that I can securely back up my personal data.

**Why this priority**: This is the foundation for all backup operations. Without proper configuration, no backups can be performed.

**Independent Test**: Can be fully tested by running the configuration command and verifying the config file is created with proper encryption keys.

**Acceptance Scenarios**:

1. **Given** no existing configuration, **When** user runs setup command, **Then** config file is created at `$HOME/.config/briefcase.toml` with default values
2. **Given** existing configuration, **When** user runs setup command, **Then** existing config is preserved and only missing values are added
3. **Given** user provides password, **When** setup completes, **Then** PasswordKey is generated and stored securely

**Performance Targets**: Configuration setup must complete within 5 seconds

**Security Requirements**: Password must be hashed using strong cryptographic algorithm before storage

---

### User Story 2 - Firefox Data Backup (Priority: P2)

As a user, I want to back up my Firefox bookmarks and passwords so that I can restore them if needed.

**Why this priority**: Firefox data contains sensitive credentials and browsing history that users want to protect.

**Independent Test**: Can be tested by running Firefox backup command and verifying encrypted files are created in temp folder.

**Acceptance Scenarios**:

1. **Given** Firefox is not running, **When** backup command runs, **Then** bookmarks and passwords are exported and encrypted
2. **Given** Firefox is running, **When** backup command runs, **Then** data is copied to temp location first, then processed
3. **Given** backup completes, **When** checking temp folder, **Then** encrypted zip file with PasswordHash exists

**Performance Targets**: Firefox backup must complete within 30 seconds for typical profile sizes (1-10MB)

**Security Requirements**: All exported data must be encrypted with PasswordHash before storage

---

### User Story 3 - Sensitive Folder Backup (Priority: P3)

As a user, I want to back up my defined sensitive folders so that my important personal files are protected.

**Why this priority**: Users have personal files outside browser data that need protection.

**Independent Test**: Can be tested by configuring a test folder and running backup command.

**Acceptance Scenarios**:

1. **Given** sensitive folder is configured, **When** backup command runs, **Then** folder contents are encrypted and zipped
2. **Given** folder contains various file types, **When** backup completes, **Then** all files are included in encrypted archive
3. **Given** backup completes, **When** checking temp folder, **Then** encrypted zip file exists with proper naming

**Performance Targets**: Folder backup must complete within 1 minute for 10MB of data

---

### User Story 4 - Cloud Sync (Priority: P4)

As a user, I want to sync my encrypted backups to cloud storage so that my data is safe even if my local machine fails.

**Why this priority**: Cloud sync provides off-site redundancy for disaster recovery.

**Independent Test**: Can be tested by running sync command and verifying files appear in cloud storage.

**Acceptance Scenarios**:

1. **Given** Dropbox is configured, **When** sync command runs, **Then** encrypted files are uploaded to Dropbox
2. **Given** OneDrive is configured, **When** sync command runs, **Then** encrypted files are uploaded to OneDrive
3. **Given** both cloud providers configured, **When** sync runs, **Then** files are uploaded to both locations

**Performance Targets**: Cloud sync must handle network interruptions gracefully with retry logic

**Security Requirements**: Cloud credentials must be stored securely and never logged

---

### User Story 5 - SSH Server Sync (Priority: P5)

As a user, I want to sync my encrypted backups to my remote server via SSH so that I have an additional backup location.

**Why this priority**: SSH sync provides secure transfer to user-controlled servers.

**Independent Test**: Can be tested by configuring SSH credentials and running sync command.

**Acceptance Scenarios**:

1. **Given** SSH credentials are configured, **When** sync command runs, **Then** files are transferred via SCP/SFTP
2. **Given** server is unreachable, **When** sync runs, **Then** appropriate error is logged and retry attempted
3. **Given** successful transfer, **When** checking server, **Then** encrypted files are present

**Performance Targets**: SSH transfer must complete within reasonable time based on network conditions

---

### User Story 6 - CLI Utilities (Priority: P6)

As a user, I want command line utilities to manage encryption keys so that I can verify and troubleshoot my backups.

**Why this priority**: CLI utilities enable advanced users to manage their backup system.

**Independent Test**: Can be tested by running CLI commands with test data.

**Acceptance Scenarios**:

1. **Given** password and filename, **When** CLI command runs, **Then** PasswordHash is displayed
2. **Given** invalid password, **When** verification command runs, **Then** appropriate error is shown
3. **Given** valid PasswordKey, **When** generation command runs, **Then** key is generated successfully

**Performance Targets**: CLI commands must respond within 1 second

---

### User Story 7 - Scheduled Service (Priority: P7)

As a user, I want the backup system to run automatically on a schedule so that I don't have to remember to back up manually.

**Why this priority**: Automatic scheduling ensures regular backups without user intervention.

**Independent Test**: Can be tested by setting up service and verifying it runs on schedule.

**Acceptance Scenarios**:

1. **Given** daily schedule configured, **When** system time reaches schedule, **Then** backup runs automatically
2. **Given** weekly schedule configured, **When** scheduled day/time arrives, **Then** backup runs automatically
3. **Given** service is running, **When** system reboots, **Then** service restarts automatically

**Performance Targets**: Scheduled backups must not interfere with system performance

---

### User Story 8 - Retention Policy (Priority: P8)

As a user, I want old backups to be automatically cleaned up according to retention policy so that I don't run out of storage space.

**Why this priority**: Retention policy prevents unlimited storage growth.

**Independent Test**: Can be tested by creating multiple backups and verifying old ones are removed.

**Acceptance Scenarios**:

1. **Given** MaxRetention=10, **When** 11th backup is created, **Then** oldest backup is removed
2. **Given** retention policy configured, **When** backup runs, **Then** only specified number of backups are retained
3. **Given** different retention per source, **When** backup runs, **Then** each source respects its own policy

**Performance Targets**: Retention cleanup must be efficient even with many old backups

### Edge Cases

- What happens when Firefox is running during backup? → Copy files to temp location first
- How does system handle network failures during cloud sync? → Implement retry logic with exponential backoff
- What happens when disk space is insufficient? → Log error and skip backup, notify user if possible
- How to handle corrupted config files? → Create backup of corrupted file and generate new default config
- What happens when encryption fails? → Log detailed error and abort operation safely

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST read configuration from `$HOME/.config/briefcase.toml` and create it if missing
- **FR-002**: System MUST generate PasswordKey from user password using strong cryptographic algorithm
- **FR-003**: System MUST export Firefox bookmarks and passwords to separate files before encryption
- **FR-004**: System MUST generate one-time PasswordHash from PasswordKey and Unix timestamp
- **FR-005**: System MUST encrypt and zip Firefox data with PasswordHash before storage
- **FR-006**: System MUST store encrypted files in `$HOME/.cache/briefcase/temp/`
- **FR-007**: System MUST support backup of configured sensitive folders with same encryption process
- **FR-008**: System MUST sync encrypted files to Dropbox when configured
- **FR-009**: System MUST sync encrypted files to OneDrive when configured
- **FR-010**: System MUST sync encrypted files to SSH server when configured
- **FR-011**: System MUST expose CLI commands for PasswordKey generation and verification
- **FR-012**: System MUST expose CLI command to show PasswordHash from password and filename
- **FR-013**: System MUST run as system service with configurable daily/weekly schedule
- **FR-014**: System MUST implement retention policy with MaxRetention (max 10) from config
- **FR-015**: System MUST log all operations to `$HOME/.cache/briefcase/log/` with detailed information
- **FR-016**: System MUST handle Firefox running state by copying data to temp location first
- **FR-017**: System MUST support proxy configuration for cloud sync operations
- **FR-018**: System MUST validate all configuration values before starting operations

### Key Entities *(include if feature involves data)*

- **Configuration**: Contains user preferences, encryption keys, source/destination settings, and retention policies
- **Firefox Data**: Includes bookmarks and saved passwords exported from Firefox profile
- **Sensitive Folder**: User-defined folder containing personal files to be backed up
- **Encrypted Backup**: Zip file containing encrypted data with PasswordHash filename
- **Log Entry**: Detailed record of all backup operations, errors, and system events
- **PasswordKey**: Cryptographic key derived from user password for encryption
- **PasswordHash**: One-time hash used for individual backup file encryption

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete initial setup and configuration in under 2 minutes
- **SC-002**: Firefox backup completes successfully for 95% of test cases within 30 seconds
- **SC-003**: Folder backup handles 10MB of data within 1 minute in 90% of cases
- **SC-004**: Cloud sync operations complete without data loss in 99% of attempts
- **SC-005**: SSH sync operations maintain data integrity in 99.9% of transfers
- **SC-006**: Scheduled backups run reliably with less than 1% failure rate over 30 days
- **SC-007**: Retention policy correctly maintains specified number of backups in 100% of test cases
- **SC-008**: CLI utilities respond to user commands within 1 second in 99% of cases
- **SC-009**: System handles Firefox running state without data corruption in 100% of test cases
- **SC-010**: All operations are logged with sufficient detail for troubleshooting in 100% of cases