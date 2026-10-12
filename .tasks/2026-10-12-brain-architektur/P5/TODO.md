# P5 continuation checkpoint

P5 is not finished, not main-merged and not live. This is preserved work for continuation, not a success report.

## Current blocker

The user directive at 2026-10-12 08:12 gives P1 live priority and prohibits new cargo-slot jobs until explicit slot clearance. No tests, Clippy, builds or alternative compiler commands may bypass that hold. P1 live and the ordered migration checkpoint remain unproven for P5.

The main orchestrator thread 89e002a7-758e-449d-9968-eec29a019a78 was inspected through the existing T3 read helper. It reports error: Claude usage limit reached. Its latest quota event at 2026-10-12 09:19 CEST states reset at 12:40 CEST. Do not restart that thread, change its model/provider or create a replacement thread without authorization. This is the main orchestrator's limit, not a claimed quota limit in this native P5 session. No message was sent into the errored thread.

The branch-finish hook demands closure of own branches, but that does not authorize premature main integration, unchecked commits, discarded WIP or touching the foreign main checkout. Keep the owned work intact until the P1/build/gate prerequisites permit completion.

## Preserved branches and files

- Primary: /home/nathanael/.worktrees/Deadlock-Brain-p5-docs, feat/brain-p5-docs-20261012, published HEAD b525f037. Published P1 checkpoint 864c468b was imported by merge 60cf63cc. Its changes do not include the promised ordered migration runner. P5 notes, SESSION_REGISTER.md and INGEST_DOCS_RUNBOOK.md are preserved here.
- Schema: /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_a4fd82b8-ae1-1, worktree-wf_a4fd82b8-ae1-1. Standalone SQL commit 835ad1699add967f1957b764428385d269a3439c is pushed to the same origin branch. Unchecked docs_schema.rs and tests/docs_schema.rs remain WIP. Production registration and reserved migration version 5 are still pending. Static SQL review found zero surviving defects; no new runtime proof.
- Original tools: /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_80c1e625-63c-1, worktree-wf_80c1e625-63c-1. Clean committed/pushed 00807dc6b11167f054c8d53d06e7e9f7e13d5aea. Old narrow checks passed, but independent readers confirmed the internal Query mismatch. Do not mistake old green tests for proof of the missing contract.
- Fresh internal fix: /home/nathanael/.worktrees/Deadlock-Brain-p5-internal-query, fix/brain-p5-internal-query-20261012, base 00807dc6. The completed fixer changed only docs_tools.rs/docs_tools_tests.rs and preserved the full valid internal Query input, including multiline and 32768 bytes, through shared bounded SQL selection. Independent read verification found no remaining concrete defect. Public search_docs remains at 512 bytes and whole ToolExecution at 16384 bytes. The internal Evidence list is bounded directly. This source fix is uncompiled, uncommitted WIP, not runtime proof.
- Ingestion and Bots: /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_7ddf485c-91f-1, worktree-wf_7ddf485c-91f-1, HEAD 62d67661; and /home/nathanael/.worktrees/Deadlock-Bots-p5-brain-answer-only, feat/bots-p5-brain-answer-only-20261012, HEAD 84fb7b1c. Both contain unchecked preserved P5 WIP. The completed modglue.rs fixer removed the public !brain pre-answer placeholder and preserved citation suffixes; independent read verification found no defect in that scoped change. Direct/slash unanswered feedback still requires precise acceptance review. 'Answer-only' means only /v1/answer, not a new endpoint. Current writer ownership is recorded below.

## Active native work

Completed workflows wq3nrgzmn / wf_5b57f1b1-e7d and w7iq5gmjf / wf_d7e94372-091 are no longer active writers. They preserve the full-input internal fix, scoped public !brain silence/citation fix and confirmed source defects. Earlier completed workflows we929p68k / wf_b166814b-196 and wxz57axlf / wf_0706b814-2d1 retain their findings and handovers. The original first attempt of wf_5b57f1b1-e7d was stopped before its corrected continuation; preserve any partial own edits. Do not separately resume any of these writer contexts.

New workflow wu3hpwysg / wf_ff3a442f-04b was launched successfully after correcting a JavaScript quoting error in the rejected invocation. The rejected call launched no agents. Its disjoint source-only ownership is:

- Ingestion: docs_extract.rs and tests/docs_extract_contract.rs; only directly necessary existing parser dependency or store-test adaptation. Fix actual HTML5 membership, physical raw-byte line spans and independently receipted nested sections. Preserve the remaining ingestion WIP.
- Schema: docs_schema.rs and tests/docs_schema.rs only. Validate the current_section publication definition and actual CHECK bounds, not just columns, owners, ACL or constraint counts. SQL and runner registration remain frozen.
- Bots: concierge.rs and brain_api.rs with in-file regressions only. Route mixed favorite/gameplay questions to the bound Brain and sanitize artifact titles without dropping validated public page URLs. Other adapter files and completed modglue.rs stay frozen.
- Read-only scope reader: inspect actual dl-bot callers/config and remaining direct/slash unanswered feedback. Legacy shared-library constructors are not automatically P5-removal targets. Distinguish mandatory interaction acknowledgment from public unanswered content.

Each finished writer receives fresh read verification through the workflow. No build, formatter, test, DB, systemd or network action is authorized by these jobs. Collect the completion notification and actual journal before assigning another writer.

Journals: /home/nathanael/.claude/projects/-home-nathanael--worktrees-Deadlock-Brain-p5-docs/2f84a0c2-f0bc-432a-b9a8-f7bca80848e1/subagents/workflows/<workflow-id>/journal.jsonl. SESSION_REGISTER.md holds ownership and the earlier stopped-context list. There are no extra T3 threads.

## Next execution sequence

1. Collect active source-fix/review results and address only confirmed defects with fresh, disjoint writers. Keep new source unchecked and uncommitted during the hold. Verify actual Git state before takeover; never recreate or discard the retained changes.
2. After P1 publishes the ordered migration checkpoint, merge it, then append P5 version 5 using its actual interface. After P1 live and explicit slot clearance, assemble the package, align shared extractor version to origin/main plus one and real tool count, and run affected formatter/build/Clippy/existing tests with the required cargo-slot, release, locked, jobs-3, sccache and real scratch-Postgres conventions.
3. Independently review the exact combined source and acceptance, then run the local merge gate on that HEAD. Use a fresh fixer on BLOCK, at most five failed rounds. Only the responsible integrator may main-merge after ALLOW. Do not use a PR or GitHub Actions as the gate.
4. Release Brain from clean verified origin/main through existing wrappers and serialized locks. Finish/publish old extractor pending runs before version activation. Prove actual system-unit memory cap and peak, successful ingestion, unchanged second run with zero new raw objects and facts, matched-query exclusion under both public grants and provider-free private access. Do not enable the docs timer until true two-run proof passes.
5. Release Bots only after Brain live proof and their own gate. Prove real isolated /v1/answer routing, no 8896/retrieve traffic and unanswered silence. Assess the exact Baba question using fetched committed source plus canonical aliases; do not substitute Billy wiring or fabricate content. Report the stale rendered Billy HTTP 404 as outside P5. Clean up only own integrated branches/worktrees after live acceptance.

## Existing evidence, not current acceptance

Baseline core suites: 283 passed, 18 ignored, zero failures at the recorded baseline, artifact /tmp/brain-p5-baseline-tw1.FVt5zk. Original tools artifacts /tmp/brain-p5-search-final.yw2m9l show build/Clippy exit 0 and 107 passing invocations, including nine repeated docs tests, or 98 distinct tests. They are not the full combined P5 suite.

Older ingestion artifacts /tmp/brain-p5-ingest-tw1 include a reported 27-passed/3-ignored earlier run and a later real-source failure docs_section_heading; the subsequent code correction is not retested. Older Bots artifacts /tmp/p5-bots-baseline-corrected, /tmp/p5-bots-changed-checks and /tmp/p5-bots-final-build-checks include reported test/privacy/Clippy failures and do not verify current WIP. Deferred commands are in the completed worker journals; reconcile them with the actual integration manifest and required flags before execution.
