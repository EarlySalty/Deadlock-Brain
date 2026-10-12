# P1 gate round 4 correction, revised to fail closed

## Latest binding scope

The latest genuine Orchestrator directive supersedes the earlier automatic-recovery brief. Gate five is the final round. Prefer fail-closed and removal over new recovery surface. A deterministic Git validation error must terminate its source run as the existing terminal failed, publish nothing from that run, and be recorded in ops. Continuing requires a new upstream revision or an operator rerun, not an automatic in-invocation recovery loop. Automatic recovery from unsupported Git targets belongs to FOLLOWUP.md, not P1 acceptance. Do not add a retry API, status or recovery subsystem.

After the verified correction, parent must have exactly one fresh reviewer read the entire diff against findings from gates one through four before running gate five. This explicit review is now required, not an optional lane. Worker must not start that reviewer or run the gate.

## Defect and minimum fix

Read `/tmp/p1-final-gate-121638e5-round4.log`. Complete immutable Git validation sibling set: record-size limits, diff-mode validation, tree type/mode validation, UTF-8 decoding and path validation. Existing ordinary errors leave the target pending and block later invocations before source discovery. Mark the owning immutable source run failed before returning the error. Preserve diagnostic context, audit data, successful-publication filtering, per-source last-complete state and lease-bound guards. Remove automatic catch/fetch/process recovery where the fail-closed contract makes it unnecessary. Inspect analogous immutable-record validation so the same finding does not reopen.

Do not blanket-classify subprocess/transport/read failures, timeouts, lease/DB errors or interrupted/incomplete streams as deterministic target errors. They retain partial same-target resume. No relabel, silent skip, fabricated success, new cancellation status, parser/value-hash change, credential or broad grant.

## Owned current state

Only `/home/nathanael/.worktrees/Deadlock-Brain-p1-gate-fix-r4-20261012`, detached base `bd0a012bcbb6725c988debbba06f84266c5ca10e`. The initial automatic-recovery workflow was stopped for the new directive. Its preserved draft changes git.rs and recovery_tests.rs, 435 additions/85 deletions at inspection; do not treat that draft as verified or a required implementation. Read before editing, preserve useful tests, and remove unnecessary additions rather than expanding it. Parent and foreign trees are read-only.

Minimum git.rs, lib.rs, necessary existing store lifecycle wiring and concrete Rust job fixtures. Schema/SQL/serving, applied migrations, existing nine tools, retrieval, maintenance, K2, P0 helper, other packages/units and rules remain untouched. No worker commit, push, gate, deploy, further agent or T3 thread. Parent owns exact-patch integration, the whole-diff reviewer and the final gate.

## Proof

Actual owned Git/Postgres orchestration for both sources: non-UTF-8/unsupported filename and gitlink failures leave raw.run terminal failed, ops job failed, no source publication, unchanged last-complete head and preserved original raw/receipt/identity audit rows. A separate explicit rerun against a corrected newer source must not be permanently wedged; reuse existing snapshot/tombstone paths rather than adding automatic recovery. Ordinary transient interruption remains partial and resumable with the same immutable target. Check actual serving rows and succeeded receipt/revision pairs. Existing relevant tests must match the revised fail-closed contract without deleting or weakening unrelated checks. Add only concrete bug/contract coverage; no baseline mandate or extra polish. Trigger NIT stays deferred.

Matching formatter, strict all-target Clippy, optimized check/build and affected tests: locked dependencies, jobs 3, shared sccache, CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache. Default cargo-slot remote routing; local only for genuinely local PostgreSQL/public-fixture probes. Existing public fixtures `/tmp/brain-job-p1-public-20261012` where needed. No unaffected repeated suites or competing full release.

## Rules and handover

English code, thinking, descriptions, notes and final strings. German only product strings. No code comments, production unwrap, needless unsafe, em dashes, secrets/credential-file reads, environment dumps, raw production DB output or private external data. Postgres only. Graphify/code-suche before code search/new code; no full extraction. Browser only Moli with guide first if needed. Never Sonnet. Queried fixer order sol, grok, opus55, astra; inherit gpt-6.1-sol, no model/effort override.

Save complete patch, source hashes, exact commands, full logs and actual counts under a fresh `/tmp/p1-gate-r4-121638e5-*`. Return promptly in compact English after verification, no extra packaging/audit loop, no ALLOW/main/live claim. Prior draft/tests are not final proof. Routing: parent `121638e5-3cc5-4f1c-870e-ccf3f7d0895b`, P1 T3 `66bd78fd-84f1-4012-8682-8fe5e8e9a301`, main orchestrator `89e002a7-758e-449d-9968-eec29a019a78`. Return through the native workflow, no SendMessage to active actors or user questions.

## Verified correction

The revised workflow task `wr12wu4aw`, actor `a596a05e8daa6f6c4`, completed FIXED. The parent independently validated the complete patch SHA-256 `af52e6368489d78af244d2c2c0f2fecc1dc0eb00910cb717d686d1f98160eea9`, all three source identities, workspace manifest/lock and final log hashes before exact integration. Only git.rs, lib.rs and recovery_tests.rs changed. Automatic invalid-target catch/fetch recovery was removed. A small process boundary marks deterministic target errors terminal failed before returning; ordinary interrupted streams/process errors remain partial. No retry API, status, schema/SQL or serving change was added.

Actual checks: library 31 passed, existing object contracts eight passed; total 39 passed, zero failed/ignored, one filtered. The unchanged full-volume public-object probe was not repeated. Twenty real Git/Postgres failure cases check both sources, pending/new runs, terminal source failure, ops `ingest_failed`, unchanged publication heads, preserved audit data and separate corrected-revision reruns. Earlier fixture execution ran 30 passed/one failed; protocol-ready fixture startup corrected the anonymous Git fetch failure, with no production retry added. No unchanged-source baseline is claimed.

The worker's formatter, strict Clippy, optimized check and binary build exited zero, but its saved commands used the historical cargo-slot source path. Parent therefore reran matching formatter/Clippy/check/build through the live `/home/nathanael/.local/bin/cargo-slot`; all four exited zero, using the optimized verify profile for compiler checks. Exact commands, logs and outcomes are saved separately as `parent-current-*`. These artifacts are not production release proof. Parent byte-checked and copied 39 evidence artifacts to `/home/nathanael/.local/state/brain-p1-proof-20261012-121638e5-Dr2MKo/gate-round-four-correction`. Main and production remain untouched by P1.

## Required whole-diff review

The completed review read all 50 changed files at `c19d5b26dcdffedfe878799f6c72a69a59b3db38` against `edf503b280777524093307a97fabbb1bdba83d7c` and disposed of all eight earlier findings. Canonical result metadata, localization scheduling, packaging, tier overflow, succeeded-only identity receipts and deterministic Git fail-closed handling are resolved. Trigger-binding hardening stays deferred.

One source-bound sibling reopened gate two before actual gate five: numeric lexemes `1e64` and `1e-64` pass extraction but expand beyond the existing stored numeric width, causing SQLSTATE 23514 and a partial pinned source run. The independent private PostgreSQL diagnostic confirmed the constraint and pending-run index consequence; no production query or duplicate Cargo suite was used. Parent checked the complete file/disposition coverage and preserved 15 artifacts in the private `whole-diff-review` proof directory. A fresh minimal extractor/fixture correction is verified in GATE-R4-NUMERIC.md: 42 actual tests passed, 1,248 PostgreSQL boundary cases and 176 original-version hashes checked; the exact three-file patch is integrated. No further whole-diff review is requested. Actual gate five is still pending and remains the final permitted gate round.
