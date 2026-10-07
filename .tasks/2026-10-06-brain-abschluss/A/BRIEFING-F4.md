# A-F4: Sheet-Batchfehler ehrlich melden

## Ziel und Vertrag

Nativer Blatt-Worker, keine weitere Delegation. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Lies `../AUFTRAG.md`, `../BRIEFING-A.md`, `INVENTUR-KERN.md` und `PAKETE.md`.

Belegter konkreter Fehler: Sheet-Sync-Lauf 19:25 meldet fehlgeschlagene Schritte und zwei Fireworks-404, endet aber mit Exit 0. Auch origin/main gibt Batch-Ergebnisse mit `failed > 0` erfolgreich zurück. Tatsächlichen gemeinsamen Rust-CLI-/Batchpfad prüfen, kleinsten Fix umsetzen: erfolgloser Batch darf nicht als erfolgreich gelten, Teilergebnisse und bekannte Diagnose müssen erhalten bleiben. Zwillingssuche an vorhandenen entsprechenden Batchaufrufen, keine neue Batcharchitektur. Keine stillen Exceptions oder neue pauschale Abbruchschwelle.

Provider-404 ist separat zu diagnostizieren, nicht durch Modellwechsel reparieren. Keine produktive Provider-, Schlüssel- oder Zeiteinstellung ändern. Wenn es ein Konfigurations-/Konto-/Produktproblem ist, nur genaue bestehende Einstellung und Fehlerklasse für Paket A nennen. Keine API-Keywerte oder private Eingaben ausgeben. Kein neuer LLM-Connector, kein zusätzliches Modell.

## Eigentum und Stand

Neuer eigener Worktree `/home/nathanael/.worktrees/brain-a-sheet-20261006`, Branch `fix/brain-a-sheet-20261006`, frisch von Brain origin/main. Inventurbasis `d6131cc52711a3e8b02d299704244f8d7dbdbce6`. Keine Sitzungsisolation per EnterWorktree ändern, literale absolute Pfade.

Nur `rust/crates/deadlock-brain/src/main.rs`, `rust/crates/dbrain-enrich/src/lib.rs` und unmittelbar zugehörige bestehende Tests. A-F3 besitzt Cargo.toml/Site-Bin, A-F1 besitzt Profil-/Gitintegration. Nicht an diesen Pfaden, Migrationen, produktiven Units, Bots oder alten Worktrees schreiben. Rust, keine neuen Code-Kommentare, keine globale Formatierung. Eigene Featurecommits und Featurepush erlaubt, keine Brain-main- oder Produktivaktionen.

## Beweis und Übergabe

Graphify vor Suche. Vorhandene Batchmodelle und Fixtures verwenden. Compiler, gezieltes fmt, Clippy und passende bestehende Suites. Die echte CLI muss bei bestätigtem fehlgeschlagenem Batch einen Fehlerexit liefern; erfolgreicher Batch und vorhandene Retry-/Teilergebnisverträge bleiben funktionsfähig. Keine neuen Tests als Selbstzweck, bestehende Tests nicht abschwächen. Testzahl und Exit sauber erfassen. Echte Nutzerdaten nicht an zusätzliche Modelle senden. Runtime-Konfigurationslesen nur minimal und ohne Secrets.

Nach geprüftem Eigenstand normal `gate_hook.py --review` gegen frisches Main. Gate einziger Reviewer; bei BLOCK Modell/Exit/volle Liste/Log zurückgeben, keine eigene neue Fixerrunde. Committrailer `Co-authored-by: GPT 6.1 Sol <modell@local>`. Paket A integriert und führt Releaseverdrahtung/Neustart/Live-Beweis aus.

Versuch 1, Worker produziert Status, Wache nach 20 Minuten. Native Rückgabe: Bestandsfund/Zwilling, Ursache, Worktree/Branch/Basis/HEAD, Diffdateien, konkrete Prüfkommandos und Zahlen, Gatewortlaut/Modell/Exit/Log, gesonderter Providerbefund und noch nötige Betriebsaktion. Root-Akte, TODO.md, C/OFFEN.md und Hauptberichte nicht schreiben.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-sheet-20261006
