# Legacy Entities

`deadlock_brain.legacy_entities` models old or removed names found in
`patch_events` that are absent from current API-backed `entities` and not already
explained by `entity_lineage`.

## CLI

```bash
./rust/target/release/deadlock-brain enrich legacy-entities --rebuild
./rust/target/release/deadlock-brain legacy --pretty
./rust/target/release/deadlock-brain timeline "Runed Gauntlet" --pretty
```

The builder creates `legacy_entities` with:

- `legacy_type`, for example `legacy_item` or `legacy_hero`
- `canonical_name` and normalized name
- first/last patch event references
- event count
- confidence
- status
- compact source samples

Suspicious parser subjects are retained with `status='suspect_parser_subject'`
instead of being treated as clean historical entities.
