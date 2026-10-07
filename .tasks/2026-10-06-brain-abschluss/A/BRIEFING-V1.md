# A-V1: kombinierten Kandidaten prüfen

Auftraggeber Paket A, Session `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Native Blattrolle Test-Wächter, keine Subagenten, T3-Threads oder Sessionnachrichten. Kein Reviewer; allein der bestehende Merge-Gate urteilt über den Code.

## Ziel und Stand

Eigener Integrationsworktree `/home/nathanael/.worktrees/brain-a-abschluss-20261006`, Branch `feat/brain-a-abschluss-20261006`, Kandidat `dcff5d9cb68d18c89fff8291d602352c6384f779`, remote gesichert. Enthält Main `10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef` und Original-Sheetfix `ca1166718b2e8a1956c30e6e9e4687484d64d74b`. Regulärer Gate im zweiten Versuch ALLOW durch gpt-6.1-sol; erster Versuch Werkzeugfehler. Kein Main-Push oder Deploy. Änderungen gegenüber dieser Mainbasis betreffen drei Rust-Dateien und eigene Akten.

F4 hat auf seinem ursprünglichen Stand 126 passende Tests mit Rust 1.97.1, frischem privatem Postgres und `--include-ignored --test-threads=1` bestanden. Quelle und Rückgabe: `A/F4-RUECKGABE.md`. Gemeinsame Compiler-/Testvalidierung des integrierten Stands steht noch aus.

## Eigentum

Source ausschließlich lesen, auch eigene Integration nicht ändern. Kein Commit, Push, Merge, Gate, Deploy, Restart oder produktiver Datenzugriff. Keine globale Formatierung. Private Prüfartefakte ausschließlich unter `/tmp/brain-a-integration-proof-20261007/`, privat anlegen, eigenen Cargo-Target und Scratch-Postgres dort. Keine Targets oder weitere Dateien in der Releasequelle. Vorhandene untracked A-Logs gehören dem Orchestrator und bleiben unangetastet. Root-Akten schreibt nur A. Graphify vor Codefrage, vorhandenen Scratch-/Prüfvertrag wiederverwenden.

## Beweis

Mit höchstens zwei Cargo-Jobs passende fmt/check/clippy und bestehende Suites für deadlock-brain und dbrain-enrich am exakten Kandidaten ausführen. Brain-serve und brain-maintenance zumindest gemeinsam kompilieren, da dieser Mainstand ausgeliefert werden soll. Repo-Prüfversion verifizieren; F4 verwendete Rust 1.97.1. Keine Quellen an einen Compiler anpassen. Keine produktiven Provideraufrufe oder Infisical-/Communitydatenabfragen. Private echte PostgreSQL-Instanz mit eindeutig eigener Socket-/Portwahl; 5433/5434 tabu. Prüflauf vollständig, keine Ausgangscode maskierenden Pipes. Ignorierte Bestandstests einschließen, seriell. Fehlender Scratchzugang ist keine Erlaubnis zu Null-Läufen. Nichtblockierende Gate-NIT betrifft genaue globale Queuezahlen, daher tatsächlich frische DB und serielle Tests.

Bei rotem Lauf Fehler und Umfang melden, nichts unterschlagen oder als vorbestehend behaupten ohne echte Zahlenbaseline. Keine Codefixes, präzise Rückgabe an A. Scratch sicher stoppen, Beweislogs erhalten.

## Rückgabe

Kandidat-SHA, genaue Befehle und Env ohne Secrets, Compiler-/fmt-/Clippy-Exit, passed/failed/ignored/filtered, Artefaktpfade und Scratch-Cleanup. Keine Fertig-/Deploybehauptung. Wache nach 20 Minuten, spätestens 30. Nur native Rückgabe, nicht AN_HAUPT-A.md beschreiben.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-abschluss-20261006
