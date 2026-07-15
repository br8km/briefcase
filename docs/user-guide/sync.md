# Remote Sync

`briefcase sync` validates enabled rclone remotes and synchronizes the entire local data directory once per enabled remote. rclone performs incremental transfer behavior.

Remote destinations use `<remote-name>:/briefcase`. SFTP destinations use `<remote-name>:/upload/briefcase`.

```bash
briefcase sync
briefcase sync --dry-run
```

Dry runs pass `--dry-run` to rclone and do not update timestamps. A successful non-dry-run sync updates `remote.<name>.last_sync` only for remotes whose sync completed successfully.

The command currently returns an error when enabled remote validation fails; it does not continue with other remotes after that validation failure.

Common provider names include Dropbox, OneDrive, iCloud Drive, and SFTP. Credentials and provider-specific settings are configured through `rclone config`, not stored in `briefcase.toml`.

For OneDrive errors involving `.7z` files and OneNote detection, enable `expose_onenote_files` in the remote's advanced rclone configuration.
