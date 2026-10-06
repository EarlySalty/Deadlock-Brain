# dl-brain MCP-Server

`dl-brain` stellt die zentrale Postgres-Patch-Historie aus `brain.patch_changes` als schreibgeschützte MCP-Werkzeuge bereit. Der Rust-Server nutzt stdio und liest `DEADLOCK_CENTRAL_DSN` aus seiner Laufzeitumgebung.

## Werkzeuge

- `patch_history(entity, ability?, stat?, since?, limit?)`
- `patch_search(text, limit?)`
- `list_patches(limit?)`
- `entity_summary(entity)`

Tool-Abfragen verwenden gebundene Parameter. Der Server setzt `statement_timeout` auf acht Sekunden und aktiviert den Read-only-Modus je Verbindung.

## Bauen

Im Verzeichnis `rust/`:

```sh
cargo build --release -p deadlock-brain --bin deadlock-brain-mcp --bin deadlock-brain-secret-exec
```

## Registrierung

Der lokale MCP-Client muss `deadlock-brain-secret-exec` mit `deadlock-brain-mcp` als Zielprogramm starten. Ersetze `<repo>` durch den absoluten Repository-Pfad:

```json
"dl-brain": {
  "command": "<repo>/rust/target/release/deadlock-brain-secret-exec",
  "args": [
    "--",
    "<repo>/rust/target/release/deadlock-brain-mcp"
  ]
}
```

Nach einem Binary-Austausch muss der MCP-Client eine neue Sitzung öffnen.
