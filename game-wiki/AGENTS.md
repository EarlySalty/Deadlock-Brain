# Game Wiki Schema

Diese Wiki-Schicht folgt dem LLM-Wiki-Muster fuer Deadlock-Brain.

## Schichten

- Raw Sources: `brain.source_documents` und `brain.entity_snapshots`. Diese Daten sind die Quelle der Wahrheit und werden hier nicht editiert.
- Wiki: Markdown unter `game-wiki/pages/`. Diese Seiten werden durch `deadlock-brain wiki rebuild` erzeugt und duerfen vom Agenten gepflegt werden.
- Schema: diese Datei. Aendere sie nur, wenn sich Struktur oder Arbeitsweise bewusst aendern.

## Regeln

- Fakten duerfen nur aus der Payload oder aus verlinkten Source-Metadaten stammen.
- Jede Seite muss Provenienz im Frontmatter behalten.
- Die vollstaendige JSON-Payload bleibt auf der Seite erhalten.
- `index.md` ist der Inhaltskatalog.
- `log.md` ist append-only.
- Bei Widerspruch gewinnt die neueste `deadlock_data`-Payload gegen aeltere Wiki- oder Creator-Aussagen.

## Query-Workflow

1. `index.md` oder die lokale Wiki-Suche lesen.
2. Relevante Seiten vollstaendig lesen.
3. Antwort nur aus Ground-Truth, verifizierten Creator-Claims und den gelesenen Wiki-Seiten synthetisieren.
4. Unsichere oder fehlende Fakten offen markieren.
