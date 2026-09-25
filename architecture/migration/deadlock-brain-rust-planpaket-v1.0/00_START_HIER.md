# Deadlock Brain · Rust, Wiki, externe Daten und Konsolidierung

**Planstand: 24. September 2026 · Version 1.0 · Drei Recherchen gemeinsam eingeplant**

Dieses Paket ersetzt v0.9. Es ist ein Umsetzungsplan, kein durchgeführter Umbau und kein Performance-Nachweis. Zusammengeführt sind Konsolidierung/Jev, vollständige Wiki-Wissensintegration sowie GitHub-/Datenquellen-/Replay-Ausbau. Der gesamte eigene produktive Backendkern einschließlich Worker, Learning, regelmäßiger Importe und Rebuild läuft im Ziel in Rust.

## Zuerst lesen: Welche Chats dürfen gleichzeitig arbeiten?

**Nicht alle Prompts gleichzeitig zur Implementierung losschicken.** Die Chatnummern sind Zuständigkeiten, keine chronologische Reihenfolge. Maßgeblich ist [10 Reihenfolge und Parallelität](10_REIHENFOLGE_UND_PARALLELITAET.md).

Zuerst Chat 00 initialisieren; danach Chat 01 und das Testdesign aus Chat 10 parallel. Nach belegtem Inventar entwerfen 02/03 gemeinsam die Verträge. Erst nach deren Integration gibt 00 die Implementierungswelle frei. 08/09 dürfen früh gegen feste Testverträge vorbereiten, aber nicht als echt integriert gelten. Chat 11 führt den Produktivwechsel erst nach bestandener Abnahme und ausdrücklicher Freigabe aus.

## So beginnen

Paket im Zielrepo unter `architecture/migration/` ablegen, [09 Startprompt](09_STARTPROMPT.md) in einem neuen Koordinationschat verwenden. Bei einem bereits begonnenen v0.9-Umbau zuerst [08 Änderungsabgleich](08_ERGAENZUNGEN_INTEGRIERT.md) durchführen: Änderungen behalten, neue Anforderungen zuordnen, keine zweite Grundarchitektur bauen.

Jeder Arbeitschat erhält seinen Auftrag, aktuelle Regeln/Verträge, den integrierten Basis-Commit und relevante Übergaben. Ein Chat kennt die Arbeit anderer Chats nicht automatisch. Repozugriff nicht voraussetzen oder Testergebnisse erfinden. `vorlagen/STATUS.md` nur beim ersten Start als `STATUS.md` kopieren; vorhandenen Status beim Update nicht überschreiben. Nur 00 integriert gemeinsamen Status und Freigaben.

## Inhalt

| Datei | Zweck |
|---|---|
| [01 Masterplan](01_MASTERPLAN.md) | Gemeinsames Ziel, Verantwortungen und Arbeitswellen |
| [02 Regeln](02_GEMEINSAME_REGELN.md) | Rust, Eigentümerschaft, Branches und Übergaben |
| [03 Datenmigration](03_DATENMIGRATION.md) | Inventar, Historie, Migration, Delta und Wiederaufbau |
| [04 Performance](04_PERFORMANCE_UND_EVALS.md) | Vergleichsmessungen und Qualitätsgrenzen |
| [05 Abnahme](05_ABNAHME_UND_CUTOVER.md) | G0–G6, Daten-/Wiki-/Replay-Abnahme und Rollback |
| [06 Verträge](06_VERTRAEGE_UND_GRENZEN.md) | Gemeinsame API-, Quellen-, Fact-, Karten- und Replay-Verträge |
| [07 Anforderungen](07_ANFORDERUNGEN.md) | Anforderung → Besitzer → Nachweis |
| [08 Ergänzungen](08_ERGAENZUNGEN_INTEGRIERT.md) | Eingang, Deduplikation, Konfliktauflösung und v0.9-Upgrade |
| [09 Startprompt](09_STARTPROMPT.md) | Kopierfertiger Einstieg |
| [10 Parallelplan](10_REIHENFOLGE_UND_PARALLELITAET.md) | Welche Chats wann starten, Voraussetzungen und Merge-Regeln |
| [11 Wiki/Wissen/Builds](11_WIKI_WISSEN_UND_BUILDS.md) | Vollständigkeit, Hero-Wissenskarten, Mechaniken und legale Builds |
| [12 Externe Quellen](12_QUELLEN_UND_SCHEMAWATCH.md) | Repo-Landschaft, API-Verträge, historische Daten und Schemaänderungen |
| [13 Replays/Population](13_REPLAYS_UND_POPULATION.md) | Rust-Replaypfad, Beobachtungen, Population und zeitliche Holdouts |
| `chats/00…14` | 15 Chat-Aufträge: Koordination plus 14 Arbeitsbereiche |
| `vorlagen/` | Übergaben, Status, Freigaben, Quellen-/Coverage-/Benchmarkvorlagen |
| [Quellen](quellen/QUELLEN_UND_ANNAHMEN.md) | Drei unveränderte Recherchen, Herkunft und verbleibende Unbekannte |

## Grenzen

Alle drei Recherchen sind in die Planung eingearbeitet. Der erneut hochgeladene Ökosystembericht ist bytegleich mit der vorherigen Datei und wird nur einmal als Quelle und Aufgabenbestand geführt. Das schließt den früheren Eingangsmangel EXT-01; es beweist keine Umsetzung, Lizenzfreigabe oder Live-Vollständigkeit.

Aktuelle Commits, aktive Dienste, Plattformverträge, Rechte, Datenmengen, Hardware und Last müssen bei Umsetzung erfasst werden. Öffentliche Repo-Historie darf keine internen Daten enthalten. Rohdaten und Replays gehören nicht pauschal nach Git. Eine nicht verfügbare oder nicht freigegebene Pflichtquelle bleibt ein sichtbarer Blocker, kein angeblich erledigter Import.
