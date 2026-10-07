# A-F1: Git-Dienstfehler und Spielwissen Stufe 1

## Ziel und Vertrag

Nativer Blatt-Worker im Paket A, keine weitere Delegation. Lies `../AUFTRAG.md`, `../BRIEFING-A.md`, `PAKETE.md`, `STAND.md` und `INVENTUR-WISSEN.md`. Verantwortlich ist Paket-A-Session `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`.

Ursache zuerst: Neun reale Ticks scheitern am GameTracking-Pin `8c7cf4e` im Schritt „Git-Dateiliste“, gleiche Git-Abfrage außerhalb der Unit erfolgreich. Source-Scopes sind bereits vorhanden. Nicht erneut Scope- oder Allokationsfixe auf Verdacht bauen. Dieselben echten Quellen und den Dienst-Ausführungskontext prüfen, tatsächlichen Unterfehler erfassen und kleinsten nötigen Rust-Fix umsetzen. Der vorhandene Profilweg muss danach Quittungen und Steckbriefdokumente erzeugen und über den vorhandenen Standardaktivierungspfad für normale Consumer bereitstellen. Keine zweite Pipeline, keine Handkorrektur von DB-Zuständen.

## Eigentum und Stand

Eigener neuer Worktree `/home/nathanael/.worktrees/brain-a-profile-20261006`, Branch `fix/brain-a-profile-20261006`, vom frisch geholten `origin/main` in `/home/nathanael/repos/Deadlock-Brain`. Startbasis der Inventur `d6131cc52711a3e8b02d299704244f8d7dbdbce6`. Vor Erstellung prüfen, dass Pfad/Branch nicht fremd belegt sind. Nicht EnterWorktree verwenden; Befehle und Edits über literale absolute Pfade.

Eigentum: `rust/crates/brain-maintenance/src/integration/`. Bestehende `rust/crates/dbrain-sources/src/git_source.rs`, `game_files.rs`, `knowledge_contract.rs`, `knowledge_import.rs`, `entity_profile*.rs`, `entity_binding.rs` nur bei konkret belegtem Bedarf. Nicht an `deadlock-brain/src/main.rs`, Site-Bin/Cargo.toml, `dbrain-enrich` oder Bots schreiben. Zusätzlichen benötigten gemeinsamen Pfad zuerst im Bericht benennen und stoppen, nicht heimlich übernehmen. Rust, keine neuen Code-Kommentare, keine globale Formatierung.

Eigene Featurecommits und Featurepush erlaubt. Kein Brain-main-Push, kein produktiver Tick, keine produktive Konfigurationsänderung, Migration, Restart oder Deploy durch diesen Worker. Paket A übernimmt gemeinsame Integration und Betrieb. Alte Branches/Worktrees und deren wertvolle Artefakte nur lesen; vorhandene brauchbare Arbeit selektiv weiterverwenden.

## Beweis und Stop

Graphify vor Bestandssuche und Impact. Bestehende Tests/Fixtures wiederverwenden, echte vorhandene Quellen statt erfundener Daten. Eigener Scratch-Postgres und eigener Raw-/Corpuspfad, keine Wiederverwendung produktiver raw_dir als Schreibziel. Produktivdaten nur read-only. Compiler, gezieltes fmt, Clippy und vorhandene passende Suites prüfen; echte Testzahlen nennen. Bestehende Rechte-/Quittungs-/Original-/Modellgrenzen und Raw-Sperren erhalten. Ein erfolgreicher Import ohne Quittung/Dokument/Aktivierung erfüllt das Ziel nicht. A-F3 liefert die Site, daher keine parallele Siteimplementierung.

Bei verifiziertem Eigenstand normal `gate_hook.py --review` gegen den frisch bestätigten Mainvorfahren. Gate ist einziger Reviewer. BLOCK als vollständige Mängelliste mit Urteil/Modell und Originalpfad zurückgeben, nicht selbst mehrere Reviewer oder weitere Fixer starten. Fachfixer folgt mit frischem Kontext durch Paket A. Ein technischer Ausfall ist kein ALLOW. Commitnachricht endet mit `Co-authored-by: GPT 6.1 Sol <modell@local>`.

## Übergabe

Versuch 1, Statusproduzent der Worker selbst, Wache durch Paket A nach 20 Minuten. Nur native Rückgabe: Worktree, Branch, Basis/HEAD, betroffene Dateien, konkrete Ursache und Zwilling, Befehle und Zahlen, Gatewortlaut/Exit/Modell/Log, noch nötige Betriebsaktion. Keine root-Akte, TODO.md, C/OFFEN.md oder AN_HAUPT-A.md editieren. Quellen und Nutzer-/Community-Daten nicht an zusätzliche Modelle senden, keine Secrets ausgeben. Bei echten Produktentscheidungen oder Außenwirkung ohne Freigabe präzise stoppen; unabhängige Pakete laufen weiter.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-profile-20261006
