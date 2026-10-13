# P1r continuation, 2026-10-13

Assigned worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p1r-hero-slice`.
Branch: `feat/brain-p1r-hero-slice-20261012`.
Parent session: `93e03a43-41c2-4076-a1ef-dee0f39790ec`.
Responsible main orchestrator: `958d03f8-4b01-466d-87e7-6b0f2ebf14d4`.
Dead prior P1r thread `a3a9902b-85bd-4ec8-a93e-0ae33a4e08bb` must not be resumed.

## Published and locally integrated

- Published feature checkpoint: `e8a1ef45788faeb988464bf44bf8f0932f12c09c`. Core 2/2b through `9f75588d` contains all seven additions and frozen migration v1. The PG executor, `find_entity` and `get_hero` are published independently of Hero review. Final Serve checks: 81 passed, 0 failed, 8 ignored; earlier Contracts and Kernel checks: 65 and 78 passed. The prior combined process was not fully green; its final focused Serve rerun and matching check, strict Clippy and fmt were green. Logs: `/tmp/p1r-wf_a314c2ca-{serve-tests-final,clippy-final,check,fmt-all}.log`.
- Published combined four-contract checkpoint and parent HEAD before the next fixes: `19dec0c96d4765cff9a9a5dd37f5ad2a84bbe7d5`, on `origin/feat/brain-p1r-hero-slice-20261012`. It includes schema fix `a4eb62cdec092b19359e4bbabf112a0f92fa84f7`, canonical receipt fix `3ae5155e181b222d5a580af041408ee6b49b413f`, and final Hero/run-local candidate `81a3914a8ba01418c0eb9283b8f300f4d44f78e3`. All four normal runtime contracts passed together; the subsequent whole-diff gate BLOCK below prevents acceptance or main integration. No main merge, gate ALLOW, deployment, production migration/ingest or timer start.
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

## Latest publication fixes and resolved twins

`46eb8e15957d5d26a7ea2614bfe2a260f2d56637` records an existing raw_bytes receipt only for a fully empty accepted FactSet. `91a0a349bfc77eae1e8871ea60bdb1e13800f92b` genuinely shares ingest-test helpers while preserving every assertion; it reduces the original tests by two physical lines. Both are ancestors of the current lock/publication candidate.

Candidate `59f23b25143786570dafc95df0c8d025ce100eb6`, clean worker `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_7f1dca25-bcd-1`, is preserved as ancestry of the integrated final candidate. It preserves `16f9c3c3` ancestry and fixes separate-run A120 -> B125 -> A120, fourth unchanged-A zero growth, extractor-family/version matching, and the cancellation race from a pooled terminal UPDATE. Terminal publication now executes on the same owned PgConnection holding session lock742110026114, independent of the pool, so capacity one works. Its real PG/source-cache checks: 17 passed, 0 ignored, check/strict Clippy/fmt passed. Original Hero and KV3 source-cache probes were actually rerun with pinned public Git fixtures. Projected combined Hero/core budget is1498/1500; one earlier reviewer miscounted1491, which is not authoritative.

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

All four runtime contracts now pass on clean candidate `81a3914a8ba01418c0eb9283b8f300f4d44f78e3`, integrated into the parent by merge `f3c5b9c6f00eeb14a47da679c1db4a58d91cf0df`. Parent checked exact source parity across rust/scripts/ops and independently measured1499 physical brain-job Rust lines including blanks. Original WIP remains preserved at the exact stash SHA above. No main merge, local gate ALLOW, deployment or production ingest is claimed.

Fresh same-run fix includes committed current-run successful applications in dedup; failed OTHER runs remain excluded. New additive core migration v7, `scripts/migrations/2026-10-13-brain-gamefacts-run-local-v7.sql`, preserves v1's UNIQUE and role guards. It adds a brain_migrate-owned SECURITY DEFINER function with fixed pg_catalog search path and EXECUTE only for brain_ingest, not table UPDATE rights. Its row-locked failed-run/receipt check permits only unpublished run-local fact snapshots. Published runs cannot be reverted to failed. Reserved wave versions2..6 remain available and must be registered in ascending order before7 when merged.

Actual Source -> default EXTRACTORS -> ingest-role preflight -> canonical Hero facts is verified. Missing consumer is an explicit failed run, not false success; matched unchanged applications and supported Ignored outcomes still succeed. Real private-PG CLI produced67heroes845facts979receipts, unchanged second run had accepted0/ignored1 and no growth, and broken preflight failed before creating another run. The private instance was removed. Source/cache/lock/migration suites:27 distinct tests passed0failed0ignored, plus8 focused reruns,35 total passes. Actual brain-job and brain-migrate binary build, all-target check, strict Clippy and formatter passed. Frozen v1/hero-profile and rendered checksum passed. Final evidence: `/tmp/p1r-wf9d-{job-tests,storage-tests,migration-tests,ingest-final,hero-final,run-lock-final,binary-build,check,clippy-final,fmt-final,budget,frozen,artifacts,cli-probe}.log`.

Author `a7b32724f28f95148` is complete and its worktree/branch are clean. The initial workflow `wf_9d131c51-631` returned an incomplete preliminary writer/review snapshot while the resumed owned writer still ran; it is not a valid final review. The final native handover above supersedes that snapshot. No new writer was started in parallel. Fresh read-only review `wf_b9e4e663-b4d` found the source-confirmed publication-integrity issue below. Its Rust reviewer completed check and strict Clippy but stopped at a formatter invocation failure (`no such command: +1.97.1`), so source/test acceptance remained incomplete. Use `/home/nathanael/.cargo/bin/cargo +1.97.1 fmt` explicitly in subsequent checks; no formatting defect was established.

## Whole-diff gate round 1 and ordered follow-up

Actual local gate completed with exit1 against the published combined checkpoint. Log: `/tmp/p1r-local-gate-round1-20261013.log`.

`[gpt-6.1-sol] BLOCK: Run-local replay can reject valid objects, and the new PostgreSQL test contradicts its migration.`

1. Frozen v1's receipt-limit trigger aggregates by raw/run/extractor across repeated applications. Same-run `[A,B,A]` with4096 receipts per valid application rejects final A with23514 and publishes B. Fix the application boundary in a new additive migration, retaining4096 as the per-application cap and4097 rejection. Inspect entity, alias, fact, relation and empty-set twins.
2. `brain-serve/src/pg_tools_tests.rs:800` resets succeeded fixture runs to failed, contrary to v7's immutable publication. The administrator UPDATE now fails23514 before final assertions. Adapt the fixture/lifecycle and assertion to valid unpublished/new-publication data, not disabled guards. The published-fact health UPDATE twins at659/683 also need adaptation for v8 without weakening result-only hash revalidation.
3. Gate release-wiring NIT: the seven-name `source::BINS` list is a minimum, not the artifact inventory. Actual `inventory()` enumerates all workspace binaries and exact manifest parity; `BUILD_ARGS` builds workspace bins. No missing binary was established from that list. Actual `units::install` omits both job templates, and startup-only cgroup peak output is not final job peak proof.

Independent database review also confirmed a source-level boundary defect: direct brain_ingest fact INSERT can associate a failed `valid_from_run` with a succeeded run's receipt; Serve selects by receipt run, exposing that row. v7 UPSERT can then rewrite/retract this visible mismatched row without changing any succeeded status. Add v8 at the common fact INSERT/UPDATE boundary, matching receipt/run, locking unpublished target run and preventing mutation of published or already-mismatched old rows. Preserve applied/published v1 and v7; no silent legacy data rewrite.

Active native fixers have disjoint ownership and no production permission:

- `wf_c1b9207b-12b`, task `whdrfk3te`, writer `a05f227691ccb578d`: new fact-integrity migration v8, storage registration/preflight and real ingest-role regression. Fresh database verifier follows its clean committed handover. No brain-job/Serve/helper writes.
- `wf_92d74192-3a9`, task `wnauaxhar`, writer `ad4f47fc09465e2d0`: wrapper installation of job service/timer templates without automatic start/enable, plus final cgroup-v1 peak capture. Fresh read-only Rust verifier follows. No storage/migration/job Rust writes.
- `wf_2238c73b-a0f`, task `wxfk4yufq`: urgent core3a test-only fixer, started after the main orchestrator's reproduced P3 request. Owns `brain-serve/src/pg_tools_tests.rs`, disjoint from both active fixers. Build the unpublished fixture failed from inception instead of demoting succeeded runs; derive tool assertions from actual registrations so extra wave2 adapters remain valid. Mandatory actual command includes `CARGO_SLOT_REMOTE=1`, `--release --locked --jobs3`, `-p brain-serve service::pg_tools`, `--include-ignored --nocapture` on real private PG. Publish this test-only core3a checkpoint before the next gate; no guard bypass, skips or weakening.
- The per-application-cap fixer starts only after the v8 handover because storage migration registration/preflight files overlap. It must be fresh, preserve1499-line brain-job scope, and inspect any additional Serve-fixture need only after the core3a writer completes. P3 reproduction log: `/tmp/brain-p3-core3-pg-tool-count-corrected-tests-20261013.log`,6 passed1failed0ignored; failure23514 at802-805.

Gate attempts:1 substantive BLOCK,0 ALLOW. Do not retry unchanged or use another model to overturn it. Maximum five failed rounds. The four-contract checkpoint remains published but is not fully accepted, main-ready or live.

## Live release preflight, historical observations only

Earlier read-only preflight observed live release `ef0418d5e8b217d052d573e40afc12c88caf260a`, system `brain-serve.service` PID3193806, NRestarts0, executable not deleted, readyz at `127.0.0.1:8788` HTTP200 JSON statusready, and empty error-priority journal since start. Live DB then lacked gamefacts/brain_tool. Recheck all details before deploy.

Installed `/usr/local/libexec/brain-release` lacks the new migration anchors. After gate ALLOW and verified origin/main, build the helper from that source through its existing central external helper target with deploy-marked cargo-slot. Replace only with no helper running and under `/opt/deadlock-brain/.deploy.lock`. The new attested install runs DBA-peer `brain-migrate up-gamefacts` before pointer activation. Do not manually apply SQL templates or invent role grants.

Normal non-secret job configuration: `/etc/deadlock-brain/brain-job.json`; socket `/run/deadlock-brain-postgresql`, port5446, DBbrain, userbrain_ingest, Infisical auth; cache `/var/lib/deadlock-brain-job`. Production brain_ingest is SCRAM, not peer. Infisical config uses existing credential path `/run/credentials/brain-job@deadlock-data.service/infisical-token`, never copies credentials. Start only system `brain-job@deadlock-data.service` after verified deploy. Timer activation waits for actual successful ingest, unchanged repeat and job peak<300MB. Cgroup-v1 peak path previously identified: `/sys/fs/cgroup/memory/system.slice/brain-job@deadlock-data.service/memory.max_usage_in_bytes`, recheck live.

Use existing `deadlock-brain answer` with normal client JSON outside the source worktree and in-process Infisical token access for synthetic public questions, never real user/community data or tokens in curl args. Required questions: Baba Steckbrief and Bebop. Inspect only time/request-hash-filtered trace and receipt metadata. Must show find_entity/get_hero without deadlock_data, authentic original raw receipts, readyz correct content and no new error-priority journal. Do not start/heal brain-maintenance or dl-knowledge or touch ai-coach.

## Next actions

1. Finish and independently inspect the urgent test-only core3a handover; publish its real-PG include-ignored proof before the next gate. Independently finish the disjoint v8 and release-wiring handovers. Preserve the original stash SHA and all clean worker commits.
2. Start a fresh ordered application-cap fixer from the verified v8 candidate. Inspect remaining Serve fixture compatibility only after the core3a writer finishes, then complete fresh Rust/PostgreSQL acceptance and actual matching runtime suites. Never duplicate or resume a still-running writer.
3. Integrate/publish the corrected combined candidate, fetch latest origin/main, and run a fresh whole-diff local `gate_hook.py --review` round2 on exact literals. No main integration before ALLOW and independent goal acceptance.
4. Merge/push verified main, deploy through brain-release, real role-correct ingest, two live answers/traces, unchanged-repeat zero growth, final job peak<300MB, readyz and clean journal. Only then own branch/worktree cleanup after checked ancestry. Never delete old salvage worktrees or foreign jobs/caches.

Public fixtures on both hosts: `/tmp/p1r-hero-a314-fixture-GInzyzvn/hero-data.json`; `/tmp/brain-core2-cache-wf-7f1dca25-SbrUuw` and `/kv3`. Pass separate dotted Cargo env.NAME.value and env.NAME.force=true. ionos remote cargo-slot has actually worked after disk repair. Do not assume old standalone cargo-remote exists. Latest local disk observation was about250GB free at13:20 on2026-10-13, superseding the earlier6GB observation; recheck before actual release and avoid needless artifacts or competing release builds.
