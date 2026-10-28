# P8 = D2: delete Brain legacy

User go: 2026-10-28, "Full P8 go" (backup first, dependency check, DB drops, code and unit cleanup, gate, deploy, gold set proof).
Spec: `.tasks/2026-10-12-brain-architektur/PAKETE.md` section P8 (in the main checkout `~/repos/Deadlock-Brain`, untracked there). Target picture: `ZIEL.md` in the same folder.

## Setup

- Worktree `~/.worktrees/Deadlock-Brain-p8-legacy`, branch `feat/brain-p8-legacy-delete-20261028`, base origin/main febfb2cd.
- Live release `/opt/deadlock-brain/current` -> releases/7a0c990b (rollback target). Main after it has only test commits.
- Brain PG: system unit `deadlock-brain-postgresql`, socket `/run/deadlock-brain-postgresql`, port 5446, `sg deadlock-brain-db -c "psql -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain ..."`. Schemas: brain (24 tables, ~2.2 GB), brain_legacy (49 tables, 540 MB), canon, raw, ops, patchnotes, serve, public. Other DBs: brain_pilot, brain_pilot_legacy.
- Cargo only with toolchain 1.97.1 and via cargo-slot per `/home/nathanael/Documents/claude-config/machines/v50671/RUST-BUILDS.md`. DB tests only on ionos.
- Release tooling: `ops/brain-release`. Use the existing release and deploy path that produced 7a0c990b (see git log and the architecture TODO entries for M2b, item lineage).

## Work, in this order

1. Baseline: run the E1 gold set (brain-eval) against live 7a0c990b and store the result. Provider is the dl-ai gateway (claude_plan haiku); token use is approved without limit.
2. Dependency inventory (write `INVENTAR.md` here): for every table in schemas `brain` and `brain_legacy`, every crate, every binary and every brain unit (system and `systemctl --user`), list who still reads or runs it: brain-serve, brain-job, deadlock-brain-site, brain-mcp, serve.* views and functions, timers, Deadlock-Bots, Twitch bot, Deadlock-2nd-Brain. Known still used: `brain.compare_artifacts_v1` via migration 30 serve wrappers (P7 decision A), probably `conversation_owners_v1` and `response_deviations_v1`. `brain-browser-consent.service` is active: keep it unless proven unused. Anything still read stays or moves into `serve`/`ops` with a new migration; do not break live readers.
3. Code removal per P8 list: `dbrain-replay`, `dbrain-wiki`, `brain-legacy-import`, `dbrain-learn`, `dbrain-enrich`, `dbrain-population`, `brain-jev`, `deadlock-brain-yt`, `dbrain-session-store`, release parts of `dbrain-retrieval`, `deadlock_data.rs`, Python `src/deadlock_brain`, `scripts/export_infisical_env.py`, plus `brain-maintenance` and other crates/binaries the inventory proves dead. `deadlock-brain` (site, mcp, secret-exec) currently depends on maintenance, learn, enrich, population: cut those paths. Workspace release build must ship at least brain-serve, brain-job, brain-migrate, deadlock-brain-secret-exec, deadlock-brain-site, brain-mcp (plus any binary a live unit still runs).
4. Backup before any drop: run `deadlock-brain-postgresql-backup.service`, verify the dump, and take an extra `pg_dump` of schemas `brain` and `brain_legacy` with path and size recorded. No drop without a verified dump.
5. DB drops only through a new brain-migrate migration (never edit applied ones): drop `brain_legacy`, `entity_profile_*`, `entity_snapshots`, `source_record_*`, `corpus_releases_v1` and other proven-dead legacy tables. Use `SET lock_timeout`. DBs brain_pilot / brain_pilot_legacy: drop if proven unused and possible with available roles, else report.
6. Central DB `deadlock.brain.*` (bots DB): retire only tables no bot reads; if any bot still reads, leave them and list readers in the report.
7. Units and folders: remove stopped/disabled legacy units (brain-maintenance, dl-knowledge, deadlock-brain-build-data, -patchnotes-sync, -sheet-sync, -wiki-refresh, -youtube-learning and their failure-notify instances, drop-ins under `~/.config/systemd/user/*.d/99-user-disabled-20261012.conf`) after checking each is legacy. `dl-brain-feeder` belongs to Deadlock-2nd-Brain: leave it. `deadlock-brain-luna-proxy`: leave it running, report whether anything still uses it. Old release folders: keep current and one rollback.

## Acceptance

- Release build ships the binaries above; live units run from the new release.
- E1 gold set not worse than the step 1 baseline (answered and correct counts).
- `\dn` in DB `brain` shows only raw, canon, docs (if present), serve, ops, patchnotes (if still needed), public.
- Live proof: brain-serve answers a hero, item and patch question; site hero and item pages 200; all brain-job timers run green once.

## Rules

- Gate: `gate-review --repo <worktree> --base origin/main` with skill `rolle-merge-schleuse`; on BLOCK fix the cause and rerun, after five failed rounds report a concrete blocker. You are integrator: after ALLOW merge to main, push, deploy origin/main via the release path, restart, live proof, remove worktree and branch.
- No code comments. No `.unwrap()` in production paths. English commit messages. No force push, no bypassed hooks. Stage only your files.
- Secrets only via Infisical export script, never printed. No `cat`/`grep` on secret files.
- Report back only: merged SHA, release, dropped objects, kept objects with reason, E1 before/after, backup path, or a concrete blocker. Note state in `TODO.md` here if you stop early.
