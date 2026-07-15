# Architecture

Briefcase is a single Rust CLI crate.

```text
src/
├── cli/        command parsing and handlers
├── config.rs   TOML loading, persistence, and validation
├── models/     configuration and backup entities
├── backup/     source export, staging, compression, and retention
├── crypto/     Argon2 derivation and AES-GCM encryption
├── sync/       rclone process integration
├── scheduler/  periodic source checks and daemon behavior
└── logging.rs  env_logger setup and monthly file output
```

The CLI coordinates services; source-specific operations remain in the backup module, and remote operations remain behind the sync module.

## Design principles

- Model the domain explicitly through configuration, source, archive, staging, remote, and scheduling concepts.
- Keep modules single-purpose with small, explicit interfaces and minimal coupling.
- Prefer safe Rust and avoid `unsafe`; any unavoidable use requires a focused justification and tests.
- Treat validation, authenticated encryption, atomic cleanup, and clear ownership of temporary data as data-integrity boundaries.
- Propagate errors with useful context so one failure does not become an unexplained cascading failure.
