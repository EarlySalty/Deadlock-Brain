# P8 inventory: Brain legacy removal

Baseline release: `7a0c990b0a0ff50811827a5a7ab662594abc0c56` (installed 2026-10-24 16:34).

## Code removed

Workspace crates: brain-legacy-import, brain-jev, brain-maintenance, deadlock-brain-yt,
dbrain-enrich, dbrain-learn, dbrain-population, dbrain-wiki, dbrain-session-store,
dbrain-replay (excluded crate).

- dbrain-reasoner: the population read (`brain.population_item_stats`) moved into
  `src/population_index.rs`; the dbrain-population dependency is gone.
- deadlock-brain CLI: `wiki`, `population`, `build` (learn), `learn`, `player`, `enrich`,
  `pull deadlock-data`; wiki_refresh module; runtime_tooling test.
- deadlock-brain `site-migrate`: compare artifacts only (no site comments/profiles).
- brain-storage: `site_profiles` module; brain-migrate `check-/up-entity-profiles`.
- dbrain-sources: deadlock_data, source_pins, wiki_capture_io, wiki_runtime,
  brain-wiki-pilot, related tests.
- Python package `src/deadlock_brain`, `pyproject.toml`, `tests/test_assets_api.py`,
  `rust/python_worker`.
- Scripts: run_build_learning, run_player_match_learning, run_build_data_with_infisical,
  run_youtube_learning_with_infisical, run_gemini_ingest, run_wiki_pilot, test_wiki_runtime,
  run_sheet_sync_with_infisical, run_isolated_legacy_import,
  run_isolated_legacy_tombstone_check, export_infisical_env.py; s12 check scripts.
- Units in repo: brain-youtube-learning, deadlock-brain-build-data, ops/brain-maintenance,
  ops/deadlock-brain-wiki-refresh, wiki-refresh.md, source-pinning.md.
- Release helper `ops/brain-release` BINS: deadlock-brain, brain-serve, brain-job, brain-mcp,
  deadlock-brain-secret-exec, deadlock-brain-site, brain-browser-consent,
  brain-patchnotes-ingest.

Kept on purpose: brain-storage entity-profile library code (still wired into dbrain-retrieval
knowledge projection and tests; brain-serve never enables entity-profile access and every read
is guarded by `to_regclass`), the release helper's maintenance-releases layout (part of the
install transaction), historical docs under docs/ and architecture/, applied migrations.

## Brain PG (port 5446, DB brain), migration v47

Dropped: schema `brain_legacy` (49 tables, 540 MB), as static DDL
`DROP SCHEMA IF EXISTS brain_legacy CASCADE` (the migration guard forbids DO/EXECUTE).

Kept in `brain`, with reason:
- site_comments_v1 (0 rows), functions append_site_comment_v1 and
  read_site_profile_bindings_v1: no code reader left; the first v47 dropped them through
  DO/EXECUTE, which the migration guard rejects. v47 was reduced to the one static statement
  that runs under every migration role, including the process e2e role without rights on
  schema `brain`. Candidates for a later migration.
- core_schema_version, source_record_revisions, source_record_heads, corpus_releases_v1,
  source_jobs_v1, source_checkpoints_v1, conversation_owners_v1, response_deviations_v1:
  brain-serve core shape and response audit probes.
- compare_artifacts_v1 plus find_published_hero_compare_v1/v2, read_compare_artifact_v1/v2:
  live brain-serve hero compare and site /compare.
- source_read_header_v1, source_original_header_v1: source reader.
- entity_profile_entities_v1, entity_profile_facts_v1, entity_semantic_projections_v1,
  entity_patch_intervals_v1, entity_derived_receipts_v1: brain-storage entity profile,
  semantic and derivation code plus local_pg_reader still read them.
- maintenance_jobs_v1, maintenance_sources_v1: pg_maintenance and the local_pg_reader
  authorization registry for maintenance-docs releases.
- source_runs, source_documents: brain-storage asset_mirror and brain-knowledge-import
  register-asset-mirror.
- entities, entity_aliases, entity_snapshots, patch_events, patch_event_enrichments,
  view patch_changes: brain-storage schema and reader code still names them.

No live scans of any kept legacy-shaped table during the full E1 run (stats t0/t1 and after
E1); they are kept because retained code reads them, not because of traffic. The first gate
round blocked the wider drop for exactly this reason.

## Central DB `deadlock`, schema brain

Not changed. Live readers: application_catalog and application_emojis (patchnotes-bot),
entity_snapshots (index scans in the millions, Bots), patch_events and
patch_event_enrichments (Bots, recent scans), patch_changes (dl-brain MCP),
hero_catalog, item_catalog, current_entity_state, feeder_runs, plan_* (Deadlock-Bots),
population_* and reasoner_* (dbrain-reasoner). The rest is still referenced by the
deadlock-brain legacy store code. No verified dump of the central DB is part of this package,
so nothing there is dropped. Only candidate without any code reader: brain.mechanic_notes.

## Backup

- Service run `brain-20261028T081907Z`: SHA256SUMS OK, full `pg_restore` read OK.
- Extra dump `/var/backups/deadlock-brain/postgresql/p8-pre-drop-20261028/brain-and-legacy.dump`
  (schemas brain + brain_legacy, 420756307 bytes, 73 table data entries, read OK).

## Host units and folders

Removed user units (copies in `~/.local/state/p8-removed-units/`): brain-forum-api-probe,
brain-forum-full, brain-maintenance (.service/.timer/.d), empty brain-serve.service.d,
deadlock-brain-build-data, deadlock-brain-patchnotes-sync, deadlock-brain-sheet-sync,
deadlock-brain-wiki-refresh, deadlock-brain-youtube-learning (all .service/.timer/.d),
the stale user drop-in deadlock-brain-site.service.d and its .bak, dl-knowledge (.service/.d).
Removed folders: `~/.local/share/deadlock-brain/wiki-refresh`, `build-data-runtime-*`.

Kept: dl-brain-feeder (out of scope), deadlock-brain-luna-proxy (running), brain-browser-consent,
`/etc/deadlock-brain/maintenance-runtime.json` (brain-knowledge-import still reads it), the rest
of `~/.local/share/deadlock-brain` (source caches, wiki corpus, evidence; not a unit dependency).
