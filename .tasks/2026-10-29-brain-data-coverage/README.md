# Brain data coverage (2026-10-29)

Goal: `get_mechanic` and `find_entity` answer from deadlock-data where the data already has the answer, with verified meanings only. Trigger traces: 1357 (Walker resists) and 1359 (parry).

- `probes.json`: 59 probes. 45 data questions (`D*`) with expected values from deadlock-data, 11 redacted member questions from `ops.answer_trace` of the last 7 days (`M*`, scrims questions excluded as private community data), the Walker (`W01`) and parry (`P01`) triggers, and the three-turn 60/40 orb follow-up (`F06`).
- Grading (`run_probes.py`): `expect` is a list of groups of alternatives. pass = answered and every group hit, partial = answered and some groups hit, fail otherwise. Numbers are normalized (thousands dot or space, decimal comma) and matched on number boundaries.
- Cost (`enrich.py`): sums `brain_provider_usage` tokens for the probe request ids from the journal and prices them (Haiku 5.5, $0.10 / $0.50 per MTok input / output).
- `FINDINGS.md`: fixed defect classes and fields left out as "meaning unverified".

## Release check

A candidate brain-serve cannot run on a side port: it needs the systemd Infisical credential, and the Discord, task-consumer and operator paths would cause side effects. The check therefore runs against the live service around the install:

```
state=~/.local/state/brain-probe/<date>-<slug>
.tasks/2026-10-29-brain-data-coverage/release_check.sh before "$state"
sudo -n -- /usr/local/libexec/brain-release install <worktree> <bundle>
.tasks/2026-10-29-brain-data-coverage/release_check.sh after "$state" <previous full sha>
```

`after` compares grades per probe with `before`, prints both counts, improved and regressed ids, and exits 1 with the rollback command when any probe got worse. `PROBE_ONLY=W01,P01` limits a run to some probes. Each run writes `<mode>.jsonl`, `<mode>.enriched.jsonl` and `<mode>.cost.json` to the state dir.
