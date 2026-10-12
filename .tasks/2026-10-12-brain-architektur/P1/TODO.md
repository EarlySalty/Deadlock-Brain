# P1 stopped after final gate BLOCK

As of 2026-10-12 16:37 CEST. The Orchestrator acknowledged that gate five exhausted the permitted loop and instructed P1 to save state, wait, and keep its branch and worktree. No new gate, merge or production change is authorized until the Orchestrator relays a user decision. This stop supersedes earlier shipping instructions and pending-action notes.

## Exact saved state

- Branch: feat/brain-p1-baba-pg-20261012, pushed to origin.
- Owned coordinating worktree: /home/nathanael/.worktrees/Deadlock-Brain-p1-baba-pg.
- Application candidate and actual gate-five HEAD: 6d0bbda7dd7715dda84ba3c1086a6c0391d1a36f.
- Freshly fetched gate base: edf503b280777524093307a97fabbb1bdba83d7c. It is an ancestor of the candidate.
- Gate command: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p1-baba-pg --base origin/main --head 6d0bbda7dd7715dda84ba3c1086a6c0391d1a36f`.
- Gate task b1vz51wi0 completed exit 1 with gpt-6.1-sol BLOCK. Five actual gate rounds have now returned BLOCK. No sixth round or new fixer was started.
- Main merge SHA: none. P1 release SHA: none. P1 has not changed main or production.

## Remaining blocking findings

1. Condition storage width is validated against compact JSON rather than PostgreSQL JSONB text. Gate's concrete case: ability_test.AbilityCooldown with Scale.Value and Scale.Type strings of 428 ASCII characters produces an accepted 1,024-byte condition; PostgreSQL's spaced representation is 1,031 bytes. The existing canon.is_atomic_json bound rejects it. Siblings: JSON scale qualifiers, GameTracking scale/upgrade qualifiers and source flags. Relevant boundaries: rust/crates/brain-job/src/extract.rs:440 and scripts/migrations/2026-10-12-brain-gamefacts-v1.sql:165.
2. Distinct accepted source fields can collapse to the same storage key. Siblings: BoundAbilities slots "1"/"01"; Upgrades tiers "0"/"00"; MaxHealth/max_health, MaxMoveSpeed/move_speed and Stamina/stamina inside JSON LevelScaling or SpiritScaling; corresponding E-prefixed GameTracking starting-stat pairs. Integer parsing and stat normalization do not reject these collisions before relation/fact insertion. Existing uniqueness boundaries: migration lines 195 and 243.

Both failures reach ordinary SQL errors rather than InvalidTarget. Object transactions roll back, but store.finish leaves the source run partial. The next invocation processes that pending target before fetching upstream. Parent confirmed these caller/schema boundaries by reading lib.rs:65-68, store.rs:349-351, :525-526, :571-572 and :608-615. The final gate findings are preserved verbatim; parent did not run a new post-gate scratch regression or attempt a fix.

The aggregate serialize-trigger-binding NIT remains deferred in FOLLOWUP.md. It is not an additional blocking finding.

## Verified work to retain

The last correction rejects unsupported numeric storage width/domain at the originating assertion boundary without changing SQL, lifecycle, supported lexemes/hashes or p1-v1. Its exact three-file patch was integrated after parent verification of 18 source identities, complete patch/log hashes and actual test footers. Actual affected tests: 42 passed, zero failed/ignored, one unchanged public-volume probe filtered. No unchanged-source baseline was rerun. PostgreSQL representation coverage: 1,248 cases across eight extractor branches and 156 values; 176 original-version proof hashes matched. Twelve numeric fail-closed cases and two transient same-target resumes passed. Formatter, strict all-target Clippy, optimized check and build passed through the live scheduler in verify profile. These are nonproduction checks, not live acceptance.

The required whole-diff review completed all 50 changed files and all eight earlier dispositions at c19d5b26. The numeric correction closed its sole reported blocker, but the actual final gate found the two additional storage-boundary siblings above. Do not treat that review or green tests as ALLOW.

## Evidence

Private durable root: /home/nathanael/.local/state/brain-p1-proof-20261012-121638e5-Dr2MKo.

- gate-round-five/gate.log and verdict.json: actual final judgment, exact HEAD/base and log SHA-256 6cf410651e82d9100ed87c149bba6a5a4d1a481aaf19fcb679cb96584a443091.
- numeric-bound-correction: 115 byte-checked artifacts, including parent-verification.json, complete.patch, source-hashes.json, checks.json, actual logs and PostgreSQL/original-hash evidence.
- whole-diff-review: 15 checked artifacts from the completed required review.
- gate-round-four-correction: 39 checked artifacts for fail-closed Git validation and the parent live-scheduler refresh.
- Temporary original final log: /tmp/p1-final-gate-121638e5-round5.log.

The owned numeric worker tree /home/nathanael/.worktrees/Deadlock-Brain-p1-numeric-fix-20261012 retains its verified uncommitted patch at detached c19d5b26. Its workflow wf_dcf6aa16-7a3/task wrflxfze9/actor a35a92a1783e14421 is complete. Do not resume or duplicate it. All P1 worktrees are retained; foreign package trees/branches remain untouched.

## Production and acceptance still unexecuted

No main merge/push, production helper replacement, release plan/build/verify/install, gamefacts migration, job config/unit installation, actual ingest, answer request, production tracing, timer enablement or P1 serve restart occurred. The latest metadata-only serve preflight still observed PID 3193806, NRestarts 0, system brain-serve.service, existing P0b release ef0418d5e8b217d052d573e40afc12c88caf260a. No secret value was exported, no credential file was read and no private data was sent externally.

All P1 live acceptance remains open: actual first ingest peak strictly below 300000000 bytes under an enforced 536870912-byte cgroup-v1 cap; independently measured unchanged second ingest with zero new raw objects; genuine indexed get_hero execution below 5 ms; a fresh answered Baba Steckbrief with ordered ops.answer_trace find_entity/get_hero calls and resolvable pinned receipts; scoped no-repository/snapshot-loading evidence; readyz 200; new PID/executable/content anchor; empty scoped error-priority journal; hourly timer heartbeat.

## Continuation point

Wait for the relayed user decision. The next authorized actor must read this file and the saved gate-five log, use the existing branch/worktree and proof rather than rebuilding P1, and obey the decision's exact scope and gate-round authority. If a further correction is authorized, its smallest originating-extractor boundaries are condition representation width and duplicate normalized assertion/relation keys. Preserve fail-closed behavior, genuine transient resumes and prior fixes; no SQL widening or blanket database-error classification is currently authorized. Merge/deploy remain forbidden without an actual permitted ALLOW. ACCEPTANCE.md and MEMORY-PROOF.md contain the prepared but unexecuted live procedure.
