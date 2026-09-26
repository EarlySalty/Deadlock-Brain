# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 26.09.2026, Integrationsbranch `migration/rust-integration`.
Maßgeblicher aktueller Bericht mit Gates und G5-Blockern: [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md). Weitere Nachweise: [LEGACY_CORE_MIGRATION.md](LEGACY_CORE_MIGRATION.md), [FINAL_LOCAL_INTEGRATION_REVIEW.md](FINAL_LOCAL_INTEGRATION_REVIEW.md), [INTEGRATION_REVIEW.md](INTEGRATION_REVIEW.md), Nachtrag Welle 1/2: [WAVE1_WAVE2_LOCAL_REVIEW.md](WAVE1_WAVE2_LOCAL_REVIEW.md), eigene Datenbankinstanz: [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md), Datenkopie: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md).

Planversion: 1.0 (Planpaket unter `deadlock-brain-rust-planpaket-v1.0/`).
Die im Auftrag genannte Datei `14_SESSION_ABLAUF_OHNE_ORCHESTRATOR.md` existiert weder im Repository (alle Refs geprüft) noch in den lokalen Planpaket-Kopien oder T3-Anhängen. Grundlage dieses Stands ist deshalb das Planpaket v1.0; die S000/Chat-Zuordnung bleibt als Herkunft erhalten, die Arbeit läuft seit 25.09.2026 als Codex-Completion-Aufträge plus lokale Integration.

Contract Version: `brain.v1` (intern), `brain.public.v1` (öffentliche API).
DB Schema: Kernschema v2 (`brain.store.v2`) aus `scripts/migrations/2026-09-24-brain-contract-v1.sql`, `2026-09-25-brain-core-jobs-v2.sql` und `2026-09-26-brain-core-compatibility-v2.sql`, nur per `brain-migrate`. Staging-Instanz: eigene PostgreSQL-Instanz `deadlock-brain-postgresql` (Port 5446, nur Unix-Socket), DB `brain` mit leerem Kern und Archivkopie `brain_legacy`, Pilot-DBs `brain_pilot` und `brain_pilot_legacy` (Legacy-Kernimport, Release `legacy-core-f07ea85c09010285`).
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
| Contracts, Policy | 02 | Core Completion, C4 (PR #43), C9 (PR #50) | 3 | Wiki-, Source-, Replay- und Domain-v2-Verträge vereinheitlicht; Consumer über typed BrainClient angebunden |
| Storage, Migration | 03 | Core Completion, C11 (PR #41), Pre-G5 | 5 | eigene Instanz, Rollen getrennt, Backup/Restore gleich; Legacy-Konverter gebaut, Patchnotes und Entitäten nur in `brain_pilot_legacy` |
| Ingestion, Feeder | 04 | Core Completion, Pre-G5 | 4 | Datei-, Legacy- und Dokumentmengen-Pfad im Kern; Patchnotes-Feed und Assets-Adapter Brain-seitig gebaut, Provider-Export fehlt; Altpfade laufen bis G5 |
| Domain, Regeln, Builds | 05 | C6 (PR #48) | 4 | Build-Legalität und Hero-Card im Kernel; E2E nur mit synthetischem Domain-Adapter, echte Spielwerte fehlen |
| Retrieval | 06 | C2/C3 (PR #45), Pooling (PR #49) | 5 | Chunking/BM25 bestanden; Lesepfad läuft seit PR #49 über einen gemeinsamen, hart begrenzten Pool |
| Provider, Jev | 07 | Core Completion | 2 | kein freigegebener Provider/Modell für den neuen Kernel; Jev Standard `Disabled` |
| Answer Kernel, API | 08 | C1 (PR #42), Pooling (PR #49) | 5 | `brain-serve` gegen eigene Instanz 18/18 E2E und 16/16 Betriebsprüfungen; 600-Request-Test nach Pooling bestanden |
| Consumer | 09 | C9 (PR #50) + Consumer-PRs in vier Repos | 3 | CLI/MCP/Docs/2nd-Brain lokal gegen brain-serve-Staging verifiziert; Twitch-Shadow-Probe entkoppelt; Adapter nicht aktiviert; Vertragslücken für Historie, Persona, Quellen-URLs |
| Qualität, Performance | 10 | PR #27, Lasttest 26.09. (nach PR #49) | 5 | 600 Requests 8/16/32 Worker ohne too many clients; Peak-Pool 4, 8 Neuverbindungen je Stufe, 0 falsche unauthorized_evidence |
| Cutover, Legacy-Ende | 11 | PR #26 via Consumer/CI | 3 | Audit-Werkzeug lokal gelaufen; keine Restore-/Writer-Probe mit Produktionsdaten |
| Wiki, Wissenskarten | 12 | Wiki Completion, C5 (PR #47), Pre-G5 | 3 | Adapter in den normalen Store (`stage_into_store`) gebaut; echter Capture wartet auf Rechteentscheidung |
| Externe Quellen | 13 | External Sources Completion (PR #36), Pre-G5 | 4 | Assets-API jetzt auch in den Kern; Match-/Meta-Adapter schreiben weiter nur in den Altstore auf DL-Main |
| Replays | 14 | Replay Decoder Completion (PR #37) | 3 | keine freigegebene echte Replaydatei; Produktentscheidung Replay in V1 oder danach offen |

## Gates

| Gate | Status | Begründung |
|---|---|---|
| G0 | teilweise belegt | S01-Inventar auf `main`, Runtime am 25.09.2026 erneut geprüft; SLO/Lastprofil weiterhin nicht freigegeben |
| G1 | bestanden (lokal) | C4-Verträge integriert und von C5/C6 genutzt; C9 (PR #50) integriert, Consumer lokal gestaget; Details in `FINAL_LOCAL_INTEGRATION_REVIEW.md` |
| G2 | teilweise | Default-E2E 18/18; echter Legacy-Release beantwortbar; echter Wiki-Pilot, echter Provider und Replay-Entscheidung fehlen; Fakt-Profil ohne Relevanzschwelle |
| G3 | teilweise | Consumer-Parität lokal belegt; Patchnotes und Entitäten im Kern (Pilot-DB); Sheet/YouTube, Provider-Exporte und Abschaltung der Direkt-Writer fehlen |
| G4 | bestanden (lokal) | Lasttest nach Pooling bestanden und im Pre-G5-Review auf neuem Stand wiederholt (600×3 Stufen, Peak 4, 8 Neuverbindungen, 0 too many clients, E2E 18/18); Isolation 54/54; Backup/Restore unverändert belegt |
| G5, G6 | offen | keine Freigabe, kein Cutover |

`offen` ist nicht `bestanden`. Kein Gate wurde durch Remote-Mocks oder synthetische Fixtures auf bestanden gesetzt.

## Historie

S000 (Chat 00) hat am 24.09.2026 Basis `55776c7` dokumentiert und die Koordinationsartefakte angelegt (`S000_BOOTSTRAP.md`, `handoffs/00-s000-koordination.md`). Die Vorbereitungs-PRs #16 bis #22 und #25 bis #30 sind vollständig in den Completion-Branches bzw. im Integrationsbranch enthalten; ihre Handoffs bleiben als Herkunft unverändert erhalten.
