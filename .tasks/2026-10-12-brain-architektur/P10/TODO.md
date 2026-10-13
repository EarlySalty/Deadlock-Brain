# P10 meta stats: state

Branch `feat/brain-p10-meta-stats-20261013`, based on core 3a `982645cf`.

## Built

- Source `deadlock-api-meta` (`rust/crates/brain-job/src/meta_source.rs`): fetches
  `v1/analytics/hero-stats` and `v1/analytics/item-stats` (`game_mode=normal`, `bucket=no_bucket`,
  items `min_matches=20`) once per UTC hour into `<source_cache>/deadlock-api-meta/<hour>/`.
  Two requests per run, 1 MiB cap per body, JSON content type required, other hour directories pruned.
  Both URLs carry `min_unix_timestamp` = start of the capture UTC day minus 30 days
  (`meta_source::window_start`), so the window no longer depends on API defaults.
  Live capture only: `--input <dir>` fails for this source. Replay would stamp archived bytes with
  the ingest time, so `snapshot_at` (= `ops.ingest_run.started`) is the capture time by construction.
- Extractor `meta_stats` (`rust/crates/brain-job/src/meta_stats.rs`): one entity per stat set
  (`meta_hero_stats`, `meta_item_stats`), one text fact per row (`hero_<id>` / `item_<id>`) holding the
  verbatim row JSON, receipt `json_pointer /<index>`.
- Snapshot semantics: revision = UTC hour. Same-hour rerun replays the cached bytes and is ignored.
  A later hour with unchanged upstream bytes dedups on content hash and writes 0 raw objects.
  Changed bytes write one new fact row per entity per run; history kept forever (no pruning).
- Migration v26 `scripts/migrations/2026-10-13-brain-gamefacts-p10-meta-stats-v26.sql`:
  views `serve.meta_stat_set`, `serve.meta_stat` (SELECT for `brain_tool` only).
- Tool `get_meta_stats` (`kind` hero|item plus `game_id` or `entity`): latest snapshot plus the
  snapshots at least 24 h, 7 d and 30 d older; each row with `snapshot_at`, `condition`, `citation`,
  `source_revision`, `patch: null`. Reads Postgres only, never deadlock-api.
  `condition.matches_since` / `matches_until` are derived from the cited capture hour
  (capture day minus 30 days up to the capture hour).
  A hero or item missing from the newest snapshot returns its newest older record as `latest`;
  its older `snapshot_at` shows that (covered in `meta_stats_pg_tests.rs`).

## Job template

Existing `ops/brain-job@.timer` (hourly, `MemoryMax=512M` in `brain-job@.service`) covers it as
instance `brain-job@deadlock-api-meta.timer`. Not enabled, not started.

## Open

- `entity` lookup needs `identity_game_id` facts from P2; until then use `game_id`.
- Merge origin/main once P1r-F lands and fix blocking core findings additively.
