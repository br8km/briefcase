# Testing

Run the complete suite with:

```bash
cargo test
cargo fmt --check
cargo clippy -- -D warnings
```

Use a test-first workflow for behavior changes: define the failing case, implement the smallest correct change, then refactor. Tests should cover configuration, backup workflows, encryption/decryption, scheduling, daemon behavior, sync, CLI arguments, and restore flows.

Tests use temporary directories for generated data and should cover both success and failure paths, including invalid configuration, missing paths, incorrect passwords, remote failures, and cleanup behavior.
