# P9 continuation point

## Pause decision

Paused by the orchestrator's explicit user decision at 16:45 on 2026-10-12. P1 is stopped and its ingestion design will be reconsidered. Start no new jobs, workers, fixers, builds, tests, gate runs or deployments. Resume only with a new briefing after the redesign decision. All owned native workers and reviewers have completed; none remains active. Preserve the branch and worktrees.

Branch: `feat/brain-p9-receipt-audit-20261012`
Worktree: `/home/nathanael/.worktrees/Deadlock-Brain-p9-receipt-audit`
Latest source checkpoint before this pause note: `c9faecc3d583b5fb001d6cae56f24599de87c134`, already pushed. The commit containing this file is the final pause checkpoint.
Client: orchestrator thread `89e002a7-758e-449d-9968-eec29a019a78`.

## Preserved implementation

- Daily `brain-job audit-receipts` samples at most 200 published receipts, rereads immutable raw objects, persists four audit outcomes in ops and quarantines mismatches or missing locators. Serve filters and receipt RLS exclude quarantined evidence without deleting canon history.
- P9 migration version 6 is appended to P1's ordered registry. P9 scratch fixtures use the existing runner and expect ledger `[1,6]`. Reconcile this with the redesigned P1 contract before execution or integration.
- Raw hashing uses the existing unfiltered JSON root adapter and an approved append-only KV3 adapter reusing the existing Scanner and `kv3::parse` per root. It retains requested hashes without semantic filtering or a second scanner. Explicit unsupported upgrade tiers produce `unsupported_locator_kind` without quarantine, including the raw-content mismatch shortcut.
- The detached audit connection explicitly uses READ COMMITTED while leaving a repeatable-read pool default unchanged. The dedicated session lock and grouped writer-lock persistence remain in place.
- Dedicated system service/timer files are committed but not installed or enabled. The service captures the actual cgroup-v1 cap and peak before oneshot cleanup. No production resource acceptance has been measured.

## Verification boundaries

Independent source reviews found no concrete defect in the migration registration or bounded raw adapter. Formatting and whitespace checks passed. No compiler checks, Clippy, builds or tests ran for the latest registry/adapter state because of the explicit P1 hold.

Earlier runtime checkpoint checks executed 28 selected tests, strict Clippy and a release build successfully. The earlier isolation/raw-locator fixer executed 30 tests before its already-running job completed under the hold. Neither result proves the latest source state. Detailed evidence and review boundaries are in `AUFTRAG.md`. The completed fixer's preserved logs are at `/tmp/brain-p9-fixer-af7c40-evidence-20261012/logs/`.

No local gate ALLOW, main integration, production migration, deployment, production audit, timer activation or live time/memory proof occurred.

## Resume after new briefing

1. Read the redesign decision and fresh P1 contract. Reassess P9's receipt/raw-object relationships, publication eligibility, migration registry, serving exclusions and tier classification against that contract. Do not automatically deploy the preserved implementation.
2. Adopt only the checkpoint/integration strategy authorized by the new briefing. Current P9 includes P1 checkpoint 2 and `cfa009c8028a64cc2cb5a86808add35d83eee469`; later P1 corrections have not been adopted. Do not assume P1 or current main is live.
3. Once explicitly released, run the affected formatter, strict Clippy, build and existing relevant PostgreSQL/job checks using the shared cargo-slot policy. Execute the new root-budget, lexeme and unsupported-tier regressions, preserving real Store ingest, quarantine and repeatable-read fixture evidence. Review the exact resulting integration state independently.
4. Only if the new briefing authorizes integration and deployment, follow the local gate and deploy wrapper. Production acceptance requires a real successful system-unit audit below 120 seconds, recorded cap `536870912` bytes and peak below `200000000` bytes. Enable the timer only after that proof. Preserve branch/worktrees until the newly authorized completion point.

Owned native worktrees retained under `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/`: `agent-af7c40f0a54ee07d8`, `agent-aae3e2f308cd607ed`, `agent-a002464a9529605e1`. Their meaningful source fixes are already integrated into the P9 branch. Do not resume the completed agents for a new redesign task.
