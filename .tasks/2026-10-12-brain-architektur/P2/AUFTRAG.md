# P2: items, abilities, compare

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 30 min | Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities

## Contract

Implement `brain-job ingest-assets`, receipt-backed item and ability facts and relations, canon-only `serve.item_card` and `serve.ability_card`, and bounded `get_item`, `get_ability`, `compare` tools. Keep both sources' facts and receipts; serving chooses a documented source priority. Preserve P1 extraction, legacy tools and pre-retrieval. Add an hourly system timer instance through the P0b installation contract.

The binding shared contract is `../BRIEFING_WELLE2_GEMEINSAM.md`; design and package acceptance are `../ZIEL.md` and `../PAKETE.md`. Ownership is new P2 modules, one new migration, append-only shared registration, the new timer instance and this directory. No other packages, user accounts, model/provider changes or data-quality repairs.

## Starting state and routing

Branch `feat/brain-p2-items-abilities-20261012`, starting HEAD `a808f5a492b306d63a2a1a42b533dd1aeaf2d290`, clean on entry. Parent package session is the P2 integrator. Main orchestrator: `89e002a7-758e-449d-9968-eec29a019a78`. No extra T3 threads. Native workers have disjoint ownership and isolated worktrees. Questions return in this thread, never to the user.

The native harness identifies the session model as `gpt-6.1-sol[1m]`. UltraCode is confirmed enabled by the harness, and workflow `wf_35d51f91-13b` successfully launched two read-only research workers. No model switch was requested.

## Native worker ownership

| Slice | Native worker | Isolated worktree | Workflow |
| --- | --- | --- | --- |
| Tool contracts and dispatch | `af01d271d6317cd28` | `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_61813c32-183-1` | `wf_61813c32-183` |
| P2 migration and canon cards | `aca36d9eb4fdaea5f` | `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_dc2ccbb4-293-1` | `wf_dc2ccbb4-293` |
| Assets job and timer instance | `a680303e187b2f80c` | `/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wf_f37b6fbd-be8-1` | `wf_f37b6fbd-be8` |

The original workflows were stopped after messaging their implementation agents created duplicate execution contexts. The three native implementation agents listed above remain the sole writers in their isolated worktrees. Their original downstream reviews did not run. Launch independent read-only specialist reviews separately after committed handover. Workers return commits and proofs to the parent; they do not publish, gate, merge main or deploy.

P1 checkpoint `3a0740be219fb4aceb304e8020501dcd7281fbd7` was merged without rebase at package commit `07afdf15c05c05fe7857560316b425ae3247e1e6`.

## Approved relation extension

The main orchestrator approved extending the existing `canon.relation` in the new P2 migration on 2026-10-12. Replace its kind CHECK to add `component_of` and `upgrades_to`. Make slot and endpoint rules conditional: `has_ability` retains its exact required slot 1..4 and hero-to-ability rules; item edges connect items only. Extend the assertion guard without weakening receipt, source, revision, publication, typing or immutability checks. Do not modify any P1 row and do not create a separate relation table. Gate notes must show before/after constraint definitions and a populated-P1 migration proof.

The main orchestrator superseded the initial P3 kind ownership on 2026-10-12: P1 will widen its still-unapplied v1 `canon.entity.kind` to `hero`, `ability`, `item`, `timer`. No other package changes that constraint. Ordered migration versions are reserved as P1=1, P3=2, P2=3, P4=4, P5=5, P9=6. P2 keeps its migration standalone until P1 publishes the ordered `(version, sql)` list, then appends only the P2=3 entry. No independent migration framework or P2 marker replaces P1's runner.

P1's `brain-job` crate was absent from the initial base and arrived at checkpoint `3a0740be`. The P0b contract was initially readable from `origin/main:ops/brain-release/P0b.md` and enters the package branch through the ordered base merge.

Shared extractor dispatch additions in `supported`, `extract_with_identities` and `prove` are append-only. The orchestrator additionally requires a one-step shared extractor-version bump per package; the later integrator uses `origin/main` plus one, never a downgrade. Backfill must be idempotent, including zero new facts on the unchanged second run. Existing tool-count test assertions must reflect the real merged registry count, not a stale P1/P2-only count.

## Acceptance

- Live `/v1/answer`: Toxic Bullets cost/effect and Haze versus Wraith weapon damage, `ops.answer_trace` records `get_item` / `compare`, without `deadlock_data`.
- Four heroes by twelve stats returns at most 16 KB. Every displayed fact has a receipt citation and patch.
- Two unchanged ingestion runs: second inserts zero raw objects and zero facts. System cgroup v1 peak below 300 MB.
- `EXPLAIN` for the actual item and comparison SQL shows indexed lookup.
- Baseline first; affected build, formatter, linter and existing tests pass. New tests only protect receipts, output bounds and scope filtering.

Integration waits for P1 live. Merge updated base, affected checks, independent intent review, local merge gate ALLOW, push `HEAD:main`, deploy verified `origin/main` through brain-release, install/enable the timer and prove the live path.
