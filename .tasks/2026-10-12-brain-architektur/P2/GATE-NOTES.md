# P2 relation and serving integration notes

## Reviewed inputs

Original standalone schema checkpoint: `639a36d41863bb4db7714d6602a063c51e7490cc`.
Original tool checkpoint: `7b9bd3ec1c6bce5426b44195c210bc8017d552b2`.
Corrected producer `19f5876b` and consumer `a1edfdf2` are jointly incorporated at package merge `2fd36868bb8feeb8e41aeab721233513de60caac`. This package checkpoint is pushed, not main-merged or deployed. Original reviews found defects; essential-card retention and wrapped-card/selected-slot corrections have component proofs recorded in `CHECKPOINTS.md`. Qualified-condition source selection and Assets backfill/prover fixes remain open. This document is not an ALLOW result. All new compiler jobs are frozen by user directive 08:12 until explicit orchestrator release after P1 live.

## Relation constraints before and after

P1 defines:

```sql
kind text NOT NULL CHECK (kind = 'has_ability')
slot smallint NOT NULL CHECK (slot BETWEEN 1 AND 4)
```

Its assertion guard allows only `hero -> ability`. Its active-slot unique index is `(source_id, from_entity_id, kind, slot) WHERE valid_to_revision_id IS NULL`.

The P2 migration replaces only the named kind/slot checks and drops the column-wide slot NOT NULL constraint:

```sql
CHECK (kind IN ('has_ability', 'component_of', 'upgrades_to'))
CHECK (
    (kind = 'has_ability' AND slot IS NOT NULL AND slot BETWEEN 1 AND 4)
    OR (kind IN ('component_of', 'upgrades_to') AND slot IS NULL)
)
CHECK (kind = 'has_ability' OR from_entity_id <> to_entity_id)
```

The guard preserves `hero -> ability` for `has_ability`, and requires two distinct `item` endpoints for `component_of` and `upgrades_to`. The existing active-slot index is unchanged. A new active-item-edge unique index covers `(source_id, from_entity_id, kind, to_entity_id)` only for open item edges. No P1 assertion row is updated by the migration.

The guard retains the existing open-run/planned-path, receipt-path, revision/source and fact-value-type checks. Receipt foreign keys, immutable-column privileges and write-lock triggers are unchanged. The migration does not alter `canon.entity.kind`; its widening is owned by P1.

## Actual serving interface

Both materialized cards expose `(entity_id bigint, key text, card jsonb)` and unique indexes on `entity_id`. The committed producer uses:

```json
{
  "identity": {"entity_id": 1, "citation": "1", "patch": null, "source_revision": "revision"},
  "facts": {"facts": [], "omitted_count": 0, "truncated": false},
  "links": {"links": [], "omitted_count": 0, "truncated": false}
}
```

Source selection is per entity/stat/condition. Non-assets numeric/JSON facts precede `brain-assets`; Assets text precedes non-assets text, with deterministic source/revision/path/receipt tie-breaks. Selected hero-slot relations prefer non-assets sources. Original facts and receipts remain stored.

The package now uses the actual producer shape through the corrected tool consumer, with a real PostgreSQL ability/slot/receipt-revalidation component proof. The independently confirmed original mismatch rejected the wrapped `facts`/`links` objects, and original slot queries bypassed `selected_relation_p2`; both are fixed. Essential evidence now precedes defaults and redundant metadata under measured serialized-byte bounds rather than a fixed 12-row prefix. Positive item producer-to-tool execution still awaits the actual P1 item-kind checkpoint. Historical absence, truncation and source conflicts remain explicit and fail closed without guessing. Provenance-only differences in equivalent qualified conditions are under a separate fresh serving fix, without merging genuine semantic conditions or rewriting stored facts.

## Outstanding gate and intent evidence

- P1 ordered runner and kind checkpoint; attach only P2 migration version 3 to the real ordered list.
- Actual populated-P1 and positive item-edge/card PostgreSQL contracts; record any skipped prerequisite separately.
- Real producer-to-tool item/ability/compare execution, serialized four-by-twelve bound and indexed EXPLAIN plans.
- Independent reviews and local merge gate ALLOW on the final merged package HEAD.
- P1 live prerequisite, verified-origin/main release install, system job memory peak, unchanged second-run zero raw objects/facts, normal answer/trace acceptance and timer activation.
