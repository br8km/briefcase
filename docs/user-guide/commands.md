# Commands

Briefcase exposes these top-level commands:

| Command | Purpose |
| --- | --- |
| `config` | Initialize, edit, validate, show, or verify configuration |
| `backup` | Create encrypted local archives |
| `sync` | Synchronize the data directory through rclone |
| `schedule` | Start, stop, or inspect the backup daemon |
| `crypto` | Validate encryption configuration or decrypt an archive |
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
```
