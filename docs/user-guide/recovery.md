# Recovery

Decrypt and extract an archive with:

```bash
briefcase crypto decrypt --input backup.7z --output ./restored
```

With a valid configuration, Briefcase uses the stored base64-encoded derived encryption key. Without a usable configuration, it prompts for the original password and derives the key using the built-in recovery salt.

The outer file is AES-256-GCM encrypted. The inner 7z archive is not password-protected. Authentication failure or an incorrect password causes decryption to fail.

`briefcase crypto validate` only checks that the configured password hash is present; it does not prompt for a password.
