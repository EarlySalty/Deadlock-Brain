# LLM-Wiki Game Knowledge Design

## Ziel

Deadlock-Brain soll Spielwissen aus `deadlock-wiki/deadlock-data` nicht nur bei jeder Frage neu aus Rohdaten rekonstruieren. Stattdessen wird nach dem LLM-Wiki-Muster eine persistente, agentenlesbare Markdown-Wiki-Schicht aufgebaut, die vollstaendige Game-Facts lokal haelt und von `ask-context` fuer Antworten genutzt wird.

## Nichtziele

- Kein Live-Scraping der Deadlock-Wiki-API als Primaerpfad.
- Kein Vektor-RAG als Voraussetzung.
- Keine von der KI frei erfundenen Wiki-Facts.

## Architektur

Es gibt drei Schichten:

1. Raw Sources: Postgres-Tabellen `brain.source_documents` und `brain.entity_snapshots`, gefuellt aus `deadlock-wiki/deadlock-data`.
2. Wiki: `game-wiki/` als Markdown-Artefakt mit vollstaendigen Entity- und Source-Eintraegen.
3. Schema: `game-wiki/AGENTS.md` beschreibt Konventionen, Provenienz, Query- und Lint-Regeln.

Die Wiki-Eintraege duerfen lesbar zusammenfassen, muessen aber die vollstaendige strukturierte Quelle als JSON-Codeblock behalten. Die Vollstaendigkeit liegt im Wiki auf Disk; der Prompt bekommt nur die relevantesten Eintraege.

## Eintragsstruktur

- `game-wiki/AGENTS.md`: Schema und Maintainer-Regeln.
- `game-wiki/index.md`: Katalog aller Eintraege nach Kategorie.
- `game-wiki/log.md`: Append-only Laufprotokoll.
- `game-wiki/pages/<source>/<entity_type>.md`: Corpus-Shard mit einem vollstaendigen Eintrag pro aktuellem Snapshot.

Jede Seite enthaelt:

- YAML-Frontmatter mit Titel, Typ, Quelle, externem ID, Snapshot-ID, Source-Document-ID, Pull-Zeit und Source-Hash.
- Eine kurze deterministische Uebersicht.
- Die vollstaendige JSON-Payload.

## Query-Verhalten

`deadlock-brain ask-context` durchsucht `game-wiki/` lokal und fuegt relevante vollstaendige Wiki-Eintraege in `ground_truth.game_knowledge` ein. Der bestehende strukturierte Kontext bleibt erhalten; Wiki-Wissen ist eine zusaetzliche Ground-Truth-Schicht.

Wenn kein Wiki existiert, bleibt `ask-context` abwaertskompatibel und meldet `available=false`.

## CLI

`deadlock-brain wiki rebuild` erzeugt das komplette Wiki aus der zentralen Postgres-DB. Optional kann `--dir` ein Zielverzeichnis setzen.

## Akzeptanz

- Rebuild schreibt Schema, Index, Log und Eintraege.
- Rebuild nutzt aktuelle Deadlock-Data-Snapshots und keine Live-Wiki-API.
- Suche liefert vollstaendige passende Eintraege, nicht nur kompakte Excerpts.
- `ask-context` rendert `ground_truth.game_knowledge` in den Prompt.
- Cargo-Tests und `cargo check` sind gruen.
