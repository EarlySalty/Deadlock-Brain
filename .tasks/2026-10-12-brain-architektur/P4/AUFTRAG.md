# P4 patches

Goal: ingest official patchnotes from the existing read-only feed into receipt-backed raw/canonical Postgres storage, expose bounded canonical patch history, and serve hero facts only for patch intervals supported by source evidence.

Write ownership: new P4 ingestion and serving modules, new P4 migration, append-only registration in the common-contract shared files, a patchnotes job instance, and this package's notes. P1/P2/P3 owned files remain read-only. No legacy timer restart, tool removal, provider changes, data-quality repair, or unrelated service effects.

Acceptance: Baba latest-patch answer through patch_history with raw receipts; indexed (entity_id, patch_id) query with LIMIT 20 below 5 ms; enforced system-cgroup ingest peak below 300 MB; unchanged second ingest stores zero raw objects; affected formatter, linter, build and existing tests pass against measured baseline. Historical requests with no verified patch interval return no evidence.

Start: 2026-10-12, native Claude Code session 83582205-a6d2-460c-9655-90e0cf147f71, gpt-6.1-sol, ultracode active as confirmed by the harness. Assigned worktree /home/nathanael/.worktrees/Deadlock-Brain-p4-patches, branch feat/brain-p4-patches-20261012, clean HEAD a808f5a492b306d63a2a1a42b533dd1aeaf2d290. Workflow w18pjd9el / wf_bd3ebfed-3cb performs three read-only contract scouts. No additional T3 threads.

Dependencies: integrate only after P1 is live. Merge published P1/main changes, never rebase. P0b install convention and committed P1 brain-job are absent at the start SHA; inspect and request their published contract rather than copying unfinished work. P2/P3 optional patch parameters require orchestrator coordination.

Routing: main orchestrator 89e002a7-758e-449d-9968-eec29a019a78. Questions end the turn with FRAGE AN ORCHESTRATOR. Built, reviewed, merged and live are reported separately; no user questions and no session-to-session coordination.

## Current integration contracts

P1 job checkpoint 3a0740be219fb4aceb304e8020501dcd7281fbd7 was merged into P4 at 61d0e934. Existing brain-job is the only job crate. Reuse extract::Assertion fields, json_pointer or occurrence-aware kv_pointer locators, exact lexical value hashes, and the existing receipt/publication protocol. Add patch dispatch through append-only supported/extract_with_identities/prove cases; no parallel receipt format. P1 Store::start is gamefiles-specific, so a new patch writer must use its own job name and lease while preserving the same transaction/publication guards.

The orchestrator reserves ordered gamefacts migration versions: P1=1, P3=2, P2=3, P4=4, P5=5, P9=6. P1 publishes the ordered migration-list checkpoint separately. P4 SQL remains standalone until then; no custom dispatcher, completion marker or base-version rewrite. P1 alone widens entity.kind in its v1. P4 never changes that base migration or constraint.

The orchestrator authorizes a shared extractor-version increment by one. At integration, the later merger chooses current origin/main's version plus one, never lower. A second unchanged run must create zero new facts. Tool-count assertions are corrected to actual merged tool counts, never weakened.

Canonical patch_history uses numeric entity (canonical.entity ID), optional since/until and limit at most 20. Actual serving delegates at this base do not register legacy patch_history. Preserve its enum/request and legacy object-shaped parse/pin rules; append a distinct numeric canonical subrequest and one canonical definition, without a new mode flag. P1 historical hero refusal and null-only citation builder stay untouched; P4-owned execution runs before that delegate. Shared game-file receipt Evidence must remain byte-for-byte equivalent to P1 representation; cite separate mapping receipts rather than changing shared Evidence for the requested historical patch.

Source metadata was independently verified through the approved read-only credential path: patchnotes_feed_reader, DB deadlock, port 5432, SELECT only. Columns id bigint, title/url required text, posted_at nullable timestamptz, raw_content nullable text. Existing IDs are updated in place; each poll rescans bounded ID pages from zero and compares deterministic raw envelopes. Store only id/title/url/posted_at/raw_content from PostgreSQL json_build_object under UTC, never translated_content. Source IDs and duplicate titles do not define chronology; explicit published timestamps do. Official post identity is source plus stable external ID, with immutable raw versions and an evidence-guarded latest published origin head so corrections cannot leave old claims active. No source refetches or missing-content repair.

Fact versioning requires explicit proof mapping source revision to official patch. Missing mappings stay NULL; do not infer them from ingestion/observation time, titles or one matched number. Existing facts are not patch-only updated. Historical reads require a published mapped source snapshot; no unknown closing endpoint treated as infinity.

Central baseline completed before P4 tracked-code edits: 266 passed, 10 ignored, zero failures for brain-contracts/brain-kernel/brain-serve/brain-storage library targets; command and full proof /tmp/brain-p4-baseline-local-20261012.log. Brain-job baseline is separately pending in /tmp/brain-p4-job-baseline-20261012.log. Do not repeat the whole central baseline in each worker. New and existing affected checks still run after integration.

## Native worker ownership

Only one live writer per row. Earlier workflows and resumed build agents were stopped after a native resumption produced overlapping own writers. Preserve their source artifacts, which belong to this P4 session; do not reset or mistake them for unrelated foreign work. New work resumes from these existing worktrees, not from new copies. Do not SendMessage to a running workflow agent; parent updates this contract and workers read it before mutation, verification and commit.

| Area | Own worktree | Exact write set |
| --- | --- | --- |
| Extraction | /home/nathanael/.worktrees/Deadlock-Brain-p4-extract-worker | new brain-job/src/patch_extract.rs, its contract test/fixtures; append module in brain-job/src/lib.rs and dispatch in extract.rs; authorized VERSION increment; minimal existing parser dependencies in brain-job/Cargo.toml and Cargo.lock only |
| Schema | /home/nathanael/.worktrees/Deadlock-Brain-p4-schema-worker | new P4 SQL migration and gamefacts_p4 contract tests; version=4 list entry only after P1 checkpoint lands; provisional custom gamefacts_p4 dispatcher stays unregistered and is not committed |
| Tools | /home/nathanael/.worktrees/Deadlock-Brain-p4-tools-worker | new pg_patch_tools.rs/tests; append canonical contract and tests in brain-contracts; module registration in service.rs only, no composition replacement yet |
| Feed/store | /home/nathanael/.worktrees/Deadlock-Brain-p4-feed-worker | new brain-job/src/patchnotes.rs and patch_store.rs, dedicated tests; registration/dispatch only after extraction ownership closes |

No code comments, production unwraps, source-data repair, secrets in output, foreign checkout mutations, main operations or deployment by leaf workers. All Rust compiler work uses cargo-slot, required release flags, central artifacts and shared sccache. No extra T3 threads or nested agents. Workers commit only their own verified files; parent performs branch integration, independent acceptance, gate and live deployment after P1 is live.
