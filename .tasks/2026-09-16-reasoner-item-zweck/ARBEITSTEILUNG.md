# Arbeitsteilung nach Nutzerkorrektur vom 16.09.2026

Diese Nutzerkorrektur hat Vorrang vor der bisherigen automatischen Opus-Routingannahme in AUFTRAG.md, ORCHESTRIERUNG.md, PAKETE.md und REGISTER.md. Fachlicher Vertrag, Rust-Ziel, Datenbankschutz und unabhängige Abnahme bleiben unverändert.

## Zuständigkeit

ChatGPT/Astra übernimmt in dieser Bearbeitung selbst API-/Quellenrecherche, Vergleich der verfügbaren Daten mit dem Rust-Modell, Mechanikprüfung, Testentwurf, unabhängige Sichtprüfung fremder Patches sowie Koordination und Sicherung der Artefakte. Diese Tätigkeiten werden nicht an neue Claude-Subagenten oder neue Claude-T3-Threads ausgelagert. Es wird auch kein anderer kostenpflichtiger Modellzugang als Ersatz gestartet.

Der bereits laufende Opus-4.8-Thread 803d3e94-9b1d-42c5-9bb7-1905c8146acc darf sein klar begrenztes Paket 0+A beenden. Kein neuer Auftrag B/C/D im selben Thread, keine zusätzlichen Unteragenten. Nicht wegen der neuen Arbeitsteilung abbrechen oder seine Dateien parallel ändern. Nach fertigem Patch folgt eine unabhängige Prüfung durch ChatGPT; das ist keine Selbstfreigabe des Implementierers. Hat ChatGPT einen späteren Produktpatch selbst geschrieben, braucht dieser wiederum einen davon unabhängigen Reviewer, nicht ChatGPTs eigene Freigabe.

Folgephasen bleiben sequenziell, aber ohne fest reservierten neuen Opus-Worker. Keine automatische Weiterleitung allein wegen eines Statuswechsels oder eines Eintrags in PAKETE.md. Aktuelle Coding-Kontingente oder Modellnamen sind kein Grund, Recherche und Prüfungen im vorhandenen Chat liegenzulassen.

## Nachweis statt Hintergrundversprechen

Nur tatsächlich ausgeführte Arbeit, erhaltene Antworten, geprüfte Dateien und gesicherte Commits als erledigt melden. Diese Datei startet weder einen Timer noch einen Agenten. Es gibt weiterhin keine eingerichtete 45-Minuten- oder Stundenwache. Die Beobachtung des laufenden Workers erfolgt während der aktuellen Bearbeitung über den bestehenden T3-Leseweg.

## Quellenarbeit

Deadlock API als Mechanik-/Asset- und Populationszulieferer nutzen. Quellenvertrag, tatsächlich gelesene Payloads, versionierte lokale Snapshots und eigene Ableitungen getrennt kennzeichnen. Eine abrufbare OpenAPI-Dokumentation ist kein Beweis für erfolgreich abgerufene Live-Heldendaten. Neue Live-Daten nicht in die laufende eingefrorene Phase-0-Messung hineinmischen. Keine neue Python-Produktlogik und keine zweite Engine. Vorhandene Werkzeuge dürfen unverändert genutzt werden.
