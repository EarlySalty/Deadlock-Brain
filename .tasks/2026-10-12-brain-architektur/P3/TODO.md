# P3 resume, 2026-10-13

Assigned worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012-v2
Starting remote HEAD: 8254393a8434b3446b9e575ca42e40639ec6d2b7, clean. Earlier verified work is in AUFTRAG.md.

## Published and built

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

Remaining: obtain the P1r preflight fix, prove a successful real CLI job and peak, demonstrate a timer-key-bound index scan without forcing the planner or mislabelling ingest_run_pkey, final local gate, then P1r handover. Current tiny real timer-only plan runs in 0.369 ms but does not demonstrate the required timer-key index scan. Do not count that as index acceptance. No ALLOW or live proof exists yet. This verified P3 checkpoint is an intermediate branch publication, not the agreed gate-ready handover or a completed deployment.

Public fixtures: GameTracking e6bb7e33d212b110d138f98427ef37f9a80c2839 at /tmp/brain-p3-public-core2-20261013.o4gMBt/gametracking.git on ionos, local symlink to /tmp/brain-p3-public-v2-20261012.TOVndF/gametracking.git. Core hero fixture 2a26e26535a390c309e300e2c7b2763fa5ad7abd at /tmp/brain-p3-core2-hero-fixture-20261013.KdtU5H/hero.git on both hosts. Fixtures are public raw data; no downloaded scripts executed.
