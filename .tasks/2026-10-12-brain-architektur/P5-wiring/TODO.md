# P5 wiring, docs search statement budget

Branch `feat/brain-p5-wiring-20261015`, gate base `d6077a8e`, migration 24 (owned, unapplied).

## Cause of the remaining 500 ms overruns

Measured with EXPLAIN (ANALYZE) inside the real reader transaction on ScratchPg. With any P9 audit present, `serve.published_receipt` calls `serve.receipt_is_published` per receipt. Restricted join collapsing (`join_collapse_limit`/`from_collapse_limit` 1 or 4) prevents the planner from pushing the receipt id into the nested publication views, so the per-section lateral of `serve.doc_section` recomputes all published receipts for every matched row. Same tests, same load band: limit 4 gave execution 110..403 ms (statement total up to 460 ms, both tests failed); planner defaults gave execution under 50 ms and statement totals of at most 107 ms.

## Fix

The Docs read transaction keeps read-only, `brain_tool`, `statement_timeout='500ms'` and adds `SET LOCAL jit=off` plus explicit `join_collapse_limit=8` and `from_collapse_limit=8`. The 500 ms limit is unchanged and no index is forced. The GIN proof test runs `VACUUM (ANALYZE)` on the owned table and uses the production transaction constant.

## Reconciliation with the integrator pool settings

The integration worktree (uncommitted) sets `jit=off`, `join_collapse_limit=4` and `from_collapse_limit=4` per session in `pg_tools::tool_pool`, which Docs shares. `jit=off` is identical. The collapse limit of 4 is measurably harmful for Docs. Since the Docs settings are `SET LOCAL`, they revert at commit and the integrator's `before_acquire` session assertion (expects 4) stays valid. Recommendation: keep the pool-level values for the game tools and keep the Docs `SET LOCAL` overrides; no merge conflict, both files are touched in different places.

## Evidence

`/home/nathanael/.local/state/brain-wave3-proof-20261015/P5-wiring/`: test-pg_docs-1.log (limit 4, 27/2), test-pg_docs-2.log (final, 29/0, 400 docs GIN execution 12.7 ms, planning 38.0 ms), check.log, clippy.log, lib and registration logs, gate logs.
