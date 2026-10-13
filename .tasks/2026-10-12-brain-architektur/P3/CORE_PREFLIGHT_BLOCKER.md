# P1r-owned CLI preflight blocker, 2026-10-13

P3 branch: feat/brain-p3-timers-20261012-v2. Published core merge: e8e3e577, includes e8a1ef45788faeb988464bf44bf8f0932f12c09c.

The actual `brain-job --config <scratch-config> ingest gametracking --input <pinned-public-repository>` exits 1 before ingestion. The compiled test log is `/tmp/brain-p3-core2-job-complete-tests-20261013.log`: nine core tests passed, two timer tests passed, the public CLI test failed.

Confirmed cause: `brain-job/src/main.rs:58` calls `PgStore::check_gamefacts_schema` as brain_ingest. `brain-storage/src/gamefacts/migrations.rs:66` performs a structural LIMIT 0 query against serve.published_receipt and all four serve.current_* views. Core migration v1 lines 137-150 give brain_ingest raw/canon/ops access, but neither serve schema USAGE nor serve view SELECT. Migration validation succeeded only as DBA/brain_migrate in previous ScratchPG tests.

Independent private PostgreSQL-16 reproduction used the actual core v1 SQL, its current rendered bounds and hero_profile.sql, and the exact structural query extracted from migrations.rs. No production database or secret was accessed. Output at `/tmp/brain-p3-schema-preflight-20261013.log`:

```
INGEST_SERVE_USAGE_AND_FACT_SELECT=f|f
ACTUAL_SCHEMA_PREFLIGHT_QUERY=permission denied for schema serve
```

The orchestrator confirmed this blocker is assigned to P1r and will forward the new core checkpoint. P3 must merge that checkpoint when forwarded and rerun the actual CLI proof.

P1r must fix the role-aware structural preflight or supply an explicitly reviewed migration granting the intended minimum access. Do not edit applied migration v1. P3 has not changed the frozen core validation or widened ingest privileges. The real CLI acceptance and successful CLI memory measurement remain blocked, not passed.

Separate core-owned review concern, not yet executed as a reproduction: historical `was_applied` lookup in brain-job/src/lib.rs skips A after A -> B -> A, leaving serving on B. Raw deduplication and renewed publication freshness need an owner decision. This is not established as an executed regression result.
