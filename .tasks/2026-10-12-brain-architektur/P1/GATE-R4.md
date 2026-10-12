# P1 gate round 4 correction

## Goal

Fix the single BLOCKING finding in `/tmp/p1-final-gate-121638e5-round4.log`. Four actual gates have returned BLOCK; parent will run gate five after this bounded correction. No extra review, baseline, polish, new thread or additional package. The serialize-trigger NIT remains deferred in FOLLOWUP.md.

The remaining assumption is that only blob/extractor/identity bounds can permanently invalidate a pinned target. Git record validation also depends on immutable target data. Complete sibling set in `rust/crates/brain-job/src/git.rs`: record-size limits, diff-mode validation, tree type/mode validation, UTF-8 decoding and path validation. These ordinary errors exit `ingest_source` before fetching a newer corrected source. A valid Git target containing a non-UTF-8/unsupported filename or gitlink can permanently wedge imports even after upstream removes it.

Classify deterministic unsupported Git target records through the existing InvalidTarget boundary, retaining useful error context. Inspect the whole sibling set and analogous immutable-record validation, not just the cited line. Do not blanket-classify process failures, transport/read failures, timeouts, lease/DB errors or interrupted/incomplete streams as deterministic source failures. Preserve same-target transient resume and immutable run identity. Existing terminal failed, full corrected snapshot, missing-path tombstones, audit preservation and successful-publication filtering remain unchanged. No new status, relabel, parser/value-hash change, silent skip, guard bypass or fabricated success.

## Ownership and state

Write only `/home/nathanael/.worktrees/Deadlock-Brain-p1-gate-fix-r4-20261012`, clean detached at `bd0a012bcbb6725c988debbba06f84266c5ca10e`. Minimum git.rs and concrete Rust job regression fixtures. lib.rs only if strictly necessary to apply the existing boundary; store/schema/serving/SQL are read-only unless a genuinely required fixture adaptation is demonstrated. Parent/foreign trees are read-only. No worker commit, push, gate, deployment, service changes, further agent or T3 thread. Parent integrates the exact verified patch and owns the final gate. No existing nine tools, retrieval, maintenance, K2, release helper, other units/packages or rule changes.

## Checkable proof

Use actual owned Git/Postgres orchestration for both source kinds. An unsupported/non-UTF-8 filename and a gitlink must fail without publication, retain the same target on an unchanged retry, then recover when a newer upstream target removes the invalid record. Verify current serving rows, unchanged original raw/receipt/identity audit data, partial assertion repair where applicable, and zero raw delta on unchanged follow-up. Exercise the remaining record/mode/path bounds with genuine or narrowly targeted parser inputs. Demonstrate an ordinary transport/interruption failure stays resumable and is not reclassified. Reuse existing recovery fixtures; do not fake the defective Git/DB path. Add only concrete bug/contract tests. Existing relevant job checks must remain green.

Matching formatter, strict all-target Clippy, optimized check/build and affected tests use locked dependencies, jobs 3, shared sccache, CARGO_INCREMENTAL=0, RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator, RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache, SCCACHE_DIR=/home/nathanael/.cache/sccache. Default cargo-slot remote routing; local override only for genuinely local PostgreSQL/public-fixture probes. Existing public fixture directory `/tmp/brain-job-p1-public-20261012` where required. No repeated unaffected suites or competing workspace release. Save complete unfiltered logs and actual counts, including earlier failures/null filters separately.

## Rules and prompt handover

English thinking, code, tool descriptions, notes and every string in the final structured result. German is only for product-facing strings, not internal reports. No code comments, production unwrap, needless unsafe or em dashes. No secrets/credential-file reads, environment dumps, raw production DB output, private external data or new stores. Graphify/code-suche before code search/new code; no full extraction. Browser only Moli with its guide first if needed. Never Sonnet. The queried fixer order is sol, grok, opus55, astra; use inherited gpt-6.1-sol with no model/effort override.

Save complete patch including new files, source hashes, exact commands, final counts and outcome under a fresh `/tmp/p1-gate-r4-121638e5-*`. Return promptly after verification with compact English structured fields, no extra audit/packaging loop. Do not claim ALLOW/main/live. Parent handles any genuine scope decision, not user.

Routing: parent `121638e5-3cc5-4f1c-870e-ccf3f7d0895b`, P1 T3 `66bd78fd-84f1-4012-8682-8fe5e8e9a301`, main orchestrator `89e002a7-758e-449d-9968-eec29a019a78`. Return only through this native workflow. No SendMessage to active workflow actors and no user questions.
