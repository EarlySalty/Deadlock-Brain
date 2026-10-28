# Docs refresh after P8 (brain)

Branch: docs/brain-20261028, base origin/main c7e12e1e.

## Checked

- Live units: `systemctl` system and `--user` for brain, deadlock-brain and public-corpus units; timer schedules (patchnotes 5 min, api-meta and assets hourly, audit-receipts 03:17, public-corpus-refresh 15 min).
- Binaries in `/opt/deadlock-brain/current/bin` and their `--help` output; crate mapping from the source tree.
- Release helper subcommands in `ops/brain-release/src/main.rs`; release layout (`current` -> c7e12e1e, rollback baseline 7a0c990b).
- brain-job sources (`sources.rs`) and the Deadlock-Docs source (`docs_source.rs`: `public/` and `internal/` HTML).
- Schemas from migration and DDL sources: raw, canon, serve, ops; `brain_legacy` gone (v47).
- Provider kind and slot from brain-serve config (non-secret keys only).

## Changed

- docs/ARCHITECTURE.md: rewritten as the single current overview.
- README.md: rewritten as a short entry with quickstart and operations pointer.
- docs/c9-consumer-contract.md, game-wiki/AGENTS.md, docs/BUILD_REASONER.md: removed references to deleted maintenance and build-data units.
- Deleted 22 docs whose whole subject was removed code (see git history of this branch).
- Historical banner added to architecture/** and infra/** markdown and three other historical docs.
- ops/brain-postgres/grants.sql: removed the brain_legacy grant block.
- ops/brain-postgres/backup.sh: removed brain_legacy from schema lists.

## Open items

- `dl-brain-feeder.timer` (user unit) is enabled, its service is inactive and it points to a Python script in the old Deadlock-Bots tree. Owner decides removal. Not documented as current.
- `brain-job@deadlock-docs` has no timer; the Deadlock-Docs import is manual. Schedule decision for the owner.
- Relation between `public-corpus-refresh.timer` and the Brain import not confirmed in code.
- `infra/cutover/LEGACY_MATRIX.csv` and `infra/cutover/RUNTIME_CHECK.md` still list removed units as 2026-09-24 state. The markdown has the banner; the CSV cannot take a banner without breaking the format. Left as a dated record.
- `docs/sqlx-offline-cache.md` uses the stale `/home/naniadm` paths and the Deadlock-Bots central migrations. Not about removed code, so out of this refresh.
- `rust/`, `scripts/`, `config/wiki-refresh.json`, `.env.example` and `infra/cutover` Rust code still mention removed names; code is out of scope for a docs refresh.
- The CLI still exposes `lineage`, `legacy`, `timeline`, `review` and `pull statlocker`; a reader may need those documented or removed.
- graphify graph file is missing in this worktree; targeted grep was used instead.
