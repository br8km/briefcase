# Platform Support

Configuration, data, and log paths contain platform-specific branches, and uninstall has Windows and non-Windows deletion paths.

The detached scheduler uses Unix `fork`, while daemon stopping is explicitly unavailable on non-Unix platforms. Therefore Linux/Unix is the verified runtime target; macOS and Windows release claims require separate build and behavior validation before being documented as supported.
