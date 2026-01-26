# Remote

- [x] add rclone `sftp`
- implement remote sync logic
  - [?] create|copy <appname> folder
  - use `sync/sync` for rclone



According to rclone document, I'd like to rewrite the config logic for this backup application like this: 

```toml

[remote.dropbox]
name = "dropbox"
enabled = true

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
There will not have api_key, api_secret etc. as they will be in the rclone config file instead. when run rclone `sync/sync` command, we use remote backend `name` string to connect if enabled.

Consider this request, make up a reasonable todo list to update the source code and documentations.
