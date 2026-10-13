# P5 continuation, 2026-10-13

Branch: `feat/brain-p5-docs-20261012-v2`.
Latest source checkpoint: `86aec5c247302baa2d21011b46ddfeca8c19a666`, pushed to origin.
Primary worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p5-docs`.
Chronology and exact evidence: [AUFTRAG.md](AUFTRAG.md).

## Current source and verification

Sparse registry versions1/5/7 and approved Core3a `982645cf325952b56c782fe690f6e23a5a76bf17` are integrated without rebase. P1r owns version7; no migration body or frozen production core was edited.

Independent Rust review found that the original compaction guard permits hidden PostgreSQL OR syntax through Unicode `or²`. Fresh fixer `a2a497c3491682e4b` completed branch `fix/brain-p5-ascii-token-20261013` at `7cb601d0e3ca933d326c565a69aed83d6bbce09e`, based exactly on55b67439. Integrated at86aec5c2; complete Rust tree matches7cb601d0 byte-for-byte. Compaction now applies only to ASCII alphabetic tokens without standalone case-insensitive OR; Unicode, digits and punctuation retain the original search. Deadlines500ms, public512/internal32768-byte bounds, ranking and original ToolRequests remain unchanged.

Real PG counterexample: against text `² omega beta`, `alpha or² omega beta alpha` does not match while the old compacted query matches with rank0.1, in both simple/German vectors. The new regression guards retrieval, tool execution and both revalidation paths for public/internal scopes, including rejection of broadened Evidence. Existing baseline23 passed; new regression before fix failed0 passed/1 failed/23 filtered on remote and local fallback. Fixed worker Docs24 passed/0 failed/0 ignored/0 filtered; shared service::pg_tools4 passed/0 failed/0 ignored/136 filtered across its target invocations; remote targeted regression1 passed/23 filtered. Formatter/check/build/strict Clippy passed. Parent matching verification `bgyj8qqw1` completed successfully: formatter, all-target check and strict Clippy pass; serial native Docs24 passed/0 failed/0 ignored/0 filtered in17.55s; shared service::pg_tools4 passed/0 failed/0 ignored/87 filtered in1.80s. All Cargo stages exited0.

## Binding ownership update, 21:55

The genuine orchestrator decision supersedes the previous P1r/Core3b handover below. Old P1r is stopped; dedicated P1r-F owns its finish, main merge, release and live proof. Core3b will not be published and P1r-F receives no Core questions.

Wait for the orchestrator's explicit message that P1r-F is merged to main. Only then fetch and merge origin/main into this published branch without rebase or force push. Re-evaluate gate findings against that actual source. Any remaining gate-blocking Core defect is now P5's responsibility in its own branch, via an additive new migration only. The genuine correction reserves version24 for that post-P1r-F additive Core repair; do not select the next free number. Versions11..20 remain reserved for P1r-F and the original P5 package version5 remains unchanged. Report use of24 when the actual repair is implemented. Never edit an applied migration. Until the explicit merge notice, continue independent P5 work and keep this branch/TODO published. No main merge or Core repair is authorized before that signal.

## Latest gate: substantive BLOCK

Changed-source gate on7cb601d0 against freshly fetched origin/main `edf503b280777524093307a97fabbb1bdba83d7c` returned BLOCK, exit1. No reroll. Evidence `/tmp/p5-ascii-gate-1-a2a497.log`.

1. Publication protection is bypassable: direct canon.fact INSERT permits valid_from_run to differ from its receipt's ingest_run_id; serve.current_fact publishes by receipt status. canon.write_run_fact checks the incoming pair but not the existing conflict row, allowing a mismatched visible row to be replaced by an unpublished receipt without retracting the succeeded run. Gate anchors migration7:17; v1 fact constraints/grants and serving selection confirm the paths statically.
2. Receipt cap aggregates all applications sharing raw_object/run/extractor. Two individually valid4096-row A applications in one fresh [A,B,A] run can exceed that aggregate cap, reject final A and publish B. Gate anchor v1:55; entity/alias/fact/relation receipts and empty-set markers share the affected write path. Static verification confirms the aggregate trigger key; no parent-native Core repro is claimed.

The earlier instruction to route both findings to a fresh P1r owner fixer is superseded by the binding P1r-F update above. Await the explicit main-merge notice, merge origin/main without rebase, then independently verify any surviving findings and repair them additively in version24 on this branch. Do not edit applied migrations or reroll the current BLOCK unchanged. Previous memory-only gate failures are historical, not the latest gate verdict.

## Other open acceptance

- Canonical/public versus scoped internal Evidence remains undecided. Docs Prose/numeric receipts are incompatible with shared canonical receipt resolution and trace capture. Never relabel private Docs Public or decorate canonical Evidence. Public tools need canonical-result binding after the boundary is decided.
- Owned generated vectors/GIN in migration5 are approved, but the Docs-limited INSERT/UPDATE trigger exception to populate an owned table from canon.fact is unanswered. No core hook, trigger, poller or refresh was added; existing fact GIN does not accelerate reconstructed serving search.
- Docs adapters are not composed in the forwarded production runtime. A fixture is not runtime reachability. Release NIT inspected read-only: ops/brain-release/src/units.rs:9 lists only brain-serve, deadlock-brain-site and brain-browser-consent services; install():128 iterates that list, and main.rs:136/144 calls it. No brain-job service/timer template or separate job-unit install path appears in the searched release module/systemd inventory. This is a missing wrapper-delivery path, not a claim about actual live units; no installer/service action was run.
- Combined ingestion peak including Git/Postgres, real find_entity("Baba") alongside ingested Docs, independent whole-final-diff acceptance and live function remain unproven. Pure extractor/contracts remain798/max799 lines. Separate Store proof stays unmerged;16 real corpus rejections were not repaired out of scope.

## Independent work in progress

Verification-only worker `ac6d053a000dfc88c` is resumed in its own worktree, pinned to ca357031. Goals remain combined ingestion/Git/Postgres memory accounting and actual FindEntity("Baba") after production Docs ingestion, reusing retained proofb17d4a6. Initial published diagnostic `VERIFY_NATIVE_AC6D_20261013.json` at worker28b6a963 records0 runtime cases,0 memory samples and no FindEntity output. Remote standalone fixture failed Cargo/SQLx metadata; native slot wait was cancelled after1009s, not a production regression or measured resource peak. Its own privilege-diagnostic unit was removed. Parent imports only this diagnostic JSON, not uncompiled temporary code or over-budget Store tests. Resume uses the established pinned workspace if needed for temporary verification-only bin wiring, correct central artifacts/metadata, and one longer bounded native slot wait. No temporary manifest/fixture is committed as production code. Private-PG/cgroup diagnostics remain distinct from live deployment. No Core/migration changes, main fetch/merge, source content repair or unchanged-source gate retry. Parent owns existing AUFTRAG/TODO; worker may add only new round evidence. Do not duplicate while resumed.

## Resume

Continue proofs and owned P5 work that do not depend on Core3b. Await the explicit orchestrator P1r-F main-merge notice, then merge origin/main and fix independently verified surviving Core blockers with additive migration24. Do not contact stopped P1r or ask P1r-F Core questions. Obtain genuine decisions/forwarded composition for the other boundaries. Use a fresh fixer per substantive BLOCK, matching checks and a changed-source canonical gate. P1r-F owns its main integration/deploy/live verification. Discord remains unchanged until the replacement Brain is live.

Do not start Brain maintenance/dl-knowledge, touch ai-coach, foreign checkouts/resources or production secrets/data. Bots WIP remains preserved at3c28ce2583d1d06a78d91966a454d60b6e9d6fc0.

TESTNACHWEIS[TW-1]: 28 passed, 0 ignored | Baseline: 0 rot
MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 0 | Gate: inherited worker BLOCK on frozen Core

The protocol counts the parent feature merge and push, not a main merge or new parent gate attempt. Existing-suite baseline is23/0/0; the separately added concrete regression was red before the fix.
