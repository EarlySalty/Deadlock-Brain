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
- This needs an orchestrator decision: a per-round input cap, a larger cumulative cap, or smaller tool results.
- Effects specific to the scratch environment:
  - strace slows pool connects, so requests return unavailable after the 250 ms pool wait.
  - serve.timer lookups take about 1 s on the scratch cluster.

## Open for integrator
- Reduce the live brain-serve.json budgets to the validated range before deploy.
- P4 (patch_history) and P5 (search_docs) are not wired on this base. The docs and patch E1 categories depend on them.
