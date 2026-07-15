# Logging Reference

Logging is implemented with `env_logger` and the `log` facade.

- Default level: `Info`.
- Output: stderr and a local monthly file.
- Filename: `YYYY-MM.log`.
- Format: plain text with local timestamp, level, target, and message.
- Linux log location: `~/.local/share/briefcase/logs/`.

JSON encoding, configurable levels, size-based rotation, and a maximum file count are not currently implemented.

Logging should capture important events, failures, and state changes without exposing passwords, encryption keys, tokens, remote credentials, or sensitive file contents.
