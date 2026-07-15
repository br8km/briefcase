# CLI Reference

The CLI currently accepts string actions for `config` and `crypto` rather than typed subcommands.

`config` actions are `init`, `edit`, `validate`, `show`, and `verify`. `init` requires `--password` and `--password-hint`; `verify` requires `--password`.

`backup` and `sync` each accept `--dry-run`. `crypto decrypt` requires `--input` and `--output`; `crypto validate` checks the configured password hash.

`version` prints the package version. `install` is supported on Unix/Linux and accepts an optional destination directory through `--path`. The default binary directory is `$XDG_BIN_HOME` when set, otherwise `$HOME/.local/bin`. Configuration is created separately by `config init`.

The authoritative parser definitions are [src/cli/mod.rs](../../src/cli/mod.rs:12), [src/cli/config.rs](../../src/cli/config.rs:14), and [src/cli/crypto.rs](../../src/cli/crypto.rs:10).
