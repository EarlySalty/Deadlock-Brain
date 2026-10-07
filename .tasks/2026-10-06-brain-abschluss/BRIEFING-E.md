# Paket E: Deadlock-API als Datenquelle für das eine Brain

Rolle: Blatt-Worker (GPT 6.1 Sol). Keine weiteren Threads, native Subagenten nur für Recherche.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`. Steuerung: `VON_HAUPT.md`.

## Hintergrund

Das Grundding scheitert heute an Spielwissen: Im aktiven Korpus gibt es keine Steckbrief-, Patchnotes- oder Patch-Story-Quelle (`A/G1-RUECKGABE.md`). Der Build-Reasoner ist gesperrt, weil nur 47 statt 100 Matches nach dem Patch vorliegen. Nutzerwunsch: Im Kern brauchen wir eine aktuelle Datenbank. Die Deadlock-API (deadlock-api.com) liefert Patchdaten, Helden, Items, Builds und Matchstatistiken. Prüfen, was wir davon nutzen können, damit unsere Datenstände einfacher und aktueller werden.

## Auftrag (zuerst nur lesen und messen, keine Produktänderung)

1. Bestand im Brain mit Graphify (Skill `code-suche`) zuerst: Welche Deadlock-API-Endpunkte nutzt Deadlock-Brain heute schon (z. B. Build-Daten-Timer `deadlock-brain-build-data`, Patchnotes-Sync, Reasoner-Matchdaten, MCP `/v1/mcp`), welche eigenen Scraper oder Parser machen dasselbe parallel (Patchnotes, Wiki, Spieldateien, Steckbriefe), und in welche Tabellen im Brain-Postgres (Port 5446) schreiben sie.
2. Angebot der API am echten Endpunkt messen, nicht aus Erinnerung: Assets-API (Helden, Items, Fähigkeiten mit Werten je Patch), Patches/Changelog, Build-Suche, Analytics (Item- und Heldenstatistik, Matchzahlen nach Datum und Rang), Match-Metadaten, MCP/SQL-Zugang. Je Endpunkt: Inhalt, Aktualität (wie schnell nach einem Patch), Versionierung je Patch, Limits und Nutzungsbedingungen.
3. Gegenüberstellung in `E/DEADLOCK-API.md`: Datenbedarf des Brains (Steckbriefe aktueller Patch, Patch-Story und Änderungshistorie, Itemwerte für den Reasoner, Matchmenge nach Patch, Builds) gegen API-Angebot gegen heutigen eigenen Weg. Je Zeile: ersetzen, ergänzen oder behalten, mit Begründung.
4. Konkreter Vorschlag für eine einzige Spieldaten-Datenbank im Brain-Postgres, die aus der API gespeist wird (Tabellen, Abholtakt, Patcherkennung, wie daraus Steckbriefe und Patch-Story in den aktiven Korpus kommen). Bestehende Tabellen und Ingest-Wege wiederverwenden, keinen zweiten parallelen Pfad. Gesondert beantworten: Reichen die Analytics-Matchdaten der API, um die 100-Match-Grenze des Reasoners ehrlich zu erfüllen, ohne die Grenze zu senken?
5. Kurzfazit oben in `AN_HAUPT-E.md`, danach stehen bleiben, bis `VON_HAUPT.md` (Abschnitt Paket E) die Umsetzung freigibt.

## Grenzen

- Keine Nutzer- oder Community-Daten an die API schicken (keine Steam-IDs unserer Mitglieder). Öffentliche Spiel- und Aggregatdaten abfragen ist erlaubt.
- Keine Main-Pushes, Releases, Installationen, Neustarts oder Ticks. Der gemeinsame Release-Hold (`A/RELEASEFENSTER.md`) gilt.
- A besitzt die Korpusaktivierung; E liefert Analyse und Plan, baut nichts doppelt zu A.
- Deutsch, ohne Em-Dashes. Rust only für spätere Umsetzung, keine Code-Kommentare, Secrets nie im Klartext.
