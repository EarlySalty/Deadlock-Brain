# Game Wiki Schema

Diese Wiki-Schicht folgt dem LLM-Wiki-Muster fuer Deadlock-Brain.

## Schichten

- Raw Sources: Die Quelldaten liegen in der Brain-DB (Schema `raw`, siehe `docs/ARCHITECTURE.md`). Diese Daten sind die Quelle der Wahrheit und werden hier nicht editiert.
- Wiki: Markdown unter `game-wiki/pages/`. Der Befehl `deadlock-brain wiki rebuild` ist mit P8 (2026-10-28) entfernt. Die Seiten sind eingefrorener Bestand ohne automatischen Rebuild und duerfen vom Agenten gepflegt werden.
- Schema: diese Datei. Aendere sie nur, wenn sich Struktur oder Arbeitsweise bewusst aendern.

## Regeln

- Fakten duerfen nur aus der Payload oder aus verlinkten Source-Metadaten stammen.
- Jeder Eintrag muss Provenienz behalten.
- Die vollstaendige JSON-Payload bleibt im Eintrag erhalten.
- `index.md` ist der Inhaltskatalog.
- `log.md` ist append-only.
- Bei Widerspruch gewinnt die neueste `deadlock_data`-Payload gegen aeltere Wiki- oder Creator-Aussagen.

## Query-Workflow

1. `index.md` oder die lokale Wiki-Suche lesen.
2. Relevante Eintraege vollstaendig lesen.
3. Antwort nur aus Ground-Truth, verifizierten Creator-Claims und den gelesenen Wiki-Eintraegen synthetisieren.
4. Unsichere oder fehlende Fakten offen markieren.
