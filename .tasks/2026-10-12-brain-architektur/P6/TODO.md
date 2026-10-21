# P6 tools-only public answers: state 2026-10-21

Branch `feat/brain-p6-tools-only-20261015`, gate base `d6077a8e`. Migration 29 is reserved but unused.

## Done
- Removed pre-retrieval and the old tools.
- Added the K2 value gate (structured numeric claims).
- Added tools-only budget validation:
  - rounds 1..=4
  - input 1..=32000
  - tool calls 1..=8
  - tool result bytes 1..=16384
- Release and retrieval pins are now optional.
- Shortened the K2 instruction.
- The faults fixture uses the 32k production cap.
- Config fixtures follow the pinless contract.
- The tool instruction now asks for tool calls before an empty answer. It no longer names tools that are absent on this base (search_docs, patch_history).
- Affected tests are green except three ignored live probes. They load `/home/nathanael/.config/deadlock-brain/brain-serve.json`, whose budgets (20 rounds, 20M input) violate the new validation.

## Blocked: E1 against scratch DB
Proof lives in `/home/nathanael/.local/state/brain-wave3-proof-20261015/P6/` (`e1-run1.*`, `debug-subset/`, harness `e1-scratch.sh`).

- run1 (before the tools-first instruction and before the harness used the brain_service login):
  - answered 0/101
  - non-answerable compliance 18/24
  - peak RSS 12.7 MB
  - git execve 0
  - manifest/corpus/release openat 0
- With the tools-first instruction, the model calls find_entity/get_hero, then every hero and item case ends in budget_exceeded.
  - Cause: `max_input_tokens` is charged cumulatively across rounds at 1 token per byte. Round 1 plus one hero profile (up to 16384 bytes) already exceeds 32000.
- Resolved by orchestrator decision A, see below.
- Effects specific to the scratch environment:
  - strace slows pool connects, so requests return unavailable after the 250 ms pool wait.
  - serve.timer lookups take about 1 s on the scratch cluster.

## Decision A: per-round input cap
- The kernel tool loop checks `max_input_tokens` per model round, not cumulatively. The total stays bounded by at most 4 rounds. The round limit and all other budgets are unchanged, and there is no separate total-budget knob.
- New kernel tests:
  - a 4-round lookup with a 16 KB hero profile at 32000 is answered
  - a single round over 32000 (two 16 KB results) is rejected with budget_exceeded
- Updated assertions that encoded the old cumulative cap:
  - in the kernel: the round-2 context gets the full per-round cap; an unaccounted tool failure reserves a full round cap
  - in pg_tools: the real Baba/Abrams lookup answers at 32000, and an 8000 cap still rejects

## Decision a+b: single-counted evidence, byte-to-token estimate
- Evidence that a tool result already carries is listed in the prompt by id, citation and source_basis with `content_in_tool_result: true`; its content is no longer repeated. Citations and receipts are unchanged.
- Input tokens are estimated as bytes divided by `kernel.input_bytes_per_token` (brain-serve config, default 2, validated 1..=4). Kernel and provider transport both use it.
- From round 2 on, the kernel uses the provider-reported input of the previous round plus the estimated byte growth when the previous round was accounted.
- Measured default: the first run at 3 bytes per token (`e1-run3-dedupe`) still ended 52 times in budget_exceeded. The reason was the `usage_budget` check: after get_hero the provider reported about 11.7k input tokens, more than the transport reservation of payload bytes divided by 3. Dense tool JSON comes to about 2 bytes per token. At 2, hero-001 to hero-003 answer correctly (`e1-diag3r2`).
- Full E1 at 2 (`e1-run4-ratio2`): 32/101 answered, 27 correct, non-answerable 22/24 (baseline 11/101, 6 correct). Hero 15/30 answered (14 correct), item 17/27 (13 correct), docs 0/10, patch 0/16, guide 0/8, timer 0/10. budget_exceeded 3, all at the 4-round limit (item-022 comparison, patch-002, guide-007), none on single hero or item lookups. RSS 13.5 MB, 0 execve, 0 release or corpus opens.
- Unchanged: 32000 input cap per round, at most 4 rounds, at most 8 calls, at most 16384 bytes per tool result.
- serve.timer on scratch: EXPLAIN ANALYZE about 2.0 to 2.4 s (`explain-urn.txt`, `explain-soul_box.txt` in the proof directory). serve.current_fact runs DISTINCT ON over all 226989 facts with an external merge sort (42 MB on disk at work_mem 4 MB) before the timer key filter applies. The cost comes from the view structure and the data volume, not from the scratch cluster, so production with the same fact count is affected too. It exceeds the 500 ms statement timeout of the tool session, so every get_timer call ends as unavailable, and it needs a separate migration.

## Open for integrator
- Before deploy, the integrator must lower the live `/home/nathanael/.config/deadlock-brain/brain-serve.json` budgets (currently 20 rounds and 20M input) to the validated values: at most 4 rounds and at most 32000 input tokens per round. The worker did not change the live config.
- P4 (patch_history) and P5 (search_docs) are not wired on this base. The docs and patch E1 categories depend on them.
