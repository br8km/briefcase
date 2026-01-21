# Temp


- Password Derivation
- Master Keys
- Key Files



## Features



- The arguments for hashing algorithm include: `BakingHint` string from config, `datetime` string from realtime unix timestamp, and `BakingPwd` environment variable.

- Export Firefox bookmarks from provided profile folder, and Use generated `BakingHash` as password to zip it. The zipped file will be named as `firefox_bookmarks_latest.zip` and stored in `.cache/baking/`.

- Export Firefox saved passwords from provided profile folder, and Use generated `BakingHash` as password to zip it. The zipped file will be named as `firefox_saved_passwords_latest.zip` and stored in `.cache/baking/`.

- For each folder from `source.dirs` of config file, Use generated `BakingHash` as password to zip it, then named as `source_dirname_latest.zip` and stored in `.cache/baking/`

- Schedule to run backup process Hourly for Firefox bookmarks and saved passwords

- Schedule to run backup process Daily for `source.dirs` folders

- Use 
- When backup process running, check if the `<prefix>_latest.zip` is same as The Latest `<prefix>_<datetime>` files, if no then start sync process for backends

- Schedule to run this back up daily, with maximum retention limit `MaxRetention` from config file. The zipped files will be So eventually we have file names as: `firefox_bookmarks_20260110.zip` and `firefox_saved_passwords_20260110.zip`


- Logging



## Features


- Create `.config/baking/baking.toml` as config file if not exist with default arguments
- Read config file and Generate unique hash string as Master password
- Storage Master password into `.config/baking/.baking.hash`
- Create `.cache/baking` folder if not exist
- Export Firefox bookmarks as `firefox_bookmarks.html` from provided profile folder in config, then zip it to file: `firefox_bookmarks.zip`
- Export Firefox saved passwords as `firefox_passwords.csv` from provided profile folder in config, then zip it to file: `firefox_passwords.zip`
- Zip files of each backup folder from provided config and zip it with corresponding names from folder name and datetime 
- The Application can also expose encrypt/decrypt feature as command line utility to generate Master password or unzip zipped files.

- Sync this ultimate file with backends, eg: `Onedrive`, `Dropbox`, `Rclone`, etc.
- Use tiered schedules (hourly, daily, weekly, monthly) for different data types. For `source.dirs`, schedule hourly backups, for `source.firefox`, schedule daily backups.
- Retention: Keep multiple generations (e.g., monthly/yearly copies for longer periods).
- Backup Type: Use full, incremental, or differential backups, with weekly fulls recommended.


---

## File Path

- Config File Path: `XDG_CONFIG_HOME/<appname>/cfg.toml` or `$HOME/.config/<appname>/cfg.toml`
- Hash File Path: `XDG_CONFIG_HOME/<appname>/.hash` or `$HOME/.config/<appname>/.hash`
- Caching Files Path: `$HOME/.cache/<appname>/`
- Logging Files Path: `$HOME/.log/<appname>/`


## Simple Version


/speckit.constitution Create principles focused on code quality, testing standards, user experience consistency, and performance requirements. Follow Test-Driven-Development approach. Minimize unsafe Code.

/speckit.specify 

Build an application that can help me schedule daily/hourly backups for important folders and Firefox bookmarks, Firefox saved passwords. 

It will generate Strong hash string as dynamic password to encrypt and then zip those files/folders above. The hashing algorithm use arguments from config files and realtime unix datetime timestamp. 

With zipped files, the app will implement retention policy by `MaxRetention` parameter from config file. Then sync to remote cloud storage providers like Onedrive, Dropbox and/or custom SFTP servers, etc.


/speckit.plan The application uses latest stable version of Rust with minimal number of libraries. Prefer mature and proven audited crates for crypto methods.


/speckit.tasks


/speckit.implement
