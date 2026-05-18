# Patch Event Enrichment

`deadlock_brain.patch_event_enricher` adds deterministic structured details for
existing `patch_events` rows. It does not call AI services.

## Table

`build_patch_event_enrichments(conn, rebuild=False)` creates
`patch_event_enrichments` if needed.

Columns:

- `patch_event_id`: unique reference to `patch_events.id`
- `stat_name`: parsed changed stat or relationship name
- `old_value`, `new_value`: parsed values, with shared units removed when possible
- `unit`: shared compact unit such as `%` or `s`
- `ability_name`: ability prefix parsed from lines such as
  `Serrated Knives spirit scaling reduced from 0.15 to 0.13`
- `secondary_entity_name`: referenced entity for relationship changes such as
  `Now upgrades from High-Velocity Rounds`
- `confidence`: deterministic parser confidence from `0.0` to `1.0`
- `flags_json`: JSON array with parser hints, for example `ability_prefix`,
  `non_numeric_value`, `removed_grant`, or `unparsed`

## Supported Patterns

Initial patterns cover:

- `Cooldown increased from 24s to 27s`
- `Serrated Knives spirit scaling reduced from 0.15 to 0.13`
- `Moved from T3 Item to T2`
- `Now upgrades from High-Velocity Rounds`
- `No longer grants +20 Health per stack`

Unknown lines still receive an enrichment row with `confidence=0.0` and the
`unparsed` flag, so downstream consumers can distinguish missing processing
from unsupported syntax.
