# dl-brain MCP-Server

Zweck: `dl-brain` stellt die zentrale Postgres-Patchhistorie aus `brain.patch_changes` als schreibgeschützte MCP-Tools bereit.

## Tools

- `patch_history(entity, ability?, stat?, since?, limit?)`
- `patch_search(text, limit?)`
- `list_patches(limit?)`
- `entity_summary(entity)`
- `change_lookup(text, entity?, since?, until?, limit?, offset?)`
- `video_evidence(video_id, text?, limit?)`
- `patch_insight(patch_url)`

`change_lookup` löst bekannte Entity-Aliase auf und behandelt Suchzeichen wörtlich. `video_evidence` liefert Caption-Text mit Zeitmarken, keine Bildbelege. `patch_insight` trennt gespeicherte Hypothesen von belegten Patchänderungen.

## Rust-Server

Der neue MCP-Server liegt als Rust-Binary `deadlock-brain-mcp` im Quellcode. Er ist für den vorhandenen schreibgeschützten Postgres-Pool und MCP über stdin/stdout vorgesehen. Build- und Laufzeitnachweis sind offen. Die lokale MCP-Registrierung wird durch diesen Auftrag nicht geändert. Für die Nutzung muss sie nach erfolgreichem Build autorisiert auf dieses Binary zeigen.

Die Tools erscheinen nach Neustart des MCP-Clients.
