# Installation

## From source

Briefcase requires Rust 1.75 or newer.

```bash
git clone https://github.com/br8km/briefcase.git
cd briefcase
cargo build --release
```

The resulting binary is `target/release/briefcase`.

## Runtime dependency

Remote synchronization requires an installed and configured `rclone` executable. Local backup and recovery do not require a remote provider.

The source contains platform-specific path handling, but detached scheduling is Unix-oriented. Verify the target platform by building and testing it before relying on cross-platform release support.
