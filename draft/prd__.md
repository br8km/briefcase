# Product Requirement Document

---

I am building an **CLI** application on **Linux** for backup personal sensitive data (which include Firefox bookmarks and saved passwords, defined folder of important documents) to some remote cloud storage providers and/or SFTP servers.


## Coding Principles

- Test-Driven-Development
- Minimize unsafe Code
- Robust Error Handling
- Data Integrity and Safety
- Modularity and Organization
- Detailed Logging


## Application Features

- **Config**: The app can init with default config and create file if not exist. It can also edit and validate config.

- **Auth**: The app use **Rclone** 


## Naming and Locations

- The name of application: `<appname>` = `briefcase`
- The application will use **TOML** as config file format
- The config file location: `$Home/.config/<appname>/<appname>.toml`


## Requirements

- Performance, security, accessibility, and scalability considerations.


## Assumptions & Constraints

- Any technical or regulatory limitations and open questions.