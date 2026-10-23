# Docs page publication check (fix/brain-docpage-publication-20261023)

## Defect
Once any audited quarantine or audit rejection exists, a docs search that validates two or more pages exceeded the 500 ms statement guard. Under `join_collapse_limit=8`/`from_collapse_limit=8`, the `serve.published_receipt` joins in `serve.doc_page`, `serve.doc_section`, `serve.doc_search_hit` and the `found` step of `search.sql` were planned without outer keys. The view then ran `serve.receipt_is_published` for every docs receipt on every page loop.

## Fix
- Migration 34 (`scripts/migrations/2026-10-23-brain-gamefacts-keyed-publication-v34.sql`): `doc_page`, `doc_section` and `doc_search_hit` probe `serve.published_receipt` by key through `OFFSET 0` laterals and subqueries.
- `serve.published_receipt` and `serve.receipt_is_published` are unchanged, so the P9 semantics and every gamefacts and patch tool view stay as they were.
- `rust/crates/brain-serve/src/pg_docs/search.sql`: the final publication join becomes a keyed lateral.
- Guard and P5 transaction settings are unchanged.

## Rejected variants
- Inlining the bad-peer check into `serve.published_receipt` doubled planning time, and reader tests hit the 500 ms wall budget.
- A single-statement SQL `receipt_is_published` was slower than the plpgsql original once the views were keyed (search 112-144 ms against 80-90 ms). It also slowed `p2_hero_slot` from about 35 ms to 60 ms.

## Proof
Proof directory: `/home/nathanael/.local/state/brain-wave2-proof-20261014/docpage-fix/`.

Real-data scratch DB with the audit seed (5000 audit rows, 1000 rejections, 999 audited and 3 shadowed quarantines):
- `Steam verknüpfen`, EXPLAIN ANALYZE: 2080 ms before (`explain-before-steam-verknuepfen.txt`), 56 ms execution and 31 ms planning after (`explain-after-steam-verknuepfen.txt`).
- Warm search timings after (`search-timing-after.txt`): Steam verknüpfen 61-117 ms, Steam 61-68 ms, Lane Rotation Heilung 37-50 ms, Baba bewegt 5-8 ms.
- Tool views (`tools-timing-after.txt`) match the base function measurements.
- The EXACT alias lookup times out at 20 s under this seed with the base function as well (see open items below).

Regression test `audited_publication_keeps_multi_page_docs_search_within_statement_budget` (5002 audit rows, 1001 rejections, 1002 audited quarantines, shadowed rows):
- Base hits the statement timeout.
- The fix runs in 45 ms execution and 31 ms planning.
- Rejected, audited and shadowed pages stay hidden, and internal pages are unreachable from the public scope.

## Open: E1 docs and refusal rerun
- State: blocked by provider quota. The run `e1-fix-docs-refusal-audited` reached the audit-seeded scratch DB but all 32 cases returned `unavailable`.
- Cause: the local router at `127.0.0.1:18769` answered HTTP 429 `The usage limit has been reached`. Its reset is announced for 2026-11-17.
- The serve log shows no docs time limit hit and the Postgres error log is empty.
- To continue, once the quota is back run from the worktree:
  `P6_E1_NOSTRACE=1 DOCPAGE_SEED_SQL=<proof>/seed-audit.sql <proof>/e1-scratch.sh rust/target/verify <proof> <proof>/../docs-search-fix/gold-docs-refusal.json fix-docs-refusal-audited`
- Targets: docs at least 7/10, refusals 20/22.

## Open: EXACT alias lookup under audit rows
The EXACT alias lookup from `pg_tools` needs 20 s or more under the audit seed, with both the base and the fixed state. It runs `serve.receipt_is_published` for every alias and identity row. It needs its own task in the gamefacts tool path.
