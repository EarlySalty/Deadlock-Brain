# Projektstatus · Deadlock Brain Rust + Wiki + Daten

Statuswerte: `offen`, `bereit`, `in_arbeit`, `review`, `blockiert`, `integriert`.
Arbeitsmodus getrennt: `prepare_only`, `implement`, `integration_test`, `cutover_rehearsal`.
`integriert` verlangt echten Commit + Prüfbeleg. Leere Werte sind unbekannt.

Planversion: 1.0
Quelleneingang: U1/U3/U4 vorhanden und geplant; U4-Duplikat dedupliziert
Basis-Commit: offen
Contract-Version: offen
DB-Schema-Version: offen
Aktives Knowledge-/Corpusrelease: offen
Letzte geprüfte Runtime: nicht geprüft
Koordinator: Chat 00

## Pakete

| Chat | Status | Arbeitsmodus/Freigabe | Basis/Ergebnis-Commit | Übergabe | Nächster Schritt |
|---|---|---|---|---|---|
| 00 | offen | noch nicht erteilt | — | — | Koordination initialisieren |
| 01 | offen | noch nicht erteilt | — | — | Inventar/Baseline nach Initialisierung |
| 02 | offen | noch nicht erteilt | — | — | G0; Contracts/Workspace mit 03 |
| 03 | offen | noch nicht erteilt | — | — | G0 Schemaentwurf; Implementierung G1 |
| 04 | offen | noch nicht erteilt | — | — | G1 Worker-/Feederports |
| 05 | offen | noch nicht erteilt | — | — | G1 Fachparität/Rules/Builds |
| 06 | offen | noch nicht erteilt | — | — | G0 Probe; G1 Suchmodul |
| 07 | offen | noch nicht erteilt | — | — | G0 Vertrag; G1 Provider |
| 08 | offen | noch nicht erteilt | — | — | G1 feste Mocks; danach Echtintegration |
| 09 | offen | noch nicht erteilt | — | — | G1 Contract-API; danach echter 08-Kern |
| 10 | offen | noch nicht erteilt | — | — | Testdesign parallel zu 01 |
| 11 | offen | noch nicht erteilt | — | — | Runbook vorab; Cutover erst G4 + Freigabe |
| 12 | offen | noch nicht erteilt | — | — | G0 Wiki-Discovery; G1 Parser/Karten |
| 13 | offen | noch nicht erteilt | — | — | G0 Quellenprüfung; G1 API/Git/Schema |
| 14 | offen | noch nicht erteilt | — | — | G0 Replayzugang; G1 Decoder/Observations |

## Gates

G0: offen · G1: offen · G2: offen · G3: offen · G4: offen · G5: offen · G6: offen
Jede Freigabe auf Contract-/Code-/Knowledgeversion beziehen; bei relevanter Änderung neu prüfen.

## Eingang und verbleibende Blocker

| ID | Thema | Auswirkung | Besitzer | Nachweis |
|---|---|---|---|---|
| EXT-01 | geschlossen: Zusatzrecherchen eingegangen und geplant | keine erneute Dateianforderung; Umsetzung nicht behauptet | 00 | 08 + Quellenhashes + R26–R60 |
| IST-01 | Liveinventar noch offen | Ist-Zustand nicht belegt | 01 | Code-/Runtime-/Dateninventar |
| PERF-01 | Hardware/Last/SLOs offen | keine gemessene Leistungszusage | 01/10 | Benchmarkprofil |
| RIGHTS-01 | konkrete Quellen-/Fixture-/Publikationsfreigaben offen | betroffenen Abruf/Import/Export vorab prüfen | 00/01/12–14 | Quellenregister/Policy |

## Nächste freigegebene Arbeit

Noch keine Implementierung freigegeben. 00 initialisiert; danach 01 und 10 Testdesign. Bei einem bestehenden Projektstatus diese Vorlage NICHT über laufende Commits/Entscheidungen kopieren.
