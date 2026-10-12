# P1 gate round 1 correction

## Goal and boundaries

Fix the two BLOCKING findings in `/tmp/p1-final-gate-121638e5-round1.log`, using the actual runtime paths. P1 must ship quickly. No additional reviews, polish, later-package work, model changes, parser/value-hash changes, or existing-nine-tool behaviour changes. The release packaging NIT is not blocking: the existing release helper inventories workspace binaries dynamically, and P0b.md documents explicit P1-owned system-unit installation. Do not edit P0-owned release code.

1. Canonical `find_entity` and `get_hero` dependency revalidation currently compares receipts but misses result metadata. Prove and fix the case where an additional matching entity makes a limit-1 lookup ambiguous without changing retained receipts. Also reject changed get_hero truncation/omitted counts when retained receipts are unchanged. Revalidation must preserve the bounded deterministic result contract, selected request parameters and existing tool behaviour.
2. Incremental ingestion processes changed paths only, although Wiki English/German localization depends on hero/ability identity JSON. When identities change or disappear and localization bytes do not, localized names/aliases must be re-evaluated, stale assertions retired and newly eligible translations extracted. Exercise the real incremental ingest orchestration, not only manual localization store calls. Preserve immutable source receipts and per-object transactions. Unchanged follow-up ingestion must still insert zero raw objects. Do not manufacture changed raw blobs for identical content.

## Ownership and starting state

Use only `/home/nathanael/.worktrees/Deadlock-Brain-p1-gate-fix-r1-20261012`, a clean owned detached checkout at `7ec83af20e4dfd7ce913b44ae55fedc84f659ee4`. Native agents share the filesystem. Parent coordinating tree and all foreign trees are read-only.

Write only the minimum necessary P1 canonical serving modules/tests, brain-job modules/tests and, only if unavoidable for an additive canonical dependency contract, the relevant authorized contract file. No changes to applied migrations, release helper, other packages or working rules. Do not globally format unrelated files. No commit, push, main merge, production action, further agent, T3 thread or extra review. Parent integrates verified patches and runs the actual gate on committed HEAD.

## Working rules

English code/work/report; German product-facing errors with umlauts. No code comments, production unwrap or needless unsafe. No secrets, credential files, environment dumps, raw production DB output or external private data. Postgres only. Graphify/code-suche before targeted reading or new code. Browser only Moli, if genuinely needed. Do not touch ai-coach, maintenance, existing retrieval behaviour, K2, other units or foreign processes. Query roles from the existing pyramid; the configured fixer first model is gpt-6.1-sol, matching the inherited native model. Never Sonnet. Do not hard-code another model/effort.

## Verification and handover

Fix first, no pre-fix full baseline. Add only concrete regression coverage for these findings. Run formatter, strict all-target Clippy, matching existing affected tests and build/check through cargo-slot with locked dependencies, jobs 3 and the configured optimized-check policy. Shared sccache, CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache. Default remote routing is restored; local override only for genuinely local PostgreSQL/public-fixture dependencies. Existing public fixtures: `/tmp/brain-job-p1-public-20261012`; set BRAIN_JOB_PUBLIC_FIXTURES for applicable tests. Do not queue a competing full workspace release or repeat unaffected full suites.

Save the exact patch including new files and logs under a newly owned `/tmp/p1-gate-r1-121638e5-*` directory, not the clean source. Return files changed, verified failure scenarios, actual passed/failed/ignored/filtered counts, formatter/Clippy/build outcomes, source comparison or hashes, patch and proof paths, and any real blocker. Do not claim main, gate ALLOW or live completion. Run useful checks immediately after implementation. No unnecessary waiting or baseline lane.

Routing: parent session `121638e5-3cc5-4f1c-870e-ccf3f7d0895b`, P1 thread `66bd78fd-84f1-4012-8682-8fe5e8e9a301`; responsible main orchestrator `89e002a7-758e-449d-9968-eec29a019a78`. Return through this native workflow only. Ask no user questions. Resolve code facts yourself; report a genuine necessary scope deviation to the parent.
