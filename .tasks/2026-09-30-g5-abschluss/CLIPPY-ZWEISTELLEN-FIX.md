status: aktiv, genau zwei konkrete Clippy-Auto-Deref-Befunde
Datum: 2026-09-30

# Zwei belegte Auto-Deref-Lints schließen

Bestehender Autor66adf9ee-bc03-4ff3-91da-73cd8efc5e72, unverändertes Modell. Eigener Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930, sauberer gepushter HEADb3523fa246c39249f42cd58343f1785445da0323. Intent562a877b-0939-440a-964d-1145d9e9431a. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen.

## Tatsächlicher neuer Befund

R1-Testquellenfix89491987 ist unabhängig mit dcfee1d statisch GO. Keine erneute Konkurrenztest- oder Produktarchitekturrunde. Der danach explizit zugeteilte einzelne Paket-Clippy ist beendet mit Exit101. Der Slot wurde sofort zurückgegeben; Cliptest wartet. Beleg SLOT-D-CLIPPY-NACHWEIS.md in dieser Akte, vollständiger Log /home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/brain-g5-clippy-b3523fa-slot-20260930.log.

```text
crates/brain-storage/src/pg_jobs.rs:36:17
lock_source(&mut **tx, &lease.source_id)
help: try: tx

crates/brain-storage/src/pg_release.rs:132:25
lock_source(&mut *tx, source)
help: try: &mut tx
```

Beide clippy::explicit_auto_deref unter -D warnings. Genau diese zwei unnötigen expliziten Dereferenzen anpassen, gleiche Quellsperren und Transaktionsgrenzen erhalten. Kein allow-Attribut, keine Lintabschwächung, keine sonstige Umgestaltung, keine Code-Kommentare. Bestehenden Speicherpfad verwenden, keine neuen Bausteine.

## Freigabepunkt

Nur Quellarbeit und gezielte statische Prüfung. KEIN Cargo/Compiler, Test, DB-/Rollenfixture, Import, Runtime oder Service. Der zugeteilte Einzellauf ist verbraucht und zurückgegeben; einen Wiederholungslauf teilt der Integrator gesondert zu. Kein neuer Cache, keine ENV-/Secretarbeit.

Eigenen Zweistellenfix und kurzen getrennten Bericht committen und auf demselben Featurebranch pushen. Tatsächliche SHAs, Diffumfang und statische Prüfungen melden, dann stoppen. Kein Main-Merge oder Branchcleanup trotz Abschluss-Hook. Bestehende Worktrees sind ausdrücklich als aktiv gegen Cleanup gesperrt, nicht entsperren. Keine neue Session, Modelländerung oder Reset. Bei unerwartetem Typ-/Scopeproblem konkret melden statt zusätzlichen Umbau oder Lauf selbst zu starten.
