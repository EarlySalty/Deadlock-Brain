# Entity Lineage

`deadlock_brain.lineage` builds deterministic rename and rework relationships
from existing `patch_events`.

## API

```python
from deadlock_brain.lineage import build_entity_lineage, lineage_lookup_names

summary = build_entity_lineage(conn, rebuild=True)
names = lineage_lookup_names(conn, "Stalker")
```

The module creates `entity_lineage` with one row per detected relationship.

Supported relationship types:

- `rename`: old name to new name, for example `Backstabber -> Stalker`.
- `replaced_by`: a patch line explicitly replaces one concept with another.
- `rework`: a patch line indicates functional rework or major reshaping.

Each row stores the source patch event, source/target names, optional owner
entity such as a hero for ability renames, confidence, and compact patch
metadata.

## CLI

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli enrich lineage --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli lineage Stalker --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli timeline Stalker --descending --pretty
```

`context`, `timeline`, and `review` use lineage names as extra lookup names.
That means a current item such as `Stalker` can include old `Backstabber`
patch events, and querying `Backstabber` still reaches the same historical
chain even though the current Deadlock API no longer exposes that old name.
