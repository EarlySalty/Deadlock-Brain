# Audit of existing patch and entity structures (2026-10-29)

| structure | rows | filled by | read by | live writer |
|---|---|---|---|---|
| brain.patch_events | 36,202 | legacy `deadlock-brain` crate (pg_patchnotes, api_sync, pg_steam_news), dbrain-normalize patch.rs/legacy.rs | `brain.patch_changes` view, dl-brain MCP | none, frozen since 2026-10-11 |
| brain.patch_event_enrichments | 33,209 | dbrain-enrich build_patch_event_enrichments | `brain.patch_changes` view | none |
| brain.entities | 926 | dbrain-normalize entities.rs, resolve_gaps.rs | brain-storage entity_profile.rs, local_pg_reader.rs, brain site, dbrain-retrieval, dbrain-reasoner | none, last update 2026-10-03 |
| brain.entity_aliases | 3,832 | same | same | none |
| brain.entity_snapshots | 56,818 | same | same | none |
| brain.entity_patch_intervals_v1 | 0 | nothing ever | nothing | none |
| canon.entity / canon.fact | 512k facts | brain-job@assets, @deadlock-data, @deadlock_patchnotes_db, @item-lineage (live timers) | serve.* views, brain-serve `patch_history` (serve.p4_patch_catalog, serve.p4_patch_change) | yes |

## Quality of the legacy data
- patch_changes entity_type: hero 7,788, general 7,019, item 3,723, ability 1,243, objective 462, rest small.
- The general bucket is mostly misfiled: 1,044 lines name an item or ability, 142 a hero, about 5,100 hit a planned mechanic entity by keyword, 30 are media noise (png, mp4).
- Old and new value both present on 4,626 of 7,788 hero rows and 227 of 1,243 ability rows.
- No parent context survives into the view, so about 2,050 continuation lines ("Reduced from 300 to 250") lose their subject.
- serve.p4_patch_change (live) holds only 1,943 hero-bound rows from 35 patches.

## Can they carry the per-entity timeline?
- brain.entity_patch_intervals_v1: no. It FK-chains into entity_semantic_projections_v1 (58k rows) and entity_profile_facts_v1, keyed by legacy brain.entities, which have no live writer and miss heroes added after 2026-10-03. Using it means reviving the whole legacy entity stack.
- brain.patch_events + enrichments: usable as input text only (they hold the Shop and Map Rework notes that raw lacks), not as the target.
- canon.fact: yes, with no new table. It already has `valid_from_patch` and `valid_to_patch` (FK to the canon patch entity), unused today (0 of 512k rows). One entity record = canon.entity; the headline is the open fact; history is closed facts with patch bounds and a receipt pointing at the patch line. `patch_history` already reads canon through serve, so "value at date" becomes one interval lookup.
- One blocker: UNIQUE (entity_id, stat_key, valid_from_run) means historical rows for one stat need distinct ingest runs, or a new migration that adds valid_from_patch to the key. Decide at design time.

## Recommendation
Write the timeline into canon (entity + fact intervals + receipts per patch line), expose it through a new serve view and the existing patch_history tool. Keep brain.patch_events as read-only input. Leave entity_patch_intervals_v1 unused; drop it in a later migration once no reader is confirmed.
