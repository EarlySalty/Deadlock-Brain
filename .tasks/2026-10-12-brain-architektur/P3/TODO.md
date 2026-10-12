# P3 pending dependency and verification

Status: P1 checkpoint 2, 3ef22729188f435f070e6ef8182a90a28df6659c, merged without rebase in 9f3766af5a2e8681c78855098a533ac4eb158286. P3's reserved version 2 is appended to the published registry. Verification is blocked on the orchestrator's explicit release of build slots after P1 is live. The 08:12 CEST directive prohibits new cargo-slot jobs. No P3 build job is running. The orchestrator explicitly confirmed preserving this branch/worktree, without main merge or discard; a WIP checkpoint is authorized.

Published interface-only checkpoint: 3bf1b2e28f6c53b7be89932f68264d3a03a28474, pushed to origin/feat/brain-p3-timers-20261012. The isolated contract check passed 108 tests with zero ignored and the production consumer build passed. It is not a runtime or live acceptance proof.

Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012
Evidence and decisions: AUFTRAG.md in this directory.

## Resume without rebuilding prior work

1. P1 checkpoint 2 integration is complete. The registry contains versions 1 and 2; P3's SQL remains timer-specific. Private direct-SQL fixtures now stamp their installed versions, matching P1's runner-owned ledger convention. Later P1 corrections arrive through main after P1 merges; do not chase later P1 branch commits or request further interfaces.
2. Keep all Cargo jobs held until the orchestrator explicitly frees the slots. Non-build source, SQL and notes work is allowed. Existing P3 runtime files are restored and byte-verified; the uniquely identified own stash was removed only after comparison.
3. Once slots are released, run the corrected real-PG indexed-key test, the previously unreached storage checks and the explicit public Store proof with both public source checkouts. Measure actual scoped cgroup limit/peak separately from compiler memory. Recheck matching formatter, strict lint, build and existing tests on the integrated state.
4. Require P1's old ingestion runs to be completed and published before activating the incremented extractor. Integrate the current main state without downgrading the extractor version and recalculate the tool-definition count. Gate the verified implementation and hand its branch to the responsible integrator. No P3 main merge or deploy before P1 live and gate ALLOW.
5. Prove live get_timer with a genuinely sourced timer, raw-object receipt resolution, no fabricated urn first-spawn value, ordered trace without legacy asset loading, production indexed EXPLAIN and actual job/serve memory. Cleanup only after responsible integration and live acceptance.

The first edited-target run had one P3 plan-fixture failure; its correction has not run. No full green P3 PostgreSQL verification, gate ALLOW, main merge, deploy or live proof is claimed. Preserve unfinished files rather than discard or commit them as verified implementation merely to satisfy the stop hook.
