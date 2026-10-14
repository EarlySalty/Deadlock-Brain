# Wave-2 integration handover

## 1. User stop and authoritative state

Paused on 2026-10-14 at the user's request: write one handover covering everything still open and start no more agents. Do not resume workflows or start agents automatically. This document is the authoritative continuation point; older integration TODO entries predate the interrupted P9 correction.

The active P9 fixer workflow `w43vjknm2`, run `wf_b45b9f68-a7c`, agent `aff66956bd6fc853c`, was stopped successfully. It returned no completed fix handover. The earlier integration workflow `w47vbhgcg` had already ended after its P9 phase failed; its P4, P5 and P10 agents never started. No new agents were started for this handover.

A process-name/cwd-only check after stopping found no task-owned Cargo, rustc or PostgreSQL process in this integration worktree. The only matching Python process was the inspection itself. This is not a claim about foreign processes or every temporary directory. Two earlier nested command IDs (`b0q7ea68k`, `bc9d007lb`) were no longer registered when stopping was attempted. Their incomplete output is not a successful test result.

### Git identity

| Item | Exact state |
| --- | --- |
| Worktree | `/home/nathanael/.worktrees/Deadlock-Brain-integration-w2` |
| Branch | `integrate/brain-wave2-20261014` |
| HEAD and fetched integration remote | `b5e57d3b27badafff44c60ac6f9854a62d7c37dd` |
| Pending MERGE_HEAD | P9 `27ad636b9816013c5cb60855fc2af02d947fcd18` |
| Fetched `origin/main` | `c90dff3459e5721ad2e459a4275dca22a1c18b43` |
| Latest main change | `fix: supervise scratch PostgreSQL cluster lifetimes` |

`git merge-base --is-ancestor c90dff3459e5721ad2e459a4275dca22a1c18b43 b5e57d3b27badafff44c60ac6f9854a62d7c37dd` returned **1**. The new main commit is not included in the integration HEAD. Preserve and reconcile it before final acceptance/publication; do not overwrite main with the older integration ancestry.

P9 is a resolved but **uncommitted** merge. Its original integration changes are staged. The interrupted fix adds unstaged changes and one untracked SQL file. Nothing in that unfinished correction is claimed accepted, committed, pushed or deployed. Do not reset, clean, abort the merge or discard files without preserving this state.

### Recovery backup

`/home/nathanael/.local/state/brain-wave2-proof-20261014/USER-HANDOVER-g_1bxa1l/`

Contains:
- `pending-tracked-from-b5e57d3b.patch`: complete tracked worktree diff against HEAD, including staged merge resolutions and unstaged correction/diagnostics.
- `draft-v28.sql`: the untracked corrective migration, copied separately because Git diff excludes untracked files.
- `integration-TODO-before-handover.md`: pre-handover task note.
- `snapshot.json`: exact HEAD/MERGE_HEAD and file checksums.

The patch does not by itself recreate Git's pending merge metadata. The original worktree remains the primary continuation point. The final handover and current task note are copied into this backup too. No red source merge was committed merely to save notes.

## 2. Approved package order and what is complete

The final instructed order remains **P3, E1, P2, P9, P4, P5, P10**. Use these exact approved commits, not later branch tips.

| Package | Approved branch and SHA | Current status |
| --- | --- | --- |
| P3 | `feat/brain-p3-timers-20261012-v2`, `10502e6d7a8d6b3c8fd6e31bf0169ba29b3e6985` | Integrated and pushed. Matching check/Clippy/fmt passed; 182 tests passed, 0 failed/ignored. Actual merge `8f1d41c2`; subsequent owned notes-only `d20245f4` preserved without history rewrite. |
| E1 | `feat/brain-e1-gold-set-20261013`, `0e1aa497bf9d35db083253ba66acfb817a733b91` | Integrated and pushed through `6bc8a5cd`. Check/Clippy/fmt and 6 evaluator tests passed. No new live evaluation. |
| P2 | `feat/brain-p2-items-abilities-20261012-v2`, `9dc39a74324c5ac7f6c88f66d4c28f6510565112` | Integrated with verified repair at `b5e57d3b27badafff44c60ac6f9854a62d7c37dd`, pushed to the integration branch. Later P9 interaction is still red. |
| P9 | `feat/brain-p9-receipt-audit-20261012-v2`, `27ad636b9816013c5cb60855fc2af02d947fcd18` | Package ALLOW was on `03968113`, followed only by documentation. Merge prepared, not committed; performance correction unfinished. |
| P4 | `feat/brain-p4-patches-20261012-v2`, `35a54ce3465c02ca57bfa2b542481cee6ea2d7ab` | Approved, not attempted. Must follow green P9. |
| P5 | `feat/brain-p5-docs-20261012-v2`, `b663d2a48bc79ae2019a7517ee06fc5f7dafadd3` | ALLOW on `29ef0bd2`, documentation only afterward. Not attempted. Final architecture/acceptance boundary remains unresolved. |
| P10 | `feat/brain-p10-meta-stats-20261013`, `5e10cbf5efce409b1e641aa81fdd6c7ad7e0cc12` | Approved, not attempted. Follows P5. |

P2 repair branch/worktree remain available: `fix/brain-wave2-p2-query-20261014`, `/home/nathanael/.worktrees/Deadlock-Brain-wave2-p2-query-fix`. Its pushed commit is `b5e57d3b`; do not clean it up before final live acceptance and checked origin/main ancestry.

The P2 repair split expanded card reads into bounded fragments in one snapshot and bounded peer joins. Its final fmt/check/strict all-target Clippy passed. Complete shared serving: **11 passed, 0 failed/ignored**. Root asset helpers: **17 passed, 0 failed/ignored**. The separately reported exact public case passed once but overlaps the shared group; do not add it again as a distinct test. Evidence: `P2-fixer-final-*.json` in the proof root. No timeout, canonical contract or migration was changed by that committed P2 repair.

## 3. Immediate blocker: P9 publication performance

### Confirmed pre-correction failure

Evidence directory:
`/home/nathanael/.local/state/brain-wave2-proof-20261014/P9-adoption-094128-b10e43a4/`

Read `P9-handoff-evidence.json`, `P9-blocker.json`, `P9-linked-results.json` and `P9-remaining-results.json` before rerunning anything.

P9 integration fmt/check/strict Clippy passed, but the selected distinct tests were **66 passed, 5 failed, 0 ignored**. This is not a green package.

- Existing `brain-storage/tests/gamefacts_p2_cards.rs` failed at its real `SELECT card FROM serve.item_card WHERE entity_id=$1`, SQLSTATE **57014**, with the unchanged **500 ms** guard and existing **75,000-receipt** fixture. Both remote and local attempts failed. The same unchanged target passed before P9 in `P2-retry1-storage-integration.log`.
- Four actual shared serving tests failed: qualifier/grid executor and revalidation, real public item executor, captured hero-slot executor, and complete-response slot replay. Exact names/locations are in `P9-handoff-evidence.json`.

P9 replaces `serve.published_receipt` with global publication/quarantine/shadowing checks. Expanded nested serving plans are the measured repair area. Do not conclude that all failures are ordinary host load or a pool timeout.

### Interrupted corrective draft

The stopped fixer left these **unstaged** tracked changes:

| File | Purpose/state |
| --- | --- |
| `rust/crates/brain-storage/src/gamefacts/migrations.rs` | Registers draft version 28. |
| `rust/crates/brain-storage/src/gamefacts/migrations_tests.rs` | Adjusts registry/late-upgrade checks and adds publication-function role/visibility checks. |
| `rust/crates/brain-storage/tests/gamefacts_p2_cards.rs` | Temporary timing/EXPLAIN/catalog diagnostic hooks through `BRAIN_P9_PLAN_OUTPUT`; original assertion retained. |
| `rust/crates/brain-serve/src/pg_asset_executor_tests.rs` | Temporary failed-comparison EXPLAIN hook through `BRAIN_P9_COMPARE_PLAN_OUTPUT`; original failure still propagates. |

Untracked draft: `scripts/migrations/2026-10-14-brain-gamefacts-bounded-publication-v28.sql`.

That SQL adds receipt/peer indexes, a `STABLE STRICT SECURITY DEFINER` predicate `serve.receipt_is_published(bigint)`, restricted EXECUTE grants, and a replacement publication view. **It is an unfinished candidate, not an approved final design or applied production migration.** Review semantics and role exposure, including dynamic audited/shadowed rejection of older and late/backdated facts, aliases, relations and identities. Do not merely accept a green direct-card check as proof of the full contract. Remove temporary diagnostics once the cause is repaired, without weakening assertions.

Fixer evidence:
`/home/nathanael/.local/state/brain-wave2-proof-20261014/P9-fixer-uGTWHbii/`

Latest completed `cached-predicate-results.json` records fmt/check/strict Clippy exit 0 and **23 tests passed, 0 failed/ignored** across direct card (1), audit/roles (4), migrations (12), publication/run-lock (6). These checks precede the last comparison-diagnostic edit and are not final verification of the current tree.

`cached-predicate-serving.log` is **incomplete**, with no suite summary or final successful exit. It contains a genuine Compare failure, a passing real public Toxic Bullets result (12,111 bytes, 27 receipts), another captured-slot failure, and the start of replay. Do not invent a complete pass count or treat this as green. `compare-diagnostic-compare.log` is also incomplete; no completed comparison-plan file was available at handover.

Large natural plans exist as `*-plan.json` in that directory. Inspect targeted fields instead of dumping megabytes. Several earlier candidates failed; no root-cause repair was finally accepted or committed.

P9's already-staged parser integration repair must also be retained: P3 timers reuse the existing brain-storage parsers instead of path-included duplicate modules, with visibility adjustments and retained canonical parity/hash checks. Details are in the existing integration TODO. Do not undo it while fixing SQL.

### Migration identity

Original approved final union: `[1,2,3,4,5,6,7,8,9,10,11,13,23,26]`.

Current staged P9 union: `[1,2,3,6,7,8,9,10,11,13]`.

Current working draft adds `28`; retain that addition in later union tests only if the correction is kept. Versions 24, 25 and 27 are reservations, not automatic registrations. P4 uses 4/23, P5 uses 5, P10 uses 26. Never modify or renumber approved/applied SQL. Recheck current main and allocations before finalizing any new migration. Baseline identity evidence: `seven-package-migration-identity.json`; interrupted fixer preservation evidence: `P9-fixer-uGTWHbii/approved-migrations-preserved.json`.

## 4. Other open source and acceptance work

### P4

Merge after green P9, preserving all tools, source/extractor registries, P2 bounded queries and P9 quarantine checks. Verify appropriate compiler, formatter, Clippy and affected existing tests, then commit/push the green state.

Live setup still lacks the normal JSON configuration file `/var/lib/deadlock-brain-job/deadlock_patchnotes_db`:

```json
{
  "infisical_config": "/etc/deadlock-brain/infisical-job.json",
  "max_posts": 5000,
  "max_row_bytes": 8388608,
  "timeout_seconds": 15
}
```

These values come from the approved source handover. `PATCHNOTES_FEED_READONLY_DSN` exists by **name** in Infisical; no value was read. Verify the actual feed role/read-only access at rollout. The existing Infisical integration loads the required snapshot map. No custom connector or secret file is needed. Consecutive lowercase bullet continuation in `patch_extract.rs:368` is a documented nonblocking follow-up.

### P5: unresolved architecture decision

Authoritative detail:
`/home/nathanael/.local/state/brain-wave2-proof-20261014/p5-boundary-decision.md`.

No answer was received to the earlier explicit architecture question. Status questions, continuation instructions and automatic task notifications do not approve the proposed Docs-only canon.fact writer/trigger or a substitute read model. Do not silently drop P5 from the release either.

Remaining functional gaps in the exact approved source:
- Production public tool composition does not yet include DocsTools; scoped internal Docs retrieval is not composed into the operator path.
- Public SearchDocs creates numeric Prose evidence rather than canonical published receipt/result binding. SearchDocs is not already covered by the compiled canonical dependency/request binding rule. Keep excerpts in sections and reuse the exact canonical receipt formatter; do not relabel internal evidence public.
- The existing per-chunk simple-config fact GIN does not index complete reconstructed-section search. German stemming, titles/headings, phrase/negation and chunk boundaries cannot be repaired by a naive candidate filter. The proposed generated projection needs its unresolved writer decision. A candidate/fallback replacement is not clearly authorized either and does not prove generally indexed section FTS.

Important reuse finding: private Unix-socket **`/v1/operator/query` already exists**. `InternalApiService::new` accepts a RetrievalPort. Its internal mode requires actor `second-brain`, channel `internal`, scope `second_brain.internal`, empty provider egress, and performs local retrieval/cache revalidation without a provider call. Production currently composes ReleaseRetriever. Reuse this route if composition is approved; **no new endpoint or AI-egress exception is needed**. Preserve its deliberate public-release alias mode. Source support is not evidence that Docs is currently reachable in the deployed service.

Full read-only scout report:
`/tmp/claude-1000/-home-nathanael--worktrees-Deadlock-Brain-integration-w2/24058df1-f798-4e2f-b3b7-78b6358c349c/tasks/w9r3y4ect.output`.

### P10

Merge after P5. Verify real P2 `identity_game_id` data against GetMetaStats lookup, not only the synthetic plain-key fixture. Preserve canonical binding, current/quarantine guards and migration 26. The user chose hourly snapshots retained indefinitely; do not introduce retention deletion or fabricate new snapshots when unchanged data deduplicates.

### Current main and build infrastructure

The fetched scratch-PostgreSQL lifecycle fix `c90dff34` must be preserved during integration. Earlier verification delays also included foreign processes holding build slots and remote filesystem exhaustion. Do not stop foreign processes, delete locks or manually remove remote workspaces. These historical observations are not a claim that current infrastructure is still in the same state.

## 5. Remaining closure sequence

1. Resume only when authorized; inspect the preserved worktree and evidence first. Finish the P9 cause-level repair, remove temporary diagnostics, reconcile current main safely, and complete all affected existing card/Compare/slot/replay/audit/publication/role checks without relaxing the 500 ms guards. Commit/push only a verified green P9 state.
2. Integrate exact P4, P5 and P10 in order, preserving the corrective migration if accepted. Run matching check/Clippy/fmt/tests after each package, and push green intermediate states. Resolve P5's genuine boundary before claiming combined acceptance.
3. Run one final appropriate combined verification and one independent acceptance review against the actual user goal and all package contracts. Run local gate with literal absolute paths: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --base origin/main --head HEAD` from this integration worktree. Review/gate evidence applies to the exact final SHA. On BLOCK, fix causes with fresh context only when agents are authorized again; the user's gate policy has **no round limit**. Never bypass hooks.
4. Only the responsible integrator publishes verified main, using `git push origin HEAD:main`, then performs **one** brain-release build/verify/install through the existing wrapper and one common live acceptance. Preserve newer main changes. Do not operate in the foreign canonical checkout, rebase, force-push, or build a release per package.
5. Run E1 exactly once against the final live service, compare its unchanged gold set with the saved baseline, then perform checked-ancestry branch/worktree cleanup only after live acceptance.

No combined final run, final acceptance critic, final gate, wave-2 main publication, release build, deploy, production ingest, timer activation or final E1 live run has happened.

### Release and runtime evidence

Existing root wrapper: `/usr/local/libexec/brain-release`, commands `plan SOURCE`, `build SOURCE NEW_BUNDLE`, `verify SOURCE BUNDLE`, and `sudo -n /usr/local/libexec/brain-release install SOURCE BUNDLE`. Its install path runs the checked migrator before switching the release and restarting affected app services. Generic job templates do not automatically enable the package timers. Do not needlessly rebuild the already-corrected root helper. Final inventory must include brain-job, brain-migrate and brain-eval; do not assume an old fixed binary count.

Prepared but **not executed against production**:
- `/home/nathanael/.local/state/brain-wave2-proof-20261014/verify-wave2-runtime.py` (syntax checked only).
- `live-procedure.md` and `final-packages-live-procedure.md` in the same directory.

Refresh actual service names, ports, PIDs, release pointer and baseline before deployment. Verify exact executable Git SHA separately from knowledge-release fields, changed PIDs, correct `/proc/PID/exe` without deleted, manifest/artifact hashes, enforced cgroup limits, real endpoint content/type, and journal `-p err` since install. The old running P1r SHA/PIDs are historical observations, not current live proof.

### Outstanding live acceptance by package

| Package | Required live evidence, all still open |
| --- | --- |
| P3 | Real gametracking ingest and unchanged replay; GetTimer through `/v1/answer`, answer_trace and resolvable canonical receipts; real job cgroup peak below 300 MB; correct numbers/units/conditions and no invented Soul Urn first-spawn time. Do not seed benchmark aliases or present private fixture cardinalities as production facts. |
| P2 | Toxic Bullets cost/effects, Haze/Wraith weapon comparison, positive hero+slot-4 ultimate for both; actual answer/tool traces without deadlock_data fallback and resolvable receipts; real Assets cgroup below 300 MB; controlled Assets timer activation per package notes. |
| P9 | Final released audit binary and specific audit-receipts service/timer installed; real audit under 120 seconds, cgroup-v1 cap 536870912 bytes and peak below 200000000; serving quarantine demonstrated safely without poisoning healthy production data. Timer `OnCalendar=*-*-* 03:17:00`, randomized delay up to 10 minutes, Persistent=yes; service uses `audit-receipts`, not generic ingest. Its two bare numeric hook outputs need correct interpretation. Private released-binary negative probe is described in final-packages-live-procedure.md. |
| P4 | Actual patch_history with FindEntity/GetHero, cold/warm numeric traces and canonical receipts; real read-only feed ingest and unchanged replay; safe private oversized-skip/count/later-post continuation proof; publication/quarantine revalidation and historical_patch_unavailable; whole ToolExecution below 16,384 bytes; five-minute patch timer, not the generic hourly schedule. |
| P5 | Real corpus ingest, poison isolation and unchanged replay without raw/canon growth; retain the known 16 invalid_docs instead of silently repairing unrelated content; exclude doc_page from FindEntity; reachable canonically bound public SearchDocs and locally reachable scoped internal retrieval with enforced egress denial; natural real query plans and independent simple/German phrase semantics; actual process-tree/cgroup peak below 300 MB. Discord remains unchanged until separately approved cutover. |
| P10 | Live-only capture of both `/v1/analytics/hero-stats` and `/v1/analytics/item-stats`, normal/no_bucket, items min_matches=20 and existing time windows; JSON content type and 1 MiB/body cap; two ingests in the same UTC hour with ignored=2 and zero raw growth; observe the next real UTC hour separately without clock manipulation; real P2 identity lookup, GetMetaStats as brain_tool and `/v1/answer`, newest published canonical receipt/hour; then enable `brain-job@deadlock-api-meta.timer`. Unchanged snapshots may retain their last-change timestamp. |

### E1 final comparison

Gold schema `brain.e1-gold.v1`, **125 cases: 101 answerable, 24 non-answerable**. Do not change gold expectations to fit observed responses. Baseline is one live run plus offline re-scoring, not two live runs:

| Metric | Saved baseline |
| --- | --- |
| Answered answerable | 11/101, 10.9% |
| Correct answerable | 6/101, 5.9% |
| Precision | 6/11, 54.5% |
| Citation | 11/11, 100% |
| Non-answerable compliance | 20/24, 83.3% |
| Transport errors | 0 |

Status totals: answered 13, insufficient_evidence 84, unauthorized_evidence 18, unavailable 10. Baseline files: `E1-baseline-20261013.json` and `.txt` in the proof root, and the approved E1 task directory. Use the existing runner with the token only in its immediately needed environment variable through Infisical, never argv or a file. No `--limit`, no second live run, no invented numeric improvement threshold.

### Data, operational safety and cleanup

Initial pre-wave-2 DB aggregates were migrations `[1,7,8,9,10]`, 2 raw objects, 1691 facts, 1959 receipts and 1959 published receipts. These are historical baselines. Existing DBA peer metadata path was `sudo -n -u deadlock-brain-pg psql -X -h /run/deadlock-brain-postgresql -p 5446 -d brain`; verify current details before use and output only safe aggregates/metadata. Never put DSNs in argv or PGDATABASE, or print unredacted connection errors.

The PostgreSQL backup service was already failed before this wave. It remains an out-of-scope follow-up, not a proven new regression. P4's bulletin NIT likewise remains open but nonblocking.

After actual live acceptance, check `git merge-base --is-ancestor <branch-sha> origin/main` and its exit code for each package/fix branch before deleting anything. Preserve foreign, pre-existing or unmerged branches/worktrees. No cleanup has occurred.

Production Rust only; normal checks through `/home/nathanael/.local/bin/cargo-slot +1.97.1`, `--locked --jobs 3`, `CARGO_REMOTE_JOBS=3`, shared cache/artifact rules. Preserve required test flags `--include-ignored --test-threads=1 --nocapture`. No competing release, code comments, production unwrap, new secret/token files, direct canon/raw grants to tools, external AI transfer of internal data, unapproved providers/models/budgets, browser fallback other than Moli, or changes to ai-coach. Never start brain-maintenance or dl-knowledge as part of this handover. Current user instruction remains: **no new agents**.
