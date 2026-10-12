# P4 paused for the P1 ingestion redesign

## Binding continuation boundary

Human orchestrator decision, 2026-10-12 16:45 CEST: P1 is stopped and its ingestion design will be rethought. P4 depends on that design and is paused. Stop workers/fixers and start no new jobs. The only current authorization is WIP preservation, push, this continuation note and settlement of this session. Keep every own branch and worktree. Resume only with the promised new briefing, not on the previous P1 estimate or a build-slot notification alone.

Main orchestrator: `89e002a7-758e-449d-9968-eec29a019a78`. Native P4 session: `83582205-a6d2-460c-9655-90e0cf147f71`. No extra T3 threads were created. Prior native runs have completed or were stopped; no own worker/fixer remains active. `CronList` confirmed no scheduled jobs at this preservation point. No new compiler, formatter, test, gate, database, runtime or service job was started.

## Primary package snapshot

Branch: `feat/brain-p4-patches-20261012`.
Worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p4-patches`.

The preservation WIP is the commit containing this TODO; its final pushed SHA is reported in the paused-thread handover. Its parent checkpoint merge is `e3386c81a2dac63bcf2ab95add7924a68418917c`, which merged only authorized P1 checkpoint `3ef22729188f435f070e6ef8182a90a28df6659c`. No rebase or later P1 feature correction was adopted. Later P1 design and contract changes require the new briefing.

Package snapshot contains the reserved P4=4 tuple in `rust/crates/brain-storage/src/schema.rs`, its byte-identical owned SQL dependency `scripts/migrations/2026-10-12-brain-gamefacts-p4-patches-v1.sql`, and package notes. The registry handoff passed independent static review and byte-equality checks only. The four leaf implementations below are preserved on their own pushed branches, not integrated into this package snapshot. Do not mistake the root for the final composed P4 implementation.

## Pushed leaf snapshots

These exact SHAs were confirmed with `git ls-remote` after ordinary, non-force pushes. They are explicitly unverified WIP, not ALLOW or live checkpoints.

| Area | Branch | Pushed WIP SHA |
| --- | --- | --- |
| Extraction | `feat/brain-p4-extract-worker-20261012` | `b954a5334e3047598ef082232762b1d0f5ee26f9` |
| Schema | `feat/brain-p4-schema-worker-20261012` | `b1a1f6b6f0cb42dde0fa1445ed9777dab862f8cf` |
| Tools | `feat/brain-p4-tools-worker-20261012` | `b7118233d64434f3384d3e54a8240f7991d32568` |
| Feed/store | `feat/brain-p4-feed-worker-20261012` | `7ec537e37b1c78aa950242bd3c2ea119cc170cfe` |
| Isolated registry handoff | `worktree-wf_8091de50-aae-1` | `dedaaf1e28a76681b86eb260a29ed41836637946` |

Preserved worktrees, in the same row order:

1. `/home/nathanael/.worktrees/Deadlock-Brain-p4-extract-worker`
2. `/home/nathanael/.worktrees/Deadlock-Brain-p4-schema-worker`
3. `/home/nathanael/.worktrees/Deadlock-Brain-p4-tools-worker`
4. `/home/nathanael/.worktrees/Deadlock-Brain-p4-feed-worker`
5. `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_8091de50-aae-1`

Excluded local scaffolds remain unregistered and uncommitted as previously required: extraction `rust/crates/brain-patches/` and schema `rust/crates/brain-storage/src/gamefacts_p4.rs`. They remain in the retained worktrees; no artifact was discarded. Do not register or delete them without the new scope decision.

## What the WIPs preserve

- Extraction: declared heading-boundary replay, complete qualifier/multi-subject handling, mandatory full-catalogue public proof, lexical hashes, 512/1024-byte limits and proven timer-kind support. Round-3 static reader found no supported residual; no fresh compilation or test execution.
- Schema: latest successful same-upstream published snapshots, pending-closure preservation, unknown published closure refusal and same-raw no-title withdrawal/recovery. Concrete PostgreSQL regression sources are saved, not executed against these fixes.
- Tools: canonical numeric patch_history and optional historical hero serving, single-tracer composition, P1-equivalent gamefile Evidence, pending-close guards and the new actual-query PostgreSQL contract source. No current composed/compiler/live proof.
- Feed/store: unregistered patchnotes ingestion/store/timer source, unchanged raw envelopes, atomic receipt-backed publication and deterministic complete published-catalogue cache invalidation. No actual feed ingestion, unchanged rerun, SQL composition, timing or cgroup proof.
- Registry: reserved tuple 4 plus unchanged owned SQL dependency. P1 entry 1 and exact recorded/listed-set logic were preserved. No independent marker/dispatcher or P1 base-migration change.

Detailed historical findings, baseline logs, contracts and native workflow IDs remain in `AUFTRAG.md`. No P4 migration was executed, no timer installed, no main merge or deployment performed. Legacy patchnotes-sync remains stopped and was not touched.

## Resume only after the new briefing

1. Read the redesigned P1 contract and new package boundaries. Re-establish ownership and permitted verification before launching workers. Do not assume the cancelled P1 ingestion/publication design or its marker/receipt/serialization APIs remain the target.
2. Inspect these pushed WIPs and retained worktrees, then adapt only what the new briefing still requires. Keep published history; do not rebase or pre-emptively merge old P1 feature corrections. Reconcile the reserved migration registry and owned standalone fixture bootstrap against the newly accepted design.
3. Once explicitly permitted, compose extraction/schema/tools/feed, register ingest-patchnotes append-only, reconcile the shared extractor version and actual tool counts, and verify formatter/build/linter/affected existing tests plus real PostgreSQL contracts. Old baseline was 287 passed, 12 ignored, 0 failed; it is not current proof for these WIPs.
4. Independently verify real feed/catalogue behavior, exact receipt/Evidence identity and trace count, corrections/withdrawal/re-extraction, late catalogue publication, unchanged second ingest with zero new raw/facts/changes, indexed bounded query below 5 ms and enforced system-cgroup peak below 300 MB. Existing literal-Baba fixtures are roster/release descriptions, not a proven balance transition.
5. Only the newly authorized integrator may later run the local gate, integrate to main, deploy verified origin/main, prove the live Baba answer and enable the new timer after actual run proof. This preservation handover does not authorize those actions or branch/worktree cleanup.

Reference artifacts, verify existence before use: `/tmp/brain-p4-public-patch-fixtures.DcWouVth/` and the older snapshot defect reproduction `/tmp/brain-p4-db-verify-ytMBjhUL/result.json`. Neither is current runtime proof.
