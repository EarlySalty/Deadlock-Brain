# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 26.09.2026, Integrationsbranch `migration/rust-integration`.
Maßgeblicher Bericht mit allen Zahlen, Commits und Blockern: [INTEGRATION_REVIEW.md](INTEGRATION_REVIEW.md), Nachtrag Welle 1/2: [WAVE1_WAVE2_LOCAL_REVIEW.md](WAVE1_WAVE2_LOCAL_REVIEW.md), eigene Datenbankinstanz: [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md), Datenkopie: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md).

Planversion: 1.0 (Planpaket unter `deadlock-brain-rust-planpaket-v1.0/`).
Die im Auftrag genannte Datei `14_SESSION_ABLAUF_OHNE_ORCHESTRATOR.md` existiert weder im Repository (alle Refs geprüft) noch in den lokalen Planpaket-Kopien oder T3-Anhängen. Grundlage dieses Stands ist deshalb das Planpaket v1.0; die S000/Chat-Zuordnung bleibt als Herkunft erhalten, die Arbeit läuft seit 25.09.2026 als Codex-Completion-Aufträge plus lokale Integration.

Contract Version: `brain.v1` (intern), `brain.public.v1` (öffentliche API).
DB Schema: Kernschema v2 (`brain.store.v2`) aus `scripts/migrations/2026-09-24-brain-contract-v1.sql`, `2026-09-25-brain-core-jobs-v2.sql` und `2026-09-26-brain-core-compatibility-v2.sql`, nur per `brain-migrate`. Staging-Instanz: eigene PostgreSQL-Instanz `deadlock-brain-postgresql` (Port 5446, nur Unix-Socket), DB `brain` mit leerem Kern und Archivkopie `brain_legacy`.
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
| Contracts, Policy | 02 | Core Completion, C4 (PR #43) | 3 | Wiki-, Source-, Replay- und Domain-v2-Verträge vereinheitlicht; Consumer-Lücken (C9) offen |
| Storage, Migration | 03 | Core Completion, C11 (PR #41) | 5 | eigene Instanz, Rollen getrennt, Backup/Restore gleich; kein Konverter Alttabellen nach `SourceRecordV2` |
| Ingestion, Feeder | 04 | Core Completion | 4 | nur Dateiquelle im neuen Kern; bestehende Feeds laufen weiter im Altpfad |
| Domain, Regeln, Builds | 05 | C6 (PR #48) | 4 | Build-Legalität und Hero-Card im Kernel; E2E nur mit synthetischem Domain-Adapter, echte Spielwerte fehlen |
| Retrieval | 06 | C2/C3 (PR #45) | 5 | Chunking/BM25 bestanden; Lesepfad verbindet je Operation neu (Pooling fehlt) |
| Provider, Jev | 07 | Core Completion | 2 | kein freigegebener Provider/Modell für den neuen Kernel; Jev Standard `Disabled` |
| Answer Kernel, API | 08 | C1 (PR #42) | 5 | `brain-serve` gegen eigene Instanz 18/18 E2E und 16/16 Betriebsprüfungen; 600-Request-Test nicht bestanden |
| Consumer | 09 | Consumer/CI Completion (PR #39 und PRs in vier Repos) | 2 | Adapter nicht aktiviert; Vertragslücken für Historie, Persona, Quellen-URLs |
| Qualität, Performance | 10 | PR #27, Lasttest 26.09. | 5 | 600 Requests 8/16/32 Worker ohne too many clients, aber ~10 Neuverbindungen je Anfrage |
| Cutover, Legacy-Ende | 11 | PR #26 via Consumer/CI | 3 | Audit-Werkzeug lokal gelaufen; keine Restore-/Writer-Probe mit Produktionsdaten |
| Wiki, Wissenskarten | 12 | Wiki Completion, C5 (PR #47) | 3 | C5 offline grün; echter Capture wartet auf Rechteentscheidung, C5-Store nur am eigenen Scratch-Cluster |
| Externe Quellen | 13 | External Sources Completion (PR #36) | 4 | Live-Contract Assets/OpenAPI lokal bestanden; Produktionsjob ohne Commit-Pin würde fail-closed abbrechen |
| Replays | 14 | Replay Decoder Completion (PR #37) | 3 | keine freigegebene echte Replaydatei lokal vorhanden |

## Gates

| Gate | Status | Begründung |
|---|---|---|
| G0 | teilweise belegt | S01-Inventar auf `main`, Runtime am 25.09.2026 erneut geprüft; SLO/Lastprofil weiterhin nicht freigegeben |
| G1 | teilweise | C4-Verträge integriert und von C5/C6 genutzt; Consumer-Vertragslücken (C9) offen |
| G2 | teilweise | Default-E2E 18/18 gegen die eigene Brain-Instanz; echter Wiki-Pilot, Replay und echter Provider fehlen |
| G3 | offen | Staging-Datenbank vorhanden, aber kein Consumer (C9 fehlt) |
| G4 | offen | Lasttest nicht bestanden (kein Pooling); Backup/Restore bestanden |
| G5, G6 | offen | keine Freigabe, kein Cutover |

`offen` ist nicht `bestanden`. Kein Gate wurde durch Remote-Mocks oder synthetische Fixtures auf bestanden gesetzt.

## Historie

S000 (Chat 00) hat am 24.09.2026 Basis `55776c7` dokumentiert und die Koordinationsartefakte angelegt (`S000_BOOTSTRAP.md`, `handoffs/00-s000-koordination.md`). Die Vorbereitungs-PRs #16 bis #22 und #25 bis #30 sind vollständig in den Completion-Branches bzw. im Integrationsbranch enthalten; ihre Handoffs bleiben als Herkunft unverändert erhalten.
