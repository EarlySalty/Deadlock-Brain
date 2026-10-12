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

## Current integration boundary

User directive 08:12 on 2026-10-12 forbids new cargo-slot jobs until P1 is live and the orchestrator explicitly releases slots. Both fresh fixers started no compiler jobs. Do not ask P1 for more interfaces or merge original defective slices independently to satisfy the stop hook.

- Coupled producer/consumer code is saved at `2fd36868`; component proofs and limitations remain in `CHECKPOINTS.md`. New source-only package merge `c0670afb8c7666abebc561f104cd0ed04ae02c35` incorporates ordered-registration handover `2bf240d88c5e0e50c0ba0c3aad2b07e8c01e8582`, published P1 checkpoint `3ef22729188f435f070e6ef8182a90a28df6659c`, Assets replay/prover fix `192c2e4e5c488982f51b90f287272b9573631e95` and semantic selection fix `37ea7c37834963bb2ab45b8e75fde5f35cb6cb09`. All four ancestry checks and the merge diff check passed. This is feature-branch source integration, not combined acceptance, gate approval, P1 live proof or deployment.
- The actual P1 runner is adopted unchanged, with only P2 migration 3 appended to its list `[1,3]`. P1 v1 SQL and runner tests, plus the selection SQL, are byte-identical to their published checkpoints. Registration handover changes four owned files; it prepares populated-P1 runner/idempotence, mandatory item-edge/card, GetItem producer/consumer and lexical item-ingestion/repeat contracts, removing item-kind simulation and prerequisite skips. These bodies and all new build, clippy, PostgreSQL and runtime checks remain unexecuted. The shared extractor stays at inherited `p1-v2` until reconciliation with the actual final main state.
- Formatting-only correction `4a9dc01d8350474fcb1fb232c516f5d5ee1ee1b1` is incorporated at parent merge `b696daec77aa6bf0fba61ea5949c24af5e8bb8b1`. Parent inspected its single-file diff: whitespace and 39 optional trailing commas, with SQL/runtime code unchanged. The worker reports 647 unchanged literals and remaining tokens; the parent's supplementary scripted comparison was hook-blocked before execution and supplies no additional proof. Parent pinned workspace `cargo fmt --all -- --check` actually exited 0, log `/tmp/brain-p2-integrated-fmt.XPbfne6B.log`. Independent source-only contract research at `2bf240d8` retained no static contradiction in replay/prover scope, not an executable result. The specialized Rust reviewer stopped before any command because its mandatory Cargo phase conflicts with the hold; formal Rust review remains deferred.
- Adversarial reading-only review found a PLAUSIBLE composite/window predicate-pushdown barrier in the selection candidate. It did not prove scan count or a timeout. Deferred actual compare EXPLAIN must test entity/stat filtering through both windows; a confirmed defect requires a fresh scalar-projection fix preserving all columns and ambiguity rules.
- After explicit slot release and P1 live proof: reconcile actual `origin/main`, verify the coherent candidates, actual registry count and monotone shared extractor version, then independent intent review/gate/live acceptance/ancestry-safe cleanup. Verify essential evidence against combined Gamefiles plus EN/DE cards, not English-only synthetic projection sizes. Public EN/DE source bytes are now prepared below, but the fixture-backed extraction/proof contract has not run.

Installed cargo-slot currently rewrites non-deploy verification commands to incremental `verify`, despite requested release flags. This was read from the installed wrapper, not executed or bypassed; record actual log profiles and leave true release artifacts to the deploy wrapper. Detailed frozen checkpoint notes remain recoverable from commit `6dcc2b91`; no current TODO is kept because `ABLAUF.md` reserves that file for actual quota exhaustion.

Public fixture preparation uses `/tmp/brain-p2-public-pair.5gz51bxr/{items-en.json,items-de.json}`. Both bounded identity-encoding downloads returned application/json, 6,157,215 and 6,311,063 bytes. The actual `/v1/assets/steam-info` endpoint reported client version 6774 before and after. EN SHA256: `31c96630cba83b9fdbbedf62bb76a70cc3334667f05d2694c58d1d73791fe596`; DE: `a15e62045e4a592db62a63b8c854059c0ad046172f1ad02e91334db4a010d7b1`. Reading both public records confirms Toxic Bullets cost 3200 and quoted property values `4`, `1.9`, `-35`. This is source-data preparation, not ingestion, receipts, PostgreSQL or live proof. Initial preparation in the separate directory `KmZx38Br` used an incorrect Steam route returning 404; it is not the version-bracketed fixture pair.

The deferred local gate entry is now located and read, without running it:

```sh
/home/nathanael/.venvs/gpt-workers/bin/python /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/Deadlock-Brain-p2-items-abilities --base main --head HEAD
```

The legacy hook names are symlinks to this same script. No ALLOW/BLOCK result exists; the gate waits for the real combined checks and acceptance prerequisites.
