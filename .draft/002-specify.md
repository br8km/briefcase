# Specify


Build a backing up application which keep personal important files safe.


## Requirements


- Use TOML config file, create and fill with default values if not exist

- Use Cache folder to store temp encrypt-zipped files, create if not exist

- Generate Strong `BakingHash` string as password to zip files if arguments present

- The hashing algorithm arguments include: `BakingHint` from config file, `datetime` string from unix timestamp, `BakingPwd` from environment variable 

- Store `BakingHash` as hidden file to same folder with config file

- Export Firefox bookmarks|saved_passwords from provided profile folder, and Use generated `BakingHash` as password to zip them. The zipped file will be stored in Cache folder

- For each folder from `source.dirs` of config file, zip them using `BakingHash` as password and store them as well. The filename format : `!!!!!!`

- Use Daily backing up policy for Firefox datas

- Use Hourly backing up policy for `source.dirs` folders

- Implement retention policy by `MaxRetention` from config file

- Organize zip files with index as suffix of filename, eg: `filename.zip.20260110.130510` to `filename.zip.10` for files zipped before this backing up process, and `filename.zip.latest` for current backing up process

- If latest zipped files are same as the most recent files(same prefix part in filename), ignore; if not then re-edit file names and start sync process to available backends


- Customize Detailed Logging

- Documentation: Create a clear, concise, and periodically updated document for easy understanding

- The application can also expose encrypt/decrypt feature as command line utility to generate `BakingHash` for unzip zipped files.


Example `.config/<appname>/cfg.toml`:

```toml

[general]

BakingHint = "What is your favorite color?"
MaxRetention = 10
# SOCKS_PROXY=

[source]

dirs=[
 "/path/to/important/dir1/",
 "/path/to/important/dir2/",
]

firefox=[
 "/path/to/firefox/profile1/",
 "/path/to/firefox/profile2/",
]

[backend.rclone]

enable = false
api_key = ""

[backend.onedrive]
enable = false
api_key = ""

[backend.dropbox]
enable = false
api_key = ""

[backend.ssh]
# NoPassword Login + scp command to transfer
enable = false
api_key = ""


```


## References

- Dropbox
	- `https://docs.rs/dropbox-sdk/latest/dropbox_sdk/`
	- `https://github.com/dropbox/dropbox-sdk-rust`

- Onedrive
	- `https://docs.rs/onedrive-api/latest/onedrive_api/`
	- `https://github.com/oxalica/onedrive-api`
	- `https://github.com/sreeise/graph-rs-sdk`

- RClone
	- `https://docs.rs/rclone-api/latest/rclone_api/`
	- `https://github.com/rclone-ui/rclone-sdk`


## File Path

- Config File Path: `$HOME/.config/<appname>/cfg.toml`
- Hash File Path: `XDG_CONFIG_HOME/<appname>/.hash` or `$HOME/.config/<appname>/.hash`
- Caching Files Path: `$HOME/.cache/<appname>/`
- Logging Files Path: `$HOME/.log/<appname>/`