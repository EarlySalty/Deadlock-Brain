# Wave-2 integration handover

## P4 integrated, 2026-10-21

Exact P4 `35a54ce3465c02ca57bfa2b542481cee6ea2d7ab` is merged on top of `d6077a8e`. Six conflicts were resolved as append-only unions in contracts/tools, job extractor/source registries, serve modules and migration registry/tests. The path-included P4 port test includes the existing P2 asset module; its upgrade test expects the integrated migration union. Approved migration SQL blobs remain unchanged.

Additive migration 31 (29 is reserved for P6, 30 for P7) replaces the redundant reader-coherence joins of version 23 to `serve.published_receipt` with metadata joins to `canon.receipt`; selected identities and facts still originate in `serve.current_entity`/`serve.current_fact`, which enforce the P9/v28 publication and quarantine checks. Reader fingerprints are checked for version 23 or 31.

The brain-serve tool pool disables PostgreSQL JIT in its session setup and asserts `jit=off` on every acquire, so reused production connections keep it. The 500 ms statement limit is unchanged. A previous join/from collapse limit of 4 was removed: it made the real public item executor and the captured slot executors time out; without it and with JIT off all four asset executor tests pass locally (evidence `/home/nathanael/.local/state/brain-wave2-proof-20261014/integration-resume-20261021/`). Remote ionos test runs lack the local `/tmp` fixtures; fixture-dependent groups run with `CARGO_SLOT_REMOTE=0`.

The patch source RSS contract (`VmHWM < 300000000`) failed once at 303706112 bytes. Cause: sqlx never shrinks a connection's write buffer, so every pooled connection that carried a 48 MiB `raw.object` insert kept that buffer; how many of the three pool connections were hit varied with timing (234 to 303 MB). `gamefacts::capture` now runs insert and lookup on one acquired connection and calls `shrink_buffers()` after the insert. Three local runs measured 198971392 to 199049216 bytes; the threshold is unchanged.

Next: P10 `5e10cbf5` registers version 26, then origin/main. P5 remains deliberately excluded.

## P10 integrated, 2026-10-21

Migration 26 (meta stats views over `serve.published_receipt`) sits between 23 and 28 in the registry; the union is now 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 13, 23, 26, 28, 31. `get_meta_stats` joins the canonical tool set beside timers and asset tools. Verification: fmt, check, strict clippy and 158 tests in 16 groups green locally (`p10-*.log` in the evidence directory).

## origin/main c4bf51b5 integrated, 2026-10-21

Clean merge (scratch PostgreSQL supervisor, entity profile cut, brain-release provenance). The scratch supervisor is an `#[ignore]`d harness entry `supervise_cluster` that only works when started by `ScratchPg` with its environment; under the mandatory `--include-ignored` any filter matching it runs it directly and it panics. Test commands therefore add `--skip supervise_cluster`; no real test is skipped. Verification: fmt, check, strict clippy for contracts, job, storage, serve, maintenance and ops/brain-release; 218 tests in 19 groups green (`main-*.log`).

## P5 wiring 0dc6f130 integrated, 2026-10-21

Migration 24 and docs migration 5 join the registry; the union is now 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 23, 24, 26, 28, 31. The service composes `PgPatchToolPort` inside `DocsTools`, and the find queries keep the published-receipt identity joins together with the doc-kind exclusion. The pool keeps `jit=off` with no collapse limit, and docs transactions set their own limits. The GIN pushdown positive control lost to P4's `canon.fact_patch_reference` btree on the tiny fixture. Its canonical EXPLAIN now runs in a separate transaction that drops that index and then rolls back, and the proof records `canonical_without_index`. Production indexes, the 500 ms limit and the P9 guards are unchanged. The ScratchPg child `hold_cluster` blocks on stdin, so test commands also skip it and close stdin. Verification: fmt, check and strict clippy pass. 284 tests in 23 groups are green: pg_docs 30/30 with 400 docs at 9.8 ms execution and 30 ms planning, and both P2 slot executor tests pass under the 500 ms limit (`p5-*.log`).

## P7 Steckbrief cut eab6bb69 integrated, 2026-10-22

Migration 30 sits between 28 and 31; the union is now 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 23, 24, 26, 28, 30, 31. In registry order v30 runs after P5's v24, so `serve.site_doc_section` comes from the migration and `site-role.sql` grants it. The site test no longer stubs `serve.doc_meta`, `doc_tsv` and `doc_section` and no longer reapplies v30; both collided with the real P5 objects. It publishes one public and one internal doc page through canonical receipts instead. It checks that the internal page exists in `serve.doc_section` and still answers 404 on the site. Verification: fmt, check and strict clippy (including brain-maintenance and deadlock-brain) pass, site 4/4 (`p7-*.log`). Environment-dependent failures are unchanged from the P7 branch. Four brain-maintenance runner tests and `postgres_revocation_after_scan_blocks_actual_provider_dispatch` need the legacy cluster. `credential_transfer_child` is a child helper. Two brain-storage lib tests need `BRAIN_MAINTENANCE_TEST_PG_SOCKET` or `BRAIN_CORE_TEST_PG_SOCKET`. In the full run one ScratchPg supervisor start (`pg_ctl -w -t 5`) failed in `gamefacts_patch_migration`, and the rerun was 2/2 green. The cause is not established, because the supervisor deletes `postgres.log` on failure.

Deploy prerequisites: `brain_migrate` needs EXECUTE on `brain.read_compare_artifact_v1` and `_v2`, otherwise legacy compare links return 503; confirm in `ops.gamefacts_schema_version` that no database has v30 applied, because v30 was edited before any application. Five brain-maintenance tests need the legacy cluster `/tmp/brain-maintenance-test-20261002` and fail identically on `d6077a8e` (evidence in the P7 proof directory).

## P9 verified continuation, 2026-10-14

The P9 fixer completed the scoped publication repair on the pending exact merge of `27ad636b9816013c5cb60855fc2af02d947fcd18`. Additive migration 28 replaces repeated global shadowing expansion with indexed receipt-local checks and statement-level empty-boundary gating. Comparison materializes selected facts once per entity while preserving counts and limits. The SECURITY DEFINER predicate has fixed `pg_catalog,pg_temp` search_path and EXECUTE only for the tool/service roles; canonical/raw access remains denied. All 37 approved migration blobs and the staged P3 parser integration repair are preserved. Temporary diagnostics are removed.

Final diagnostic-free verification: fmt/check/strict all-target Clippy exit 0; publication/cards 7, audit/roles 4, migrations 12, serving 11, parsers 17, timer parser 4, assets 17. Total 72 passed, zero failed or ignored: all original 71 plus the predicate/role contract test. The unchanged 75,000-receipt fixture passes its 500 ms guard; late/backdated shadowing and all four serving regressions pass. Evidence and per-group logs: `/home/nathanael/.local/state/brain-wave2-proof-20261014/P9-fixer2-141703/boundary-init-results.json`. The complete verification runner exited 0.

Current registry: `[1,2,3,6,7,8,9,10,11,13,28]`. Subsequent expected integration union: `[1,2,3,4,5,6,7,8,9,10,11,13,23,26,28]`. P4/P5/P10, combined gate, main integration, release and live proof remain the integrator's responsibility. No release, deployment, main change or timer activation occurred. The records below describe the superseded pause and initial failure, not the final verified state.

## User-requested pause, 2026-10-14

The user requested one complete handover and no new agents. The active P9 fixer workflow `w43vjknm2` was stopped. Continue from [UEBERGABE.md](UEBERGABE.md), not from the historical continuation instruction below. P9 is still an uncommitted merge; an unfinished additive version-28 publication draft and test diagnostics remain unstaged/untracked. Some draft checks passed, but the complete serving group is not green. Fetched `origin/main` is now `c90dff34`, not yet in the integration HEAD. No new agents, main update, release or deployment are authorized by this pause. The source snapshot and migration backup are at `/home/nathanael/.local/state/brain-wave2-proof-20261014/USER-HANDOVER-g_1bxa1l/`.

## Historical integration-phase record before the interrupted fixer

Scope: integrate exact approved commits in order P3, E1, P2, P9, P4, P5, P10 on `integrate/brain-wave2-20261014`. P2 is adopted and pushed. P9 remains an owned, resolved, uncommitted merge because existing P2 serving tests fail with its quarantine view. No main update, final gate, release, deployment, production call or timer activation occurred.

## Approved checklist

- [x] P3 `10502e6d7a8d6b3c8fd6e31bf0169ba29b3e6985`: check, strict all-target Clippy and fmt exit 0; matching tests 182 passed, 0 failed, 0 ignored. Preserved merge `8f1d41c2` and notes commit `d20245f4` without rewriting.
- [x] E1 `0e1aa497bf9d35db083253ba66acfb817a733b91`: check, strict all-target Clippy and fmt exit 0; evaluator tests 6 passed, 0 failed, 0 ignored. No registered migration or live evaluator run.
- [x] P2 `9dc39a74324c5ac7f6c88f66d4c28f6510565112`: adopted exact verified repair merge `b5e57d3b27badafff44c60ac6f9854a62d7c37dd` by normal owned-merge abort and fast-forward after exclusive external backup. Pushed and verified the remote branch SHA. Existing `P2-fixer-final-*.json` proves fmt/check/strict Clippy exit 0, exact public executor 1 passed, complete shared serving 11 passed and root asset helpers 17 passed, zero failures/ignored. The exact case overlaps the shared group. These unchanged checks were not rerun for adoption. The repair preserves one-snapshot card reads and bounded peer joins; no SQL migration or timeout changes.
- [ ] P9 `27ad636b9816013c5cb60855fc2af02d947fcd18`: prepared exact merge on `b5e57d3b`, two registry conflicts resolved. Current fmt/check/strict all-target Clippy exit 0. Selected distinct tests: 66 passed, 5 failed, 0 ignored, 304 filtered. Existing direct item-card test fails with SQLSTATE 57014 on both remote and local scratch PostgreSQL; shared serving reports 7 passed/4 failed. No green P9 stage, commit or push is claimed.
- [ ] P4 `35a54ce3465c02ca57bfa2b542481cee6ea2d7ab`: not attempted; follows P9. Bulletin lowercase-continuation NIT remains nonblocking follow-up.
- [ ] P5 `b663d2a48bc79ae2019a7517ee06fc5f7dafadd3`: not attempted; follows P4. Exact approved code may be integrated, but architecture/final acceptance remains explicitly pending in external `p5-boundary-decision.md`.
- [ ] P10 `5e10cbf5efce409b1e641aa81fdd6c7ad7e0cc12`: not attempted; follows P5. Registers version 26.

Current pending registry: `[1,2,3,6,7,8,9,10,11,13]`. Expected final actual union: `[1,2,3,4,5,6,7,8,9,10,11,13,23,26]`. Reservations 24, 25 and 27 are not registrations. All 37 migration files present in the adopted P2 and exact P9 trees retain their approved Git blobs. No SQL was edited or renumbered.

## Integration resolutions

P3 and E1 merged without source conflicts. P2 retains the six resolved tool/source/registry conflicts and the separately verified bounded query repair. P9 combines versions 2, 3 and 6 with the existing registry and asserts the exact current union, preserving all core schema checks and its late-version-6 upgrade regression.

P9 exposed a compiler/lint conflict in P3's path-included parser copies. Timer extraction now imports the same existing parsers through `brain-storage`; only module/type/parse visibility changes, not parser executable bodies or budgets. The existing depth/core parity test resides in the canonical verifier tests, and the existing KV3 test also verifies selected lexeme hashes. No tests or assertions were removed, no lint suppressions were added. Canonical parser tests and the actual timer parser contract run explicitly instead of duplicated copied-module tests.

## Concrete blocker and continuation

Proof directory: `/home/nathanael/.local/state/brain-wave2-proof-20261014/P9-adoption-094128-b10e43a4/`. Initial owned P2 index/diff/task note and the P5 boundary are backed up there. `P9-linked-results.json` records passing fmt/check/Clippy, audit 4, parsers 17, timer parser 4 and migrations 11. Its card target exits 101 before the run-lock binary executes; remote/local retries are two executions of one distinct failed test, not two distinct failures. `P9-remaining-results.json` records the separately completed run-lock 6, shared serving 7 passed/4 failed and root assets 17 passed. Its outer runner exit 0 is evidence collection, not a green stage.

Direct failure: `rust/crates/brain-storage/tests/gamefacts_p2_cards.rs:257`, `SELECT card FROM serve.item_card WHERE entity_id=$1`, SQLSTATE 57014 under the unchanged 500 ms readonly role, after the existing 75,000-receipt bulk fixture. The exact unchanged card test previously passed in `P2-retry1-storage-integration.log` in 12.72 seconds. Shared serving failures are the qualifier/grid executor, real public item executor, captured 164-slot executor and complete-response slot replay. `P9-blocker.json` and the exact per-command logs preserve evidence; no failed prepared plan or SQL repair is claimed.

Assign a fresh fixer for this substantive P2/P9 serving interaction. Preserve the staged owned merge and parser integration repair, exact incoming SHA, approved migrations, readonly guards, 500 ms statements/deadlines, canonical receipts and publication/quarantine semantics. No speculative SQL, planner override, larger timeout or new migration was introduced. P9 stays uncommitted while these existing tests are red. Continue P4, P5, P10 only after a verified green P9 package. Do not repeat already-passing unchanged groups.

## Subsequent integrator obligations

- Parent owns combined tests, independent acceptance critic, local gate, main merge/push, one release and live proof. Each BLOCK requires a fresh fixer; no round-count cutoff. No intermediate gate was requested here.
- P3: real Timer ingest/repeat, canonical answer receipts and `ops.answer_trace`, actual unit/cgroup peak and timer-key access. Unsupported Soul Urn first spawn remains evidence-free.
- P2: normal `/v1/answer` for Toxic Bullets, Haze/Wraith comparison and positive slot 4 ultimate selection; canonical receipts, correct tool trace, actual Assets cgroup below 300 MB and controlled timer activation. Recheck the genuine current P9 interaction, not historical isolated passes.
- P9: release inventory contains brain-job and brain-migrate; explicitly install/inspect its specific audit service/timer. Genuine deployed audit must finish below 120 seconds with enforced cgroup-v1 cap 536870912 bytes and peak below 200000000 bytes, and prove serving quarantine before activation.
- P4: productive patch_history with FindEntity/GetHero, cold/warm numeric traces, canonical receipts, unchanged replay, oversized-row continuation, publication/quarantine revalidation and historical_patch_unavailable. Inspect actual units; do not start brain-maintenance or dl-knowledge.
- P5: runtime composition, canonical public SearchDocs binding and indexed FTS remain final-acceptance gaps. Private Unix-socket `/v1/operator/query` is an existing supported internal route, not a missing endpoint. Retain scoped private evidence and provider-egress denial. No canon.fact trigger, substitute candidate/fallback model, endpoint or architecture approval may be inferred. Complete the corpus/poison/replay, process/cgroup and query-plan checks in its package notes after the pending decision.
- P10: verify real P2 identity_game_id lookup, both captured stats endpoints, 2 raw objects/about 198 facts, content type/1 MiB cap, same-hour zero-write replay, newest canonical receipt/hour and timer activation after live checks.
- E1: compare the final genuine run with its 125-case baseline and preserve baseline/source-check caveats. The baseline is one live run re-scored offline, not two runs.

All PostgreSQL checks here use existing isolated ScratchPg instances, not production credentials or containers. No inherited-failure exemption is claimed. The pre-existing PostgreSQL backup failure remains out of scope.
