# Installation

## From source

Briefcase requires Rust 1.75 or newer.

```bash
git clone https://github.com/br8km/briefcase.git
cd briefcase
cargo build --release
```

The resulting binary is `target/release/briefcase`.

On Unix/Linux, install the binary into the default user binary directory with:

```bash
./target/release/briefcase install
```

The command uses `$XDG_BIN_HOME` when set, otherwise `$HOME/.local/bin`, and installs the executable as `briefcase`. Use `--path` to select another destination directory:

```bash
./target/release/briefcase install --path "$HOME/.local/bin"
```

Installation only copies the binary. Run `briefcase config init` afterward to create the configuration and generate the password hash and encryption key. The `install` command is not available on Windows; use `scripts/install.ps1` there.

## Runtime dependency

Remote synchronization requires an installed and configured `rclone` executable. Local backup and recovery do not require a remote provider.

The source contains platform-specific path handling, but detached scheduling is Unix-oriented. Verify the target platform by building and testing it before relying on cross-platform release support.
