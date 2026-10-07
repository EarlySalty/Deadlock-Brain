[Orchestrator] Paket S. Worktree /home/nathanael/.worktrees/sheet-sync-exit-20261005, Branch fix/sheet-sync-exit-20261005. Freigabe erst nach fremdem Prüfer. Kein Push, kein Deploy.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/sheet-sync-exit-20261005

# Paket S: Sheet-Sync meldet Erfolg bei fehlgeschlagenen Schritten

Rolle: Blatt-Worker. Du startest keine weiteren Threads und keine Subagenten für denselben Schreibpfad.

Auftraggeber: Kopf, T3-Thread 31575951-23e7-4dd9-b1af-8700f7ff45fe.
Bericht: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/AN_HAUPT-S.md

## Ziel

`deadlock-brain-sheet-sync.service` ist am 05.10.2026 um 19:25:28 CEST mit systemd-Result `success` und Status 0 zu Ende gegangen. Im Journal desselben Laufs stehen danach trotzdem diese Ergebnisse:

- `refresh-sheet` hat in `unmatched_heroes` zwei Werte, die mit `ERROR: Request failed for https://assets.deadlock-api.com returned code 500` beginnen. Das sind Texte aus dem Sheet, keine Heldennamen. Daneben stehen echte unbekannte Namen wie Baba und Doorman.
- `enrich patch-impact --limit 50` hat `{"failed": 50, "processed": 50, "success": 0}` gedruckt.
- `enrich meta-trends` hat für Abrams und Infernus `HTTP 404` mit `Model not found` gedruckt und `{"processed": 2, "success": 0, "failed": 2}` zurückgegeben.

Das Skript `scripts/run_sheet_sync_with_infisical.sh` hat `set -e`. Es ruft nacheinander `refresh-sheet`, `learn analyze-next --limit 20`, `enrich patch-impact --limit 50` und `enrich meta-trends`. Ein Schritt mit Exit 0 lässt den nächsten laufen. Deshalb wurde die Unit grün.

Auf `origin/main` geben `run_patch_impact_batch` und `run_meta_trend_analysis_with_chat` in `rust/crates/dbrain-enrich/src/lib.rs` ein `Ok` zurück, auch wenn `failed` größer als 0 ist. Die CLI druckt das JSON und beendet sich mit 0.

Soll: Nach dem gedruckten JSON endet der Prozess mit einem Fehlerstatus, sobald `failed` größer als 0 ist. `refresh-sheet` endet ebenfalls mit einem Fehlerstatus, wenn ein Eintrag in `unmatched_heroes` mit `ERROR:` beginnt. Gewöhnliche unbekannte Namen bleiben in der Liste und lassen den Lauf erfolgreich. Die Zeilen werden nicht gelöscht. `assets.deadlock-api.com` wird nicht erneut abgerufen. `learn analyze-next` bleibt unverändert.

## Modell

Der laufende Unit-Prozess kommt aus `/home/naniadm/Documents/Deadlock-Brain`, Commit `2734c2d` vom 24.09.2026. Dieses Binary sendet `accounts/fireworks/models/deepseek-v4-flash`. Daher die 404.

`origin/main` (`e56e075d`) liest das Modell aus `/var/lib/deadlock/llm-model-selection.json`. Dort steht aktuell `accounts/fireworks/models/deepseek-v4p1-flash`, geprüft am 05.10.2026 um 00:16 UTC. Diese Datei, der Resolver und der Modellkatalog bleiben unangetastet. Kein Fireworks-Aufruf, kein Probe, kein neues Modell.

## Eigentum

Worktree: `/home/nathanael/.worktrees/sheet-sync-exit-20261005`
Branch: `fix/sheet-sync-exit-20261005`
HEAD beim Start: `e56e075d486a75f83f4954b58d8113588082d3f1`

Schreiben darfst du nur in:

- `rust/crates/dbrain-enrich/src/lib.rs`
- `rust/crates/deadlock-brain/src/main.rs`
- `rust/crates/dbrain-normalize/src/sheet_stats.rs`
- die Tests direkt neben diesen Dateien

Fremde Änderungen, das Checkout `feat/brain-rust-cutover-20260919`, `/home/naniadm/Documents/Deadlock-Brain`, systemd-Units, `/etc/deadlock-brain`, Wartung, Scopes, Profile, Migrationen und Python bleiben unangetastet.

## Beweis

Ein Test mit einer vorgegebenen Chat-Antwort, die einen Fehler liefert, zeigt: Meta-Trends und Patch-Impact drucken die Zählung und liefern danach einen Fehler, sobald `failed` größer als 0 ist. Ein Sheet-Lauf mit einem `ERROR:`-Namen in `unmatched_heroes` endet fehlerhaft. Ein unbekannter normaler Heldenname endet erfolgreich. Bestehende Tests dieser Crates bleiben grün. Kein Live-Lauf, kein Dienstneustart.

## Abschluss

Kein Commit auf `main`, kein Push, kein Merge, kein Deploy, kein `systemctl`. Bericht mit Branch, SHA, Testbefehl und Ergebnis nach `AN_HAUPT-S.md`. Der Kopf gibt den Stand danach an einen fremden Prüfer.
