status: aktiv
Datum: 2026-10-03

# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 03.10.2026, Vorbereitung des G5-Cutovers. Frisch geholtes `origin/main`: `511a347b653beba13c2bf130f4bead7a7196cc2a`. Paket Z arbeitet in `feat/brain-fertig-z-20261003`. Dieser Stand ist kein neuer Test-, Deploy- oder G5-Nachweis.

## Betreiberentscheidung und tatsächlicher Betrieb

Der Hauptorchestrator hat G5 mit Rückweg im Auftrag des Nutzers freigegeben: `.tasks/2026-10-03-brain-fertigstellung/AUFTRAG.md`. Die fehlende Betreiberentscheidung vom September ist damit geschlossen. Der Production-Cutover ist weiterhin nicht ausgeführt; G5 bleibt offen. G6 wurde nicht begonnen.

Die Laufzeitaufnahme vom 03.10. ist in [CUTOVER_RUNBOOK.md](CUTOVER_RUNBOOK.md) festgehalten. `brain-serve` und Maintenance verwenden den Releasepfad mit Namen `511a347b653beba13c2bf130f4bead7a7196cc2a`; der getrennte CLI-Link zeigte bei der Aufnahme auf `be2aa6bd5a6504e99693f7dd1edaa16e76be91b4`. Prozessbinary und Serverartefakt haben denselben Hash. Die unabhängige Zuordnung der Binary-Bytes zum Source-SHA ist noch nicht geschlossen. Ein laufender Server und erfolgreiche Health-/Ready-Endpunkte beweisen weder Consumer-Aktivierung noch G5.

Die bisherigen Legacy-Writer und Timer bleiben bis zum jeweiligen belegten Schreibwechsel erhalten. P stellt Publisher und Patchnotes-Sync um, S den CLI-/Build-Publish-Pfad, Q Sheet und YouTube. YouTube-Lernen bleibt nach dem aktuellen Bereichsvertrag pausiert. Neue Writer werden nicht parallel zu ihren alten Gegenstücken gestartet.

## Aktueller Produkt- und Nachweisstand

| Bereich | Nachgewiesener Stand | Offener Nachweis |
| --- | --- | --- |
| Kern, Match, Assets, Meta und Population | Die integrierten Rust-Verträge und lokalen Nachweise liegen vor. Q arbeitet am aktuellen Kern und dessen Verdrahtung. | Aktueller Provider-Shadow, SLO/Lastprofil, gebundener Datenstand und Live-Verifikation des G5-Kandidaten. |
| Wiki | Store- und Releasepfad sind lokal geprüft. Der separate Wiki-Auftrag läuft und ist geschützt. | `ENDE.md` oder Abschlussbericht auf aktuellem main, Lizenz-/Rechtebindung und echter Wiki-Pilot. Die dokumentierte Ausnahme gilt frühestens am 04.10.2026 um 14:08:55 UTC. |
| Provider | Betreiberentscheidung: bereits freigegebenes neuestes stabiles DeepSeek Flash bei Fireworks über den zentralen Weg. Die Patchnotes-Übersetzung bleibt nach Auftrag bei Perplexity `sonar-pro` und dem vorgegebenen Prompt. | Q-Shadow-Bericht und dessen freigegebene Messgrundlage; kein neuer Anbieter oder eigenmächtiger Modellwechsel. |
| Replay | R führt Replay V1 über bestehende berechtigte Demowege fort. | Echter Korpus und Decode-/Store-Beweis oder ehrliche Grenze. Ein fehlender berechtigter Demoweg blockiert G5 nach Auftrag nicht. |
| Consumer | Twitch #984 ist gemergt. K arbeitet an produktiven Bots-, Docs- und 2nd-Brain-Anbindungen. | Tatsächliche Consumer-Anfragen mit Serverereignis, Request-ID, passender Wissensfreigabe und privatem Transport für 2nd-Brain. |
| Patchnotes und Steam | P und S arbeiten auf übernommenem Rust-Bestand weiter. Die älteren Provider-/Vertragsprüfungen sind historische Nachweise. | P-/S-Übergaben mit Merge-SHA, effektiver Unit, Datenziel und echter Publisher-/Publish-Wirkung. |

## Gate-Status

| Gate | Status | Begründung |
| --- | --- | --- |
| G0 | teilweise belegt | Inventar, September-Archiv und aktuelle Laufzeitaufnahme sind Zeitstände. Q-SLO/Lastprofil sowie ein frisch gebundener Legacy-Datenübergang bleiben offen. |
| G1 | bestanden, lokal und historisch | Die Verifikation gehört zu `022f8a9`; sie wird nicht als neue Verifikation von `511a347` ausgegeben. |
| G2 | teilweise | Echter Wiki-Pilot, aktueller Provider-Shadow und Replay-Korpus beziehungsweise dokumentierte Replay-Grenze fehlen noch. Die Betreiberentscheidungen zu Provider und Replay stehen im Auftrag. |
| G3 | teilweise | Gesamtdatenfrische, einzelne Writerwechsel und produktive Consumer-Beweise fehlen. Aktive P/S/Q/K-Arbeit ersetzt keinen Live-Nachweis. |
| G4 | bestanden, lokal und historisch | 1.011 Tests und die damaligen Lastläufe gehören zu `022f8a9`. Der vollständige Rückweg des aktuellen Daten-/Consumer-Cutovers ist noch nicht bewiesen. |
| G5 | offen, Betreiberentscheidung erteilt | Noch kein Production-Cutover. Voraussetzungen und reversible Schrittfolge stehen im Runbook. |
| G6 | offen | Beobachtungsfenster und Legacy-Ende wurden nicht gestartet. Ein fehlerfreier vollständiger Timer-Tageszyklus nach G5 ist Voraussetzung. |

## Vor G5 zu schließen

1. P/S/Q/K müssen ihren Abschluss oder eine ausdrücklich nichtblockierende Grenze mit SHA und Live-Beweis ablegen. Wiki-Abschluss oder die zeitgerecht dokumentierte Ausnahme bleibt zusätzliche Startbedingung.
2. Ein vorhandener Brain-Release-Weg muss aktuellen Remote-main, Buildherkunft und Deploy-flock belegen. CLI, Server und Maintenance werden gemeinsam betrachtet. Es wird kein Twitch-Installer für Brain verwendet.
3. Der Übergang vom fortgeschriebenen DL-Main-Bestand über das isolierte Archiv in den Kern braucht einen frischen gebundenen Snapshot und einen Rückweg mit Bestandserhalt. Das September-Archiv reicht dafür nicht.
4. Q-SLO und K-Consumeranfragen werden nach dem Wechsel erneut geprüft. Alte Writer werden erst nach erfolgreicher Beobachtung endgültig deaktiviert; ersetzte neue Rust-Units unter gleichem Namen bleiben aktiv.

## Historische Verifikation

Die vollständige lokale Verifikation vom September lief auf `022f8a981c2164f6d8d4302bae2194e100c4f65c`, damals integriert über PR #59. Format, Clippy mit `-D warnings`, Workspace-Tests und Release-Build endeten jeweils mit Exit 0: 1.011 passed, 0 failed, 75 ignored, 0 filtered. Acht zuvor ignorierte Tests bestanden separat und sind nicht zu den 1.011 addiert. Im Prozess-E2E wurden bei 8, 16 und 32 Workern jeweils 600/600 Antworten erzielt; beobachtetes Poolmaximum 4. Die unabhängige R5-Codeabnahme und das lokale Gesamtgate gehören zum früheren Head `72db816056fb0ed53810ab77ea4417dc0812e7ca`.

Die Werte 943, 958 und 980 Workspace-Tests sowie frühere E2E-Läufe gehören zu weiteren früheren Codeheads. Sie sind keine frische Abnahme des aktuellen main. GitHub Actions sind kein Merge-Gate; die früheren CI-Ausfälle sind historische Werkzeugbefunde und werden nicht zur G5-Freigabe umgedeutet.

Details: [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md), [FINAL_LOCAL_INTEGRATION_REVIEW.md](FINAL_LOCAL_INTEGRATION_REVIEW.md), [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md). Datenkopie und Instanz: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md). Pfadverantwortung: [PFAD_OWNER.csv](PFAD_OWNER.csv). Gate-Daten: [GATES.csv](GATES.csv).
