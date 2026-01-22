# Feature Specification: Sensitive Data Backup

**Feature Branch**: `001-sensitive-data-backup`  
**Created**: 2026-01-19  
**Status**: Draft  
**Input**: User description: "@PRD/v1/specify.md @PRD/v1/config.default.toml @PRD/v1/config.testing.toml "

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Manual Local Backup (Priority: P1)

As a Linux user with sensitive personal data, I want to manually back up my Firefox bookmarks and saved passwords along with a specified sensitive folder to encrypted local storage so that my data is securely protected against loss or corruption.

**Why this priority**: This is the core functionality that provides immediate value by enabling secure local backups of critical personal data.

**Independent Test**: Can be fully tested by running the backup command and verifying encrypted files are created locally, delivering secure data protection without remote features.

**Acceptance Scenarios**:

1. **Given** valid config with Firefox and folder sources enabled and paths exist, **When** user runs backup command with correct password, **Then** Firefox data is exported to temp folder, sensitive folder copied, data compressed and encrypted into dated zip files in local data directory.
2. **Given** backup files exceed max retention limit, **When** new backup completes, **Then** oldest zip files are automatically removed.
3. **Given** invalid source directory path in config, **When** user runs backup, **Then** error is reported and backup fails gracefully.

**Performance Targets**: Backup must complete within 5 minutes for up to 32MB of data.

**Security Requirements**: All data must be encrypted using strong hashing algorithm with user-provided password, data at rest must be protected.

---

### User Story 2 - Remote Data Sync (Priority: P2)

As a user with local backups, I want to sync my encrypted backup files to remote cloud storage providers so that my data has off-site redundancy and availability.

**Why this priority**: Adds data redundancy and accessibility, building on local backup foundation.

**Independent Test**: Can be tested by running sync command after local backup, delivering remote storage capability independently.

**Acceptance Scenarios**:

1. **Given** local backups exist and valid remote config (Dropbox/OneDrive/iCloud/SFTP), **When** user runs sync command, **Then** zip files are uploaded to remote storage.
2. **Given** dry-run option enabled, **When** user runs sync, **Then** sync operations are simulated without actual upload.
3. **Given** remote config invalid or credentials missing, **When** user runs sync, **Then** warning message displayed and sync skipped.

---

### User Story 3 - Automated Scheduled Backups (Priority: P3)

As a user who wants hands-off operation, I want the application to run as a background daemon service that automatically performs backups at configured frequencies so that I don't have to remember to run manual backups.

**Why this priority**: Enhances user experience with automation, reduces manual effort.

**Independent Test**: Can be tested by configuring schedule and verifying backups occur at specified intervals.

**Acceptance Scenarios**:

1. **Given** frequency set to 'daily' for a source, **When** daemon is running, **Then** backup automatically executes once per day.
2. **Given** different frequencies for different sources, **When** daemon running, **Then** each source backs up according to its own schedule.

---

### User Story 4 - Data Management and Monitoring (Priority: P3)

As a user managing my backups, I want detailed logging, cleanup capabilities, and the ability to decrypt and restore data so that I can monitor operations and recover data when needed.

**Why this priority**: Provides operational visibility and recovery capabilities.

**Independent Test**: Can be tested by examining logs, running clean command, and decrypting files.

**Acceptance Scenarios**:

1. **Given** operations performed, **When** user checks logs, **Then** detailed JSON logs with levels are available, rotated monthly and by size.
2. **Given** sync successful, **When** user runs clean command, **Then** temp files are deleted.
3. **Given** encrypted zip file and correct password, **When** user runs decrypt command, **Then** original files/folders are restored.

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- What happens when Firefox is running during backup? System should copy relevant files/databases to temp folder and proceed with backup.
- How does system handle invalid or non-existent source directory paths? Report error and fail backup.
- What happens when user provides incorrect password? Validation fails, operation aborted.
- How does system handle network failures during remote sync? Retry mechanism or mark as failed with logging.
- What happens when remote storage is full? Sync fails with appropriate error message.
- How does system handle concurrent backup attempts? Prevent multiple simultaneous backups.
- What happens when temp directory lacks sufficient space? Backup fails with space error.
- How does system handle corrupted Firefox profile data? Skip Firefox backup and continue with folder if possible.

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST initialize default configuration file with required settings and warn if file already exists.
- **FR-002**: System MUST edit and validate configuration variables, reporting errors for invalid paths or settings.
- **FR-003**: System MUST accept user password input with password hint for authentication.
- **FR-004**: System MUST generate PasswordKey from user password using strong hashing algorithm and store in config.
- **FR-005**: System MUST export Firefox bookmarks and saved passwords to temporary folder during backup.
- **FR-006**: System MUST copy sensitive folder contents to temporary folder during backup.
- **FR-007**: System MUST compress Firefox and folder data into encrypted zip files using PasswordKey and datetime, storing in data directory.
- **FR-008**: System MUST remove oldest zip files based on max retention setting.
- **FR-009**: System MUST sync zipped data to configured remote cloud storage providers (Dropbox, OneDrive, iCloud).
- **FR-010**: System MUST sync zipped data to remote SFTP servers.
- **FR-011**: System MUST support dry-run mode for sync operations.
- **FR-012**: System MUST provide detailed logging with configurable levels, storing in JSON format.
- **FR-013**: System MUST rotate logs based on monthly time and 10MB size limits, keeping maximum 3 files, with filenames formatted as `<%Y-%m>.log`.
- **FR-014**: System MUST delete temporary files after successful sync operations.
- **FR-015**: System MUST delete log files when requested.
- **FR-016**: System MUST validate user password against stored PasswordKey for decryption.
- **FR-017**: System MUST decrypt and restore original files/folders from encrypted zips.
- **FR-018**: System MUST run as background daemon service for scheduled backups.
- **FR-019**: System MUST support different backup frequencies (hourly, daily, weekly) per source.
- **FR-020**: System MUST enforce retention policy with configurable max retention (0-10).

### Key Entities *(include if feature involves data)*

- **Configuration File**: Stores user settings including password key, source paths, frequencies, remote credentials, and retention settings.
- **Backup Zip Files**: Encrypted compressed archives containing Firefox data and sensitive folders, named with datetime stamps.
- **Log Files**: JSON-formatted operation logs with levels, rotated by time and size.
- **Temporary Files**: Unencrypted data staging area during backup process.
- **Remote Storage**: Cloud providers and SFTP servers for off-site data storage.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Users can complete a full backup process (config to encrypted storage) in under 5 minutes for up to 32MB of data.
- **SC-002**: System achieves 95% success rate for backup operations over a 30-day period.
- **SC-003**: Encrypted data remains recoverable with correct password in 100% of test cases.
- **SC-004**: Remote sync operations complete within 10 minutes for 32MB of data with valid network connection.
- **SC-005**: Scheduled backups execute at configured frequencies with 98% reliability.
- **SC-006**: Log files provide sufficient detail to diagnose issues in 90% of failure scenarios.
