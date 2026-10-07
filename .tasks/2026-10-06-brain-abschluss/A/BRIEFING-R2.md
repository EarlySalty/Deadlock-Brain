# A-R2: engsten ENV-Testbefund auf Feature korrigieren

## Ziel und Vertrag

Direkter neuer Orchestratorauftrag im laufenden Brain-Abschluss: Der neu hinzugefügte Test enrichment_cli_exits_on_failed_batches_and_keeps_their_summary in deadlock-brain/tests/runtime_tooling.rs nutzt DSN, Datenpfad und Fireworksendpoint über ENV entgegen dem Workspacevertrag. Ursache prüfen und eng korrigieren, bestehende Prüfung und realen CLI-Exit-/Persistenznachweis erhalten. Keine neuen Config-ENV, keine Secretwerte/DSNs in Logausgaben, kein Sonderprovider oder Modell-/Timeoutwechsel, keine Produktiv- oder Nutzer-/Communitydaten.

Gemeinsames Releasefenster ist verbindlich: fremder Live-Agent hat alleinigen Build-/Install-/Restart-/Tickbesitz und führt freigegebene fde910f6-Recovery aus. A-Main 8d61a949 bleibt unverändert und ist vorerst nicht aktiviert. Du darfst nur einen geprüften Featurecommit vorbereiten und gegebenenfalls regulär auf genau deinen Featurebranch sichern. Kein Main-Push, Merge nach main, Releasebuild, Install, Restart, Tick, Konfig-/Writeränderung oder Recoveryaufruf.

## Eigentum

Eigener sauberer Worktree /home/nathanael/.worktrees/brain-a-runtime-config-20261007, Branch fix/brain-a-runtime-config-20261007, Start 8d61a949c9856a69543747b0e59dbfb5bbcbe440, Gitbaum 500040ac54a6e5d2e1a60cbe7ea6f52bf1ad9ff6. Primäres Eigentum nur rust/crates/deadlock-brain/tests/runtime_tooling.rs. Falls der bereits vorhandene explizite Config-/Infisicalweg im bestehenden CLI-Enrich-Einstieg eine minimale Verdrahtung benötigt, darfst du nur diese unmittelbar nötige Stelle und zugehörige eng bestehende Config-/Enrichfunktion anpassen. Keinen parallelen Configloader oder Connector bauen, keine Public-API-Brüche. Bei unvermeidlich größerem Eingriff exakten Befund und Lösungsrichtung an A melden, nicht Scope passend erweitern. Andere Sourcebereiche, Migrationsdateien, Profile/Maintenance, Invite, Site, Reasoner und Root-Akten tabu. Rust ohne neue Code-Kommentare.

## Bestand und Arbeitstand

Graphify zuerst. A-Abfrage runtime_tooling im vorhandenen Brain-Graph lieferte keine Knoten. Danach Fundstelle bestätigt: rust/crates/deadlock-brain/tests/runtime_tooling.rs:154-221, insbesondere 157 und 166-168. Der Test prüft leerer Batch Exit 0, tatsächlicher fehlgeschlagener Patchbatch Exit 1, gespeicherte Diagnose, erneuter Retry Exit 1 und Meta-Fehler Exit 1. Nicht löschen, abschwächen oder durch bloße Fakefunktion ersetzen. Git-/PATH-/Cargo-Werkzeugsteuerung und gezielte negative ENV-Override-Prüfung sind etwas anderes als neue produktive Config-ENV; den tatsächlichen Vertrag belegen statt pauschal jedes .env zu entfernen.

Vorhandene 8d-Integration bestand Format, Compiler, striktes Clippy und 126 reale Tests mit privater Scratch-PG. Die vorige Testharness-Durchführung mit ENV ist gerade der gemeldete Befund, kein Freifahrtschein. Der warme Debugtarget /tmp/brain-a-integration-proof-20261007/target ist aktuell keinem lebenden Compiler zugewiesen. Du darfst ihn exklusiv für die passende erneute Prüfung verwenden. Die alten Beleglogs und Ergebnismanifeste nicht überschreiben. Neue eigene Belege /tmp/brain-a-runtime-config-proof-20261007/. Keine fremden Targets oder PIDs; eigener PG, Loopback, TCP aus, ggf. vorhandenen eigenen gestoppten Harness korrekt fortsetzen statt kalt doppelt bauen. Test-/Datenzugang nur über vorhandenen expliziten sicheren Vertrag, keine Credentials im Tooloutput.

## Beweisziel und Gate

Passende bestehende Suite tatsächlich vollständig, include-ignored/test-threads=1 und ausgeführte DB-Prüfung nachweisen. fmt, Compiler, passende strenge Clippy-Prüfung ohne Unterdrückung. Rot exakt melden und selbst beheben, kein behaupteter Altfehler ohne Zahlengegenprobe. Keine pauschale breite neue Testsuite, aber die betroffene bestehende CLI-/Enrichprüfung darf nicht übersprungen werden. Befehl/Quelle/Fingerprint und passed/failed/ignored/filtered getrennt nennen, eigene Scratch-PG nach tatsächlichem Exit stoppen.

Nach verifiziertem Featurecommit regulären unveränderten Selbstgate ausführen: python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <eigener absoluter Worktree> --base <frischer tatsächlicher voller Main-SHA> --head <voller eigener Feature-SHA>. Genau ein Git-Schritt pro Bash-Aufruf mit literalen absoluten Pfaden, nur eigene Dateien, kein add -A, Forcepush, Reset oder Stash. Bei BLOCK Befund zurück an A für frischen Fixer, kein Modellroulette, kein eigenes Reviewer-Thread. Main bleibt unabhängig von ALLOW gesperrt. Nur lauffähiger grüner Featurebackup erlaubt.

## Routing

Blatt-Worker A-R2, alleiniger Ereignisproduzent, keine zusätzlichen Agenten/Reviewer/T3-Threads oder Sessionnachrichten. Auftraggeber Paket A 2c7de4c9-bac4-43ad-b91a-f8ac889f09b4; Hauptorchestrator 3fcd8f71-443e-48ae-825c-527eb52fbe56. Rückgabe direkt an A mit Ursache, Quellenanschluss, exakten Änderungen, SHA/Tree, Prüfungen, Gatewortlaut/Modell/Exit/Belegen und getrenntem offenen Main-/Runtimeabschluss. Root-Akte schreibt nur A. Wache 20 Minuten, spätestens 30; keine kurzen künstlichen Compilerabbrüche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-runtime-config-20261007
