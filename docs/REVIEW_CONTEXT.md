# Review Context

`deadlock-brain review <query>` ist eine reine Kontextaufbereitung fuer Analyse-
und Review-Antworten.

Der Befehl ruft keine OpenAI-/API-Dienste auf. Er liest aus der zentralen
Postgres und verdichtet die Brain-Daten zu:

- `entity_summary`: kanonischer Match, Entity-Typ, Aliase und Metadaten-Hints
- `current_stat_hints`: aktuelle Sheet-Werte fuer Heroes, mit kompakter Auswahl
- `timeline_signals`: Patch-Events, Change-Type-Zaehler, strukturierte Stat-Aenderungen
- `open_questions`: explizite Luecken und Unsicherheiten
- `source_references`: Patch-, Sheet- und Entity-Referenzen fuer spaetere Antworten
- `prompt_de`: deutscher Prompt-Entwurf fuer eine spaetere KI-Antwort

Der Prompt weist ausdruecklich an, Namen von Items, Heroes und Abilities auf
Englisch zu behalten und Bewertung sowie Unsicherheiten auf Deutsch zu erklaeren.

Beispiel:

```bash
./rust/target/release/deadlock-brain review Shiv --limit-events 80
```

Das Ergebnis ist JSON und kann direkt serialisiert oder in einen Modellaufruf
gegeben werden.
