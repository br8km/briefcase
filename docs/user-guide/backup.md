# Local Backups

`briefcase backup` processes every enabled source independently.

- Firefox exports bookmarks from `places.sqlite` to `bookmarks.html` and copies `logins.json`, `key4.db`, and `prefs.js` when present.
- Folder sources are copied recursively under a `sensitive/` staging directory.
- Staging data is compressed as a 7z archive and then encrypted with AES-256-GCM.
- Archives are written to the Briefcase data directory as `Firefox_<timestamp>.7z` or `Folder_<timestamp>.7z`.
- Retention is enforced independently for Firefox and Folder archives.
- The matching source `last_backup` is updated only after that source completes.

The current implementation enforces the 32 MiB staging limit for Firefox backups. Folder backups do not currently receive the same size check.

Dry runs return before reading the configured encryption key and only report that enabled sources would be backed up; they do not inspect or copy source data.
