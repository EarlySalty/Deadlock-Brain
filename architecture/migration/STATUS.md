# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Statuswerte: `offen`, `bereit`, `in_arbeit`, `review`, `blockiert`, `integriert`.
Arbeitsmodi: `prepare_only`, `implement`, `integration_test`, `cutover_rehearsal`.

Planversion: 1.0
Quelleneingang: U1, U3 und U4 im Planpaket vorhanden, U4 Duplikat laut Paketmanifest dedupliziert
Basis Commit: `55776c7532bb461fab93bbf7c582e2a40a81d2c9`
Contract Version: offen
DB Schema Version: offen
Aktives Knowledge oder Corpus Release: offen
Letzte geprüfte Runtime: nicht geprüft
Koordination: S000, Chat 00

## S000 Stand

S000 dokumentiert den vorhandenen Rust Workspace und richtet die Koordinationsartefakte ein. Produktiver Code, Datenbank und Runtime bleiben in diesem Arbeitspaket unverändert.

Der Basis Commit enthält `rust/Cargo.toml` mit elf Workspace Mitgliedern. Vorhandene `dbrain` Fachcrates werden in `PFAD_OWNER.csv` reserviert. Noch nicht belegte Pfade bleiben offen, statt durch neue Module ersetzt zu werden.

## Pakete

| Chat | Status | Arbeitsmodus oder Freigabe | Basis oder Ergebnis | Übergabe | Nächster Schritt |
|---|---|---|---|---|---|
| 00 | review | S000 Koordination | Basis `55776c7` | `S000_BOOTSTRAP.md` | S000 integrieren |
| 01 | bereit | `prepare_only` | `55776c7` | offen | Code, Daten, Runtime und Quellen inventarisieren |
| 02 | offen | nach G0 | `55776c7` | offen | Contracts und Workspace nach G0 |
| 03 | offen | Schemaentwurf nach G0 | `55776c7` | offen | reale Migrationspfade mit 01 und 02 festlegen |
| 04 | offen | nach G1 | `55776c7` | offen | Worker und Feeder |
| 05 | offen | nach G1 | `55776c7` | offen | Fachparität, Regeln, Builds und Learning |
| 06 | offen | Probe nach G0, Implementierung nach G1 | `55776c7` | offen | Retrieval |
| 07 | offen | Vertrag nach G0, Implementierung nach G1 | `55776c7` | offen | Provider und Jev |
| 08 | offen | feste Mocks nach G1 | `55776c7` | offen | Answer Kernel |
| 09 | offen | Contract API nach G1 | `55776c7` | offen | Consumer und Adapter |
| 10 | bereit | `prepare_only` | `55776c7` | offen | Testdesign parallel zu Chat 01 |
| 11 | offen | Runbook nach G1, Cutover nach G4 und Freigabe | `55776c7` | offen | Cutover und Rückweg |
| 12 | offen | Discovery nach G0, Implementierung nach G1 | `55776c7` | offen | Wiki und Wissenskarten |
| 13 | offen | Quellenprüfung nach G0, Implementierung nach G1 | `55776c7` | offen | externe Quellen und Schema |
| 14 | offen | Replayzugang nach G0, Implementierung nach G1 | `55776c7` | offen | Replays und Observations |

## Gates

G0: offen
G1: offen
G2: offen
G3: offen
G4: offen
G5: offen
G6: offen

S000 gibt keine Implementierung frei. Chat 01 und Chat 10 erhalten `prepare_only`.

## Offene Fakten

| ID | Thema | Auswirkung | Besitzer | Geplanter Nachweis |
|---|---|---|---|---|
| IST-01 | Liveinventar offen | aktiver Runtimezustand ist nicht belegt | 01 | Code, Runtime und Dateninventar |
| PERF-01 | Hardware, Last und SLOs offen | keine Leistungszusage | 01,10 | Benchmarkprofil |
| RIGHTS-01 | Quellen, Fixtures und Publikationsrechte offen | betroffene Abrufe und Exporte brauchen Prüfung | 00,01,12,13,14 | Quellenregister und Policy |
| PATH-01 | DB Migrationen und zentrale Deploymentpfade nicht in S000 verifiziert | Owner bleibt reserviert, reale Pfade folgen aus Inventar | 01,03,11 | Pfad und Serviceinventar |
| CONTRACT-01 | Contract und Schema Version noch nicht festgelegt | G1 nicht erreichbar | 02,03 | integrierte Versionen und Tests |
