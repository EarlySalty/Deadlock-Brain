# Discord-Fixrunde 1: gebaut, Gatewerkzeug ohne Urteil

Rückgabe A-F2c am 07.10.2026. Eigener Worktree `/home/nathanael/.worktrees/brain-a-discord-20261006`, Branch `fix/brain-a-discord-20261006`, sauber. Feature-SHA `b29a55a4c147e0d40b47eca2678d2aee49ea4923` remote gesichert. Kein Main-Merge, Deploy oder Neustart.

Nur `rust/crates/dl-brain/src/brain_api.rs` geändert. Rekonstruierter Text wird nach dem Linkentfernen erneut bis zum linkfreien Ergebnis geprüft. Vier Gatebeispiele, gemischte Wrapper und 32-fache Verschachtelung abgedeckt. Beide Antwortstatus, Leerfall, UTF-16-Grenze und Ereignisidentität bleiben erhalten. Zwei Regressionstests vor dem Fix rot, danach grün.

## Prüfungen und Grenzen

Bibliothekslauf 38 bestanden statt zuvor 36, keine ignoriert. Antwortprüfungen 6 bestanden. Compiler und Dateiformat Exit 0. Normales Clippy Exit 0, vier unveränderte Warnungen; strenger Lauf vorher/nachher Exit 101 mit identischen Diagnosen. Workspaceformat vorher/nachher dieselben 83 Abweichungsblöcke.

Zusätzlicher breiter Paketlauf 37 bestanden, ein Scrim-Test mit `Elapsed(())` rot. Unveränderte Central-Bibliothek separat: 20 bestanden, derselbe Scrim-Test rot. PG-Integration 16 bestanden, ein Fehler wegen fehlender privater Infisical-Momentaufnahme im bestehenden Migrator; nach diesem Abbruch keine weiteren Tests ausgeführt. `core_users_roundtrip`: fünf bestanden. Eigene Scratchinstanz entfernt. Logs unter `/tmp/brain-a-discord-f2c-20261006/`.

## Gate

Basis `e1f11614e437d5e4e5610f5a9c5d997913f292ad`, erster Lauf Exit 2, kein Urteil. Konfigurierte Kette unverändert. Alle drei Wrapper scheiterten mit `unshare: unshare failed: Cannot allocate memory`. Log `/tmp/brain-a-discord-f2c-20261006/f2c-gate.log`.

A prüfte danach sauberen Worktree, HEAD und frisches Main. Der exakt eine reguläre Wiederanlauf des unveränderten Kandidaten bestand, Exit 0: `[gpt-6.1-sol] ALLOW: No confirmed merge-blocking defect.` Log `/tmp/brain-a-discord-f2c-20261006/f2c-gate-retry.log`. Kein Wechsel des Kritikers, keine Umgehung. Nichtblockierender Hinweis: wiederholtes Linkscannen kann quadratisch sein, tatsächliche gemeinsame Clientantwortgrenze noch nachlesen. Keine weitere Sourceänderung.

TESTNACHWEIS[TW-1]: 38 passed, 0 ignored | Baseline: 0 rot im Bibliothekslauf; zusätzliche rote Läufe oben

MERGEPROTOKOLL[MS-1]: 17 Git-Schritte einzeln | Anläufe: 1 | Gate: kein Urteil, Werkzeugfehler, Exit 2
