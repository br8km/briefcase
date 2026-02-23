# Research Findings: Sensitive Data Backup

**Date**: 2026-01-19
**Feature**: Sensitive Data Backup Implementation

## Rust CLI Best Practices

**Decision**: Use Clap for subcommand structure with async command handlers, layered config loading, and secure error handling.

**Rationale**: Enables modular CLI design with async support for I/O operations in backup tools. Layered config prevents injection, and typed errors improve reliability.

**Alternatives Considered**: 
- Custom argument parsing: Rejected for lack of subcommand support and boilerplate.
- Sync operations: Rejected for blocking UI during file transfers.

## AES-256 Encryption

**Decision**: Use aes-gcm crate with PBKDF2 key derivation, unique IVs per encryption, and envelope encryption pattern.

**Rationale**: Provides authenticated encryption preventing tampering, secure key derivation, and unique IVs avoid reuse vulnerabilities.

**Alternatives Considered**:
- ChaCha20-Poly1305: Faster but AES-256 specified by user.
- Non-authenticated AES: Rejected for lack of integrity protection.

## 7Zip Compression

**Decision**: Use sevenz-rust crate with LZMA2 preset 9 and AES encryption integration.

**Rationale**: High compression ratio suitable for backup, pure Rust implementation, and built-in encryption support.

**Alternatives Considered**:
- flate2 (GZIP): Rejected for lower compression ratio.
- zstd: Rejected for not being 7Zip format as specified.

## Rclone Integration

**Decision**: Use std::process::Command with tokio for rclone invocation, with folder sync (single rclone sync per remote) for optimal performance.

**Rationale**: 
- Simpler error handling than librclone RPC
- Process spawning overhead eliminated by using folder sync instead of per-file sync
- rclone handles incremental sync (only new/modified files) automatically
- Better reliability through native CLI rather than RPC

**Alternatives Considered**:
- librclone: Rejected for requiring separate server process and complexity
- Per-file sync: Rejected for higher overhead and multiple process spawns
- RC API: Rejected for requiring separate server process

## TOML Configuration

**Decision**: Use toml + serde with deny_unknown_fields, custom validation, and layered loading (defaults → file → env).

**Rationale**: Type-safe parsing, prevents typos, and hierarchical loading supports complex config needs.

**Alternatives Considered**:
- JSON: Rejected for less human-readable.
- Custom format: Rejected for ecosystem maturity.

## Logging

**Decision**: Use tracing with structured logging, configurable levels, stderr output, and monthly rotation with `<%Y-%m>.log` filenames.

**Rationale**: Rich context for debugging, async-safe, and integrates well with CLI apps.

**Alternatives Considered**:
- env_logger: Rejected for less structured output.
- Custom logging: Rejected for reinventing ecosystem.

## Github Actions CI/CD

**Decision**: Matrix builds across Rust versions, cached dependencies, coverage reporting, and security scanning.

**Rationale**: Ensures cross-platform compatibility, fast builds, test coverage visibility, and secure deployments.

**Alternatives Considered**:
- Single job: Rejected for missing multi-version testing.
- No caching: Rejected for slower builds.