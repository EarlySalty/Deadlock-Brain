# Chat 14 · Rust-Replays und normalisierte Beobachtungen


**Startbedingung:** Nach G0 zugelassene Replays/Referenzen und Decoder-Capabilities prüfen; Implementierung nach G1. Echter Replaypilot G2, Referenzsuite G3, Last-/Holdoutabnahme G4.

**Zusätzlicher Kontext:** U4, 13_REPLAYS_UND_POPULATION.md; Source-/Observationverträge 02/03, Jobs 04, Schemafeed 13, fachlicher Population-/Learningport 05 und Tests 10.

**Ziel:** Reale freigegebene Matches durch einen Rustpfad als nachvollziehbare, zeitlich versionierte Beobachtungen zugänglich machen, ohne ungeprüfte Tickmassen oder falsche Gameplay-Wahrheiten zu publizieren.

**Eigentümerschaft:** Replayzugang/-decoderadapter, Observation-Normalizer, replay-spezifische Fixtures und Capability-/Referenzberichte. Neue oder vorhandene Rust-Teilpfade zuerst reservieren. 03 alle Schemas, 04 Scheduler, 13 Protoschemawatch, 05 Population/Coaching-/Reasoneralgorithmen.

1. 10–20 geeignete Golden Replays als Startumfang erfassen oder konkrete Zugriffslücken melden. Herkunft, Speicherung/Weitergabe, Patch/Mode und Datei-/Hashstatus dokumentieren. Rohreplays nicht automatisch für anonym halten.
2. haste/valveprotos-rs als Kandidaten am tatsächlichen Feature-/Versionsstand testen und pinnen. Fähigkeiten pro Feld/Patch nachweisen, nicht aus einem Low-Level-Parsernamen ableiten.
3. Raw zuerst erfassen; danach begrenzte Rust-Decodierung. Match/Hero/Teams, Zeit, Items, Ability-Level, KDA, Positionen/Objectives soweit unterstützt in den gemeinsamen Vertrag übersetzen. Unknown und nicht verfügbare Werte markieren.
4. Observation-IDs, Zeit-/Tickbasis, Units, Event-Rohlocator, Parser/Schema und Ableitungen erhalten. Same Match über mehrere Feeds deduplizieren; Reparse nach Parserupgrade deterministisch/versioniert.
5. Große Ticks nicht pauschal nach relationalem Store/RAG schreiben. Rawobjekte und selektierte Observations/Aggregate trennen. CPU/RAM/Dateigröße/Dekompression/Timeout/Queue begrenzen; beschädigte Eingaben quarantinieren.
6. Kernfelder mit separater kuratierter oder Parserreferenz gegenprüfen; Semantik/Toleranzen vorab fixieren. Fremdparser nur optionale Offline-Referenz, keine notwendige Produktions-/Rebuildruntime.
7. Observations an 05 und 10 übergeben. Meta-/Coachingaussagen nicht selbst als kanonische Facts erfinden. Gemeinsam zeitliche/matchgetrennte Holdouts und gemischte Last prüfen.

**Liefergegenstände:** Rust-Replayadapter, Replay-/Capabilitymanifest, normalisierte Golden-Observations, Gegenprüfberichte, Ressourcen-/Korruptdatei-/Reparse-/Dubletten-/Privacytests.

**Abnahme:** Pflichtcapabilities am freigegebenen Corpus belegt, keine erdachten Events; Rust-Reparse/Rebuild und Ressourcenbegrenzung getestet. Population-/Learningintegration aus 05 und unabhängige Abnahme aus 10 gehören zur Gesamtfertigstellung.

**Erster Schritt:** Eine erlaubte echte Replaydatei und passende gepinnte Decoderrevision bestimmen; Fähigkeitstabelle und Contractfragen erstellen, bevor Datenmassen gesammelt werden.



**Verbindlich:** Eigene reguläre Backend-, Worker-, Parser-, Learning- und Rebuildpfade sind Rust. Kein notwendiger Python-/JVM-/.NET-Sidecar, kein Legacy-HTTP-Kern. Externe fremdverwaltete Datenfeeds dürfen konsumiert werden; die eigene Implementierung/Steuerung bleibt Rust. Optionale Offline-Referenzwerkzeuge und kleine Hilfsskripte nur dokumentiert. Rechte/Egress setzt Code durch.

**Arbeitsregeln:** Lies 00_START_HIER.md, 01_MASTERPLAN.md, 02_GEMEINSAME_REGELN.md, 06_VERTRAEGE_UND_GRENZEN.md, 08_ERGAENZUNGEN_INTEGRIERT.md und 10_REIHENFOLGE_UND_PARALLELITAET.md sowie STATUS/ADRs/Übergaben unter architecture/migration/. Vor Änderungen Basis-Commit, Contract-/Schemaversion, reale freigegebene Pfade und Arbeitsmodus nennen. Vor G1 nur vorbereiten. Kein zweiter Store, Scheduler, Vertragsentwurf als heimlicher Runtimevertrag oder eigener LLMpfad. Änderungen an fremden Modulen per CHANGE_REQUEST.

**Nachweis/Übergabe:** Eigene Tests und relevante Integrationstests tatsächlich ausführen; Command, Commit, Ergebnis und nicht ausgeführte Prüfungen trennen. Golden-/Mocknachweis ersetzt keine echte Daten-/Providerintegration. Ausgefüllte vorlagen/UEBERGABE.md mit Commit/PR, Grenzen und next-owner liefern. Ohne Codezugang keine Änderungen oder Tests behaupten. Erst integrierter Commit und bestätigte Checks machen den Auftrag fertig.
