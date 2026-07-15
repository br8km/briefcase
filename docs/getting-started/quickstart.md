# Quickstart

## Initialize configuration

```bash
briefcase config init --password "your-strong-password" --password-hint "hint"
```

This creates the default configuration at the platform configuration directory under `briefcase/briefcase.toml`, generates an Argon2 password hash, derives the stored encryption key, and initializes data and log directories.

## Configure sources

Edit the generated file with:

```bash
briefcase config edit
```

Enable a source and set its existing directory path. Validate before running a real backup:

```bash
briefcase config validate
```

## Back up and sync

```bash
briefcase backup
briefcase sync
```

Use `--dry-run` to preview either command. A successful backup persists `last_backup` for each source that completed. A successful non-dry-run sync persists `last_sync` for each remote that completed.

## Recover data

```bash
briefcase crypto decrypt --input Firefox_2026-01-19_12-00-00.7z --output ./restored
```

If the configuration is unavailable, the command prompts for the original password and derives the same recovery key.
