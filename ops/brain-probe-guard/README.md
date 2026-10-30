# Brain probe guard

Every switch of `/opt/deadlock-brain/current` (each `brain-release install` or `rollback`) triggers the user unit `brain-probe-guard.service`. `brain-probe-guard.path` watches the directory `/opt/deadlock-brain`, because inotify on the `current` symlink itself misses the atomic rename; `brain-probe-guard.timer` reruns the guard every ten minutes as a fallback. Both are no-ops while the live SHA equals `last-sha`. The guard waits until `brain-serve` answers `readyz` from the new release, runs the 59 probes from `.tasks/2026-10-29-brain-data-coverage` exactly as committed in the live SHA, retries probes that hit a provider outage once after five minutes, and diffs the result against the last clean run.

State lives in `~/.local/state/brain-probe/deploy/`:

- `<UTC time>-<sha>/after.jsonl`, `before.jsonl`, `diff.txt`: one run with its before/after diff
- `baseline.jsonl`, `baseline-sha`: last run without regression or outage, the next run's `before`
- `last-sha`: the guard runs once per live SHA; `guard.sh --force` reruns it
- `history.jsonl`: one line per run, `status` 0 clean, 1 regression, 2 provider outage left after the retry

Exit 1 prints the rollback command to the last clean SHA. The guard never rolls back by itself. Cost is about $0.30 to $0.50 per run.

## Install

```text
install -D -m 0755 ops/brain-probe-guard/guard.sh ~/.local/share/brain-probe-guard/guard.sh
install -D -m 0644 ops/brain-probe-guard/systemd/brain-probe-guard.path ~/.config/systemd/user/brain-probe-guard.path
install -D -m 0644 ops/brain-probe-guard/systemd/brain-probe-guard.service ~/.config/systemd/user/brain-probe-guard.service
install -D -m 0644 ops/brain-probe-guard/systemd/brain-probe-guard.timer ~/.config/systemd/user/brain-probe-guard.timer
systemctl --user daemon-reload
systemctl --user enable --now brain-probe-guard.path brain-probe-guard.timer
```

Result of the last run: `journalctl --user -u brain-probe-guard -n 20`.
