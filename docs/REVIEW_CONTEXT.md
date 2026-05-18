# Review Context

`deadlock_brain.review_context.build_review_context(conn, query, limit_events=80)`
ist eine reine Kontextaufbereitung fuer Analyse- und Review-Antworten.

Die Funktion ruft keine OpenAI-/API-Dienste auf und aendert keine CLI. Sie nutzt
`retrieval.build_entity_context` als Eingang und verdichtet dessen Daten zu:

- `entity_summary`: kanonischer Match, Entity-Typ, Aliase und Metadaten-Hints
- `current_stat_hints`: aktuelle Sheet-Werte fuer Heroes, mit kompakter Auswahl
- `timeline_signals`: Patch-Events, Change-Type-Zaehler, strukturierte Stat-Aenderungen
- `open_questions`: explizite Luecken und Unsicherheiten
- `source_references`: Patch-, Sheet- und Entity-Referenzen fuer spaetere Antworten
- `prompt_de`: deutscher Prompt-Entwurf fuer eine spaetere KI-Antwort

Der Prompt weist ausdruecklich an, Namen von Items, Heroes und Abilities auf
Englisch zu behalten und Bewertung sowie Unsicherheiten auf Deutsch zu erklaeren.

Beispiel:

```python
import sqlite3

from deadlock_brain.review_context import build_review_context

conn = sqlite3.connect("data/deadlock_brain.sqlite3")
conn.row_factory = sqlite3.Row

context = build_review_context(conn, "Shiv", limit_events=80)
```

Das Ergebnis ist ein normales `dict` und kann spaeter direkt serialisiert oder in
einen Modellaufruf gegeben werden.
