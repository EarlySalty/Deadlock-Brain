# S10 Messbedingungen und G0–G4-Protokoll

Basis: `30326512568b7370524956839100462ba71bdb92`. Arbeitsmodus: `prepare_only` mit ausführbaren Offline-Werkzeugen. Produktionsdienste und Datenbanken bleiben unverändert.

## Getrennte Nachweisarten

| Artefakt | Gemessen oder geprüft | Nicht nachgewiesen |
|---|---|---|
| `support-inventory.json` | 224 tatsächliche Fälle in sechs sauberen Docs-Dateien, zwölf davon unbeantwortbar; Commit und Dateihashes | Antwortqualität, Recall oder bestandene Consumer-E2E-Prüfung |
| `baseline.json` nach dem Lauf | Fünf Prozess-Warmmessungen des vorhandenen Population-Codes gegen exakte Kleinreferenzen | Datenbank, Retrieval, Modell, interaktive p95/p99, Legacy-Vergleich oder Rust-Beschleunigung |
| `design.json` | 40 Szenariospezifikationen und Profilbindung | 40 ausgeführte Sicherheits-/Fehlertests |
| Werkzeugtests | Prüflogik, Dateischutz, Metrikberechnung, Versionsbindung, Holdout und Gegenproben | Gesamtsystemfreigabe |

Die Baseline enthält fünf Batches mit je 5.000 Funktionspaaren. Ihre Verteilung beschreibt Batch-Mittelwerte, nicht 25.000 unabhängige Anfrage-Latenzen. Hardwarewerte werden vom tatsächlich ausführenden Host erfasst. Speicher-/Netzwerkprofil, Containergrenzen und nicht erhobene Ressourcen bleiben `null`. Die Messung erzeugt keine Last auf dem laufenden Brain-Dienst.

## Spätere Vergleichsgruppen

A: gepinnter Legacy-Zustand. B: Rust-Parität bei gleichen Daten und Verfahren. C: Rust mit geändertem Retrieval. D: je ein ausdrücklich benannter Jev-Eingriff. Nicht mehrere Änderungen gleichzeitig als Rust-Effekt ausgeben. Die Jev-Version ist in A/B/C `disabled`; D benötigt eine eigene gepinnte Funktionsversion. Getrennte Profile und identische Vergleichskohorten sind Pflicht. Das aktuelle Werkzeug aktiviert keine dieser Varianten.

Vor den echten Läufen freigeben: Zielhardware, CPU-/RAM-Limits, Speicher und Netzwerk, Rohdaten-/Indexgröße, Corpus-/Knowledge-/Modell-/Schema-/Suchversionen, Seed, Ankunftsrate, Parallelität, Dauer, Warmup sowie absolute SLOs. Drei oder mehr Wiederholungen für kalte und warme Zustände bleiben im Entwurf Pflicht. Zieldurchsatz und Grenzwerte sind noch `null`, nicht geschätzt.

Echte interaktive Messungen jeweils unter Leerlauf, Wiki-Ingest, Git-Ingest, Replay-Decoding, Embeddinglast und gemischter Hintergrundlast durchführen. Für G4 Last bis zur vorher freigegebenen Zielkapazität und kontrolliert darüber steigern; Queueentwicklung, begrenzte Backpressure und Erholung nach Entlastung nachweisen. Zeitstempel ab geplanter Ankunft verwenden; verlorene, abgebrochene und fehlerhafte Anfragen im Nenner behalten.

Neben Rohdaten und p50/p95/p99 je Stufe gehören Throughput, Fehlerraten, RAM/CPU/IO, Ingestlag, Rebuilddauer, Tokens und alle Kosten in den Bericht. Jeder Fall muss seine erforderlichen Assertions und unabhängigen Belege tragen. Unterlegene Antwortqualität oder geringerer Recall ist kein isolierter Geschwindigkeitsgewinn. Konfidenzintervalle nicht aus abhängigen Wiederholungen derselben Match-/Quellenfamilie bilden.

## Gate-Protokoll

| Gate | S10-Urteil | Fehlender echter Nachweis / Besitzer |
|---|---|---|
| G0 | offen | S01 integriert, reale End-to-End-Baseline, Hardware-/Lastfreigabe und Quellenrechte; 00/01/10 |
| G1 | offen | Gemeinsame Contracts, Schema und integrierte Zuständigkeiten; 00/02/03 |
| G2 | nicht geprüft | Echter Pilot mit realem Daten-/Consumer-Pfad; Fachpakete |
| G3 | nicht geprüft | Vollständiger Datenabgleich, Domain-/Consumerparität und Pflichtquellen; Fachpakete |
| G4 | blockiert | Gesamtstack, unabhängige A/B/C/D-Qualitäts-/Lastnachweise, Sicherheit, Restore, Writer-Cutover, Rollback und blockierte Altpfade; 10/11 |

Diese Tabelle ist ein S10-Prüfprotokoll, keine Änderung des zentralen Gate-Registers. Offene G0–G3-Evidenz kann weder durch die Offline-Baseline noch durch erfolgreiche Werkzeugtests ersetzt werden.
