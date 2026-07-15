# Agent Instructions

## Scope

These instructions apply to the Briefcase repository.

## Repository map

- `src/`: Rust application code.
- `tests/`: unit and integration tests.
- `docs/`: canonical user, reference, operations, development, and release documentation.
- `Cargo.toml`: package metadata and dependencies.

## Working rules

- Inspect the relevant source, tests, configuration, and documentation before changing behavior.
- Treat runtime source and tests as authoritative; keep documentation honest about implemented versus planned behavior.
- Preserve unrelated user changes in the worktree.
- Use `apply_patch` for file edits.
- Do not commit changes or create branches unless explicitly requested.
- Avoid destructive commands unless explicitly requested.
- Keep changes focused and avoid unrelated refactors.

## Rust conventions

- Follow stable `rustfmt` formatting and idiomatic Rust naming.
- Keep modules cohesive and functions focused.
- Propagate errors with `Result` and `?`; add context at application boundaries.
- Prefer explicit types and meaningful names.
- Add documentation comments for public APIs.
- Use async tokio APIs for I/O-bound work where the surrounding interface is async.

## Security

- Never log or commit passwords, keys, tokens, credentials, or backup archives.
- Validate paths and configuration before processing.
- Use secure defaults and authenticated cryptography.
- Keep generated test data in temporary directories.

## Validation

For Rust changes, run the narrowest relevant tests first, then:

```bash
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

For documentation-only changes, run `git diff --check` and verify local links and referenced paths.
