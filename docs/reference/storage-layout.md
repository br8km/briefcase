# Storage Layout

Briefcase uses platform directories supplied by the `dirs` crate:

- Configuration: platform config directory `/briefcase/briefcase.toml`.
- Data: platform data directory `/briefcase/data` (Windows uses the local data directory).
- Logs: platform data directory `/briefcase/logs` on non-Windows; Windows uses the config directory.
- Scheduler PID file: runtime directory `/briefcase_daemon.pid`, falling back to `/tmp`.
- Backup staging: operating-system temporary directory under a unique `briefcase_*` directory.

The staging directory is removed when its `TempDir` value is dropped.
