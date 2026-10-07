status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-r

[Orchestrator] Paket R: Replay V1

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig.

## Ziel

Replay-Decoder und Reportpfad liegen laut STATUS.md nur offline vor, ohne freigegebenes `.dem`. Replay ist Teil von V1 (AUFTRAG.md Entscheidung 5): echte Demos über bestehende berechtigte Wege beschaffen, dekodieren, Ergebnisse über `SourceRecordV2` in den normalen Store schreiben und aus Brain abfragbar machen.

## Beschaffung (nur berechtigt)

Mögliche Wege prüfen: Match-Salts und Replay-URLs aus dem Steam-Bot (Memory: Salts klappen nur zu rund 10 %, nicht neu drosseln lassen, Valve-GC nicht überlasten), öffentliche Demo-Links der Deadlock-API, eigene Matches des Nutzers. Kein Umgehen von Zugriffssperren. Höchstens eine kleine Stichprobe (Größenordnung 10 bis 50 Demos), Platz auf dem Host ist knapp: Demos nach Dekodierung löschen.

Gibt es nach ehrlicher Prüfung keinen berechtigten Weg, dokumentierst du das mit Belegen in `UEBERGABE.md`. Das blockiert G5 nicht.

## Eigentum

Deadlock-Brain Replay-Crates und Replay-Reportpfad (per Graphify `replay` ermitteln), `architecture/migration/replays/`. Gemeinsame Dateien nur nach Ankündigung.

## Arbeitsstand

Worktree `~/.worktrees/brain-fertig-r`, Branch `feat/brain-fertig-r-20261003` von `origin/main`. Zum Draft-PR #46 (`codex/fix-c10-real-replay-validation`): verwertbare Commits übernehmen.

## Beweisziel

Anzahl echter Demos, Dekodier-Erfolgsquote, Beispielabfrage aus Brain mit Quelle und Version, wiederholter Import idempotent.

## Routing

Paket R, Versuch 1, Produzent `teil-r`. Rest siehe GEMEINSAM.md.
