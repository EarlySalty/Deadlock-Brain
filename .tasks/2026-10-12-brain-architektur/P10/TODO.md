# P10 meta stats: state

Branch `feat/brain-p10-meta-stats-20261013`, based on core 3a `982645cf`.

## Built

- Source `deadlock-api-meta` (`rust/crates/brain-job/src/meta_source.rs`): fetches
  `v1/analytics/hero-stats` and `v1/analytics/item-stats` (`game_mode=normal`, `bucket=no_bucket`,
  items `min_matches=20`) once per UTC hour into `<source_cache>/deadlock-api-meta/<hour>/`.
  Two requests per run, 1 MiB cap per body, JSON content type required, other hour directories pruned.
  Both URLs carry `min_unix_timestamp` = start of the capture UTC day minus 30 days
  (`meta_source::window_start`), so the window no longer depends on API defaults.
  Live capture only: `--input <dir>` fails for this source.
- `snapshot_at` is the UTC capture hour of the cited raw object (`source_revision`), not the ingest
  run start. Selection is driven by capture order, never by ingest order: view
  `serve.meta_stat_capture` walks the stat set's raw objects through the partial index
  `raw.object meta_stat_capture_order (path, revision COLLATE "C" DESC, id DESC)`, takes the newest
  1000 captures (about 41 days) with their newest succeeded applying run, then looks up the key's
  fact per capture via `fact_history`. `latest` and the 24 h / 7 d / 30 d points are picked by
  `snapshot_at DESC, run_id DESC` inside that window. A key absent from all of the newest 1000
  captures returns `meta_stats_not_found`.
- `meta_stats_pg_tests.rs` publishes runs out of capture order and then 9000 later runs carrying
  older capture hours; `latest` and the comparison points must still follow capture time. With
  that data the tool query ran in about 55 ms on a loaded host (statement timeout 500 ms).
  Known edge: bytes identical to an older capture (A, B, A) reuse the old raw object and its hour,
  so the repeated A ranks by its first capture hour.
- Extractor `meta_stats` (`rust/crates/brain-job/src/meta_stats.rs`): one entity per stat set
  (`meta_hero_stats`, `meta_item_stats`), one text fact per row (`hero_<id>` / `item_<id>`) holding the
  verbatim row JSON, receipt `json_pointer /<index>`.
- Snapshot semantics: revision = UTC hour. Same-hour rerun replays the cached bytes and is ignored.
  A later hour with unchanged upstream bytes dedups on content hash and writes 0 raw objects.
  Changed bytes write one new fact row per entity per run; history kept forever (no pruning).
- Migration v26 `scripts/migrations/2026-10-13-brain-gamefacts-p10-meta-stats-v26.sql`:
  index `raw.meta_stat_capture_order`, views `serve.meta_stat_set`, `serve.meta_stat_capture`,
  `serve.meta_stat` (SELECT for `brain_tool` only).
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

## Measurements (scratch PG 16, HEAD afb29633 binaries)

- Live proof `/tmp/p10final`: live run 1 (hour 2026-10-13T23:00Z) wrote 2 raw objects,
  69,822 bytes, 198 facts; same-hour rerun 2 wrote 0 (ignored 2); run 3 (hour 2026-10-14T00:00Z)
  wrote 2 raw objects, 69,780 bytes, 198 facts; rerun 4 wrote 0. Peak RSS 11.5 MB, 0.5 s per run.
  `get_meta_stats` hero 1 as `brain_tool`: 7.8 ms, 1 row, 1.1 KB, `snapshot_at` 2026-10-14T00:00:00Z.
- Size: per stat-set snapshot facts about 775 B/row, receipts 157 B/row, raw object 12.2 KB
  (TOAST compressed; plain body hero 27 KB, items 42 KB). Both sets changing every hour
  (197 rows) is about 208 KB per hour, so about 1.82 GB per year. Synthetic year load (hero changed
  hourly, items every 6 h, 2874 hours) measured 222 MB, about 77 KB per hour, so about 0.68 GB per year.
- Tool query as `brain_tool` with that synthetic history (2874 hero captures): 24 to 33 ms for a
  hero, 8 ms for an item, 4 rows, 4.3 KB result. Plan: index only scan on
  `meta_stat_capture_order`, `fact_history` per capture, `receipt_object_run`, no seq scan on
  large tables.
- Core finding (frozen, not changed here): ingest duration of accepted runs grows with source
  history, average 0.41 s over the first 650 runs and 2.05 s after about 2600 runs
  (`was_applied_active` in `brain-storage/src/gamefacts.rs` scans the path history). Not blocking at
  one run per hour; worth a core fix with an index or a latest-applied pointer.

## Open

- `entity` lookup needs `identity_game_id` facts from P2; until then use `game_id`.
- Merge origin/main once P1r-F lands and fix blocking core findings additively.
