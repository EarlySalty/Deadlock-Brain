# M1 handoff: entity-profile activation verification

Stopped by user decision on 2026-10-12 00:20. brain-maintenance and dl-knowledge are off until the Brain architecture rewrite. This file is input for that rewrite.

## State of this branch

- Branch `fix/brain-m1-activation-batch-20261011`, based on origin/main 51c460c5. Not merged, not released.
- The last commit (v2) was never compiled or tested. The previous version (cached verifier) passed clippy and the brain-storage and brain-maintenance tests; v1 projection compiled and ran in the probe.
- Live on main from earlier M1 work: stale-receipt fix 51c460c5 (`same_derivation_inputs`). 286 stale docs re-derived in the first tick on it.

## What blocks activation

`brain-candidate-activate` verifies every `git-game-facts-derived` document in the candidate (379 docs for entity-profiles-0071c396...). The old timeout was `180 s + 3 * health_timeout_ms` = 270 s, so the child was killed every time ("Prozesszeit überschritten") and the tick ended with ENTITY_PROFILE_REFRESH_FAILED.

## Per-document cost (measured read-only on the real 379-doc candidate)

| Variant | Per doc | Projected 379 docs | Peak RSS | Failures |
|---|---|---|---|---|
| Old code on main | ~90 s (2 full original snapshots per doc) | hours | live brain-maintain 3.43 GB | n/a |
| Cached snapshot + shared parse cache | ~9.5 s warm, 26 to 58 s cold | ~65 min | 2.59 GB at doc 3, 3.43 GB at doc 34 | 0 of 34 |
| v1 projection of all facts | ~5 s warm, 100 s first doc | ~32 min | 4.94 GB | 0 of 8 |
| v2 bound-fact projection | not measured | | | |

Candidate snapshot load alone: 25 to 31 s and about 1.1 GB RSS.

## Where time and memory go

- `store.snapshot(release)` loads every pinned record including multi-MB vdata. About 1.1 GB per entity-profiles release.
- `CorpusSnapshot::authorized()` validates and **clones all records**. In the activate child, `visible = candidate.authorized(...)` keeps a second full copy of the candidate for the whole run and is only used for `contains`. The child also holds the base snapshot, so it carries three copies of about 1.1 GB each before any document is checked.
- `derive_git_profile` (independent path) per doc: `authorized()` on the original snapshot, `ReleaseReadManifest::from_snapshot`, then sha256 and a full JSON parse (`EntityDocumentContext::new`) of every pinned multi-MB record. All of these are pure functions of the immutable original release.
- The old verifier also ran the producer `BatchDeriver::derive` per doc only to obtain the bindings. That brings a 64 MB blob cache, a 256 MB parse cache, and two `entity_derivation_inputs` calls per doc.
- Projecting every fact of every game-file record (v1) is worse than the snapshot: each `EntityProfileFact` clones origin and document metadata.

## Facts for the rewrite

- Bindings for a document can be read directly from Postgres via `PgStore::entity_derivation_inputs(original_release_id, operator, entity_key)`. It reads the release JSON, the `read_header_json` of the pinned revisions, and the `brain.entity_profile_facts_v1` rows with their semantic projections. Producer derivation is not needed for this.
- The set of original facts that any binding references can be read in one query per original release: `SELECT source_id, logical_id, revision, fact_id FROM brain.entity_profile_facts_v1 f WHERE ($1::jsonb->f.source_id->>f.logical_id)::bigint = f.revision` with `$1` = the release's `source_revisions`.
- Receipts live in `brain.entity_derived_receipts_v1` (`receipt_json`, keyed by derived source, logical id and revision) and name the `original_release_id`. Live data had only two distinct original releases across 379 docs.
- Independent verification only needs, per document: the bound records (hash check, header without content and facts, the bound facts), the referenced git blobs, the patch story and the stored receipt. None of this needs the full snapshot or the full vdata tree.
- Separating `project_original_release` (release to verified projection) from `derive_git_profile_projected` (per-doc work that consumes only the projection) is the shape that can move into Postgres: store the projection rows once per original release and stream them per document.
- Activation timeout must scale with document count: the branch uses `270 s + 5 s * git documents`. With ~5 s/doc measured, 10 s/doc would be needed for a real margin.
- Memory: the host OOM-killed brain-maintenance on 2026-10-12 around 00:14 with MemoryMax=infinity and about 12 concurrent cargo jobs. Any rewrite needs a hard memory limit on the unit and a streaming design under roughly 500 MB.
