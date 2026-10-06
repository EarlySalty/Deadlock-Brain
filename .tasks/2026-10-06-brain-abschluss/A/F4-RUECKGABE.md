# A-F4: Batch-Exit korrigiert, ALLOW

Native Rückgabe vom 06.10.2026. Feature `ca1166718b2e8a1956c30e6e9e4687484d64d74b` auf `origin/fix/brain-a-sheet-20261006` gepusht, eigener Worktree `/home/nathanael/.worktrees/brain-a-sheet-20261006` sauber. Basis d6131cc5, frische Gatebasis 10ebbb208aaaac9ddf6e07bc89a6af4a620f86ef. Keine Main-/Deploy-/Restartaktion durch F4.

## Wirkung

Gemeinsamer Batchabschluss schreibt vollständige Ergebnisse, danach Exit 1 bei Fehlern statt falschem Erfolg. Zwillingspfade `learn analyze-next`, `player analyze-next`, `enrich patch-impact` einschließlich Einzelheld und `enrich meta-trends` angeschlossen. Gespeicherter Status analysis_failed zählt nicht als Erfolg; fehlender/leerer/ungültiger Providertext wird als Fehler gespeichert. Teilergebnisse, Diagnose und Wiederaufnahme bleiben erhalten, erfolgreiche/leere/Dry-run-Batches bleiben erfolgreich. Bestehender Runner mit set -euo pipefail braucht dafür keinen weiteren Codefix.

Nur drei Dateien: deadlock-brain/src/main.rs, dbrain-enrich/src/lib.rs, deadlock-brain/tests/runtime_tooling.rs; 282 hinzugefügt, 28 entfernt.

## Prüfungen

Repo-Prüfversion Rust 1.97.1: eigene Dateiformatprüfung, cargo check und striktes Clippy mit -D warnings Exit 0. Vollständige passende Suites mit include-ignored und test-threads=1 gegen frischen eigenen Unix-Socket-Postgres: 126 bestanden, 0 fehlgeschlagen, 0 ignoriert, 0 gefiltert. CLI-Prozess bei Patch-/Meta-Fehlern Exit 1 samt JSON, leerer Batch Exit 0, Teilbatch 1 Erfolg/2 Fehler, Wiederaufnahme verarbeitet beide erfolgreich und bewahrt Diagnose. Scratch gestoppt.

Erster Scratchlauf 3 bestanden/4 rot wegen selbst falscher DSN EmptyHost; korrigierter voller Lauf grün. Zusatzclippy mit Rust 1.99 rot wegen vier map_or_identity-Lints außerhalb der Hunks, keine Unterdrückung und keine unbelegte Altbaselinebehauptung. Logs in rust/target/a-f4-evidence/ im eigenen Worktree vor Cleanup sichern.

## Gate

Regulärer Aufruf `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-a-sheet-20261006 --base origin/main`, Exit 0.

Wortlaut: `[claude-opus-5-5 (nach Ausfall von gpt-6.1-sol: review_gate: codex failed: bwrap: Creating new namespace failed: Cannot allocate memory)] ALLOW: Kein mergeblockierender Defekt im Diff erkennbar.`

Automatischer technischer Rückfall, kein BLOCK-Neuwürfeln. Danach kein Sourceedit. Nichtblockierende Testhinweise: Patchfixture setzt genau drei offene Ziele voraus, CLI-Fixture leere DB; tatsächlicher Lauf mit frischem Scratch und serieller Testausführung. Gate-Log rust/target/a-f4-evidence/gate.log.

## Offener Betriebsvertrag

Sheet-Unit läuft weiterhin checkoutgebunden, zuletzt Exit 0 trotz Fehlern. Nach Integration aus geprüftem Release verdrahten und echten Exit-/Journalbeleg erbringen. Provider separat offen: HTTP 404, NOT_FOUND/param model, „not found, inaccessible, and/or not deployed“. Zentrale vorhandene Auswahl fireworks/accounts/fireworks/models/deepseek-v4p1-flash, zuletzt geprüft 05.10.2026 00:16:30 UTC. Keine Änderung von Provider, Modell, Schlüssel oder Zeiteinstellungen, kein zusätzlicher echter Provideraufruf durch F4. Antwort allein beweist nicht, welche der drei Providerursachen vorliegt.

TESTNACHWEIS[TW-1]: 126 passed, 0 ignored | Baseline: nicht erhoben

MERGEPROTOKOLL[MS-1]: 14 Git-Schritte einzeln | Anläufe: 1 | Gate: ALLOW durch claude-opus-5-5 nach technischem Ausfall der ersten Stufe
