# docs-search-fix

Branch `fix/brain-docs-search-20261023`, base `179c41d4`.

## Root cause

1. `search.sql` only used `websearch_to_tsquery` with AND semantics. Model queries carry synonyms and
   filler words (for example `Steam account mit Discord Server verknüpfen verbinden`), so almost no
   section contained every term.
2. Broader matches ran into the 500 ms guard: `serve.doc_section` validates publication per candidate
   row (about 7.6 ms per row). Even `Steam` (131 candidates) timed out.
3. Commands are indexed as file tokens (`/checkrank`); a search without the slash found nothing.
4. The `search_docs` description did not say the docs are German or what they cover, so the model
   searched in English or not at all.

## Change

- Migration 33 `scripts/migrations/2026-10-23-brain-gamefacts-docs-search-v33.sql`:
  `serve.doc_section` keeps the v24 body and validation, but emits its key columns from
  `canon.doc_search` so key joins use the primary key. New view `serve.doc_search_hit` exposes, for
  `brain_tool` and `brain_service`, the keys, scope and both tsvectors of the newest published run per
  page (rows of unpublished or failed newer runs do not hide the published page). No body text.
- `search.sql`: the strict websearch tier stays first. Only when it has no hit and the query has no
  operators (`-`, `OR`, quotes), it falls back to sections that match at least two query words (or the
  only word), each word also matching its `/command` form. The entity filter runs on the candidates
  before any limit; full `serve.doc_section` validation runs lazily in rank order and stops after
  `limit + 1` validated sections, which also yields the exact truncation flag.
- `search_docs` tool description in `service.rs`.

## Known limit (pre-existing)

With any audit rejection or `audited` quarantine row present, `serve.published_receipt` calls
`serve.receipt_is_published` per receipt, and `serve.doc_page` inside `serve.doc_section` then costs
about 270 ms per page. A search that validates two or more pages exceeds 500 ms; the original
`search.sql` fails the same way (`Steam verknüpfen` timed out on the same data). The hit view itself
stays at about 60 ms in that mode. Fixing it needs a keyed rewrite of `serve.doc_page` or of the P9
publication check, which is outside this fix.

## Evidence

`/home/nathanael/.local/state/brain-wave2-proof-20261014/docs-search-fix/`
- Base (same build without the fix): docs 0/10 (`e1-base-docs.txt`), integrated head 1/10.
- Fix round 0: docs answered 8/10, correct 7/10, citations 8/8; unanswerable 20/22 as the base
  (`e1-fix-docs-refusal.txt`).
