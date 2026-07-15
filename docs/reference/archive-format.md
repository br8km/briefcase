# Archive Format

Each backup is a file named `Firefox_<local-time>.7z` or `Folder_<local-time>.7z`.

The pipeline is:

1. Copy or export source data into a temporary directory.
2. Compress the directory into an unencrypted 7z archive.
3. Encrypt the archive bytes with AES-256-GCM.
4. Remove the unencrypted temporary archive.

The encrypted file contains a random nonce and authenticated ciphertext. The current implementation also prefixes a random 32-byte salt, but decryption derives the key from the configured key and does not use that stored salt.
