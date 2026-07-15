# Scheduling

Start the scheduler with:

```bash
briefcase schedule start
briefcase schedule start --detach
briefcase schedule start --force
briefcase schedule status
briefcase schedule stop
```

The daemon checks once per hour. Each enabled source is evaluated independently against its own `last_backup` and frequency. A source with no previous timestamp is due immediately.

When a scheduled backup succeeds, the source timestamp is persisted. If enabled remotes exist, the daemon also performs an automatic sync and persists successful remote timestamps.

Detached operation uses Unix process forking. The non-Unix stop path is currently not implemented.
