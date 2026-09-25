# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 26.09.2026, Integrationsbranch `migration/rust-integration`.
Maßgeblicher Bericht mit allen Zahlen, Commits und Blockern: [INTEGRATION_REVIEW.md](INTEGRATION_REVIEW.md).

Planversion: 1.0 (Planpaket unter `deadlock-brain-rust-planpaket-v1.0/`).
Die im Auftrag genannte Datei `14_SESSION_ABLAUF_OHNE_ORCHESTRATOR.md` existiert weder im Repository (alle Refs geprüft) noch in den lokalen Planpaket-Kopien oder T3-Anhängen. Grundlage dieses Stands ist deshalb das Planpaket v1.0; die S000/Chat-Zuordnung bleibt als Herkunft erhalten, die Arbeit läuft seit 25.09.2026 als Codex-Completion-Aufträge plus lokale Integration.

Contract Version: `brain.v1` (intern), `brain.public.v1` (öffentliche API).
DB Schema: `scripts/migrations/2026-09-24-brain-contract-v1.sql` und `2026-09-25-brain-core-jobs-v2.sql`, additiv, ohne Down-Migration.
Aktives Knowledge oder Corpus Release in Produktion: keins aus dem neuen Kern. Produktiv läuft weiterhin der Bestandspfad (siehe Runtime).
Letzte geprüfte Runtime: 25.09.2026 23:45 CEST, unverändert gegenüber S01 (siehe INTEGRATION_REVIEW.md, Abschnitt Runtime).

## Nachweisstufen

Jede Zeile nennt die höchste tatsächlich erreichte Stufe. Eine höhere Stufe gilt nur mit eigenem Nachweis.

1. Code implementiert
2. offline getestet (Fixtures, lokale Fault-Server, isolierte Wegwerf-DB)
3. lokal integriert (auf `migration/rust-integration` zusammen gebaut und getestet)
4. echter Datenpilot (echte Daten, lokal, begrenzt)
5. staging validiert
6. production freigegeben

| Bereich | Chat | Quelle der Arbeit | Stufe | Offener Kernpunkt |
|---|---|---|---|---|
| Contracts, Policy | 02 | Core Completion (PR #38) | 3 | Wiki-/Replay-Felder (Bedingungen, Varianten, Unknown-Zeit) fehlen im gemeinsamen Vertrag |
| Storage, Migration | 03 | Core Completion | 4 | echter Pilot mit Absturz, Neustart und leerem Rebuild bestanden; keine Down-Migration, kein Upgrade gegen Produktionsschema |
| Ingestion, Feeder | 04 | Core Completion | 4 | nur Dateiquelle im neuen Kern; bestehende Feeds laufen weiter im Altpfad |
| Domain, Regeln, Builds | 05 | Core Completion (Regelevaluator) | 2 | Build-Legalität und Hero-Karten nicht an Kernel/API angeschlossen |
| Retrieval | 06 | Core Completion | 4 | kein Chunking: echte Dokumente sprengen das Default-Budget |
| Provider, Jev | 07 | Core Completion | 2 | kein freigegebener Provider/Modell für den neuen Kernel; Jev Standard `Disabled` |
| Answer Kernel, API | 08 | Core Completion | 4 | kein Composition Root, kein Dienst, kein Binary |
| Consumer | 09 | Consumer/CI Completion (PR #39 und PRs in vier Repos) | 2 | Adapter nicht aktiviert; Vertragslücken für Historie, Persona, Quellen-URLs |
| Qualität, Performance | 10 | PR #27 via Consumer/CI | 3 | nur Loopback-Messung im Pilot, keine Staging-Last |
| Cutover, Legacy-Ende | 11 | PR #26 via Consumer/CI | 3 | Audit-Werkzeug lokal gelaufen; keine Restore-/Writer-Probe mit Produktionsdaten |
| Wiki, Wissenskarten | 12 | Wiki Completion (PR #35) | 3 | Wiki lokal erreichbar (HTTP 200), aber Discovery scheitert per Design an 85 932 Seiten |
| Externe Quellen | 13 | External Sources Completion (PR #36) | 4 | Live-Contract Assets/OpenAPI lokal bestanden; Produktionsjob ohne Commit-Pin würde fail-closed abbrechen |
| Replays | 14 | Replay Decoder Completion (PR #37) | 3 | keine freigegebene echte Replaydatei lokal vorhanden |

## Gates

| Gate | Status | Begründung |
|---|---|---|
| G0 | teilweise belegt | S01-Inventar auf `main`, Runtime am 25.09.2026 erneut geprüft; SLO/Lastprofil weiterhin nicht freigegeben |
| G1 | offen | Workspace und Verträge integriert, aber Source-v2-, Fact-, Karten- und Observation-Verträge nicht vereinheitlicht |
| G2 | nicht bestanden | lokaler Rust-Durchstich mit echten Daten läuft, mit Default-Konfiguration aber nur 10 von 15 Fällen; Wiki-, Replay- und Build-Leg fehlen |
| G3 bis G6 | offen | keine Staging-Umgebung, kein Consumer aktiv, keine Freigabe |

`offen` ist nicht `bestanden`. Kein Gate wurde durch Remote-Mocks oder synthetische Fixtures auf bestanden gesetzt.

## Historie

S000 (Chat 00) hat am 24.09.2026 Basis `55776c7` dokumentiert und die Koordinationsartefakte angelegt (`S000_BOOTSTRAP.md`, `handoffs/00-s000-koordination.md`). Die Vorbereitungs-PRs #16 bis #22 und #25 bis #30 sind vollständig in den Completion-Branches bzw. im Integrationsbranch enthalten; ihre Handoffs bleiben als Herkunft unverändert erhalten.
