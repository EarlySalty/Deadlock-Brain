status: aktiv, enges unabhängiges Nachreview zweier mechanischer Produktquellstellen
Datum: 2026-09-30

# Zwei Auto-Deref-Korrekturen auf d35a11c prüfen

Derselbe Reviewer52c34332-8cdf-4772-9e1f-42aba432c6cf, vorhandener Worktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, Branch review/pre-g5-core-abnahme-20260929. Intent562a877b. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen.

Quelle /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930. Tatsächlicher gepushter Fix d35a11ccfa790d2835e45ed8409910154ac13375 gegen b3523fa246c39249f42cd58343f1785445da0323, zwei Dateien,2 Einfügungen/4 Löschungen. Eigener Autorbericht folgt separat; nicht auf bewegliches WIP prüfen.

Dein GOdcfee1d für Synchronisationsfix8949198 bleibt bestehen. Danach exakt zugeteilter Paket-Clippy lief aufb3523fa, Exit101. Zwei neue Lints, beide durch Produktfixe878530 eingeführt, keine alte Baseline. Vollständiger Beleg SLOT-D-CLIPPY-NACHWEIS.md. Slot ist zurückgegeben; keine Runtimezuteilung.

Das vollständige semantische Delta:

- brain-storage/src/pg_jobs.rs:36: lock_source(&mut **tx, &lease.source_id) wird lock_source(tx, &lease.source_id).
- brain-storage/src/pg_release.rs:132: lock_source(&mut *tx, source) wird lock_source(&mut tx, source), Rustfmt fasst die vorhandene await/map_err-Kette in eine Zeile.

Prüfe ausschließlich: Vorschläge entsprechen dem tatsächlichen Clippy-Lint explicit_auto_deref; Deref-Coercion erreicht dieselbe mutable PgConnection innerhalb derselben Transaktion; Sperrreihenfolge/Scope und Fehlerweitergabe unverändert; keine Lintunterdrückung oder versteckte Begleitänderung. Kein erneuter Entwurf oder Vollreview akzeptierter B2-/Testteile. Produktquelle betroffen, deshalb dieses enge unabhängige Nachreview.

Nur statisch, keine Compiler, Tests, DB, Dienste oder Importe. Keine Code-Kommentare, keine Produktdateien ändern. Bericht CLIPPY-ZWEISTELLEN-NACHREVIEW.md mit GO/BLOCK und SHA im bestehenden eigenen Reviewbranch committen/pushen, dann Schluss. Nach fertigem Bericht kein Main-Merge oder Löschen trotz Stop-Hook; offene Branches sind bewusst geschützt. Bei GO genau denselben einen Paket-Clippy erneut beantragen, nicht selbst ausführen. Kein zusätzlicher Check, keine automatische Kette oder neue pauschale Freigabefrage.
