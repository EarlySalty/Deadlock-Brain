# P1 gate round 2 correction

## Goal

Fix the two BLOCKING findings in `/tmp/p1-final-gate-121638e5-round2.log`. P1 live acceptance is the critical path. One fresh bounded coder, fix first, no baseline, extra reviews or polish. Parent owns integration and the next committed-HEAD gate.

1. A permanently malformed or oversized pinned source target currently wedges `ingest`: `pending` runs are processed before upstream is fetched. Recover when upstream has corrected the source, for both Wiki and GameTracking, preserving immutable target/extractor identity, successful raw objects/receipts and correct publication. Use the existing failed lifecycle where possible, not new cancellation/superseding statuses. Same-target transient interruption must remain resumable. Never relabel an old extractor run, fabricate success, delete audit data or bypass guards. The approved future extractor-version cutover precondition still requires finishing/publishing old unfinished runs before a bump.
2. JSON Upgrades accepts u32::MAX and performs unchecked tier+1. Match the valid three-tier GameTracking domain, reject or omit unsupported tiers without panic/wrap and without a false receipt condition. Valid parser values and their value hashes must remain unchanged. Add concrete boundary/proof coverage, including u32::MAX.

## Recovery correctness

`raw.run` already permits running, partial, failed, succeeded; `run_incomplete_source_idx` currently covers every non-succeeded row. `Store.finish` currently uses partial for retryable interruption. If existing failed is made terminal, preserve partial resume and enforce proper immutable/lease-bound transitions. Do not just free the index and advance the head: partially committed assertions and validity endpoints from the bad target can leak into a later snapshot. A corrected revision that reverts a partially processed file, deletes a newly introduced path, or keeps localization bytes unchanged must publish only corrected source evidence. A full corrected snapshot plus necessary tombstones is one possible bounded approach. Verify actual serving rows, not only run counts. No data loss or new cancellation status.

The v1 migration remains unapplied: a fresh read-only audit of all seven connectable non-template databases in the primary port-5446 Brain cluster found no gamefacts schema hits. Minimal changes to this unapplied v1 and its invariant/tests are authorized if required for the existing failed lifecycle. Preserve all historical applied migrations, registry order, version ownership, role boundaries, serialize guards and receipt immutability. No placeholder migration, manual production migration or later-package implementation.

## Ownership

Only `/home/nathanael/.worktrees/Deadlock-Brain-p1-gate-fix-r2-20261012`, clean detached at `6bcafec8f64e34168cd9e5c68edd1deca10ef1b0`. Minimum brain-job modules/tests and, only as needed for the lifecycle contract, gamefacts-v1 SQL plus authorized storage invariant/tests. Parent and foreign trees are read-only. No commit, push, main merge, deploy, further agent, new T3 thread or extra review. Do not modify existing nine tools, retrieval, maintenance, K2, P0 release helper, other units or working rules. Avoid global formatting.

English code, thinking, notes and final report. German product-facing errors only. No code comments, production unwrap or needless unsafe. No secrets/credential-file reads, environment dumps, raw production DB output or external private data. Postgres only. Graphify/code-suche before code search/build. Browser only Moli if required. Never Sonnet. The inherited gpt-6.1-sol is the pyramid's first fixer model; no model/effort override.

## Verification

Concrete tests must exercise real Git/Postgres ingest orchestration: corrected newer target after permanently malformed/oversized pending target for both sources; same-target interruption/resume; partial assertions reverted/deleted in the corrected target; stable raw/receipt/identity preservation; unchanged follow-up raw delta zero. Use owned scratch databases, never production mutations. Parser boundary tests must verify supported 1..3 conditions and proof/hash behaviour, not merely no panic.

Run matching formatter, strict all-target Clippy, optimized check/build and existing affected tests with locked dependencies, jobs 3, shared sccache and CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache. Default cargo-slot remote routing; local override only for genuinely local PostgreSQL/public-fixture probes. Existing fixtures `/tmp/brain-job-p1-public-20261012`, BRAIN_JOB_PUBLIC_FIXTURES where applicable. No competing full workspace release or unaffected repeated suites. New tests guard these concrete bugs only.

Save complete patch including new files and exact logs under a fresh `/tmp/p1-gate-r2-121638e5-*` directory. Return actual passed/failed/ignored/filtered counts, all outcomes, changed files and lifecycle semantics, valid parser/value-hash compatibility, patch and source hashes. No gate ALLOW/main/live claim. Genuine necessary scope deviation goes to parent, not user. No relabel, cancel/supersede state or weakened protection shortcut.

Routing: parent `121638e5-3cc5-4f1c-870e-ccf3f7d0895b`, P1 T3 `66bd78fd-84f1-4012-8682-8fe5e8e9a301`; main orchestrator `89e002a7-758e-449d-9968-eec29a019a78`. Return through this native workflow only. No user questions.
