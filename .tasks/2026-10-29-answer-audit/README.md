# Brain answer audit

Goal: catch wrong Brain answers automatically, without an LLM judge.

## Parts

- `brain-job --config /etc/deadlock-brain/brain-job.json answer-probe`
  runs the 59 probes from `rust/crates/brain-job/probes/answer-probes.json`
  (copy of `.tasks/2026-10-29-brain-data-coverage/probes.json`) against the
  live Brain on 127.0.0.1:8788. Grading matches `run_probes.py`. Results land in
  `ops.answer_probe_run` and `ops.answer_probe_result`. A run is `regressed`
  when any probe grade drops against the last `ok` run of the same probe set,
  `degraded` when unavailable answers exceed max(3, probes/10, 2x baseline).
- `brain-job --config ... answer-audit [--since 24h] [--trace <id>] [--record]`
  checks answered `ops.answer_trace` rows against `brain.patch_changes`,
  `ops.patch_listing` and `canon.fact`. Flag kinds: `wrong_patch_date`,
  `stale_history` (answer stops at a patch row that a later row superseded),
  `stale_value`, `unsourced_change`, `unsourced_number`. `--record` stores the
  run in `ops.answer_audit_run` / `ops.answer_audit_flag`, links the probe run
  of the last 26h and purges rows older than 90 days. Without `--trace` the
  output ends with a 7-day history and the latest probe regressions.
- `ops/brain-job@answer-audit.timer` runs both nightly at 04:10 (+10 min
  jitter). Cost is about $0.5 per probe run.

## Not done on purpose

- No probe run after each release install: the brief allows nightly only, a
  higher frequency needs approval.
- No Discord posts or DMs. The report is the CLI output and the ops tables.

## Replay of the historic F06 failure

Stale answers 1715, 1372, 1793, 1819 and 1940 are flagged `stale_history`
with expected `2025-02-25 Map Rework Update: Soul Orb denies changed from
65/35% to 50/50%`. Correct answers 1393 and 1791 stay clean.

See `REPORT.md` for the first real run.
