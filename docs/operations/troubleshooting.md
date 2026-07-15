# Troubleshooting

## Source path errors

Run `briefcase config validate` and confirm every enabled source directory exists and is readable.

When accessing a Firefox profile, run Briefcase as the same user as Firefox and verify the profile files are readable.

## Decryption failures

Use the same password used during initialization, verify the archive was not corrupted, and remember that the configuration stores the derived key used by normal backup operations.

## Sync failures

Confirm `rclone` is installed, configure the named remote with `rclone config`, and run `briefcase sync --dry-run`. The command validates enabled remotes before syncing.

If authentication fails, re-authenticate the rclone remote. If OneDrive reports `nameAlreadyExists` or cannot create an upload session for a `.7z` file, enable `expose_onenote_files` in its advanced configuration.

## Daemon issues

Check `briefcase schedule status`, inspect the configured log directory, and verify that source timestamps and frequencies are valid.

If resource usage is high, reduce backup frequency, check for unexpectedly large folders, and use `briefcase backup --dry-run` to confirm which sources are enabled.
