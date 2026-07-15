# Configuration Reference

The runtime configuration model contains:

- `general`: password hint, Argon2 password hash, base64-derived encryption key, retention limit, and optional editor.
- `source.firefox`: enabled flag, profile directory, frequency, and `last_backup`.
- `source.folder`: enabled flag, directory, frequency, and `last_backup`.
- `remote.<name>`: rclone remote name, enabled flag, and `last_sync`.

Configuration loading validates retention, enabled source paths, and enabled remote names. Remote API credentials are intentionally outside this file and belong to rclone.
