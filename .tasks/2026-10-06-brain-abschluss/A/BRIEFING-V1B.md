# A-V1b: neuen gemeinsamen Brain-Stand im warmen Prüftarget abschließen

Native Test-Wächter-Blattrolle. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`, Hauptorchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine weiteren Agenten oder Threads, keine Nachrichten an andere Sessions, keine eigenen Reviewer. Root-Akten schreibt A.

## Ziel und vorhandener Beweis

A hat dcff5d9cb68d18c89fff8291d602352c6384f779 vollständig geprüft: Rust 1.97.1, Compiler und striktes Clippy Exit 0, 126 echte Tests mit privater PG und include-ignored/test-threads=1 grün. Belege und exakter Befehl in `A/V1-RUECKGABE.md`. Der reguläre Gate hatte ALLOW.

Beim frischen Fetch ist Main inzwischen fde910f6a0199c00f44083e73fc8f4c5e4f80b86. A hat diesen Stand normal in die eigene Integration gemergt. Neuer Kandidat `1e925d5fd6852e855a09cfdc47051f979491c6ff`. Genau drei zusätzliche Main-Dateien: brain-maintenance/src/integration/entity_profiles.rs, brain-storage/src/pg_jobs.rs, brain-storage/src/pg_release.rs. Keine eigene erneute Fixarbeit. A führt den normalen Gate parallel selbst aus, keinen weiteren Review starten.

## Eigentum und Prüfweg

Originalworktree `/home/nathanael/.worktrees/brain-a-abschluss-20261006` und Branch feat/brain-a-abschluss-20261006 ausschließlich lesen. Kein Sourceedit, Commit, Merge, Push oder Runtimeeingriff. Untracked Logs gehören A. Fremde A-/Kanon-/T3-Bäume tabu.

Vorhandene private exakte Prüfkopie `/tmp/brain-a-integration-proof-20261007/source/Deadlock-Brain/`, Geschwisterkopie Deadlock-Bots und warmer Cargo-Target `/tmp/brain-a-integration-proof-20261007/target` weiterverwenden. Keine neue Sourcekopie, kein neuer Target und kein Cachelöschen. Zunächst bisherigen privaten Sourcezustand mit dem belegten dcff vergleichen, danach nur die drei nachweislich neuen Main-Dateien in der privaten Prüfkopie auf den exakten neuen Gitstand bringen. Alle 1325 Dateien abschließend mit dem eigenen Original vergleichen. Diese mechanische Kopie ist kein Produktfix. Bei unerwarteten Mismatches melden statt fremde Arbeit überschreiben.

Compiler, striktes Clippy und dieselbe vollständige Suite auf dem exakten neuen kombinierten Stand nachhalten. Dieselbe Rust/Cargo 1.97.1, locked/offline, ein Cargo-Job, vorhandene commands.json und erfolgreiche continued-Logs als Vorlage. Alle erforderlichen DSN bleiben ausschließlich an den eigenen privaten PG gebunden, keine Infisical-/Prod-/Providerabfragen.

Der private PG-Cluster `/tmp/brain-a-integration-proof-20261007/pg/cluster`, Socket `/tmp/brain-a-integration-proof-20261007/pg/socket`, Port 56187, User brain_a_test, TCP deaktiviert, wurde nach der erfolgreichen Suite durch A gestoppt. Vor neuer Suite wieder die tatsächliche private Identität prüfen. Vorhandene Suites haben globale Queuenutzungsannahmen: echten frischen privaten Datenbankzustand herstellen, keine Fixtures gegen Prod laufen lassen und keinen fehlenden Harness als Produktbug melden. Nach tatsächlichem Testexit eigenen Scratch wieder stoppen, Belege erhalten.

## Abschluss und Zeitregel

Keine eigenen 70-/140-/180-Sekunden- oder 20-/30-Minuten-Abbruchbudgets. Solche Abbrüche haben zuvor lediglich kalte Compiler beendet. Lange Befehle als native Hintergrundtasks starten und bis zum tatsächlichen Exit nachhalten. Die Wache nach 20, spätestens 30 Minuten ist Fortschrittskontrolle, kein Ende des Auftrags. Kein Tooltimeout als selbst auferlegte Aufgabenfrist. Warme Prüfläufe weder stoppen noch kalt ersetzen.

Neue Nachweise in derselben vorhandenen privaten Prüfablage, eindeutig `main-fde-...` benennen. Ergebnis an A mit exaktem HEAD/Sourceidentität, vollständigen Befehlen, Compiler-/Linterexit, passed/failed/ignored/filtered und privatem PG-Stop. Kein grüner Null-Lauf und keine unbelegte Baselinebehauptung. Nicht bei gestarteten Compilerprozessen als fertig zurückgeben.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-abschluss-20261006
