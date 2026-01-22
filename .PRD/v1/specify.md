# Product Requirements Document: `briefcase`

---

## Overview

A CLI application to back up small size (0~10MB) personal sensitive data on Linux machine.


## Features


### Config

1. init default config and create config file, prompt warnings if config file exists already
2. edit and validate config variables, report errors eg: source dir path not existence
4. accept user input `Password` with `PasswordHint`
5. generate `PasswordKey` from user input `Password` using strong hashing algorithm and store it into config file


### Backup

1. export Firefox bookmarks and saved passwords into `DirTemp` folder 
2. copy sensitive folder content into `DirTemp` folder
3. use `PasswordKey` and current datetime string (format: `%Y-%m-%d %H:%M:%S`) to generate one-time `PasswordHash` string, then compress Firefox data and sensitive folder with this `PasswordHash`, store them into `DirData` folder, naming them as `Firefox_<datetime>.zip`, `Folder_<datetime>.zip`  
4. remove oldest zip files base on max retention config settings


### Sync

1. sync zipped data to remote cloud storage providers like `Dropbox`, `iCloud`, `Onedrive`, etc.
2. sync zipped data to remote `SFTP` servers
3. dry-run options


### Logging

1. detailed logging with levels, store logs in `DirLog` folder
2. logging file format: `JSON`, filename: `<%Y-%m>.log`
3. rotate logs based on time(monthly) and size (10MB), keep maximum 3 logging files


### Clean

1. delete files inside `TEMP` after sync operation success
2. delete log files


### Crypto

1. validate user input password with stored `PasswordKey` from config file
2. decrypt zipped data back to original files/folders


### Schedule

- The app can serve as background daemon service and schedule backup actions frequently
- `Frequency` - string, options: `hourly`, `daily`, `weekly`
- can setup different `Frequency` for each source

### Retention

- Them app will implement retention policy for extra security measures
- `MaxRetention` - integar, value=(0, 10), defaults to 10


## Monitoring

- backup success rates


## Edge Cases

- If Firefox is running during backup attempt, we can copy the relative file/database into `TEMP` folder and then do the rest of encrypt/zip/sync works


## Testing

1. Firefox profile folder for testing is `ProjectRoot/test_data/firefox.profile` folder
2. Sensitive folder for testing is `ProjectRoot/test_data/sensitive`
3. use dry-run option when run unit testing for remote sync operations
4. prompt warning message if no valid remote sync config when run integration testing


## Naming and FilePath Locations

- The name of application: `<appname>` = `briefcase`
- The config file location: `$Home/.config/<appname>/<appname>.toml`
- `DirTemp` folder: `~/.cache/<appname>/`
- `DirData` folder: `~/.local/<appname>/dat/`
- `DirLog` folder:  `~/.local/<appname>/log/<appname>.log`
- Log file name format: `<%Y-%m>.log`


--- 

<!-- append -->

## Error Handling

- config `PasswordHint` and `PasswordKey` cannot be empty
- config `[source.firefox]` `dir` must be valid path

## Issues

<!-- - [?] system daemon service: NOT available for sync operations -->
- [?] firefox bookmarks export: just copy sqlite file is enough? MUST export to `firefox_bookmarks_<datetime>.html` files.

