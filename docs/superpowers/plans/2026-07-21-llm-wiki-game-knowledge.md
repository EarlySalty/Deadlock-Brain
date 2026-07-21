# Plan: LLM-Wiki Game Knowledge

## Schritte

1. Tests fuer Wiki-Suche und Prompt-Integration ergaenzen.
2. Neues Modul `dbrain_retrieval::game_wiki` bauen:
   - Dateistruktur schreiben.
   - Vollstaendige Wiki-Eintraege aus `brain.entity_snapshots` erzeugen.
   - Lokale Markdown-Suche fuer Query-Kontext anbieten.
3. `AskContextOptions` um `game_wiki_dir` erweitern und `ask_context` mit `ground_truth.game_knowledge` verdrahten.
4. CLI-Command `deadlock-brain wiki rebuild` einbauen.
5. Changelog ergaenzen.
6. Formatieren, Tests, Check.

## Verifikation

- `cargo test -p dbrain-retrieval game_wiki`
- `cargo test -p dbrain-retrieval prompt`
- `cargo check -p dbrain-retrieval -p deadlock-brain`
- Bei DB-Zugriff: `deadlock-brain wiki rebuild --dir <tmp>` gegen zentrale DB.
