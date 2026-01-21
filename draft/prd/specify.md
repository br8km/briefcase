# Product Requirements Document: `briefcase`

---

## Overview

A CLI application to back up small size (0~10MB) personal sensitive data on Linux machine.


## Features


### Config Management

1. INIT command to setup default config and create config file, prompt warnings if config file exists already
2. EDIT command to change config variables
3. CHECK command to validate config variables, report errors if any
4. AUTH command for accept user input `Password` with `PasswordHint`
5. generate `PasswordKey` from user input `Password` using strong hashing algorithm and store it into config file


### Backup Operation

1. BACKUP command to Export Firefox bookmarks and saved passwords into `TEMP` folder and Copy personal sensitive data into `TEMP` folder
2. CLEAN command can be use to delete files inside `TEMP`


### Encryption and Decryption

1. zip with encrypt for Firefox data and sensitive data, then store into `DATA` folder, new file naming format would be `Firefox_<datetime>.zip`, `Data_<datetime>.zip`
2. validate input password with stored `PasswordKey`
3. decrypt sensitive data back to original files/folders

### Sync

1. sync data to remote cloud storage providers like `Dropbox`, `iCloud`, `Onedrive`, etc.
2. sync data to remote `SFTP` servers
3. dry-run options

### Logging

1. detailed logging with levels
2. delete log files if using CLEAN command 


### Testing

1. Auto generate mock data for testing
2. use dry-run option to test remote sync operations


## Naming and FilePath Locations

- The name of application: `<appname>` = `briefcase`
- The application will use **TOML** as config file format
- The config file location: `$Home/.config/<appname>/<appname>.toml`
- `DirTemp` folder: `~/.cache/<appname>/`
- `DirData` folder: `~/.local/<appname>/dat/`
- `DirLog` folder:  `~/.local/<appname>/log/<appname>.log`


## Schedule

- The app can serve as background daemon service and schedule backup actions frequently
- `Frequency` - string, options: hourly|daily|weekly

## Retention

- Them app will implement retention policy for extra security measures
- `MaxRetention` - integer, value=(0, 10), defaults to 10


## Edge Cases

- If Firefox is running during backup attempt, we can copy the relative file/database into `TEMP` folder and then do the rest of encrypt/zip/sync works
