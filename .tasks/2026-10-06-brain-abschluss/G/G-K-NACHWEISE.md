# G-K: Kernel-/Cache-Rückgabe und Fehlerabrechnungsgrenze

Stand: 07.10.2026. Workflow `wf_bade480e-cb2`, Task `w120utj39`, abgeschlossen. Produkt-WIP uncommittiert; noch kein Paketgate oder Laufzeitabschluss.

## Tatsächlicher Anschluss

`Kernel::new` bleibt kompatibel. Neuer `Kernel::with_tools(port, resolver, provider_identity)` und öffentlicher `GameContextResolver` mit verpflichtenden Methoden `resolve` und `validate`. Serverseitiger Anfrage-Pin entsteht vor Cache-/Flight-Schlüsselbildung. Bestehende Outcome-/Cachebausteine erweitert, keine SQL-Logik, kein zusätzlicher Werteleser oder zweiter Cache.

Geänderte Dateien: `brain-kernel/src/{lib.rs,execution.rs,flight.rs,cache.rs,outcome.rs}`. Die Acht-Tool-Fassung wird generisch getragen. Callzuordnung, konkrete Unteranfragen, sämtliche Quellen-/Rechte-/Publikationsabhängigkeiten vor Modellweitergabe, finaler Ausgabe und Wiederverwendung geprüft. Ursprüngliche Deadline und kumulierte Restbudgets bleiben gebunden; wiederholte Inputinhalte und Retries werden konservativ reserviert.

## Nachgeprüfter Prüfstand

Bereichsführung las tatsächliche Exitdateien und Testmarker. Format, Clippy, Compiler und damaliger Verbrauchercheck Exit 0. Gesamtsuite Exit 101: 49 passed, 18 failed, 0 ignored; Baseline ebenfalls Exit 101 mit 34 passed, denselben 18 failed, 0 ignored. Fehlernamensmengen sind gleich, keine hinzugekommene oder verschwundene fehlgeschlagene Prüfung. Getrennte Zustandsprüfung: 32 passed, 0 failed, 0 ignored, 4 filtered. Diese Teilprüfung wird nicht zur Gesamtzahl addiert. Neue Fachfälle insgesamt 15.

Rohbelege `G/pruefungen/g-k/`, insbesondere `baseline-complete.log`, `test-complete-r2.log`, `test-zustaende.log` und zugehörige Exits. Tests verwenden `--locked --offline --jobs 3`, Debugtarget `/tmp/brain-g-k-20261007-target`, vorhandenen Buildslot und `--include-ignored --test-threads=1`; Gesamtsuite zusätzlich `--no-fail-fast`. Isolierte Portfixtures belegen Kernelzustände, keinen echten G-V-/E-/Luna-Weg.

Sieben von acht Einträgen des Quellfingerprints stimmen bei Übernahme. Die fünf eigenen Kerneldateien sind unverändert. `brain-contracts/src/provider_input.rs` wurde seit der Prüfung vom inzwischen exklusiv gestarteten JSON-Worker erweitert; alter Verbrauchercheck ist dafür kein Integrationsbeweis. Die Fortsetzung muss gegen den abschließenden tatsächlichen Vertragsstand neu prüfen.

## Konkrete offene Lieferung

Fehler von `answer_turn` und Tool-`execute` tragen bislang nur `PortError`, keine gemessene Usage. Der Kernel kann den tatsächlichen Verbrauch fehlgeschlagener Aufrufe deshalb nicht vollständig im Antwortzähler ausweisen. Eine kompatible typisierte Fehlerabrechnung ist vor Produktabnahme nötig; keine erfundene gemessene Usage oder Schätzung als Ersatz. Diese Vertrags-/Provider-/Kernel-Fortsetzung wird erst nach tatsächlichem Abschluss des laufenden JSON-Vertragsworkers zugeteilt, nicht parallel in `brain-contracts`.

G-V-Dispatcher, E-Herkunftsanschluss und echte Luna-Brücke bleiben Integrationsaufgaben. Kein grüner Gesamtlauf, Main-Merge, Deploy oder Livebeweis behauptet.

TESTNACHWEIS[TW-1]: 49 passed, 0 ignored | Baseline: 18 rot
