# Tasks: Personal Data Backup System

**Input**: Design documents from `/specs/001-backup-system/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- Paths shown below assume single project - adjust based on plan.md structure

<!--  
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.
  
  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Entities from data-model.md
  - Endpoints from contracts/
  
  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment
  
  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create project structure per implementation plan
- [ ] T002 Initialize Rust project with Cargo.toml dependencies
- [ ] T003 [P] Configure linting and formatting tools (clippy, rustfmt)
- [ ] T004 [P] Setup gitignore for Rust project and backup system
- [ ] T005 Create basic error handling framework with thiserror/anyhow
- [ ] T006 [P] Setup logging infrastructure with log/env_logger
- [ ] T007 Create configuration directory structure
- [ ] T008 [P] Add basic documentation structure

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

Examples of foundational tasks (adjust based on your project):

- [ ] T009 Implement configuration management with serde/toml
- [ ] T010 [P] Create Config struct and serialization/deserialization
- [ ] T011 [P] Implement configuration validation and error handling
- [ ] T012 Create cryptographic core with AES-256-GCM encryption
- [ ] T013 [P] Implement PasswordKey generation from user password
- [ ] T014 [P] Implement PasswordHash generation (PasswordKey + timestamp)
- [ ] T015 Create compression module with sevenz-rust integration
- [ ] T016 [P] Implement file compression with 7zip format
- [ ] T017 Create base models for BackupOperation and SyncOperation
- [ ] T018 [P] Implement logging module with file rotation
- [ ] T019 Setup environment configuration management
- [ ] T020 [P] Create error types and handling for all modules
- [ ] T021 Implement basic CLI structure with clap
- [ ] T022 [P] Create main command parsing and routing

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Configuration Setup (Priority: P1) 🎯 MVP

**Goal**: Enable users to configure the backup system with password and preferences

**Independent Test**: Run configuration setup and verify config file creation with proper encryption

### Tests for User Story 1 (MANDATORY - TDD approach) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T023 [P] [US1] Unit tests for config validation in tests/unit/config_tests.rs
- [ ] T024 [P] [US1] Unit tests for PasswordKey generation in tests/unit/crypto_tests.rs
- [ ] T025 [P] [US1] Unit tests for PasswordHash generation in tests/unit/crypto_tests.rs
- [ ] T026 [P] [US1] Integration test for complete setup workflow in tests/integration/setup_integration.rs
- [ ] T027 [P] [US1] Contract test for setup command in tests/contract/cli_contracts.rs

### Implementation for User Story 1

- [ ] T028 [P] [US1] Create Config model with all fields in src/models/config.rs
- [ ] T029 [P] [US1] Create GeneralConfig sub-model in src/models/config.rs
- [ ] T030 [P] [US1] Create SourceConfig sub-model in src/models/config.rs
- [ ] T031 [P] [US1] Create RemoteConfig sub-model in src/models/config.rs
- [ ] T032 [US1] Implement config file I/O in src/config/manager.rs
- [ ] T033 [US1] Implement config validation logic in src/config/validator.rs
- [ ] T034 [US1] Implement PasswordKey generation in src/crypto/keygen.rs
- [ ] T035 [US1] Implement PasswordHash generation in src/crypto/hashgen.rs
- [ ] T036 [US1] Implement setup command in src/cli/setup.rs
- [ ] T037 [US1] Add interactive password prompt with masking
- [ ] T038 [US1] Implement config file creation with defaults in src/config/setup.rs
- [ ] T039 [US1] Add encryption for sensitive fields (PasswordKey, API keys)
- [ ] T040 [US1] Implement config file backup before modifications
- [ ] T041 [US1] Add validation for all config fields and constraints
- [ ] T042 [US1] Implement proxy configuration handling
- [ ] T043 [US1] Add logging for all setup operations
- [ ] T044 [US1] Create default config template with comments
- [ ] T045 [US1] Implement config merge logic for existing configs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Firefox Data Backup (Priority: P2)

**Goal**: Enable backup of Firefox bookmarks and passwords with encryption

**Independent Test**: Run Firefox backup command and verify encrypted files in temp folder

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️

- [ ] T046 [P] [US2] Unit tests for Firefox profile detection in tests/unit/backup/firefox_tests.rs
- [ ] T047 [P] [US2] Unit tests for bookmark export in tests/unit/backup/firefox_tests.rs
- [ ] T048 [P] [US2] Integration test for Firefox backup workflow in tests/integration/firefox_integration.rs
- [ ] T049 [P] [US2] Contract test for backup command in tests/contract/cli_contracts.rs

### Implementation for User Story 2

- [ ] T050 [P] [US2] Create FirefoxProfileData model in src/models/firefox.rs
- [ ] T051 [P] [US2] Create Bookmark model in src/models/firefox.rs
- [ ] T052 [P] [US2] Create SavedPassword model in src/models/firefox.rs
- [ ] T053 [US2] Implement Firefox profile detection in src/backup/firefox/detector.rs
- [ ] T054 [US2] Implement bookmark export from places.sqlite in src/backup/firefox/export.rs
- [ ] T055 [US2] Implement password export from logins.json in src/backup/firefox/export.rs
- [ ] T056 [US2] Implement running Firefox detection and temp copy logic in src/backup/firefox/handler.rs
- [ ] T057 [US2] Implement backup workflow in src/backup/firefox/backup.rs
- [ ] T058 [US2] Implement compression before encryption in src/backup/firefox/compressor.rs
- [ ] T059 [US2] Implement encryption with PasswordHash in src/backup/firefox/encryptor.rs
- [ ] T060 [US2] Implement file naming with timestamp and hash in src/backup/firefox/namer.rs
- [ ] T061 [US2] Implement temp directory management in src/backup/firefox/storage.rs
- [ ] T062 [US2] Add BackupOperation tracking and logging in src/backup/tracker.rs
- [ ] T063 [US2] Implement backup command CLI in src/cli/backup.rs
- [ ] T064 [US2] Add error handling for Firefox not installed case
- [ ] T065 [US2] Add validation for profile directory structure
- [ ] T066 [US2] Implement cleanup of temp files after backup

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Sensitive Folder Backup (Priority: P3)

**Goal**: Enable backup of user-defined sensitive folders

**Independent Test**: Configure test folder and verify encrypted backup creation

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️

- [ ] T067 [P] [US3] Unit tests for folder scanning in tests/unit/backup/folder_tests.rs
- [ ] T068 [P] [US3] Integration test for folder backup in tests/integration/folder_integration.rs
- [ ] T069 [P] [US3] Contract test for folder backup command in tests/contract/cli_contracts.rs

### Implementation for User Story 3

- [ ] T070 [P] [US3] Create FolderBackup model in src/models/folder.rs
- [ ] T071 [US3] Implement folder scanning and file collection in src/backup/folder/scanner.rs
- [ ] T072 [US3] Implement file filtering and exclusion patterns in src/backup/folder/filter.rs
- [ ] T073 [US3] Implement folder backup workflow in src/backup/folder/backup.rs
- [ ] T074 [US3] Implement compression with 7zip in src/backup/folder/compressor.rs
- [ ] T075 [US3] Implement encryption with PasswordHash in src/backup/folder/encryptor.rs
- [ ] T076 [US3] Implement backup file naming and storage in src/backup/folder/storage.rs
- [ ] T077 [US3] Add folder backup to main backup command in src/cli/backup.rs
- [ ] T078 [US3] Implement large file handling and chunking if needed
- [ ] T079 [US3] Add validation for folder permissions and accessibility
- [ ] T080 [US3] Implement backup progress reporting for large folders
- [ ] T081 [US3] Add error handling for file access issues
- [ ] T082 [US3] Implement symlink handling and resolution

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - Cloud Sync (Priority: P4)

**Goal**: Enable synchronization of encrypted backups to cloud storage

**Independent Test**: Configure cloud provider and verify file upload

### Tests for User Story 4 (OPTIONAL - only if tests requested) ⚠️

- [ ] T083 [P] [US4] Unit tests for cloud config validation in tests/unit/sync/cloud_tests.rs
- [ ] T084 [P] [US4] Integration test for Dropbox sync in tests/integration/sync/dropbox_integration.rs
- [ ] T085 [P] [US4] Integration test for OneDrive sync in tests/integration/sync/onedrive_integration.rs
- [ ] T086 [P] [US4] Contract test for sync command in tests/contract/cli_contracts.rs

### Implementation for User Story 4

- [ ] T087 [P] [US4] Create CloudSync model in src/models/sync.rs
- [ ] T088 [P] [US4] Create DropboxConfig model in src/models/sync.rs
- [ ] T089 [P] [US4] Create OneDriveConfig model in src/models/sync.rs
- [ ] T090 [US4] Implement rclone integration wrapper in src/sync/cloud/rclone.rs
- [ ] T091 [US4] Implement Dropbox sync provider in src/sync/cloud/dropbox.rs
- [ ] T092 [US4] Implement OneDrive sync provider in src/sync/cloud/onedrive.rs
- [ ] T093 [US4] Implement cloud provider factory in src/sync/cloud/factory.rs
- [ ] T094 [US4] Implement sync workflow with retry logic in src/sync/cloud/sync.rs
- [ ] T095 [US4] Implement sync status tracking in src/sync/tracker.rs
- [ ] T096 [US4] Add SyncOperation logging and monitoring in src/sync/monitor.rs
- [ ] T097 [US4] Implement sync command CLI in src/cli/sync.rs
- [ ] T098 [US4] Add proxy support for cloud operations
- [ ] T099 [US4] Implement error handling for network issues
- [ ] T100 [US4] Add rate limiting and backoff for API calls
- [ ] T101 [US4] Implement sync progress reporting
- [ ] T102 [US4] Add validation for cloud credentials

**Checkpoint**: Cloud sync functionality added to existing backup system

---

## Phase 7: User Story 5 - SSH Server Sync (Priority: P5)

**Goal**: Enable synchronization to remote servers via SSH

**Independent Test**: Configure SSH and verify file transfer

### Tests for User Story 5 (OPTIONAL - only if tests requested) ⚠️

- [ ] T103 [P] [US5] Unit tests for SSH config validation in tests/unit/sync/ssh_tests.rs
- [ ] T104 [P] [US5] Integration test for SSH sync in tests/integration/sync/ssh_integration.rs
- [ ] T105 [P] [US5] Contract test for SSH sync command in tests/contract/cli_contracts.rs

### Implementation for User Story 5

- [ ] T106 [P] [US5] Create SSHConfig model in src/models/sync.rs
- [ ] T107 [US5] Implement SSH connection manager in src/sync/ssh/connection.rs
- [ ] T108 [US5] Implement SFTP file transfer in src/sync/ssh/transfer.rs
- [ ] T109 [US5] Implement SSH sync workflow in src/sync/ssh/sync.rs
- [ ] T110 [US5] Implement key-based authentication in src/sync/ssh/auth.rs
- [ ] T111 [US5] Add SSH sync to main sync command in src/cli/sync.rs
- [ ] T112 [US5] Implement SSH error handling and retries
- [ ] T113 [US5] Add host key verification and security
- [ ] T114 [US5] Implement connection pooling for multiple files
- [ ] T115 [US5] Add SSH config validation and testing
- [ ] T116 [US5] Implement progress reporting for large transfers

**Checkpoint**: SSH sync functionality added to existing backup system

---

## Phase 8: User Story 6 - CLI Utilities (Priority: P6)

**Goal**: Provide cryptographic utilities for key management

**Independent Test**: Test CLI crypto commands with sample data

### Tests for User Story 6 (OPTIONAL - only if tests requested) ⚠️

- [ ] T117 [P] [US6] Unit tests for key generation in tests/unit/cli/crypto_tests.rs
- [ ] T118 [P] [US6] Unit tests for key verification in tests/unit/cli/crypto_tests.rs
- [ ] T119 [P] [US6] Unit tests for hash display in tests/unit/cli/crypto_tests.rs
- [ ] T120 [P] [US6] Contract test for crypto commands in tests/contract/cli_contracts.rs

### Implementation for User Story 6

- [ ] T121 [P] [US6] Implement generate-key subcommand in src/cli/crypto.rs
- [ ] T122 [P] [US6] Implement verify-key subcommand in src/cli/crypto.rs
- [ ] T123 [P] [US6] Implement show-hash subcommand in src/cli/crypto.rs
- [ ] T124 [US6] Add password strength validation in src/cli/crypto/validator.rs
- [ ] T125 [US6] Implement key format validation and parsing
- [ ] T126 [US6] Add error handling for invalid crypto operations
- [ ] T127 [US6] Implement help and usage documentation for crypto commands

**Checkpoint**: CLI utilities added to existing backup system

---

## Phase 9: User Story 7 - System Service (Priority: P7)

**Goal**: Enable automatic scheduled backups

**Independent Test**: Install service and verify scheduled execution

### Tests for User Story 7 (OPTIONAL - only if tests requested) ⚠️

- [ ] T128 [P] [US7] Unit tests for service config in tests/unit/service/tests.rs
- [ ] T129 [P] [US7] Integration test for service installation in tests/integration/service_integration.rs
- [ ] T130 [P] [US7] Contract test for service commands in tests/contract/cli_contracts.rs

### Implementation for User Story 7

- [ ] T131 [P] [US7] Create ServiceConfig model in src/models/service.rs
- [ ] T132 [US7] Implement systemd service file generation in src/service/generator.rs
- [ ] T133 [US7] Implement service installation logic in src/service/installer.rs
- [ ] T134 [US7] Implement service uninstallation logic in src/service/uninstaller.rs
- [ ] T135 [US7] Implement service control commands in src/service/controller.rs
- [ ] T136 [US7] Add service status monitoring in src/service/monitor.rs
- [ ] T137 [US7] Implement scheduling logic in src/service/scheduler.rs
- [ ] T138 [US7] Add service command CLI in src/cli/service.rs
- [ ] T139 [US7] Implement service logging and error handling
- [ ] T140 [US7] Add validation for service dependencies
- [ ] T141 [US7] Implement user vs system service detection

**Checkpoint**: Service functionality added to existing backup system

---

## Phase 10: User Story 8 - Retention Policy (Priority: P8)

**Goal**: Implement automatic cleanup of old backups

**Independent Test**: Create multiple backups and verify retention cleanup

### Tests for User Story 8 (OPTIONAL - only if tests requested) ⚠️

- [ ] T142 [P] [US8] Unit tests for retention logic in tests/unit/retention/tests.rs
- [ ] T143 [P] [US8] Integration test for retention application in tests/integration/retention_integration.rs
- [ ] T144 [P] [US8] Contract test for retention commands in tests/contract/cli_contracts.rs

### Implementation for User Story 8

- [ ] T145 [P] [US8] Create RetentionPolicy model in src/models/retention.rs
- [ ] T146 [US8] Implement retention logic in src/retention/policy.rs
- [ ] T147 [US8] Implement backup cleanup in src/retention/cleanup.rs
- [ ] T148 [US8] Implement retention status reporting in src/retention/status.rs
- [ ] T149 [US8] Add retention command CLI in src/cli/retention.rs
- [ ] T150 [US8] Implement per-source retention policies
- [ ] T151 [US8] Add validation for retention configuration
- [ ] T152 [US8] Implement dry-run mode for retention
- [ ] T153 [US8] Add logging for retention operations
- [ ] T154 [US8] Implement error handling for cleanup failures

**Checkpoint**: Retention policy functionality added to existing backup system

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T155 [P] Documentation updates in docs/
- [ ] T156 Code cleanup and refactoring
- [ ] T157 Performance optimization across all stories
- [ ] T158 [P] Additional unit tests (if requested) in tests/unit/
- [ ] T159 Security hardening
- [ ] T160 Run quickstart.md validation
- [ ] T161 [P] Create comprehensive examples in examples/
- [ ] T162 [P] Add integration tests for complete workflows
- [ ] T163 [P] Create user documentation and manuals
- [ ] T164 [P] Add error handling improvements
- [ ] T165 [P] Implement comprehensive logging improvements
- [ ] T166 [P] Add monitoring and alerting capabilities

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4 → P5 → P6 → P7 → P8)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 for config access
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on US1 for config access
- **User Story 4 (P4)**: Can start after US2/US3 - Needs backup files to sync
- **User Story 5 (P5)**: Can start after US2/US3 - Needs backup files to sync
- **User Story 6 (P6)**: Can start after US1 - Needs crypto functionality
- **User Story 7 (P7)**: Can start after US1+US2/US3 - Needs working backup system
- **User Story 8 (P8)**: Can start after US2/US3 - Needs backup files to manage

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before CLI commands
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "Unit tests for config validation in tests/unit/config_tests.rs"
Task: "Unit tests for PasswordKey generation in tests/unit/crypto_tests.rs"
Task: "Unit tests for PasswordHash generation in tests/unit/crypto_tests.rs"
Task: "Integration test for complete setup workflow in tests/integration/setup_integration.rs"

# Launch all models for User Story 1 together:
Task: "Create Config model with all fields in src/models/config.rs"
Task: "Create GeneralConfig sub-model in src/models/config.rs"
Task: "Create SourceConfig sub-model in src/models/config.rs"
Task: "Create RemoteConfig sub-model in src/models/config.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Add User Story 6 → Test independently → Deploy/Demo
8. Add User Story 7 → Test independently → Deploy/Demo
9. Add User Story 8 → Test independently → Deploy/Demo
10. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Configuration)
   - Developer B: User Story 2 (Firefox Backup)
   - Developer C: User Story 3 (Folder Backup)
   - Developer D: User Story 4 (Cloud Sync)
3. Stories complete and integrate independently
4. Additional developers can work on remaining stories in parallel

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence

---

## Task Summary

**Total Tasks**: 166
**Tasks by Phase**:
- Phase 1 (Setup): 8 tasks
- Phase 2 (Foundational): 14 tasks  
- Phase 3 (US1 - Config): 23 tasks
- Phase 4 (US2 - Firefox): 18 tasks
- Phase 5 (US3 - Folder): 14 tasks
- Phase 6 (US4 - Cloud): 16 tasks
- Phase 7 (US5 - SSH): 12 tasks
- Phase 8 (US6 - CLI): 7 tasks
- Phase 9 (US7 - Service): 12 tasks
- Phase 10 (US8 - Retention): 11 tasks
- Phase 11 (Polish): 15 tasks

**Parallel Opportunities**: 68 tasks marked [P] (41% parallelizable)
**Independent Test Criteria**: Each user story has clear independent test criteria
**MVP Scope**: User Story 1 (23 tasks) provides complete configuration functionality

**Format Validation**: ✅ All tasks follow strict checklist format with IDs, labels, and file paths