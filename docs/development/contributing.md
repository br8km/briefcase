# Contributing

## Workflow

1. Create a feature branch from `main`.
2. Make focused changes and add tests.
3. Run formatting, linting, and the relevant test suite.
4. Update user documentation and configuration examples when behavior changes.
5. Open a pull request with a conventional commit-style summary.

## Required checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

Public APIs should have Rust documentation comments. Tests should cover both successful and failing paths, and generated sensitive data belongs in temporary directories rather than the repository.

Follow stable Rust style, `rustfmt`, `clippy`, and the Rust API Guidelines. Keep public APIs, complex functions, module boundaries, and user workflows documented as they evolve.

Never commit passwords, encryption keys, rclone credentials, or backup archives.

## Commit messages

Use the conventional format `type(scope): description`. Common types are `feat`, `fix`, `docs`, `test`, `refactor`, and `chore`.

## Pull requests

Before opening a pull request, confirm that tests, formatting, clippy, documentation, and breaking-change notes are up to date. Include reproduction steps and environment details for bug fixes.

Security issues should be reported privately rather than through a public issue.

Contribution opportunities include performance work, verified platform support, better CLI progress output, new backup sources, monitoring, and an extensible source system.
