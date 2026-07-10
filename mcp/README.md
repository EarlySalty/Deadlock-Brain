# dl-brain MCP-Server

Zweck: `dl-brain` stellt die zentrale Postgres-Patch-Historie aus `brain.patch_changes` als read-only MCP-Tools bereit.

## Tools

- `patch_history(entity: str, ability: str | None = None, stat: str | None = None, since: str | None = None, limit: int = 100)`
- `patch_search(text: str, limit: int = 50)`
- `list_patches(limit: int = 100)`
- `entity_summary(entity: str)`

Eine generische SQL-Luke ist bewusst entfernt; falls sie spaeter noetig wird, dann mit eigener read-only-DB-Rolle.

## Registrierung

Eintrag in `/home/naniadm/Documents/.mcp.json`:

```json
"dl-brain": {
  "command": "python3",
  "args": [
    "/home/naniadm/Documents/Deadlock-Brain/mcp/server.py"
  ]
}
```

Die Tools erscheinen erst nach Claude-Neustart.
