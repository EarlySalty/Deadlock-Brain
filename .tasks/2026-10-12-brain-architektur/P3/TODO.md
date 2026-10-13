# P3 resume, 2026-10-13

Current candidate includes the automatic, no-rebase merge of explicitly approved origin/main 0b105f92abaa2697b9c93e20483c60e8d7c8a3b1, published as c9c8c936, and the subsequently fetched release-only main fix eea3c4ecd890f0620a701697ce116d14a25076d4, published as 87ed7cc7, followed by the automatic citation-fix merge f948a44b. Every merge was without rebase or conflicts. This follows the direct 23:15 instruction to merge later P1r-F fixes again. Hero and TimerExtractor remain registered, with sparse migrations 1, 2, 7, 8, 9, 10. Both additive P3 registrations survived unchanged. No applied migration was edited. The earlier implementation checkpoint 87f68163 and its gate history are retained below; a new actual gate on this candidate remains pending. No ALLOW or P3 live deployment is claimed.

## Current main-merge verification

The resumed native read-only fixer independently verified that the two prior BLOCKING causes and template-installation NIT are resolved in the approved main code. Migration 8 enforces Fact/Receipt run consistency with publication-synchronized row locks; migration 10 protects the other direct INSERT twins; migration 9 separates receipt caps by application transaction. Both job templates are installed separately from enabling/restarting long-running services. No P3 production correction or migration 22 is currently necessary.

One suspected application_xid spoof bypass was specifically falsified: core v1 grants INSERT only to named Receipt columns, not the table, so brain_ingest cannot supply application_xid. The new schema preflight checks those denied column privileges, default, cap body, deferred trigger and index. An existing concrete SQLSTATE 42501 spoof test is present in migrations_tests.rs. The fixer read the test but did not execute it; the actual affected migration suite subsequently passed 10/0/0, including the denied application_xid spoof. JSON receipt verification now rejects ambiguous decoded keys even for raw_bytes/byte_ranges/empty FactSets; JSON numeric lexemes and the existing KV3 receipt path remain intact.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

Actual main-based checks so far:
- Four-package formatter passed, log /tmp/brain-p3-main-fmt-20261013.log.
- Full brain-job with original public Hero and Timer fixtures: 19 passed, 0 failed, 0 ignored (5 lib, 7 Hero, 4 core ingest, 3 Timer), remote EXIT=0, log /tmp/brain-p3-main-job-corrected-tests-20261013.log. Separate dotted Cargo env.NAME.value/env.NAME.force=true settings are required; the earlier inline-table CLI attempt failed configuration parsing before tests, log /tmp/brain-p3-main-job-tests-20261013.log.
- The first affected storage run omitted the newly inherited BRAIN_CORE2_KV3_REPOSITORY. Application cap passed, but Core2 had 12 passed/1 failed because the fixture cache path was missing, both remotely and in the wrapper's local retry. This was a setup failure, not a source-regression baseline. Log: /tmp/brain-p3-main-storage-tests-20261013.log. The corrected run now uses the existing read-only attested /tmp/brain-core2-cache-wf-7f1dca25-SbrUuw/kv3, verified original blob e0b9830a7f69726c933da90446e669ad276e73f8 with 2567685 bytes. No fixture cache was modified. Corrected affected storage tests passed 32/0/0 (application cap 1, Core2 13, published writes 4, run lock 6, timer parser 8), remote EXIT=0, log /tmp/brain-p3-main-storage-kv3-configured-tests-20261013.log.
- Actual private CLI/index/RSS proof passed with registered SQL 1/2/7/8/9/10. brain_ingest first accepted=2/rejected=0/ignored=0; unchanged repeat accepted=0/rejected=0/ignored=2, stable raw/fact/receipt counts 2/14/26. Successful process RSS was 8828 KB first and 6548 KB repeat. The 1500-alias public catalog fixture begins unpublished and is published only after its legitimate canonical INSERTs, respecting the new guards. Exact timer query ran brain_readonly -> brain_tool, read-only, 500 ms timeout, no planner forcing: natural timer_key_idx Index Scan, 4 rows/1 loop, 2 hit blocks/0 reads, startup 0.011 ms/total 0.012 ms; Execution Time 3.636 ms, Planning Time 3.843 ms. Timer facts remain 14. Log: /tmp/brain-p3-main-cli-index-memory-20261013.log; full plan /tmp/brain-p3-index-probe-20261013-fm6gl6e7/plan.json. Private PostgreSQL was independently confirmed stopped, pg_ctl status EXIT=3. These are private process/offline-index proofs, not live-unit/cgroup proof.

Final main-based verification task bed8uplmy completed with EXIT=0 on 2026-10-14:

- The first migration run was 9 passed/1 failed: its newly inherited sparse-registry assertion expected [1,7,8,9,10], omitting P3's existing additive version 2. Only that expected list was corrected to [1,2,7,8,9,10]. No test was removed, weakened or skipped. Corrected `CARGO_SLOT_REMOTE=1 cargo-slot test --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-storage gamefacts::migrations -- --include-ignored --nocapture` passed 10/0/0, 44 lib tests filtered, remote EXIT=0. Logs: /tmp/brain-p3-main-migration-tests-20261013.log and /tmp/brain-p3-main-migration-registry-corrected-tests-20261014.log.
- `CARGO_SLOT_REMOTE=1 cargo-slot test --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-serve service::pg_tools -- --include-ignored --nocapture` passed 7/0/0, 87 lib tests filtered, remote EXIT=0. Log: /tmp/brain-p3-main-pg-tool-tests-20261014.log. The actual Timer ToolResult remains 14721 bytes, below the strict 16384-byte bound; its internal ToolExecution is 19526 bytes and is not that bounded result.
- `CARGO_SLOT_REMOTE=1 cargo-slot clippy --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-job -p brain-serve -p brain-storage -p brain-contracts --all-targets -- -D warnings` passed, remote EXIT=0. Log: /tmp/brain-p3-main-clippy-20261014.log.
- `cargo-slot fmt --manifest-path rust/Cargo.toml --check -p brain-job -p brain-serve -p brain-contracts -p brain-storage` passed, empty log /tmp/brain-p3-main-final-fmt-20261014.log, pipeline EXIT=0.

Current main-based affected checks total 68 passed, 0 failed, 0 ignored across the four completed runs (19 Job, 32 Storage, 10 Migration, 7 PG tools). No source-regression baseline is claimed. The inherited 112-pass Contracts suite is historical and is not added to this current total. Source/fixture setup failures above were resolved, not omitted. Final branch publication and actual gate remain pending; live answer/trace and system-unit/cgroup proof remain the integrator's work.

TESTNACHWEIS[TW-1]: 68 passed, 0 ignored | Baseline: not measured

## Subsequent main release update

Fresh origin/main advanced to eea3c4ecd890f0620a701697ce116d14a25076d4 after the first verified merge. Its six-file delta contains the inherited credential-startup mitigation and owner notes, not application/core/migration changes. It was merged automatically without rebasing or conflicts. The job remains oneshot, the three application units use exec, and post-start checks are combined into one fail-fast short shell without changing the asserted cgroup/credential boundaries. No unit, override, credential or live service was modified by P3.

Matching helper checks completed in task b38f2oii6 with EXIT=0: `cargo-slot +1.97.1 fmt --manifest-path ops/brain-release/Cargo.toml --all -- --check`; `CARGO_SLOT_REMOTE=1 cargo-slot +1.97.1 test --manifest-path ops/brain-release/Cargo.toml --locked --jobs 3 units::tests -- --include-ignored --nocapture` passed 7/0/0, 25 filtered; matching remote `check --all-targets` and `clippy --all-targets -- -D warnings` also passed. Logs: /tmp/brain-p3-later-main-release-{fmt,tests,check,clippy}-20261014.log. Total matching checks now comprise 75 passed, 0 failed, 0 ignored. Existing private Timer index/RSS evidence still records its actual 0b105f92 source snapshot, not a new live probe.

The fresh native whole-diff reviewer completed at clean published 87ed7cc7eee2dcdd346b4898b0b60858cb3e191f with no verified P3-owned defects; exact canonical receipts, current-view-only serving, bounds and 787-line extractor/test budget were confirmed. Corrected initial compilation, role-required Clippy and formatter exited 0; check log /tmp/brain-p3-whole-review-check-20261014.log. Its unnecessary additional full Contracts task bq1kd0gkb was canceled before compilation/testing, actual EXIT=143, zero tests, and is not included in passing counts.

During that verification main advanced to f948a44b, the citation-copying correction in provider_input.rs/transport.rs and owner notes. It has now been automatically merged into P3 without conflicts. Matching parent formatter/check/strict-Clippy completed successfully; provider library tests passed 27/0/0 and the existing final-answer fault contract passed 1/0/0, local wrapper EXIT=0. Logs: /tmp/brain-p3-citation-main-{fmt,check,clippy,provider-tests,fault-tests}-20261014.log. The test wrapper used its inherited sticky local route; compilation/testing completed, not skipped. Main's reminder preserves exact IDs and grounding rejection but covers successful FindEntity/GetHero only. The independent delta reviewer confirmed a medium integration omission: GetTimer's successful result batch and corresponding regression-matrix case are excluded. This is verified prompt coverage, not an observed Timer model failure.

A fresh native coder is correcting that exact predicate and matrix in isolated worktree /home/nathanael/.worktrees/Deadlock-Brain-p3-timer-citations-20261014, branch fix/brain-p3-timer-citations-20261014, starting from reviewed 87ed7cc7 and merging f948a44b first. Write scope is provider_input.rs only, with scoped Contracts checks; no API/model/budget/hash-policy/migration change and no live probe. Parent retains P3 integration, independent fix review and actual gate. No additional migration 22 is needed for this prompt coverage correction.

The isolated coder published the exact fix in f682b579ee1b085db69ac1d26d1daed37cc75e6e on fix/brain-p3-timer-citations-20261014, clean, after merging f948a44b. Its own provider_input.rs diff is +12/-1: GetTimer joins the successful-tool predicate and existing matrix (three tools x two transports x two quality modes, twelve combinations). Actual scoped provider_input tests passed 10/0/0, 60 filtered; matching formatter/all-target check/strict Clippy exited 0. Remote logs with confirmed jobs3: /tmp/brain-p3-timer-citations-20261014.vKsSCs/{check-jobs3,clippy-jobs3,provider_input-tests-jobs3}.log; formatter fmt.log. First remote verification used four wrapper jobs and the coder repeated the matching checks under confirmed three jobs; only the corrective runs are cited here. Independent fix verification and P3 integration are pending; no final gate or live probe by the coder. No API/model/budget/hash-policy/migration change, and no migration22.

Current matching completed checks total 113 passed, 0 failed, 0 ignored: prior 75 plus provider27/fault1 and isolated corrected Contracts10. The canceled extra whole-suite run contributes zero, and historical Contracts112 is not counted.

Native session register for this continuation: parent session 48c9ec82-f12d-4b49-9132-f890317954d9; read-only rust reviewer a0597e36b3822f766, actual Agent start and completed whole/delta reports, independent fix verification resumed; isolated coder af4b62b5545310346, actual Agent start and completed published fix f682b579. Both inherit GPT 6.1 Sol; no extra T3 thread was created.

TESTNACHWEIS[TW-1]: 75 passed, 0 ignored | Baseline: not measured

## Earlier user route decision, 2026-10-13 21:55 Europe/Berlin

The orchestrator forwarded the user's replacement route: P1r-F is a dedicated finish team going directly to gate, main integration, release and live proof. The old P1r thread is stopped; core 3b will NOT be published. P1r-F answers no core questions. The previous core-3b dependency and ownership restriction below are historical, not the current next action.

1. Keep the P3 branch published and finish core-independent work. This is complete: Timer extraction/serving, public receipts, poison isolation, unchanged repeat, real A -> B -> A recovery, private ingest-role CLI, process RSS, natural timer-key index and the existing affected checks are recorded below. No unchanged full-suite or known-BLOCK gate rerun is useful before the base changes.
2. Wait for the responsible orchestrator's explicit message that P1r-F is merged to main. Then freshly fetch origin/main and merge it into this branch, without rebasing or force-pushing.
3. P3 then owns any remaining core correction blocking its gate, within this branch, additively. If a new core migration is necessary, use EXACTLY version 22, as the follow-up user correction reserved it for P3. Do not pick the next free version. Versions 11 through 20 remain reserved for P1r-F. P3's original package migration remains version 2. Never edit applied migrations.
4. Run matching build/formatter/linter and existing affected tests plus real timer/CLI evidence against the merged base; perform the actual local gate. A fresh native fixer is required per substantive BLOCK round. No ALLOW claim until the gate returns ALLOW. Main integration, release and live acceptance still require the responsible integrator.

No P1r-F question, new thread, unforwarded checkpoint merge, version-22 migration or production change has been attempted during this hold.

## Actual local gate, round 2

`python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p3-timers --base origin/main --head feat/brain-p3-timers-20261012-v2` completed on published 87f681633da7c4a2442d192a0345eda7dc79ce26 with EXIT=1. Base origin/main was freshly fetched and remained edf503b280777524093307a97fabbb1bdba83d7c. Log: /tmp/brain-p3-core3a-local-gate-20261013.log.

Actual configured verdict: `[gpt-6.1-sol] BLOCK: Published data remains mutable, and valid same-run replays can be rejected.`

1. Reported BLOCKING: core v7 fact write permits a failed valid_from_run paired with a succeeded receipt; direct v1 INSERT grants permit additions to published runs. Reported twins include receipts, entities, identities, aliases, facts and relations.
2. Reported BLOCKING: the cumulative receipt cap for the same raw/run/extractor can reject a valid repeated A after B in one run at the cap, leaving B while the run succeeds. Core receipt-producing paths are the empty marker, entities, aliases, facts and relations.
3. Reported NIT: verify that release units::install actually installs both added job templates.

A fresh native read-only fixer independently confirmed both BLOCKING mechanisms from the executable code/SQL and checked all twins. These are code-verified findings, not newly executed runtime reproductions. Core v1 allows six direct INSERT paths and publishes facts by receipt run while valid_from_run is independently set; core v7 does not guard those direct INSERTs. A succeeded receipt paired with a failed fact run can expose a row, and a subsequent run-local UPSERT can retract it. Required owner correction: additive DB-enforced run/receipt consistency and published-content write protection synchronized with publication; do not edit frozen migrations 1/7.

The second finding is a concrete per-application/cumulative-bound mismatch: central limits allow 4096 FactSet rows, but the deferred v1 receipt cap aggregates raw/run/extractor. Valid same-run A and B each emit 4096 receipts; the final A targets the original raw ID, reaches 8192 in that group, and rolls back. The Runner records an object rejection and still succeeds, leaving B. The rejection is observable in counters; no fully silent failure claim is made. Empty-marker, entity, alias, fact and relation receipt insertion paths all use the same helper. Entity-identity reuses its entity receipt. Required owner correction: preserve the per-application bound and correct renewed-publication order, not increase the limit or skip the final A.

For the NIT, units::install does exist at ops/brain-release/src/units.rs:90-158. Its registry at lines 9-25 includes only three long-running services, not either job template. The targeted release-code search found no other job-template installation path. Template installation must remain separate from enabling/starting instances; simply adding templates to the existing start-and-cgroup-check list would be incorrect. No live installation or service state was inspected.

All corrections are shared-core/release-owner work for P1r, not P3. No Timer extractor/source/tool/view fix is necessary for these findings. The actual BLOCK and successful core-3a checks were sent to the responsible orchestrator; the confirmed ownership report is the next handover. Frozen shared core remains unchanged. No unchanged gate retry, alternate-reviewer vote, trigger bypass, test skip, main merge, deployment or cleanup was attempted.

WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

The confirmed report was handed over after publication of cb471776edc3ba2ae6eec55e28b072d6bcc9f164. The responsible orchestrator's recorded response at 2026-10-13T14:34:34Z assigned both corrections and template-only installation to P1r checkpoint core 3b. P3 must wait for the explicitly forwarded complete checkpoint, merge without rebasing, run affected checks and the next real gate. No unforwarded later owner SHA is authorized.

At the next scheduled watch the orchestrator thread reported a real quota error at 2026-10-13T15:06:28Z, with reset 18:50 Europe/Berlin. This is the orchestrator's limit, not a P3 limit. Actual local time was verified as 2026-10-13T17:14:17+02:00. No resend before reset, model switch or alternate-thread bypass. Latest observed owner publication was 45906677172abef9efd25ee37aab82e115afe5c1 (v9 receipt-cap checkpoint and ordered insert-guard notes), not a forwarded complete core-3b checkpoint; it was not merged. P3 code and migrations remain unchanged and saved on origin.

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 2 | Gate: BLOCK

## Core 3a verification

- Exact shared suite: `CARGO_SLOT_REMOTE=1 cargo-slot test --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-serve service::pg_tools -- --include-ignored --nocapture`. Real remote PostgreSQL run: 7 passed, 0 failed, 0 ignored, 87 lib tests filtered, remote EXIT=0. Log: /tmp/brain-p3-core3a-pg-tool-tests-20261013.log. The failed-run fixture has 2 canon facts, 1 failed run, 0 published receipts and 0 Hero profiles. Both get_hero and find_entity return evidence-free errors for that fixture. Fresh r2 values and receipts are validated; the old dependency is rejected for Provider, Publication and Cache.
- Timer PG integration remains green with canonical receipt equality and exact original lexemes. Bounded ToolResult is 14721 bytes; internal ToolExecution is 19526 bytes. Only ToolResult is subject to the strict 16384-byte bound.
- Four-package formatter passed, EXIT=0, log /tmp/brain-p3-core3a-fmt-20261013.log. Strict all-target brain-serve Clippy passed, EXIT=0, log /tmp/brain-p3-core3a-clippy-20261013.log. Other production packages are unchanged from the already verified core-3 checkpoint; no redundant full-suite rerun was performed.

TESTNACHWEIS[TW-1]: 7 passed, 0 ignored | Baseline: not measured

The inherited red fixture described below is resolved by the approved owner correction. Existing private ingest-role CLI, repeat, reversion, exact receipts, process-RSS and natural-index evidence remains attached to e51a2bd8 because no implementation or migration changed in core 3a. Only P1r integrates/deploys and proves live answer/trace and system-unit/cgroup behavior after ALLOW.

## Earlier core 3 verification and owner-test blocker

- Real private PostgreSQL CLI as brain_ingest succeeded on both pinned GameTracking objects: first accepted=2/rejected=0/ignored=0, unchanged rerun accepted=0/rejected=0/ignored=2. Raw/fact/receipt counts remained 2/14/26. Process RSS peaks were 8816 KB first run and 6360 KB repeat. These are successful process measurements, not production-unit or cgroup peaks.
- The same isolated CLI/index probe used actual registered migration SQL 1, 2, 7 and the exact timer_query.sql as brain_readonly -> brain_tool, read-only with 500 ms timeout. The labelled 1500-alias public catalog benchmark still has a natural timer_key_idx Index Scan, four matching aliases, two hit blocks, Execution Time 3.525 ms, Planning Time 3.457 ms. Timer facts stayed at 14. Log: /tmp/brain-p3-core3-cli-index-memory-20261013.log; full plan: /tmp/brain-p3-index-probe-20261013-2q30lr4w/plan.json. See timer-index-proof.json for the qualified current offline evidence.
- Actual temporary A -> B -> A verification restored 120.00 after 125, accepted=1/rejected=0/ignored=0, two restored facts and raw_count=4. The first observational assertion incorrectly expected ignored=1; it was corrected against the new latest-application contract, then the real run passed. Log: /tmp/brain-p3-core3-job-corrected-tests-20261013.log. Temporary instrumentation was removed exactly; the published extractor/test budget remains 787 lines. Final published timer tests also passed separately.
- Completed affected checks: brain-job 19 passed (5 lib, 7 Hero, 4 core ingest, 3 timer), brain-contracts 112 passed (69/9/23/11), storage parser/run-lock 14 passed (8/6), storage migration tests 3 passed. Every completed check had zero failed/ignored tests. Logs: /tmp/brain-p3-core3-job-corrected-tests-20261013.log, /tmp/brain-p3-core3-contract-tests-20261013.log, /tmp/brain-p3-core3-parser-and-lock-tests-20261013.log, /tmp/brain-p3-core3-migration-tests-20261013.log. No baseline claim is made.
- The real Timer PG integration passed with exact canonical Evidence equality through CanonicalReceipt::from_evidence and format_receipt, matching returned row citations and Evidence IDs. Bounded ToolResult is 14721 bytes; internal ToolExecution is 19526 bytes. Read-only session, timeout, receipts, missing/history/auth errors, truncation and dependency revalidation remain exercised. The shared PG suite is NOT green: 6 passed, 1 failed, 0 ignored. See the remaining owner blocker below.

Final independent formatter and strict four-package all-target Clippy passed after removing the temporary observation, logs /tmp/brain-p3-core3-final-fmt-20261013.log and /tmp/brain-p3-core3-final-clippy-20261013.log. Targeted published timers_ingest rerun passed 3/0/0, log /tmp/brain-p3-core3-final-timer-tests-20261013.log. The fresh native read-only fixer/reviewer independently confirmed the original three blockers are resolved, canonical Timer wiring and bounds are correct, and no new P3-owned correctness/security fix is necessary.

### Remaining inherited test blocker, assigned to P1r core 3a

The exact shared PG command `CARGO_SLOT_REMOTE=1 cargo-slot test --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-serve service::pg_tools -- --include-ignored --nocapture` reached pg_tools_tests.rs:802-805 and failed, remote EXIT=101, 6 passed/1 failed/0 ignored. The unchanged inherited test demotes succeeded fixture runs to failed; migration 7 correctly rejects this with SQLSTATE 23514 from ops.keep_published_run. Log: /tmp/brain-p3-core3-pg-tool-count-corrected-tests-20261013.log. The new trigger was not bypassed and no existing test was skipped or weakened.

The earlier additive tool-count assertion was adjusted from 3 to 4 because GetTimer joins the core tools and ThirdAdapter. The orchestrator subsequently assigned the parent test and registry-tolerant count to P1r core 3a and forbade further pg_tools_tests.rs edits. Its publication-mutation test body remains unchanged. A fresh reviewer verified that the failing mutation is also present in 19dec0c9 and is a Core test-contract mismatch, not a P3 production defect.

The orchestrator explicitly ordered: finish and publish the 19dec0c9 merge; keep core files frozen; merge the forwarded core 3a checkpoint without rebasing; rerun the shared PG suite and the gate with a fresh fixer per BLOCK round. Do not repeatedly rerun the known-red suite unchanged. Formal final gate waits for core 3a. P1r alone integrates/deploys, proves live answer/trace and system-unit/cgroup behavior, and cleans up after the agreed handover.

## Earlier verification history

Assigned worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012-v2
Starting remote HEAD: 8254393a8434b3446b9e575ca42e40639ec6d2b7, clean. Earlier verified work is in AUFTRAG.md.

## Published and built

Verified P3 implementation checkpoint c8c5125c is committed and pushed. The worktree was clean after publication; later proof-note updates are a separate documentation checkpoint.

P1r core 2/2b through 9f75588d was merged at 7c820bec. The PG executor checkpoint e8a1ef45788faeb988464bf44bf8f0932f12c09c is merged at e8e3e577 and pushed to this branch. Tool contract conflicts retain both P3 and P1r additions. Remote `cargo-slot check --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-serve` passed, log /tmp/brain-p3-pg-checkpoint-build-20261013.log.

P1r alone integrates/deploys per the current resume contract. P3 does not merge main, deploy or operate services. P3 is not gate-ready or live.

## Verified P3 implementation checkpoint

- TimerExtractor uses limits::RAW_BYTES and validate_numeric_lexeme without the temporary f64 projection. Exact lexeme receipt hashes are [u8;32]. JSON bytes are not prefix-normalized; KV3 normalization matches the central verifier (BOM first, then whitespace).
- Lazy gametracking source loads misc.vdata and generic_data.vdata at one pinned revision, through one sources.rs entry. Allowlisted public origin, cache lock, disabled credential helpers/hooks, blob:none filtering. No timer source loading in brain-serve.
- serve.timer stays serve.current_*-only. Matching source/raw-object joins and the mandatory Goldbox snapshot anchor hide vanished optional facts and whole vanished box variants. Core v1 SQL is unchanged; P3 migration v2 has not been applied to production.
- GetTimer is wired into P1r's existing PG executor and canonical result binding. It uses the shared isolated brain_tool pool, 500 ms read-only session, bounded result/citation/evidence handling and revalidation. No new pool/provider/configuration. Missing timer evidence and historical patches return evidence-free errors. No Urn first spawn is invented.

Extractor plus extractor/parser tests: 787 lines (87 timers.rs, 230 timer_values.rs, 329 timers_ingest.rs, 141 timers_parser_contract.rs). Serving integration tests are separate from the extractor count.

## Completed checks

- Current strict remote Clippy: `CARGO_SLOT_REMOTE=1 cargo-slot clippy --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-job -p brain-serve -p brain-contracts --all-targets -- -D warnings`, EXIT=0. Log /tmp/brain-p3-pg-wiring-clippy-20261013.log. The subsequent test-only change removes the uncommitted failing CLI helper and restores the original direct Store test; rerun lint after final changes.
- Current isolated PG executor tests: `CARGO_SLOT_REMOTE=1 cargo-slot test --manifest-path rust/Cargo.toml --release --locked --jobs 3 -p brain-serve service::pg_tools -- --include-ignored --nocapture`: 6 passed, 0 failed, 0 ignored, 86 lib tests filtered; other test binaries filtered. Log /tmp/brain-p3-timer-pg-executor-remote-tests-20261013.log. Real PostgreSQL proves read-only role, statement timeout (57014), receipt bytes, missing/history/auth denial, row trimming and revalidation. Large Timer ToolResult is 13635 bytes. Internal ToolExecution with dependency metadata is 17334 bytes; do not describe that envelope as under 16 KB.
- Earlier real Store public-fixture runs had 3 timer tests passed, no failures or ignored. The poisoned objects produce two rejections and succeeded run, unchanged bytes create no new raws, changed data publishes, and vanished optional variant rows change from 2 to 1. Latest full job/parser/contracts reruns are pending below.
- Owned rustfmt and `git diff --check` passed on intermediate states. No production services have been started, enabled, restarted or healed.

## Confirmed core blocker

See CORE_PREFLIGHT_BLOCKER.md. An actual CLI test added during this session failed before ingestion; the failure was independently reproduced using real core SQL and its exact structural query on private PostgreSQL-16: brain_ingest cannot read serve.* in check_gamefacts_schema. No core fix or widened grant has been attempted.

The newly introduced, uncommitted failing CLI test helper was removed, restoring the existing direct Store/public source acceptance path rather than publishing a known-red new test. This does not establish CLI success: successful CLI ingest, CLI peak memory, real system-unit/cgroup proof and the live answer/trace remain explicitly open. The failed process peak of 6188 KB is not a successful ingest memory measurement.

## Completed verification and continuation

Check task bbgb7im4e completed successfully:
1. Full brain-job tests with `--include-ignored --nocapture` and Cargo `--config` env paths for public Timer and Hero repositories: 12 passed, 0 failed, 0 ignored, 0 filtered. Log /tmp/brain-p3-core2-store-registry-tests-20261013.log.
2. `brain-storage --test timers_parser_contract --include-ignored`: 8 passed, 0 failed, 0 ignored, 0 filtered. Log /tmp/brain-p3-parser-corrected-tests-20261013.log.
3. brain-contracts full tests with `--include-ignored`: 109 passed, 0 failed, 0 ignored, 0 filtered across four nonempty binaries. Log /tmp/brain-p3-pg-contracts-final-tests-20261013.log.
Together with the six PG executor checks, this is 135 passing tests, no ignored or failed tests. No baseline pass/fail claim is made.

Current full four-package formatter check passed: `cargo-slot fmt --manifest-path rust/Cargo.toml --check -p brain-job -p brain-serve -p brain-contracts -p brain-storage`. Log /tmp/brain-p3-current-fmt-check-20261013.log is empty, EXIT=0. Strict all-target Clippy for those same four packages passed, log /tmp/brain-p3-final-all-targets-clippy-20261013.log, remote EXIT=0. After restoring the timer test database's original brain_ingest NOLOGIN fixture, targeted timer checks task b7u6gldkl passed: 3 passed, 0 failed, 0 ignored, 0 filtered. Log /tmp/brain-p3-final-timer-store-tests-20261013.log. Latest actual timer-only query Execution Time is 0.369 ms; its ingest_run primary-key index is not a timer-key index proof.

TESTNACHWEIS[TW-1]: 135 passed, 0 ignored | Baseline: not measured

The workspace's .test routing marker from the confirmed CLI failure is retained. For independent tool and Store/parser/contracts checks only, it is temporarily moved to .test.p3-cli-blocked-20261013 and restored on exit. The known CLI failure is not rerun unchanged or described as fixed. ionos disk-full was cleared by the orchestrator; no cargo-remote workspace deletion was performed.

A native read-only reviewer rechecked the current prefix correction and PG timer wiring: no new verified P3 correctness/security findings; prior prefix finding fixed. The reviewer confirmed the core preflight blocker and that the 16 KB contract bounds ToolResult, not the internal ToolExecution dependency envelope. No new reviewer build/run or write occurred.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

The orchestrator has assigned both the serve-read preflight blocker and A -> B -> A freshness to P1r and will forward a new core checkpoint. A -> B -> A is observed against the real Store: restoring 120.00 after 125 yields ignored=1 and still serves 125. See CORE_PREFLIGHT_BLOCKER.md and /tmp/brain-p3-core-aba-observation-20261013.log. Temporary observation code was removed; no stale-value assertion was added to the published tests. P3 remains unchanged in the core; the owner checkpoint must reconcile raw-byte deduplication with renewed publication.

## Timer-key index proof

The catalog-cardinality benchmark now proves the timer-key-bound Index Scan naturally, without changing planner settings. See timer-index-proof.json. The real public GameTracking source produced 14 timer facts; these stayed unchanged while 1500 aliases from pinned public convars.json were projected as clearly labelled benchmark rule entities/aliases. That background projection is not a new production extractor or a claim about the live catalog.

The exact timer_query.sql ran as brain_readonly -> brain_tool in a read-only transaction with 500 ms statement timeout. EXPLAIN (ANALYZE, BUFFERS, FORMAT JSON) shows Index Scan using timer_key_idx, Index Cond `((alias_norm)::text = 'soul_box'::text)`, 4 rows, 2 shared-hit blocks. Execution Time 3.506 ms, Planning Time 3.904 ms. Full plan on ionos: /tmp/brain-p3-index-probe-20261013-yl307673/plan.json; local log /tmp/brain-p3-catalog-index-locked-proof-20261013.log. The actual cargo-remote workspace lock protected the verification binary/source snapshot. Private PostgreSQL stopped afterward.

Fixture loading used a DBA-only scratch login because the core ingest-role preflight is still blocked. Its successful job peak of 8600 KB is a source-processing observation, not proof of the production ingest role or a system-unit cgroup peak. Those acceptance items remain open.

The one-shot admin probe /tmp/brain-p3-index-probe-20261013.py has since been prepared to accept a final optional job_role argument, restricted to brain_core_test or brain_ingest, and to check unchanged repeat raw/fact/receipt counts. Its brain_ingest variant is not yet executed. After the forwarded P1r checkpoint, recompile through cargo-slot, copy this updated temporary probe to ionos, then execute it under /home/naniadm/cargo-remote/locks/ws-c07868587935c61f.lock with the same public fixture paths plus final argument brain_ingest. This is disposable verification tooling, not production code. Do not substitute the DBA result for that check.

The read-only reviewer confirmed this qualified offline index claim with zero new findings: the evidence proves access under the disclosed benchmark cardinality, not the live plan, ingest-role CLI authorization or cgroup peak. Later optional-role/repeat-check probe edits are not evidence of the completed original run.

Post-publication formatter and strict four-package all-target Clippy also passed on c8c5125c, logs /tmp/brain-p3-published-fmt-check-20261013.log and /tmp/brain-p3-published-all-targets-clippy-20261013.log. Production/test files were restored exactly after the temporary A/B/A observation; remaining changes are proof notes only.

Remaining: merge the next P1r preflight checkpoint when forwarded, obtain the core reversion-contract decision, prove a successful real CLI job and peak, resolve the actual local gate findings below, then P1r handover. Live answer/trace and system-unit/cgroup proof belong to the integrator after verified main deployment. No ALLOW or live proof exists yet. This verified P3 checkpoint is an intermediate branch publication, not the agreed gate-ready handover or a completed deployment.

## Actual local gate, round 1

The gate entry point is /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py. The configured review chain starts with gpt-6.1-sol and falls back to claude-opus-5-5 only on a technical failure. No model override was used.

`python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p3-timers --base origin/main --head feat/brain-p3-timers-20261012-v2` completed with EXIT=1 on published HEAD 98aea3ca1b4a6da670d8828b7024eeeb16080375. Log: /tmp/brain-p3-local-gate-20261013.log. Gate: `[gpt-6.1-sol] BLOCK: The ingest CLI is broken, and source reversions silently retain stale values.`

1. migrations.rs:66: ingest-role structural preflight reads inaccessible serve views. Previously reproduced, assigned to P1r.
2. gamefacts.rs:222 and brain-job/src/lib.rs:84: historical was_applied skips renewed A publication after B. Previously reproduced, assigned to P1r.
3. brain-job/src/sources.rs:45: deadlock-data loads hero-data.json but the current registry only has TimerExtractor. A fresh native fixer confirmed the silent ignored-object path and traced the missing hero wiring to P1r/AUFTRAG.md:19. Both inherited P1r checkpoints had an empty extractor registry; P3 removed no hero registration. BRIEFING_P3.md:14 permits game files or deadlock-data, and the registered gametracking loader supplies both accepted KV3 files. JSON timer ingestion through deadlock-data is not an additional mandatory P3 source. This is a real inherited P1r source/registry gap, not authority to change the shared loader or waive the gate.
4. NIT in ops/brain-job@.service:14: the fresh fixer confirmed v1-specific checks, but ops/brain-release/P0b.md:3,41 explicitly specifies a v1 host and assigns the templates to P1. No target-host defect was established. Live applicability remains the integrator's check; no service operation was performed.

The fresh read-only fixer found no necessary P3-owned correction that can resolve the actual BLOCK. Shared core files remain unchanged. The two explicitly assigned owner fixes and the inherited source/registry gap need an owner checkpoint before another substantive gate round.

No main merge, deployment or cleanup was attempted. No unchanged gate rerun or waiver was used.

MERGEPROTOKOLL[MS-1]: 0 Git-Schritte einzeln | Anläufe: 1 | Gate: BLOCK

Public fixtures: GameTracking e6bb7e33d212b110d138f98427ef37f9a80c2839 at /tmp/brain-p3-public-core2-20261013.o4gMBt/gametracking.git on ionos, local symlink to /tmp/brain-p3-public-v2-20261012.TOVndF/gametracking.git. Core hero fixture 2a26e26535a390c309e300e2c7b2763fa5ad7abd at /tmp/brain-p3-core2-hero-fixture-20261013.KdtU5H/hero.git on both hosts. Fixtures are public raw data; no downloaded scripts executed.
