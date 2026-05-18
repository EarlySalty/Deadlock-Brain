# Timelines

`deadlock_brain.timeline` builds deterministic entity timelines from existing
SQLite tables. It does not change the CLI and does not call AI services or
external APIs.

## API

```python
from deadlock_brain.timeline import build_entity_timeline

timeline = build_entity_timeline(conn, "Mystic Shot", limit_events=2000)
```

Signature:

```python
build_entity_timeline(
    conn: sqlite3.Connection,
    query: str,
    *,
    limit_events: int = 2000,
    ascending: bool = True,
) -> dict[str, Any]
```

The returned dictionary contains:

- `best_match`: matched row from `entities`, or `None` when fallback matching was
  used.
- `aliases`: loaded aliases for the matched entity.
- `patches`: chronological patch groups. Each patch has `patch_snapshot_id`,
  `patch_external_id`, `title`, `url`, `date`, `source`, and `events`.
- `event_count`, `patch_count`, `enrichments`, `fallback`: compact metadata.

Each event is copied from `patch_events`, decodes `metadata_json` to `metadata`,
attaches optional `enrichment`, and adds:

- `impact_kind`: one of `numeric_buff`, `numeric_nerf`, `functional_change`,
  `rework`, `bugfix`, `added`, `removed`, `unknown`.
- `impact_level`: one of `high`, `medium`, `low`, `unknown`.
- `impact_confidence`: deterministic confidence derived from enrichment or event
  confidence.
- `impact_flags`: parser flags from `patch_event_enrichments.flags_json`.

## Classification

The classifier uses only local row data:

- `change_type` is the primary signal for `added`, `removed`, `bugfix`,
  `rework`, `buff`, `nerf`, and generic `changed` rows.
- `normalized_line` is used as a deterministic keyword fallback.
- `patch_event_enrichments` improves numeric detection through parsed
  `old_value`, `new_value`, `confidence`, `stat_name`, and `flags_json`.
- Numeric buff/nerf level is based on relative value delta: at least `25%` is
  `high`, at least `10%` is `medium`, otherwise `low`.
- Reworks, additions, and removals are `high`; functional changes are `medium`;
  bugfixes are usually `low`, or `medium` for crash/exploit-related lines.

If the optional enrichment table is missing, timelines still work from
`patch_events` alone.
