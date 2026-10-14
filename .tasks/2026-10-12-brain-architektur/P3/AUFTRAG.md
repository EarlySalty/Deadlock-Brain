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

Verified independent checkpoint 2bfe7935 is pushed to origin/feat/brain-p3-timers-20261012-v2. It is not a live tool, completed Extractor or gate-ready package.

Budget at that checkpoint: 240 parser lines and 94 parser-test lines; the core adapter and poison-run tests remain to add. Full checks, real Store proof, whole-diff fresh review, gate and live acceptance remain open.

Follow-up: the remote regression proved both empty-container cases were silently accepted (0 passed, 1 failed, 0 ignored, 7 filtered, remote EXIT=101). cargo-slot incorrectly classified that assertion failure as an environment problem and started a local retry; the harness stopped that retry at its 120-second background limit. No completed local result is claimed.

The orchestrator resolved the boundary: no shared parser changes and no duplicate parser. Mappings now mark required properties, checked after parsing against scalar numeric leaves. The complete misc snapshot requires the primary gold-box InitialSpawnTime and midboss SpawnIntervalInSeconds; generic requires RejuvinatorBuffDuration, using their exact KV3 equivalents. Other mapped properties are optional. A required empty/missing property rejects the whole object with timer_property_missing; a present wrong type rejects with timer_property_type. Optional empty containers emit no fact. Collisions are checked on scalar leaves, not empty containers. No switch to another parser without informing the orchestrator.

Required/optional verification completed: 8 passed, 0 failed/ignored/filtered; strict Clippy passed. Remote-first was configured, but the wrapper's sticky fallback ran the test locally after the earlier assertion failure; Clippy completed remotely. Logs: /tmp/brain-p3-v2-required-optional-tests-20261012.log and /tmp/brain-p3-v2-required-optional-clippy-20261012.log.

Merged only the forwarded P1r checkpoint 4cc0cf5d1843897b7519214cb2117e1aa8f848ed in attributed merge 83ff3044. Read its real lib/registry/sources, types/limits/receipts/migrations, Store, CLI and scratch test. The orchestrator approved the one-line pub mod timers export, in addition to registry and reserved migration v2 entries. Existing sha2 moved from dev-only to production dependencies because the reused parser computes real receipts; no new library.

The real TimerExtractor uses central normalization, validates the complete FactSet before Accepted, retains exact numeric text/unit/conditions in canonical Text payloads and has no database access. Its temporary numeric validation projection and input-size stand-in must be replaced by core 2's exact validate_numeric_lexeme and RAW_BYTES before gate. Core 2 also supplies streamed multi-object sources, kv_path verification and successful-object idempotence. These gaps were routed to P1r by the orchestrator; do not repeat the requests or edit those core implementations.

First real scratch Store proof: 2 passed, 0 failed/ignored/filtered; strict all-targets job Clippy passed after moving the existing sha2 dependency. The actual TimerExtractor produced two rejections (oversized text and normalized collision), one accepted object, a succeeded run, unchanged raw bytes and a leaf-hash-resolving receipt; a subsequent changed-object run was free. No unchanged-run idempotence, KV3 Store or public/live proof is claimed at this checkpoint.

Read rule updated before publishing the migration: serve.timer uses only serve.current_alias/current_entity/current_fact. Receipt-backed und aliases supply indexed four-key lookup. Entity keys include source and original entity variants so the frozen current_fact projection cannot collapse different sources. Matching raw object/source across the current views hides stale optional values after disappearance and inherits P9 quarantine. No canon/raw selects in the package view or tool. The raw joins in the scratch test are admin-only receipt verification, not serving.

The adjusted job suite completed: 9 passed, 0 failed/ignored/filtered (4 reused parser tests, 3 existing core tests and 2 P3 tests); strict all-targets Clippy passed. The real scratch test executed shared production timer_query.sql as brain_tool in a read-only transaction with 500 ms timeout, retained complete citations/units/conditions, returned no urn value and hid an old optional drop duration after disappearance. Logs: /tmp/brain-p3-v2-serve-only-store-tests-20261012.log and /tmp/brain-p3-v2-serve-only-store-clippy-20261012.log. Current extractor and its parser/Store tests: 675 lines; final source/core-2/proof count remains to measure.

TESTNACHWEIS[TW-1]: 9 passed, 0 ignored | Baseline: not separately measured

Own read-only public snapshots are captured in /tmp/brain-p3-public-v2-20261012.TOVndF: deadlock-data 0242320cbc507852aaffbcb29204bcf32394eaea and GameTracking e6bb7e33d212b110d138f98427ef37f9a80c2839. Original selected blob sizes: JSON misc 149410, JSON generic 78988, KV3 misc 147082, KV3 generic 47334 bytes. JSON primary required fields are present: gold-box InitialSpawnTime, midboss SpawnIntervalInSeconds and RejuvinatorBuffDuration. IdolParams has drop timing but no initial-spawn key. This is source inspection, not a completed public Store or live proof; no downloaded code is executed.

Full serve registration, core 2 adaptation, public Store/memory/index proof, final budget recount, fresh whole-diff review, gate and live acceptance remain open.

Takeover 2026-10-12 20:20 CEST (Opus, replaces Codex thread 54eaa518). Salvaged the previous agent's uncommitted work: timer stat keys are now canonical underscore keys in the mapping and pass through central limits::normalize_key (same stored keys as before); the scratch test gained a reusable database() helper. brain-serve timer_tools.rs and timer_tools_tests.rs are committed as an unwired draft: they target the PostgreSQL tool layer (database_error, fits, trim_row, citations) that P1r still has to publish with find_entity/get_hero; main has no such layer yet. Checks: scoped fmt passed, strict all-targets brain-job Clippy passed remotely (/tmp/brain-p3-opus-salvage-clippy.log). brain-job and parser tests were queued behind busy local slots at commit time (/tmp/brain-p3-opus-salvage-job-tests.log, /tmp/brain-p3-opus-salvage-storage-tests.log).

Open: core 2 adaptation (validate_numeric_lexeme, RAW_BYTES, kv_path, streamed source loader, idempotence), serve wiring after P1r's serve tool layer, public Store and memory/index proof, fresh whole-diff review, gate, live acceptance.
