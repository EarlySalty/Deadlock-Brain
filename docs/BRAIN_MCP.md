# Brain-MCP starten

Der Adapter liest seine Einstellungen ausschließlich aus einer normalen
JSON-Konfigurationsdatei. Der Brain-Server-Endpunkt, die freigegebenen Scopes
und der Timeout stehen dort. Das API-Token steht dort nie: `secret_reference`
benennt ausschließlich einen bereits in Infisical gespeicherten Secret-Eintrag.

Die Beispielkonfiguration liegt unter `config/brain-mcp.example.json`. Kopiere
sie in eine eigene Runtime-Datei und starte den Adapter mit einem expliziten
Pfad:

```sh
cp config/brain-mcp.example.json config/brain-mcp.json
./rust/target/release/brain-mcp --config config/brain-mcp.json
```

Das Beispiel verwendet den vorhandenen Eintrag `BRAIN_SERVE_API_TOKEN` und den
bereits freigegebenen Scope `docs.public`. Es legt weder ein Secret noch einen
neuen Zugriff an. Das Secret wird beim Start über die vorhandene lokale
Infisical-Anbindung geladen. Ein unbekannter Konfigurationsschlüssel wie
`api_token` wird abgewiesen; sein Wert erscheint nicht in Fehlermeldungen.

Die MCP-Anfrage kann keine Scopes setzen. Der Adapter verwendet nur die Scopes
aus der Konfigurationsdatei; `brain-serve` prüft sie zusätzlich gegen die
Berechtigungen des referenzierten Tokens.
