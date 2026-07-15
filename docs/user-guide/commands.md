# Commands

Briefcase exposes these top-level commands:

| Command | Purpose |
| --- | --- |
| `config` | Initialize, edit, validate, show, or verify configuration |
| `backup` | Create encrypted local archives |
| `sync` | Synchronize the data directory through rclone |
| `schedule` | Start, stop, or inspect the backup daemon |
| `crypto` | Validate encryption configuration or decrypt an archive |
| `version` | Print the current Briefcase version |
| `install` | Install the binary into a Unix/Linux user binary directory |
| `clean` | Delete all files in the data and log directories |
| `uninstall` | Delete application data, logs, configuration, and the binary |

The command parser is defined in `src/cli/`. Use `--help` on any command for the current argument surface.

Common examples:

```bash
briefcase backup --dry-run
briefcase sync --dry-run
briefcase config verify --password "your-password"
briefcase schedule start --detach
briefcase schedule status
briefcase version
briefcase install
briefcase install --path "$HOME/.local/bin"
```

`install` is currently supported on Unix/Linux only. It uses `$XDG_BIN_HOME` when set, otherwise `$HOME/.local/bin`. The `--path` value is a destination directory, and the binary is installed as `briefcase` inside it. Run `briefcase config init` separately to create the configuration.
