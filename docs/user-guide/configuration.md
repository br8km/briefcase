# Configuration

The default file is `briefcase.toml` inside the platform configuration directory under `briefcase/`. `config init` creates the directory and writes the file.

```toml
# Generated secrets are normally populated by `briefcase config init`.
[general]
password_hint = "Your password hint here"
password_hash = ""
encryption_key = ""
max_retention = 10
text_editor = "vi"

[source.firefox]
enabled = false
dir = "/path/to/firefox/profile"
frequency = "Daily"

[source.folder]
enabled = false
dir = "/path/to/sensitive/folder"
frequency = "Daily"

[remote.dropbox]
name = "dropbox"
enabled = false

[remote.onedrive]
name = "onedrive"
enabled = false

[remote.iclouddrive]
name = "iclouddrive"
enabled = false

[remote.sftp]
name = "sftp"
enabled = false
```

`max_retention` must be between 1 and 10. Enabled source paths must exist. Frequencies are `Hourly`, `Daily`, or `Weekly`.

Remote credentials are managed by rclone. Remote tables use the flattened form `[remote.<name>]`; `[remote.remotes.<name>]` is not supported.

`last_backup` and `last_sync` are application-managed local timestamps in `YYYY-MM-DD HH:MM:SS` format. Do not edit them manually unless recovering a configuration.
