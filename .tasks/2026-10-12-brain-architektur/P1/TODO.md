# P1 final stop: ingestion design to be rethought

User decision relayed by the Orchestrator at 2026-10-12 16:45 CEST: option c. P1 stops. The ingestion design must be rethought rather than continuing this correction loop. Start no new jobs, workers, fixers or gates. Keep the branch and worktree. No merge or production change. This decision supersedes earlier shipping and continuation instructions. The final action after pushing this record is the requested self-settlement.

## Exact retained state

- Branch: feat/brain-p1-baba-pg-20261012, pushed to origin.
- Owned coordinating worktree: /home/nathanael/.worktrees/Deadlock-Brain-p1-baba-pg, clean before this final documentation update.
- Branch HEAD captured for the final stop: 15e2572c265f397c8f36d4e7b31df05f484883e3. This final documentation commit follows that HEAD; its pushed SHA is reported to the Orchestrator.
- Application candidate and actual gate-five HEAD: 6d0bbda7dd7715dda84ba3c1086a6c0391d1a36f. Application code has not changed since that blocked gate.
- Freshly fetched final-gate base: edf503b280777524093307a97fabbb1bdba83d7c, verified ancestor of the candidate.
- Main merge SHA: none. P1 release SHA: none. P1 has not changed main or production.
- P1 workers/fixers are complete or explicitly stopped; no active P1 worker remains. The last numeric fixer wf_dcf6aa16-7a3/task wrflxfze9/actor a35a92a1783e14421 completed. The final gate task b1vz51wi0 completed exit 1. Do not resume or duplicate these actors. Foreign package threads, branches and worktrees remain untouched.

## All five actual gate logs

Private durable root: /home/nathanael/.local/state/brain-p1-proof-20261012-121638e5-Dr2MKo.

All five logs were copied byte-identically to all-five-gates, mode 0600 under a mode-0700 directory. all-five-gates/manifest.json records original/saved paths, complete candidate SHAs, sizes and SHA-256 hashes. Every actual verdict is gpt-6.1-sol BLOCK; there was no ALLOW or sixth gate.

| Round | Exact application HEAD reviewed | Original log | Durable log | Blocking findings |
| --- | --- | --- | --- | --- |
| 1 | 7ec83af20e4dfd7ce913b44ae55fedc84f659ee4 | /tmp/p1-final-gate-121638e5-round1.log | all-five-gates/round1.log | Canonical result metadata/dependency binding; incremental localization eligibility |
| 2 | 6bcafec8f64e34168cd9e5c68edd1deca10ef1b0 | /tmp/p1-final-gate-121638e5-round2.log | all-five-gates/round2.log | Permanently pending immutable source failures; JSON upgrade-tier overflow |
| 3 | d1b5a57dbd634385cbccc918f673012114ffec94 | /tmp/p1-final-gate-121638e5-round3.log | all-five-gates/round3.log | Failed-run identity receipts and mismatched receipt/revision projection |
| 4 | bd0a012bcbb6725c988debbba06f84266c5ca10e | /tmp/p1-final-gate-121638e5-round4.log | all-five-gates/round4.log | Deterministic Git validation bypassing terminal source failure |
| 5 | 6d0bbda7dd7715dda84ba3c1086a6c0391d1a36f | /tmp/p1-final-gate-121638e5-round5.log | all-five-gates/round5.log | PostgreSQL JSONB condition width; duplicate normalized storage keys |

Final gate command: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p1-baba-pg --base origin/main --head 6d0bbda7dd7715dda84ba3c1086a6c0391d1a36f`.

## Open findings

1. Condition storage width is checked against compact JSON rather than PostgreSQL JSONB text. Gate's concrete case: ability_test.AbilityCooldown with Scale.Value and Scale.Type strings of 428 ASCII characters produces an accepted 1,024-byte condition; PostgreSQL's spaced representation is 1,031 bytes. canon.is_atomic_json rejects it. Siblings: JSON scale qualifiers, GameTracking scale/upgrade qualifiers and source flags. Boundaries: rust/crates/brain-job/src/extract.rs:440 and scripts/migrations/2026-10-12-brain-gamefacts-v1.sql:165.
2. Distinct accepted source fields can collapse to the same storage key. Siblings: BoundAbilities slots "1"/"01"; Upgrades tiers "0"/"00"; MaxHealth/max_health, MaxMoveSpeed/move_speed and Stamina/stamina inside JSON LevelScaling or SpiritScaling; corresponding E-prefixed GameTracking starting-stat pairs. Integer parsing and stat normalization do not reject these collisions before insertion. Uniqueness boundaries: migration lines 195 and 243.

Both failures reach ordinary SQL errors rather than InvalidTarget. Object transactions roll back, but store.finish leaves the immutable source run partial; subsequent invocations process it before fetching corrected upstream. Parent confirmed the caller/schema boundaries in lib.rs:65-68 and store.rs:349-351, :525-526, :571-572 and :608-615. No new post-gate scratch regression or corrective implementation was attempted. The repeated discovery of originating source/storage mismatches is unresolved by this candidate and motivates the user's design-rethink decision.

The aggregate serialize-trigger-binding NIT remains deferred: an extension trigger can compensate for a disabled required trigger. Round one's packaging-inspection NIT was resolved by the completed whole-diff inventory/read; brain-job is in the dynamic binary inventory and P1 units have a separate documented install path. Other authorized follow-ups remain in FOLLOWUP.md, including automatic unsupported-target recovery and future extractor-version cutover. None is authorization to restart work.

## What is proven

- Ordered migration checkpoint 3ef22729188f435f070e6ef8182a90a28df6659c is committed/pushed: ordered version/SQL list, locked separate transactions, exact version-set validation and direct unapplied-v1 kind widening. Isolated recorded checks: 74 passed, one historical fixture ignored, zero failed. No production application of v1.
- Additive canonical tool contracts and bounded PostgreSQL serving paths are implemented and checked in isolation. Recorded contract checks: 108 passed, zero ignored. Serving/kernel checks: 209 passed, 14 ignored, zero failed against the recorded 203/14/0 baseline. These are not a final live answer or all-suites-on-final-HEAD claim.
- Release-hook helper correction retained matching 27 passed, zero failed/ignored. It preserves migration-before-pointer ordering and existing P0b installation boundaries. P1 system-job source and bounded observers have nonproduction verification: 58 owned controls and 56 independent checks. Harmless cgroup lifecycle measurements are not production ingest peaks.
- Earlier gate corrections have concrete saved verification for complete canonical result binding, incremental localization scheduling/deletion/resume, upgrade-tier bounds, succeeded-only receipt/identity resolution and deterministic Git fail-closed behavior. No automatic in-invocation recovery loop, new status or retry API was added.
- The last numeric correction was integrated byte-identically after parent verification of 18 source identities, patch/log hashes and actual test footers. Actual affected tests: 42 passed, zero failed/ignored, one unchanged public-volume probe filtered. No unchanged-source baseline was rerun. Real PostgreSQL representation oracle: 1,248 cases across eight extractor branches and 156 values; 176 original-version proof hashes matched. Twelve numeric terminal-failure cases and two transient same-target resumes passed. Formatter, strict all-target Clippy, optimized check and build passed through the live scheduler in verify profile. Supported source lexemes/hashes and p1-v1 remain unchanged.
- The required whole-diff review read all 50 changed files and disposed of all eight earlier findings at c19d5b26. Its numeric blocker was corrected, but actual gate five found the two additional blockers above. Green checks and review coverage do not establish ALLOW or production acceptance.

Verification scopes overlap and must not be summed. Detailed commands, actual counts, failed/inconclusive attempts and ignored/filtered boundaries remain in REGISTER.md and the referenced evidence.

## Retained evidence and trees

Under the private durable root:

- all-five-gates: five actual logs and the checksum/candidate manifest.
- gate-round-five/gate.log and verdict.json: final judgment and exact HEAD/base; log SHA-256 6cf410651e82d9100ed87c149bba6a5a4d1a481aaf19fcb679cb96584a443091.
- numeric-bound-correction: 115 byte-checked artifacts, including parent-verification.json, complete.patch, source-hashes.json, checks.json, full logs and PostgreSQL/original-hash evidence.
- whole-diff-review: 15 checked artifacts from the completed required review.
- gate-round-four-correction: 39 checked artifacts, including the parent live-scheduler refresh.

Keep the P1 coordinating branch/worktree and its owned supporting trees. The numeric worker tree /home/nathanael/.worktrees/Deadlock-Brain-p1-numeric-fix-20261012 retains its verified uncommitted patch at detached c19d5b26; the integrated application candidate is already pushed. No branch, worktree, release, cache or foreign resource was deleted for this stop.

## Production and acceptance not performed

No main merge/push, production helper replacement, release plan/build/verify/install, gamefacts migration, job config/unit installation, actual ingest, answer request, production tracing, timer enablement or P1 serve restart occurred. Latest metadata-only serve preflight observed PID 3193806, NRestarts 0, system brain-serve.service and existing P0b release ef0418d5e8b217d052d573e40afc12c88caf260a. No secret value was exported, credential file read or private data sent externally.

P1 live acceptance remains unproven: first actual ingest peak strictly below 300000000 bytes under an enforced 536870912-byte cgroup-v1 cap; independently measured unchanged follow-up with zero new raw objects; genuinely indexed get_hero execution below 5 ms; fresh answered Baba Steckbrief with ordered ops.answer_trace find_entity/get_hero calls and resolvable pinned receipts; scoped no-repository/snapshot-loading evidence; readyz 200; new PID/executable/content anchor; empty scoped error-priority journal; hourly timer heartbeat. ACCEPTANCE.md and MEMORY-PROOF.md are prepared procedures, not completed evidence.

## Future continuation boundary

This P1 task is stopped, not awaiting another small fixer or gate retry. A separately authorized continuation must start from the relayed design decision, read these five gate logs and the retained proof, and define the revised ingestion contract and verification authority. Do not silently resume this branch's correction loop, merge the blocked candidate or run the prepared production procedure. The current session only records/pushes this state and self-settles as requested.
