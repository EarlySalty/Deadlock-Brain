# P2 relation and serving integration notes

## Reviewed inputs

Standalone schema checkpoint: `639a36d41863bb4db7714d6602a063c51e7490cc`.
Tool checkpoint: `7b9bd3ec1c6bce5426b44195c210bc8017d552b2`.
These checkpoints are not yet integrated into the package branch or deployed. Independent reviews are in progress; this document is not an ALLOW result.

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

The package integration must exercise this actual producer shape through the tool consumer, not just a hand-written flat array fixture. The source-level parent inspection found that the initial tool checkpoint assumes outer `facts`/`links` values are arrays in receipt validation, while the card producer wraps both in objects. Independent verification and a fresh fixer are required before accepting the coupled checkpoints. Historical absence, truncation and source conflicts must remain explicit and fail closed without guessing.

## Outstanding gate and intent evidence

- P1 ordered runner and kind checkpoint; attach only P2 migration version 3 to the real ordered list.
- Actual populated-P1 and positive item-edge/card PostgreSQL contracts; record any skipped prerequisite separately.
- Real producer-to-tool item/ability/compare execution, serialized four-by-twelve bound and indexed EXPLAIN plans.
- Independent reviews and local merge gate ALLOW on the final merged package HEAD.
- P1 live prerequisite, verified-origin/main release install, system job memory peak, unchanged second-run zero raw objects/facts, normal answer/trace acceptance and timer activation.
