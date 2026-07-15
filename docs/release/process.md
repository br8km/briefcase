# Release Process

Before release:

1. Update the version in `Cargo.toml`.
2. Update `docs/release/changelog.md`.
3. Run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test`.
4. Verify backup and sync timestamp persistence behavior.
5. Build the release binary and verify the documented platform scope.
6. Tag the release and publish the artifacts on the repository release page.

Build and behavior-test every advertised release target independently before calling it supported. The current release metadata lists Linux x86_64, macOS x86_64, macOS arm64, and Windows x64 artifacts.
