# Tasks: Sensitive Data Backup

**Input**: Design documents from `/home/runner/Projects/Ops/briefcase/specs/001-sensitive-data-backup/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Included per TDD approach required by constitution.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Cargo.toml with Rust dependencies per implementation plan
- [x] T002 Create project directory structure per implementation plan
- [x] T003 Initialize basic main.rs with CLI framework setup
- [x] T004 [P] Configure rustfmt and clippy linting

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Create Config entity and TOML parsing in src/models/config.rs
- [x] T006 [P] Create BackupFile entity in src/models/backup_file.rs
- [x] T007 [P] Create LogEntry entity in src/models/log_entry.rs
- [x] T008 [P] Create TempDir entity in src/models/temp_dir.rs
- [x] T009 [P] Create RemoteConfig entity in src/models/remote_config.rs
- [x] T010 Setup tracing logging infrastructure in src/logging.rs
- [x] T011 Implement config validation with error handling in src/config.rs
- [x] T012 Create base CLI command structure in src/cli/mod.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Manual Local Backup (Priority: P1) 🎯 MVP

**Goal**: Enable users to manually back up Firefox and folder data to encrypted local storage

**Independent Test**: Run backup command and verify encrypted zip files created locally without remote sync

### Tests for User Story 1 (MANDATORY - TDD approach) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T013 [P] [US1] Unit tests for config validation in tests/unit/test_config.rs
- [x] T014 [P] [US1] Unit tests for backup file operations in tests/unit/test_backup_file.rs
- [x] T015 [P] [US1] Integration test for backup workflow in tests/integration/test_backup.rs
- [x] T016 [P] [US1] Contract test for backup CLI command in tests/contract/test_backup_command.rs

### Implementation for User Story 1

- [x] T017 [US1] Implement Firefox data export functionality in src/backup/firefox.rs
- [x] T018 [US1] Implement folder copy operations in src/backup/folder.rs
- [x] T019 [US1] Implement AES-256 encryption with PBKDF2 in src/crypto/encrypt.rs
- [x] T020 [US1] Implement 7Zip compression in src/backup/compress.rs
- [x] T021 [US1] Create BackupService coordinating export/compress/encrypt in src/backup/service.rs
- [x] T022 [US1] Implement backup CLI command in src/cli/backup.rs
- [x] T023 [US1] Add retention policy enforcement in src/backup/retention.rs
- [x] T024 [US1] Integrate logging for backup operations

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Remote Data Sync (Priority: P2)

**Goal**: Sync encrypted backups to remote cloud storage providers

**Independent Test**: Run sync command after local backup and verify files uploaded to remote storage

### Tests for User Story 2 (MANDATORY - TDD approach) ⚠️

- [x] T025 [P] [US2] Unit tests for remote config validation in tests/unit/test_remote_config.rs
- [x] T026 [P] [US2] Integration test for sync workflow in tests/integration/test_sync.rs
- [x] T027 [P] [US2] Contract test for sync CLI command in tests/contract/test_sync_command.rs

### Implementation for User Story 2

- [x] T028 [US2] Implement Rclone integration for file operations in src/sync/rclone.rs
- [x] T029 [US2] Create SyncService for upload operations in src/sync/service.rs
- [x] T030 [US2] Implement dry-run mode for sync operations
- [x] T031 [US2] Implement sync CLI command in src/cli/sync.rs
- [x] T032 [US2] Add temp file cleanup after successful sync in src/clean.rs
- [x] T033 [US2] Integrate logging for sync operations

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Automated Scheduled Backups (Priority: P3)

**Goal**: Run backups automatically as background daemon with configurable schedules

**Independent Test**: Configure schedule and verify backups execute at specified intervals

### Tests for User Story 3 (MANDATORY - TDD approach) ⚠️

- [x] T034 [P] [US3] Unit tests for scheduling logic in tests/unit/test_scheduler.rs
- [x] T035 [P] [US3] Integration test for daemon mode in tests/integration/test_daemon.rs
- [x] T036 [P] [US3] Contract test for schedule configuration in tests/contract/test_schedule_config.rs

### Implementation for User Story 3

- [x] T037 [US3] Implement background daemon with tokio in src/scheduler/daemon.rs
- [x] T038 [US3] Create SchedulerService for frequency management in src/scheduler/service.rs
- [x] T039 [US3] Add frequency options (hourly/daily/weekly) per source
- [x] T040 [US3] Implement schedule CLI commands (start/stop/status) in src/cli/schedule.rs
- [x] T041 [US3] Integrate logging for scheduled operations

**Checkpoint**: All core user stories should now be independently functional

---

## Phase 6: User Story 4 - Data Management and Monitoring (Priority: P3)

**Goal**: Provide data restoration, log management, and monitoring capabilities

**Independent Test**: Run decrypt command and verify data restoration, check log rotation

### Tests for User Story 4 (MANDATORY - TDD approach) ⚠️

- [x] T042 [P] [US4] Unit tests for decryption operations in tests/unit/test_decrypt.rs
- [x] T043 [P] [US4] Integration test for restore workflow in tests/integration/test_restore.rs
- [x] T044 [P] [US4] Contract test for crypto CLI commands in tests/contract/test_crypto_command.rs

### Implementation for User Story 4

- [x] T045 [US4] Implement decryption and file restoration in src/crypto/decrypt.rs
- [x] T046 [US4] Implement log rotation with monthly naming in src/logging/rotation.rs
- [x] T047 [US4] Create log management CLI commands in src/cli/logs.rs
- [x] T048 [US4] Implement crypto CLI commands (validate/decrypt) in src/cli/crypto.rs
- [x] T049 [US4] Add backup success rate monitoring
- [x] T050 [US4] Integrate comprehensive logging for all operations

**Checkpoint**: All user stories complete with full monitoring and management

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T051 [P] Add comprehensive documentation comments to all public APIs
- [x] T052 Code cleanup and clippy warning fixes across all modules
- [x] T053 Performance optimization for backup operations
- [x] T054 [P] Add security hardening (input validation, secure defaults)
- [x] T055 [P] Run quickstart.md validation and update examples
- [x] T056 Final integration testing across all user stories
- [x] T057 Update README with build and usage instructions

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-6)**: All depend on Foundational phase completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P4)
- **Polish (Phase 7)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational - Builds on US1 but independently testable
- **User Story 3 (P3)**: Can start after Foundational - Independent daemon functionality
- **User Story 4 (P3)**: Can start after Foundational - Independent management tools

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Core entities before services
- Services before CLI commands
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel
- Once Foundational completes, US1-US4 can start in parallel
- All tests for a story marked [P] can run in parallel
- Different user stories can be worked by different developers

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Unit tests for config validation in tests/unit/test_config.rs"
Task: "Unit tests for backup file operations in tests/unit/test_backup_file.rs"
Task: "Integration test for backup workflow in tests/integration/test_backup.rs"
Task: "Contract test for backup CLI command in tests/contract/test_backup_command.rs"

# Launch core implementation tasks:
Task: "Implement Firefox data export functionality in src/backup/firefox.rs"
Task: "Implement folder copy operations in src/backup/folder.rs"
Task: "Implement AES-256 encryption with PBKDF2 in src/crypto/encrypt.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test backup command independently
5. Deploy/demo MVP

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy (MVP!)
3. Add User Story 2 → Test independently → Deploy
4. Add User Stories 3 & 4 → Test independently → Deploy
5. Polish → Final release

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational done:
   - Developer A: User Story 1 (backup core)
   - Developer B: User Story 2 (sync)
   - Developer C: User Stories 3 & 4 (scheduling & management)
3. Stories integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- Each user story independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at checkpoints to validate independently
- TDD: Tests first, then implementation