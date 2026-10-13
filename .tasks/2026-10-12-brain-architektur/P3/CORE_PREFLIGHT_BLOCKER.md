# P1r-owned verification history, 2026-10-13

## Current approved main-merge candidate, verified 2026-10-14

The orchestrator's direct 23:15 go-ahead authorized merging origin/main 0b105f92abaa2697b9c93e20483c60e8d7c8a3b1 into P3. The automatic no-rebase merge retained Hero+Timer and sparse migrations 1/2/7/8/9/10. The resumed native fixer confirmed both prior core BLOCKING mechanisms and the template-installation NIT are resolved. A suspected application_xid override was disproven by the actual named-column INSERT grants and explicit preflight/SQLSTATE 42501 test. No additional P3 core correction or migration 22 is currently necessary; applied migrations remain unchanged. This is code/SQL verification, not a new gate verdict.

Actual main-based full Job tests passed 19/0/0 with original public blobs, formatter passed, and the real brain_ingest CLI accepted both Timer objects and ignored unchanged repeat with stable counts 2/14/26. Successful process RSS 8828/6548 KB; natural timer_key_idx scan under the labelled 1500-alias public catalog fixture, Execution Time 3.636 ms, Planning Time 3.843 ms. The private probe was independently verified stopped (pg_ctl status 3). See P3/TODO.md for exact logs and the corrected Cargo fixture setup. Current affected checks completed: 68 passed, 0 failed, 0 ignored (19 Job, 32 Storage, 10 Migration, 7 PG tools), strict four-package all-target Clippy and final formatter passed. The inherited sparse-registry assertion was corrected solely to include P3's existing version 2; the corrected migration suite passed. Final gate/ALLOW and live-unit/cgroup acceptance remain open. No main merge, deployment or cleanup by P3.

## Subsequent integration state

Both main updates eea3c4ec (credential startup) and f948a44b (exact citation copying) were automatically merged without rebasing or conflicts. The first is published in 87ed7cc7 and its matching helper checks passed 7/0/0 plus formatter/check/strict Clippy. The fresh whole-P3 reviewer found no verified defects at 87ed7cc7. Its later citation-delta review confirmed one prompt-coverage omission: successful GetTimer batches and the corresponding regression-matrix case do not receive the new exact-copy reminder. This is not an observed Timer model failure. The isolated native coder published the exact predicate/matrix correction in f682b579ee1b085db69ac1d26d1daed37cc75e6e on fix/brain-p3-timer-citations-20261014. Its scoped Contracts tests passed 10/0/0 and matching formatter/check/strict Clippy passed. Independent fix verification completed with no remaining concrete defect, and the correction is now automatically merged into P3 without conflicts. The actual final gate remains pending. No DB migration 22 is necessary for that narrow correction. Parent's inherited citation formatter/check/strict Clippy passed, provider library27/fault1 passed; matching completed evidence totals113/0/0, not a live model proof. See P3/TODO.md for actual session/task provenance. No P3 merge to main, deployment, service operation or cleanup.

## Earlier user route decision, 2026-10-13 21:55 Europe/Berlin

P1r-F replaces the stopped P1r thread and goes directly to gate, main integration, release and live proof. Core 3b will not be published and P1r-F answers no core questions. P3 waits for the orchestrator's explicit main-merge message, then merges freshly fetched origin/main into its published branch without rebasing or force-pushing. Any remaining core blocker is then P3's additive correction in this branch. The follow-up user correction reserves migration version 22 for such a P3 core fix; do not choose the next free version. Versions 11 through 20 belong to P1r-F, and P3's package version remains 2. Applied migrations remain immutable. Independent Timer work and proof publication are complete; no production change or new migration has been made during the hold. Final ALLOW and live acceptance are still pending.

## Earlier state after approved core 3a

The orchestrator explicitly forwarded and approved 982645cf325952b56c782fe690f6e23a5a76bf17. Its merge retains the owner's registry-tolerant assertions and failed-run fixture; pg_tools_tests.rs differs from that checkpoint only by P3's Timer test module registration. No production core or migration changed. The actual shared PG suite now passes: 7 passed, 0 failed, 0 ignored, remote EXIT=0, log /tmp/brain-p3-core3a-pg-tool-tests-20261013.log. Failed-run facts remain unpublished; both canonical tools return evidence-free errors. Formatter and strict affected-package all-target Clippy also passed. The original preflight, cross-run reversion, Hero registry and inherited fixture blockers are resolved. Actual gate round 2 on published 87f681633da7c4a2442d192a0345eda7dc79ce26 nevertheless returned BLOCK, EXIT=1: published-data INSERT/run-receipt consistency in core v1/v7, plus cumulative receipt-cap rejection for a same-run A -> B -> A replay at the cap. Log: /tmp/brain-p3-core3a-local-gate-20261013.log. A fresh native read-only fixer confirmed both code/SQL failure mechanisms, all receipt/direct-INSERT twins, and P1r ownership. units::install exists, but its registry omits both job templates; fixing installation must not automatically start instances. These are code-verified findings, not new runtime reproductions. No P3 extractor/source/tool/view change is necessary. Frozen core remains unchanged; the orchestrator has the actual gate result and receives the confirmed ownership report. No ALLOW exists. Only P1r integrates/deploys.

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
