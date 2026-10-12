# P3 timers, ingestion v2

Goal: pure receipt-backed timer extraction into Postgres, serve.timer and bounded get_timer, using P1r's per-object ingestion contract. Acceptance includes two poison-object rejections with a succeeded run, real raw-byte receipts, unchanged-content idempotence, no invented urn first-spawn value, an indexed tool query and actual memory/runtime evidence.

Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012-v2
Base: edf503b280777524093307a97fabbb1bdba83d7c
Reference: origin/feat/brain-p3-timers-20261012 at df5c759322ed1da9d08bd47f88b42e9d66b50f3d. Retained as reference, not merged.
Binding design: canonical .tasks/2026-10-12-brain-architektur/REDESIGN-INGEST.md and CONTRACT_INGEST_V2.md. Their restart/core contract supersedes the obsolete P1 base, lifecycle and version-bump portions of the common wave-2 briefing.

Ownership: timer extractor/module/tests, typed request, timer read module, additive timer view/migration v2; append-only registrations after P1r publishes its core. No old P1 Store, leases, pending-run recovery, tombstones, shared extractor bump, runner or copied runner tests. Existing bounded lossless parsers on origin/main remain reusable without copying their implementations.

Budget: extractor plus tests below 800 lines. FactSet uses central limits validation before write; normalized key collisions reject the object. Remote-first cargo-slot with required optimized/locked/job flags. Fresh whole-diff reviewer before gate; integration after P1r live and gate ALLOW. Ready-for-gate estimate: 21:00 CEST assuming the core arrives within the announced 90-minute window.

State at 18:26 CEST: typed get_timer is restored additively on the fresh main-based v2 branch, without copying P1-owned contracts. Pure timer_values.rs reuses the lossless JSON/KV3 parsers already on main; input size and normalization are supplied by its caller, not a duplicate core policy. It has no Store, database, run or old extractor dependency. The future Extractor adapter must use P1r's actual FactSet layout and central limits::validate after the published checkpoint arrives. Owned migration/read files are provisional and unregistered until then.

Completed remote-first checks: contracts 104 passed, parser mapping/receipt checks 7 passed, both zero failed/ignored. Strict --all-targets Clippy for contracts/serve/kernel and the focused parser target passed; the first lint attempt found and then removed an unused test import. Commands use CARGO_SLOT_REMOTE=1, CARGO_INCREMENTAL=0, the shared sccache, path remapping and --release --locked --jobs 3. Logs: /tmp/brain-p3-v2-contract-tests-20261012.log, /tmp/brain-p3-v2-contract-clippy-fixed-20261012.log, /tmp/brain-p3-v2-parser-tests-20261012.log, /tmp/brain-p3-v2-parser-clippy-20261012.log. Scoped formatter completed. These tests prove the independent parser/request contracts, not central normalization, FactSet limits, poison-run completion or live behavior.

TESTNACHWEIS[TW-1]: 111 passed, 0 ignored | Baseline: not separately measured

Budget currently 240 parser lines and 94 parser-test lines; the core adapter and poison-run tests remain to add. Full checks, real Store proof, whole-diff fresh review, gate and live acceptance remain open.
