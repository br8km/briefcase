# rclone Remotes

Briefcase delegates remote credentials and provider setup to rclone. Configure a remote name first:

```bash
rclone config
```

Use the same name in `[remote.<name>]` and set `enabled = true`. Briefcase supports arbitrary rclone remote names; the example configuration includes Dropbox, OneDrive, iCloud Drive, and SFTP entries.

SFTP backups are placed under `/upload/briefcase` to support a chrooted server layout.
