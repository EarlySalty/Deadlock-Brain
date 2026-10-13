# P1r continuation, 2026-10-13

Assigned worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p1r-hero-slice`.
Branch: `feat/brain-p1r-hero-slice-20261012`.
Parent session: `93e03a43-41c2-4076-a1ef-dee0f39790ec`.
Responsible main orchestrator: `958d03f8-4b01-466d-87e7-6b0f2ebf14d4`.
Dead prior P1r thread `a3a9902b-85bd-4ec8-a93e-0ae33a4e08bb` must not be resumed.

## Published and locally integrated

- Published feature checkpoint: `e8a1ef45788faeb988464bf44bf8f0932f12c09c`. Core 2/2b through `9f75588d` contains all seven additions and frozen migration v1. The PG executor, `find_entity` and `get_hero` are published independently of Hero review. Final Serve checks: 81 passed, 0 failed, 8 ignored; earlier Contracts and Kernel checks: 65 and 78 passed. The prior combined process was not fully green; its final focused Serve rerun and matching check, strict Clippy and fmt were green. Logs: `/tmp/p1r-wf_a314c2ca-{serve-tests-final,clippy-final,check,fmt-all}.log`.
- Local parent HEAD: `17ba610995aad91b6f8633f3de60a83a06ba7af1`, integrating schema fix `a4eb62cdec092b19359e4bbabf112a0f92fa84f7` and canonical receipt fix `3ae5155e181b222d5a580af041408ee6b49b413f`. Neither is separately published yet. No main merge, local gate ALLOW, deployment, production migration/ingest or timer start.
- Schema preflight now inspects required Serve view/column/type/relation structure through pg_catalog instead of selecting inaccessible Serve data. Actual brain_ingest and brain_migrate Scratch-PG checks pass; direct Serve reads remain SQLSTATE42501. Missing/renamed/wrong-type/wrong-kind structure and corrupted checksum fail closed. Eight checks passed, one unrelated ignored probe; matching check, Clippy and fmt passed. Independent source review found no verified findings. Logs: `/tmp/wf_c91ae294-schema-{check,clippy-final,migrations-final,gamefacts,baseline}.log`.
- Canonical receipt formatter: `brain_contracts::canonical_receipt::format_receipt(&CanonicalReceipt) -> Result<Evidence, PortError>`; strict reverse resolver `CanonicalReceipt::from_evidence(&Evidence) -> Result<CanonicalReceipt, PortError>`. Documentation: `docs/gamefacts-tool-receipts.md`. Compact typed metadata JSON is hashed with domain `brain.gamefacts.receipt.v1` plus NUL; prefix `gamefacts.receipt.v1:sha256:` is 28 bytes and complete IDs are 92 bytes. All Evidence fields match between tools; numeric database IDs remain internal and in cold/warm-cache traces. Full verified run: Contracts 68, Kernel 78, Serve 83 passed, 8 ignored, total 229 passed. Real PG verifies third-adapter equality, receipt/result mutations, output bounds and cold/warm traces. Check, Clippy and fmt passed; independent source review found no verified findings. Logs: `/tmp/wf_c91ae294-receipt-{tests-final,check,clippy}.log`. P4 adapts its own patch_history; P1r does not edit P4.

## Hero preservation and verified successor

The original seven owned Hero WIP files were inspected and explicitly preserved before integration:

- Unique stash tag: `p1r-original-hero-wip-93e03a43-20261013`.
- Immutable stash SHA: `15b6104604d6943e1d3c97cd73b67922ab1db9d4`.
- Files: `rust/Cargo.lock`, brain-job `Cargo.toml`, `src/lib.rs`, `src/registry.rs`, `README.md`, `src/hero.rs`, `tests/hero.rs`.
- Only these paths were included, with untracked files; this TODO was not stashed. Never use bare stash/pop, never drop an entry by an old ordinal. Preserve this exact object until the verified successor and any necessary original-data recovery are complete.

Verified Hero successor: `4864dda6bc95121d60622093480f6c1c80c9112b`, clean worker `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_a314c2ca-ba0-1`. Exactly the seven owned files, 1497 physical Rust source/test lines. It already registers Hero for deadlock-data, but its prior PG tests explicitly passed `&[&Hero]`; they did not prove default registry/source dispatch.

Mandatory original blob and real Scratch-PG Hero checks: 7 passed, 0 ignored. Public source revision `2a26e26535a390c309e300e2c7b2763fa5ad7abd`, 226182 bytes, SHA256 `947c26790e067109f6cca4085649ed56e446bc5f4c9c571cd396290f802a3456`: 67 named heroes, 845 facts, 979 receipts, two poison rejections, Baba/Bebop values and unchanged-rerun zero growth. The original 912 receipt expectation omitted 67 alias receipts and was corrected. Matching check/Clippy/fmt passed on ionos. These are compiler/runtime checks, not artifact deployment or live AI proof. Logs: `/tmp/p1r-hero-wf-remote-{hero-tests,package-tests-green,clippy,check,lines}.log`.

## Latest publication fixes and open twins

`46eb8e15957d5d26a7ea2614bfe2a260f2d56637` records an existing raw_bytes receipt only for a fully empty accepted FactSet. `91a0a349bfc77eae1e8871ea60bdb1e13800f92b` genuinely shares ingest-test helpers while preserving every assertion; it reduces the original tests by two physical lines. Both are ancestors of the current lock/publication candidate.

Candidate `59f23b25143786570dafc95df0c8d025ce100eb6`, clean worker `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_7f1dca25-bcd-1`, is not integrated into the parent yet. It preserves `16f9c3c3` ancestry and fixes separate-run A120 -> B125 -> A120, fourth unchanged-A zero growth, extractor-family/version matching, and the cancellation race from a pooled terminal UPDATE. Terminal publication now executes on the same owned PgConnection holding session lock742110026114, independent of the pool, so capacity one works. Its real PG/source-cache checks: 17 passed, 0 ignored, check/strict Clippy/fmt passed. Original Hero and KV3 source-cache probes were actually rerun with pinned public Git fixtures. Projected combined Hero/core budget is1498/1500; one earlier reviewer miscounted1491, which is not authoritative.

Fresh read-only acceptance of59f found TWO confirmed same-run twins:

1. After published A120, one common run `[B125,A120]` skips last A because committed B receipts belong to the current default-failed run and are excluded from was_applied. Final value is125 instead of120.
2. One fresh run `[A120,B125]` rejects B with23505 from frozen UNIQUE(entity_id,stat_key,valid_from_run), then publishes120. Dedup-only repair is insufficient.

Therefore59f is not latest-import accepted. The lock/cancellation fix and separate-run proofs remain valid; do not report whole-contract success yet.

## Four-item checkpoint, active ordered fixer

The orchestrator requires ONE published checkpoint containing ALL FOUR:

1. Least-privilege brain_ingest schema preflight.
2. Shared content-addressed canonical receipt format.
3. Latest-import A -> B -> A publication, including confirmed central twins.
4. Hero source/default-registry registration producing canonical Hero facts, plus a regression against a loaded source with no writer or an explicit run error. Legitimate unchanged zero-growth reruns must continue succeeding.

P3 registration finding is confirmed from commit `3e5a9c9a` on `feat/brain-p3-timers-20261012-v2`, P3/TODO.md and `/tmp/brain-p3-local-gate-20261013.log` against98aea3ca. Both inherited core checkpoints had an empty registry; P3 did not remove Hero wiring. Do not modify P3's checkout or waive its gate.

Active workflow: `wf_9d131c51-631`, task `wfq4bvzbi`, script `/home/nathanael/.claude/projects/-home-nathanael--worktrees-Deadlock-Brain-p1r-hero-slice/93e03a43-41c2-4076-a1ef-dee0f39790ec/workflows/scripts/p1r-four-contract-candidate-wf_9d131c51-631.js`.

Native fixer `a7b32724f28f95148` is running in worktree `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_9d131c51-631-1`, branch `worktree-wf_9d131c51-631-1`. Creator is parent session93e03a43; real start is recorded in the workflow journal. It integrates base17ba,59f and4864, then owns the ordered same-run/current-candidate fix and actual Hero source/registry proof. No other writer owns these paths. If frozen v1's run uniqueness requires extra permission support, the fixer is instructed to use the smallest additive core migration v7 after fetching origin, preserving reserved wave versions2..6, existing constraints, published history and least privilege. No version7 implementation or approval beyond that conditional scoped instruction is claimed yet. No new pending/resume/recovery state is allowed. It must preserve physical brain-job budget<=1500 through genuine helper reuse, not weakened tests. A fresh read-only Rust reviewer follows its committed handover. Parent performs the final local gate and publishes only when all four pass.

## Live release preflight, historical observations only

Earlier read-only preflight observed live release `ef0418d5e8b217d052d573e40afc12c88caf260a`, system `brain-serve.service` PID3193806, NRestarts0, executable not deleted, readyz at `127.0.0.1:8788` HTTP200 JSON statusready, and empty error-priority journal since start. Live DB then lacked gamefacts/brain_tool. Recheck all details before deploy.

Installed `/usr/local/libexec/brain-release` lacks the new migration anchors. After gate ALLOW and verified origin/main, build the helper from that source through its existing central external helper target with deploy-marked cargo-slot. Replace only with no helper running and under `/opt/deadlock-brain/.deploy.lock`. The new attested install runs DBA-peer `brain-migrate up-gamefacts` before pointer activation. Do not manually apply SQL templates or invent role grants.

Normal non-secret job configuration: `/etc/deadlock-brain/brain-job.json`; socket `/run/deadlock-brain-postgresql`, port5446, DBbrain, userbrain_ingest, Infisical auth; cache `/var/lib/deadlock-brain-job`. Production brain_ingest is SCRAM, not peer. Infisical config uses existing credential path `/run/credentials/brain-job@deadlock-data.service/infisical-token`, never copies credentials. Start only system `brain-job@deadlock-data.service` after verified deploy. Timer activation waits for actual successful ingest, unchanged repeat and job peak<300MB. Cgroup-v1 peak path previously identified: `/sys/fs/cgroup/memory/system.slice/brain-job@deadlock-data.service/memory.max_usage_in_bytes`, recheck live.

Use existing `deadlock-brain answer` with normal client JSON outside the source worktree and in-process Infisical token access for synthetic public questions, never real user/community data or tokens in curl args. Required questions: Baba Steckbrief and Bebop. Inspect only time/request-hash-filtered trace and receipt metadata. Must show find_entity/get_hero without deadlock_data, authentic original raw receipts, readyz correct content and no new error-priority journal. Do not start/heal brain-maintenance or dl-knowledge or touch ai-coach.

## Next actions

1. Inspect the active fixer's committed candidate and independent findings; resolve verified defects with fresh fixers, not old authors. No duplicated implementation or unchanged gate retry.
2. Integrate verified candidate into this own branch, run combined affected checks, publish all four as ONE CHECKPOINT with full SHA/API/documentation/evidence. Keep original stash SHA preserved.
3. Fresh whole-diff/Hero review, fetch latest origin/main, run `/home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review` with literal paths and at most five fresh-fixer rounds. No main integration before ALLOW.
4. Merge/push verified main, deploy through brain-release, real role-correct ingest, two live answers/traces, unchanged-repeat zero growth, peak<300MB, readyz and clean journal. Only then own branch/worktree cleanup after checked ancestry. Never delete old salvage worktrees or foreign jobs/caches.

Public fixtures on both hosts: `/tmp/p1r-hero-a314-fixture-GInzyzvn/hero-data.json`; `/tmp/brain-core2-cache-wf-7f1dca25-SbrUuw` and `/kv3`. Pass separate dotted Cargo env.NAME.value and env.NAME.force=true. ionos remote cargo-slot has actually worked after disk repair. Do not assume old standalone cargo-remote exists. Local disk was about6GB free during the last probe, so avoid needless artifacts and competing release builds.
