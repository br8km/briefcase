# XDG

We are going to follow `XDG Base Directory Specification` to refine our `briefcase` applicaton relative file/folder paths, so that this app will behave smoothly for various known operation systems: Linux, MacOS and Windows.

## config file path

- Linux: `$XDG_CONFIG_HOME/<appname>/<appname>.toml`
- MacOS: `$XDG_CONFIG_HOME/<appname>/<appname>.toml`
- Windows: `%APPDATA%\<appname>\<appname>.toml`

## Data folder for encrypto/compress files

- Linux: `$XDG_DATA_HOME/<appname>/data/`
- MacOS: `$XDG_DATA_HOME/<appname>/data/`
- Windows: `%localappdata%\<appname>\data\`

## Log folder path

- Linux: `$XDG_DATA_HOME/<appname>/logs/`
- MacOS: `$XDG_DATA_HOME/<appname>/logs/`
- Windows: `%APPDATA%\<AppName>\logs\`

## Test Data Folder

project_root/
├── Cargo.toml
├── src/
└── tests/
    └── test_data/
        ├── firefox_profile
        └── sensitive_data



Update code base and documentations with above requirements. Make sure when user run command `config init` the application will create files/folders if not exist in source code.

- Note: `<appname>`: `briefcase`
