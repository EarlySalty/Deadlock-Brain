# P1r-owned verification history, 2026-10-13

## Current state after approved core 3a

The orchestrator explicitly forwarded and approved 982645cf325952b56c782fe690f6e23a5a76bf17. Its merge retains the owner's registry-tolerant assertions and failed-run fixture; pg_tools_tests.rs differs from that checkpoint only by P3's Timer test module registration. No production core or migration changed. The actual shared PG suite now passes: 7 passed, 0 failed, 0 ignored, remote EXIT=0, log /tmp/brain-p3-core3a-pg-tool-tests-20261013.log. Failed-run facts remain unpublished; both canonical tools return evidence-free errors. Formatter and strict affected-package all-target Clippy also passed. No original functional or inherited fixture blocker remains known; the final gate is still pending. Only P1r integrates/deploys.

## Earlier core 3 state

Approved owner checkpoint 19dec0c96d4765cff9a9a5dd37f5ad2a84bbe7d5 is merged and pushed in e51a2bd8. The preflight and A -> B -> A defects documented below are resolved, not current production-code blockers. The real private brain_ingest CLI accepted both pinned Timer objects, unchanged repeat ignored both with stable raw/fact/receipt counts 2/14/26, and successful process RSS was 8816 KB. Actual reversion restored 120.00, accepted=1/rejected=0/ignored=0, with raw_count=4. Logs: /tmp/brain-p3-core3-cli-index-memory-20261013.log and /tmp/brain-p3-core3-job-corrected-tests-20261013.log. No live/cgroup proof is claimed.

A different inherited Core test remains red: pg_tools_tests.rs:802-805 tries to demote succeeded fixture runs to failed; owner migration 7 correctly rejects it with SQLSTATE 23514 from ops.keep_published_run. Shared PG suite: 6 passed, 1 failed, 0 ignored, remote EXIT=101. Exact command and log are in P3/TODO.md. A fresh reviewer verified the same mutation exists unchanged in 19dec0c9. No trigger bypass or test skip was attempted.

The orchestrator assigned that test and a registry-tolerant tool count to P1r checkpoint core 3a, forbade further pg_tools_tests.rs changes, and explicitly authorized finishing the core-3 intermediate merge. Wait for the forwarded core 3a SHA, merge without rebasing, rerun the shared PG suite and final gate. Core implementations stay frozen; only P1r integrates/deploys.

## Earlier reproduced blockers

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

## Core reversion observation, now executed

A separate core-owned freshness issue is now reproduced, not merely a review hypothesis. Temporary instrumentation in the existing private PostgreSQL timer Store test fed A (120.00), B (125), then exactly the original A bytes under revision aba_reversion. It used the real ingestion registry, TimerExtractor and GameFactsStore, writer role brain_ingest, and the actual read-only brain_tool timer SQL.

Observed output in /tmp/brain-p3-core-aba-observation-20261013.log:

```
P3_ABA_REVERSION_COUNTS=(0, 0, 1)
P3_ABA_CURRENT_VALUE="125" RESTORED_SOURCE_VALUE=120.00
```

The observational run completed: 1 passed, 0 failed, 0 ignored, 2 filtered. That green status only means the observation ran; it does not establish correct source freshness. The temporary instrumentation was removed afterward, restoring timers_ingest.rs exactly to published c8c5125c. No new test asserting this stale behavior was committed.

The orchestrator confirmed A -> B -> A is also assigned to P1r as a core blocker for the next checkpoint. P3 must remain unchanged there and merge the forwarded owner fix.

Cause remains the historical was_applied check in brain-job/src/lib.rs:84-86 and the unrestricted historical EXISTS in brain-storage/src/gamefacts.rs:221-223: an earlier successful extraction of A skips renewed publication after B. The unchanged-content skip clause in CONTRACT_INGEST_V2.md and current-source serving goals need to be reconciled by P1r/the orchestrator. Distinguish raw-byte deduplication from renewed publication freshness; do not add leases, pending state or a P3 workaround. P3 core files remain unchanged.
