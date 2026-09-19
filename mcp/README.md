# dl-brain MCP Server

`dl-brain` stellt die zentrale Postgres Patch Historie als read only MCP Werkzeuge bereit. Der Server läuft vollständig als Rust stdio Prozess.

## Werkzeuge

- `patch_history_v1(query, entity?, known_at?, limit?)`
- `patch_history(entity, ability?, stat?, since?, limit?)`
- `patch_search(text, limit?)`
- `list_patches(limit?)`
- `entity_summary(entity)`

Eine generische SQL Schnittstelle ist nicht vorhanden.

## Registrierung

Der Secret Loader gibt keine Werte auf stdout aus. Er lädt die Laufzeitvariablen über den lokalen Infisical Transport und ersetzt sich anschließend durch den MCP Prozess.

```json
"dl-brain": {
  "command": "/home/naniadm/Documents/Deadlock-Brain/rust/target/release/deadlock-brain-secret-exec",
  "args": [
    "--",
    "/home/naniadm/Documents/Deadlock-Brain/rust/target/release/deadlock-brain-mcp"
  ]
}
```

Nach einem Binary Austausch muss der MCP Client eine neue Sitzung öffnen.
