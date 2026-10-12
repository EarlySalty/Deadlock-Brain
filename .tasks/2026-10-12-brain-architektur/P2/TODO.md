# P2 v2 continuation

State 2026-10-12 20:15 CEST, Opus takeover of the dead Codex thread.

- Branch `feat/brain-p2-items-abilities-20261012-v2`, salvage head `5db851bb`.
- `7eace8ff`: Assets extractor with poison test, cherry-picked from worker worktree `wf_58db6e48-ccb-1`.
- `5db851bb`: typed get_item/get_ability/compare contracts and bounded card consumer from worker worktree `wf_374f9b3f-029-1`; not wired into the answer path.
- Schema worker: worktree `/home/nathanael/.worktrees/Deadlock-Brain-p2-v3-schema`, branch `wip/p2-v3-schema-20261012`; migration v3 file, `serve.item_card`, `serve.ability_card`, scratch-PG card test, EXPLAIN evidence. Merge into the P2 branch after review.

## Blocked on orchestrator

1. PG tool executor and find_entity/get_hero are not in core `4cc0cf5d`; waiting for the P1r SHA that publishes them.
2. `migrations.rs::ordered()` requires contiguous versions; v3 registration waits for P3 v2 or a core change.
3. Core 2 SHA (multi-object source, qualified facts, kv_path, RAW_BYTES, idempotence skip).

## Remaining after unblock

1. Wire the three tools through the P1r executor; parameterized SQL on the P2 serve views only.
2. Register the Assets source and add the hourly `brain-job@` timer instance.
3. Adapt the extractor to core 2; rerun the poison test on the real store.
4. Fresh full-diff reviewer, local gate after P1r is live, then integration in the order the orchestrator sets.
