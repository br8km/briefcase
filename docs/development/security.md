# Security Notes

- Backup payloads are encrypted with AES-256-GCM.
- Password verification uses Argon2 password hashes.
- Passwords and encryption keys must not be logged.
- The derived encryption key is currently stored in the local TOML configuration.
- 7z compression occurs before encryption and is not itself password-protected.
- GCM authentication detects ciphertext tampering, but `BackupFile.hash` is currently not populated.

These are implementation facts, not a claim that the current design has completed security review.

Security-sensitive changes must validate inputs, preserve authenticated encryption, avoid exposing secrets in logs or errors, and document any exception to memory-safe Rust. Data-integrity behavior should be tested at the boundary where files are created, encrypted, restored, or removed.
