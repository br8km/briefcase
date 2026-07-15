# Maintenance

## Clean data and logs

```bash
briefcase clean
briefcase clean --force
```

`clean` removes every entry in the Briefcase data and log directories. It is destructive and does not only remove temporary files.

## Uninstall

```bash
briefcase uninstall
briefcase uninstall --all
```

Uninstall removes data, logs, and the current binary. Without `--all`, it asks whether the configuration should also be removed.
