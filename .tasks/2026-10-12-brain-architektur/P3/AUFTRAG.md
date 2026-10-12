# P3 timers

Goal: receipt-backed timer facts in canon, a serve.timer view and a bounded get_timer tool. brain-serve reads Postgres only for this tool. No changes to legacy tools, pre-retrieval, other package content or data quality.

Acceptance: a source-backed timer question calls get_timer with a raw-object-resolving receipt, while the urn first-spawn question receives no fabricated game value. Trace excludes deadlock_data and asset-mirror loading; record brain-serve cgroup memory before and after; production query EXPLAIN uses an index scan. Existing relevant tests retain their baseline. Concrete checks cover receipts, output bounds, scope enforcement and unchanged-source backfill idempotence.

Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012
Base: a808f5a492b306d63a2a1a42b533dd1aeaf2d290

## Verified dependencies

- Read the common wave-2 contract, ZIEL.md, PAKETE.md P3, IST.md and both referenced REGELN.md files. Read the mandatory orchestration and Rust build procedures.
- Local graph is absent. Global Graphify query completed before targeted inventory and source reads. Reuse the existing canonical PgTools pool, permission check and bounded receipt-backed execution in rust/crates/brain-serve/src/pg_tools.rs.
- P0b.md is not present on this branch. Read its current origin/main version through git show, without modifying another checkout. Origin/main is ef0418d5 at preflight. P1's published branch remains a808f5a4 after git fetch origin.
- No rust/crates/brain-job directory or published brain-job source exists at this base. P1's own REGISTER.md records its ingestion implementation as unfinished. P3 cannot append an extraction registration or a subcommand to nonexistent published dispatch.
- scripts/migrations/2026-10-12-brain-gamefacts-v1.sql:149 restricts canon.entity.kind to hero and ability. A timer insert is rejected. Widening the existing check requires replacing that constraint, outside a literal no-drops interpretation. Existing facts need not be rewritten.
- rust/crates/brain-storage/src/schema.rs:278-313 applies exactly one embedded GAMEFACTS SQL string, tracked by a singleton version. There is no gamefacts migration include list, ordered history or package registration hook. A new include alone would not execute P3's migration. P1's migration function is outside P3's append-only ownership.

## Baseline launched

Command: CARGO_INCREMENTAL=0 RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache SCCACHE_DIR=/home/nathanael/.cache/sccache cargo-slot test --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-p3-timers/rust/Cargo.toml --locked --release --jobs 3 -p brain-contracts -p brain-serve -p brain-kernel -p brain-storage

Log: /tmp/brain-p3-baseline-20261012.log
Harness task: bhaohap3z
No pass count or result claimed before completion.

## Updated routing

Gamefacts version 2 is reserved for P3. P1 will publish an ordered (version, sql) registry with per-version recording. Keep P3's migration standalone until that checkpoint lands, then append the v2 entry. The orchestrator confirmed v1 is applied on no database and routed entity-kind widening to P1's unapplied v1. P3's constraint replacement has been removed; v2 now contains only timer-specific index/view/grants.

Acceptance was adjusted by the orchestrator for the source gap: prove a source-backed timer live through get_timer, and prove the urn-spawn question produces no fabricated answer. Missing urn initial-spawn data remains an open data point. No hand-entered game facts.

Timer extraction draft now exists in brain-job/src/timers.rs, exposed only as extract::timers. It reuses P1's existing Leaf/parser/assertion/hash/occurrence-aware locator contract and re-proves the entire typed timer assertion against stored raw bytes. Supported source paths are real misc-data.json/generic-data.json and GameTracking misc.vdata/generic_data.vdata. It extracts soul-box spawn/respawn fields, midboss interval/change/minimum fields, and rejuvenator durations; it deliberately does not manufacture an urn first-spawn fact or interpret the midboss zero camp delay as its map first spawn. The orchestrator authorized additive dispatch in supported/extract_with_identities/prove, one shared extractor-version increment and mechanical tool-count maintenance. Those dispatches are now wired without changing existing branches; the shared version is p1-v2. Integration must use the then-current origin/main version plus one, never a downgrade.

New read-path fixture test is drafted in timer_pg_tests.rs for real isolated PostgreSQL, role isolation, bounded results, receipts to raw IDs, index plan, missing urn evidence and permission denial. It is now registered under timer_tools.rs; execution remains pending. The first native review was incomplete, not an approval. Queued checks remain pending; do not claim their outcomes.

## Orchestrator decisions and implementation

Earlier routing authorized P3's new migration to replace the entity-kind check with hero, ability, item and timer, without data rewrites. This was superseded by the later decision recorded under Updated routing: P1 widens its unapplied v1; P3's v2 has no constraint replacement. P1 owns publishing brain-job and the shared extension interfaces. P3 may add its migration via one line in the shared include list.

Merged P1 checkpoint 3a0740be219fb4aceb304e8020501dcd7281fbd7 into this branch with a merge commit, no rebase. Its published Assertion, Atom and occurrence-aware parser/receipt contracts are the reference for the timer extractor. No alternative receipt locator is introduced.

Created the additive P3 migration and serve.timer view, a closed four-key get_timer request, parameterized LIMIT 33 query returning at most 32 facts, receipt-backed bounded execution, fail-closed historical patch handling and concrete argument/output-bound tests. These changes remain uncommitted pending verification. No production actions performed.

Original baseline completed: 383 passed, 18 ignored, zero failed, exit zero. The remote attempt could not resolve the existing cross-repository path dependency; cargo-slot retried locally and succeeded. The initial P3 run was stopped while still queued before compilation. Its replacement serial verification pipeline bwsam8jmb runs existing contracts/serve/job/storage/kernel tests, the explicitly enabled public timer extraction test, strict Clippy and production serve/job builds. Logs: /tmp/brain-p3-tests-20261012.log, /tmp/brain-p3-public-source-tests-20261012.log, /tmp/brain-p3-clippy-20261012.log and /tmp/brain-p3-build-20261012.log. Results remain pending. Workspace formatter check passes after formatting only the modified timer wiring and new modules. The same native reviewer resumed read-only, without another build.

Historical ownership mismatches on the checkpoint (dispatch/version and tool-count ownership were subsequently resolved as recorded below):

- schema.rs still has exactly one GAMEFACTS include string and a singleton migration marker, not a package include list. A new include line cannot execute an additional migration. Need the actual P1 package registry or authorization for its minimal implementation.
- extract.rs has no extractor registry. Its supported/extract_with_identities/prove functions are fixed P1 entry points. store.rs uses fixed extract::EXTRACTOR and VERSION, and does not re-extract unchanged upstream revisions. Registering timers requires routing additions and a version/backfill contract so existing P1 raw objects are processed once.
- P1-owned pg_tools_tests.rs:485 hardcodes three tool definitions including the delegate. Adding get_timer makes four; permission is needed for this mechanical existing-test assertion update.

## Authorized shared-file touches for the gate

- rust/crates/brain-job/src/extract.rs: additive timer routing in supported, extract_with_identities and prove, preserving existing paths. Shared VERSION incremented exactly once from p1-v1 to p1-v2.
- rust/crates/brain-serve/src/pg_tools_tests.rs: expected definition count updated from three to the actual four. Recompute after merging origin/main.
- rust/crates/brain-contracts/src/tools.rs and brain-serve/src/pg_tools.rs: additive typed get_timer registration and canonical execution routing, without removing legacy tools.
- schema.rs: reserved gamefacts version 2 appended to P1's ordered registry after checkpoint 2; verification remains held.

Backfill acceptance remains unproved until execution: tests/timers_store.rs seeds a valid published p1-v1 state, exercises Store's unchanged-target version backfill, checks raw reuse and uniquely current P1 facts, then requires zero additional raw objects, facts and receipts on a second unchanged run. No guards or immutable metadata are bypassed.

The inspected cached public revisions are deadlock-data 5799055efcc7a66e45e77da283983da11e00d256 and GameTracking e0b9830a7f69726c933da90446e669ad276e73f8. GameTracking midboss fields use m_iSpawnIntervalInSeconds, m_iSpawnIntervalChange and m_iSpawnIntervalMin, not m_fl. The mappings and KV fixture were corrected accordingly. The explicit public-source test now exercises production dispatch and proof for both source formats; expected cached counts are eleven misc facts and three rejuvenator facts per source. The real PostgreSQL EXPLAIN assertion specifically requires an indexed entity-key lookup, not an unrelated index elsewhere in the plan.

## First edited-target verification

Command: CARGO_SLOT_REMOTE=0 CARGO_INCREMENTAL=0 RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache SCCACHE_DIR=/home/nathanael/.cache/sccache cargo-slot test --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-p3-timers/rust/Cargo.toml --locked --release --jobs 3 -p brain-contracts -p brain-serve -p brain-job -p brain-storage -p brain-kernel

Result: 324 passed, 1 failed, 11 ignored, 0 filtered across 17 completed targets; Cargo exit 101. The storage targets were not reached after the serve failure. The failed test was P3's stricter indexed entity-key assertion. With only 40 timer facts and no background facts, PostgreSQL correctly preferred scanning the tiny fact table and joining entities by primary key. Added 2,000 synthetic background facts alongside the 2,000 existing fixture entities, retaining the indexed-key assertion and planner defaults. No production query was changed and no sequential-scan setting was disabled. The corrected fixture still requires rerun. Contracts, kernel, job extractor, completed-state backfill, and the existing P1 serve tests passed in this partial run. The later added opt-in public Store test was not yet present in the compiled artifact.

TESTNACHWEIS[TW-1]: 324 passed, 11 ignored | Baseline: 0 rot

The explicit public-source extraction/proof run passed all three tests, zero ignored, using both real source formats. Log /tmp/brain-p3-public-source-tests-20261012.log, Cargo exit 0. Production brain-serve and brain-job release build succeeded, log /tmp/brain-p3-build-20261012.log, exit 0. Strict Clippy found one type-complexity violation in the new Store test's five-field tuple; replaced it with a named sqlx::FromRow struct, not a lint suppression. The full scoped --all-targets -- -D warnings rerun completed successfully, log /tmp/brain-p3-clippy-fixed-20261012.log, exit 0. The later test-only cgroup metric helper's focused strict lint run completed successfully, /tmp/brain-p3-memory-test-clippy-20261012.log, exit 0. This is a lint result, not an executed memory proof. The initial shell pipeline did not stop after the failed test/Clippy subcommands, so its overall exit 0 is not a green verification claim; each phase's actual log and count is authoritative.

After the orchestrator routed the kind widening to P1, P3 removed the constraint change. PostgreSQL test reruns now depend on checkpoint 2 containing the widened v1 and ordered registry. Do not edit P1's v1 locally or fake the final migrated state.

TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: 0 rot

## Review findings and release preconditions

The resumed static Rust review found that the shared kv_document adapter filters out non-hero/ability leaves. P3 now invokes the same bounded kv3 parser directly for its two KV3 timer documents and selects only mapped numeric fields; JSON and legacy KV still use the original shared adapter. Receipt locators and parser budgets are unchanged. The whole KV3 document remains validated, including ignored members, with a concrete malformed-member check. This correction is confined to P3's timer module. Follow-up native review confirmed the adapter correction by static inspection and found no additional adapter defect. Runtime source counts and RSS are still required: the direct parser materializes bounded tokens/leaves before filtering. The opt-in public timer Store test exercises raw-buffer storage, real PostgreSQL receipts, proof re-parsing, both public formats and unchanged-run emptiness; execute it explicitly and measure the test binary's RSS separately from Cargo/compiler memory.

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft.

The review also confirmed that an unfinished p1-v1 raw.run cannot be resumed after activating p1-v2. Release must therefore occur only after P1 has completed and published all old runs, under the existing job/release serialization. Before extractor activation, require zero rows in raw.run where status <> 'succeeded' and extractor_version differs from the new version. Do not relabel runs or receipts, disable guards, or assume that P1 being deployed proves its job has finished. If the precondition fails, complete the old run under the P1 release first. This live precondition remains open, not a code-recovery claim.

Public source investigation is not a data fix. generic-data contains urn drop duration and rejuvenator durations, but no urn initial-spawn time. misc-data contains soul-box object spawn/respawn fields and midboss camp interval parameters. The midboss initial-spawn override in convars is -1, not evidence of a match spawn time. Do not invent first-spawn values or promote overrides/default camp offsets to actual map times. Check current upstream revisions during ingestion before declaring a final source gap.

## Published interface checkpoint and build hold

Published and pushed 3bf1b2e28f6c53b7be89932f68264d3a03a28474 on feat/brain-p3-timers-20261012. Only brain-contracts/src/tools.rs and brain-contracts/src/timer.rs are in this checkpoint: the closed typed get_timer request, canonical four-key enum and additive request validation. It does not expose a runtime timer tool and does not claim migration, runtime or live acceptance.

Isolated checkpoint verification before the build hold:

- Contract test command: CARGO_SLOT_REMOTE=0 CARGO_INCREMENTAL=0 RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache SCCACHE_DIR=/home/nathanael/.cache/sccache cargo-slot test --manifest-path /home/nathanael/.worktrees/Deadlock-Brain-p3-timers/rust/Cargo.toml --release --locked --jobs 3 -p brain-contracts. Result: 108 passed, 0 failed, 0 ignored, exit 0. Log: /tmp/brain-p3-contract-checkpoint-tests-20261012.log.
- Production consumers built with the same environment and flags using cargo-slot build for brain-serve and brain-job, with the runtime timer draft absent. Exit 0. Log: /tmp/brain-p3-contract-checkpoint-build-20261012.log. Formatter passed; the contract was included in the successful scoped strict lint run.

TESTNACHWEIS[TW-1]: 108 passed, 0 ignored | Baseline: 0 rot

All other P3 work was saved under unique stash tag p3-timers-interface-checkpoint-20261012-3cc4e12b-014d, SHA 04a63d2cfd6687452c5bb73c4efc9cdee3019391, and successfully restored after the push. Restored tracked diffs and every untracked file were compared to the stash bytes before dropping only that verified entry. Runtime implementation and task notes remain uncommitted pending P1 integration and final checks.

User directive at 08:12 CEST: P1 live has exclusive priority. No new cargo-slot tests, Clippy or builds until the orchestrator explicitly releases slots after P1 is live. No P3 build job remains running. Continue only non-build code, SQL, notes and read-only review work. Do not request further P1 interfaces. The corrected PostgreSQL plan test, storage suite, public Store/cgroup proof and production acceptance remain open.

## P1 checkpoint 2 integrated under the build hold

The orchestrator explicitly authorized preserving the branch/worktree without main merge or discard and permitted an unverified WIP checkpoint. It then supplied 3ef22729188f435f070e6ef8182a90a28df6659c from origin/feat/brain-p1-baba-pg-20261012, with reserved package versions P3=2, P2=3, P4=4, P5=5, P9=6. Later P1 gate corrections will arrive through main after P1 merges; do not chase them. The stated 13:50 CEST P1 estimate is not a live proof or a build-slot release.

Fetched that exact SHA and merged it without rebase in 9f3766af5a2e8681c78855098a533ac4eb158286. Own drafts were saved with unique stash tag p3-before-p1-checkpoint2-20261012-3ef22729-9c71, SHA 8db9ac17686e67ecf835274b19168408b19d9e0b, restored without conflicts, and compared before removing that owned entry. The shared pg_tools_tests.rs retains P1's ledger stamp and P3's definition-count adjustment.

Read the shipped ordered registry and exact recorded/listed-set check. Appended reserved version 2 with the existing standalone P3 SQL in brain-storage/src/schema.rs; did not alter migration execution, P1's v1 or its schema-check functions. P3's private direct-SQL Store and timer-tool fixtures now stamp the installed versions as P1's own direct-SQL fixtures do, because P1 removed version stamping from the SQL body. These source adaptations have not been executed. The actual registered migration path, PostgreSQL plan, full checks and live acceptance remain pending; no new Cargo job was launched.

MERGEPROTOKOLL[MS-1]: 1 Git-Schritte einzeln | Anläufe: 1 | Gate: not run; dependency merge on the P3 branch, main integration held.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: dependency merge message

The next P3 snapshot is explicitly WIP, as authorized by the orchestrator. It preserves the restored runtime draft, reserved v2 registration, adapted private fixtures and notes on the existing branch. A whitespace/diff check is not a replacement for the held formatter, linter, build, tests, gate or live proof. Resume from the preserved files after the explicit slot release, rather than recreating the implementation.
