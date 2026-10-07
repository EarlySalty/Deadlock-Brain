# K: Isolierter Botaufgabenvertrag

Worker ab6fbc939a468226d hat ausschließlich neue `brain-contracts/src/bot_tasks.rs` und `tests/bot_tasks_contract.rs` geschrieben. Keine bestehenden G-Dateien, Exports, Manifeste, Provider oder Kernel geändert. Keine neue Titelroute und kein Titelgenerator.

Der Vertrag bindet Version, Request-ID, Plattform, Ziel und Ergebnisart. Öffentlicher Guide verwendet bestehende Query/PublicAnswerResponse. Serverseitige Registrierung, Principalrechte, Zielrechte, geprüfte öffentliche Herkunft und tatsächlicher Verarbeitungsort werden gesondert verlangt. Private/ungeprüfte Eingänge und unbekannter Verarbeitungsort scheitern geschlossen. Loopback-Remoteproxy zählt remote. Persönliche Hilfe und Titel sind reservierte, nicht verfügbare Fähigkeiten, ohne Eingabeschema oder Modellaufruf.

## Nachweise

K hat die gemeldeten Workerprüfungen nachgefahren, weil deren sichtbare Befehle keinen gehaltenen Buildslot nachwiesen. Eigener Lauf b0fp1ty5c, Slot 3, Exit 0; Log `/tmp/k-contract-verification-20261007.log`. Keine Lockänderung oder zusätzlicher Targetbau.

Befehle: Cargo 1.99, Manifest `/home/nathanael/.worktrees/brain-k-ki-20261007/rust/Cargo.toml`, vorhandenes Target `/home/nathanael/.cache/deadlock-brain-shared-flash-target`. Format `-p brain-contracts -- --check`; Compiler `--locked --jobs 3 -p brain-contracts --all-targets`; Clippy ebenso mit `-- -D warnings`; Tests `--locked --jobs 3 -p brain-contracts -- --include-ignored --nocapture`.

Ergebnis: Format/Compiler/striktes Clippy Exit 0. Bestehende Unitfälle 17, neue Botfälle 18, bestehende öffentliche Contractfälle 23; zusammen 58 passed, 0 failed, 0 ignored, 0 filtered. Kein Provider, Netzwerk oder produktiver Bot lief in diesen Vertragsfällen.

TESTNACHWEIS[TW-1]: 58 passed, 0 ignored | Baseline: 0 rot

## Anschluss bleibt offen

Das Modul ist nicht exportiert oder verdrahtet. Clientlabels und boolesche Freigaben sind kein Beleg realer Herkunft. Erst nach geprüftem integriertem G-Stand registrierten Dienst, tatsächliche Kanal-/Eingangsherkunft, Verarbeitungsort sowie erneute Zustellrechte anbinden. Kein Dummyerfolg, Direktmodellfallback oder zweite Providerimplementierung. Diese isolierte Vorbereitung erfüllt noch keinen KI-Cutover.
