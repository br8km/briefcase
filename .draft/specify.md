# Specify

---

## Brief

Building a backing up application named as `briefcase` for personal sensitive data (small size 1~10 MB) security.


## Features

0. The application name is `briefcase`.

1. Read config from file location at default `$Home/.config/<appname>.toml`,  Create this file if not exist and set default configurations. 

2. Generate `PasswordKey` string using strong algorithm from user input `Password` and save it into config file.

3. Export Firefox bookmarks and saved passwords from profile into appropriate files, then generate one-time `PasswordHash` from another algorithm with two arguments: `PasswordKey` and unix datetime string; zip them with `PasswordHash`, then saved into temp folder at `$Home/.cache/<appname>/temp/`

4. Encrypt/Zip like above steps for defined personal Sensitive folder which defined from config file and save them into `$Home/.cache/<appname>/temp/` folder as well

5. Sync encrypted/zipped files to remote cloud storage: `OneDrive` and `Dropbox`

6. Sync encrypted/zipped files to remote server using SSH command

7. Expose encrypt/decrypt algorithm to Command Line Interface for generate|verify `PasswordKey`

8. Show real `PasswordHash` with CLI from password and encrypted/zipped filename

7. App will be running as system service and scheduled daily/weekly.

8. Implement retention policy by `MaxRetention` (max <= 10) argument from config file

9. Detailed Logging, filepath at `$Home/.cache/<appname>/log/`

10. Auto generate mock data for testing


## Example config file content as follows:

```toml
# Defualt config file

[general]

# required
PasswordHint = "What is your favorite color?"
# required
PasswordKey = ""
# required
MaxRetention = 10  # Max=10, Default=10


[source.firefox]
# required
enabled = false
# required
dir = "/path/to/firefox/profile/folder/"
# required, options: hourly, daily, weekly
frequency = "daily"


[source.sensitive]
# required
enabled = false
# required
dir = "/path/to/sensitive/folder/"
# required, options: hourly, daily, weekly
frequency = "daily"


[remote.dropbox]
# required
enabled = false
# required
app_key = ""
# required
app_secret = ""


[remote.onedrive]
# required
enabled = false
# required
client_id = ""
# required
client_secret = ""


[remote.icloud]
# required
enabled = false
# required
apple_id = ""
client_id = ""


[remote.sftp]
# required
enabled = false

# Need setup NoPassword Login first
username = ""
ipaddr = ""
port = 20270


```


## Edge Cases

- If Firefox is running during backup attempt, we can copy the relative file/database into temp folder and then do the rest of encrypt/zip/sync works
