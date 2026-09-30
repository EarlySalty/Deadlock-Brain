status: aktiv
Datum: 2026-09-30

# B1: vorhandene G5-Testharnesses an den Ressourcenvertrag binden

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a. Bestehender Sol-Thread 66adf9ee-bc03-4ff3-91da-73cd8efc5e72. Du bist der einzige Thread für dieses Paket. Keine Unterthreads oder Unteragenten.

## Ausgangspunkt und Scope

Eigener Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930, erwarteter sauberer HEAD ca4a8f236a75ba89ce60d2140c735acd5d1f5bee. Rust-/Lockbasis9a29b81 ist mit 954 passed/74 ignored, Clippy und Formatcheck geprüft. Diese Produktquellen nicht verändern.

Unabhängiger Reviewer ae2abe1a954eca03d863b1370df4a03496eac9b0 hat B1 konkret bestätigt. Bericht /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929/.tasks/2026-09-30-g5-abschluss/G5-NACHWEISABGLEICH.md, insbesondere Zeilen72-78. B1 ist die unmittelbar nächste Quellvorbereitung des bestehenden G5-Auftrags. Kein neuer Backend- oder Modellbau.

Erlaubter Änderungsscope: vorhandene Test-/Buildrunner scripts/test_brain_serve.sh, scripts/test_brain_core_postgres.sh, scripts/test_brain_storage_upgrade.sh, scripts/test_wiki_runtime.sh und scripts/run_local_pilot.sh sowie ein knapper eigener B1-HARNESS-ERGEBNIS.md in der aktuellen Taskakte. Bestandssuche zuerst Graphify. Kein neuer paralleler Runner; vorhandene Bausteine reuse. Keine neuen Python-/Shell-Hilfsprogramme oder produktiven Features. Keine Code-Kommentare; vorhandene Kommentare nicht erweitern. Kein Refactoring außerhalb des nötigen Argument-/Cargo-Pfads.

## Konkreter nachgewiesener Befund

- test_brain_serve.sh:55-84, test_brain_core_postgres.sh:46-50 und test_brain_storage_upgrade.sh:59-64 überschreiben CARGO_TARGET_DIR mit ROOT/rust/target und setzen zwei Cargo-Jobs.
- test_wiki_runtime.sh:51-60 startet ebenfalls Cargo mit zwei Jobs.
- run_local_pilot.sh:38-43 kompiliert vor Phasen und enthält keine Offlinevorgabe.
- Zugelassener vorhandener Targetcache: /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target. Cargo-Home /home/nathanael/.cargo. Kein neuer Cache.
- Erlaubte spätere Compilerklasse: genau ein Cargo-Job, Toolchain1.97.1, locked/offline. Diese Quelleinstellung darf keine zweite versteckte Cargoaktion parallel starten.

## Gewünschte enge Änderung

Die bestehenden Runner müssen den vorhandenen Targetcache ausdrücklich per CLI-Argument entgegennehmen und vor einer Clustererzeugung prüfen, dass er als absoluter bestehender Pfad vorliegt. Kein heimlicher Rückfall auf ROOT/rust/target und kein mkdir eines neuen Caches. Bestehende Cargo-Executable-Argumente, insbesondere Upgrade-Harness, berücksichtigen. Ein Job, locked/offline in sämtlichen Cargoaufrufen einschließlich Pilotvorbau; klare Argument-/Usagefehler vor Initdb. Keine normale ENV-Konfiguration als Bedienweg hinzufügen. Bereits vorhandene interne Übergabe synthetischer Testkoordinaten an isolierte Kindprozesse erhalten und offen dokumentieren; keine echten Credentials oder produktiven Endpunkte.

Die bisherigen Scratch-Isolationsprüfungen, Peer-/SCRAM-Fixtures, Testfilter, Assertions und Cleanup unverändert stark erhalten. Keine Tests herausfiltern, kein --ignored auf den Workspace, keine Budgets oder max_connections erhöhen. Keine eigenen Quelle-/Lizenz-/Modellfreigaben. Der Serve-E2E enthält fest 600 Anfragen je8/16/32 Worker; diese tatsächliche Last im Bericht nennen und nicht ausbauen, abschwächen oder als leichter Smoke umbenennen. Eine spätere Runzuteilung muss diese Last separat decken.

Der neu gefundene Produktivimport-Grenzfall (brain-legacy-import erlaubt nur brain_pilot und verbietet gleiche Quell-/Zieldatenbank) gehört **nicht** zu B1. Kein Importer-/Schema-/Produktcode-Fix in diesem Paket. Die reale DB brain hat Schema2/storev2 und korrekte brain_service-Grants, aber keine Sources/Releases; keine DBaktion daraus ableiten.

## Verifikation und Abgabe

Nur Quellprüfung, Argumentpfade statisch und bash -n für die geänderten vorhandenen Runner. Kein Harnessaufruf, auch kein vermeintlich harmloser --help-Aufruf, falls vor Argumentprüfung Initdb laufen könnte. Kein Cargo, Build, Test, Fetch, DB-, Last-, Modell-, Dienst- oder Deploylauf. Der Integrator baut Twitch-Releases. Keine ENV-/Credentialdateien lesen. Keine neue Wache.

Vor Abgabe Diff/Syntax selbst prüfen, exakte später ausführbare Befehle pro Harness mit absolutem vorhandenem Cache und Toolchain nennen, tatsächliche lokale Prozesse/Clusterports/Last/Cleanup aus den Quellen nennen. Keine Tests als bestanden behaupten. Unabhängige statische Abnahme folgt im vorhandenen Reviewerthread vor einer Runzuteilung; kein eigener zusätzlicher Modell-/Gateprozess.

Nur eigene Änderungen committen und auf den eigenen Branch pushen, kein Main-Merge, kein fremder Checkout. Im Bericht neuer SHA, getestete Rustbasis unverändert oder tatsächlicher Diff, statische Prüfergebnisse und Grenzen. Abschluss als B1-Quellvorbereitung, nicht G5-fertig. Bei echtem Blocker: [Bump-up] Paket B1: Grund: ... Erledigt: ... Worktree: ... Offen: ... an den Intent-Thread melden und stoppen.
