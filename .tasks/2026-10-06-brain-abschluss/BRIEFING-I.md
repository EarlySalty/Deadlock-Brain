# Paket I: Integration E und F nach main, live

Rolle: Blatt-Worker (GPT 6.1 Sol). Keine weiteren T3-Threads; native Subagenten nur als frische Fixer bei Gate-BLOCK.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`. Steuerung: `VON_HAUPT.md` Abschnitt 09:00 (Hold aufgehoben).

## Stand

- **E** API-Spiegel: Worktree `/home/nathanael/.worktrees/brain-e-deadlock-api`, Branch `feat/brain-deadlock-api-daten`, HEAD `35665b92`. Schema ALLOW, API-Fix `09af1e6b` ALLOW. Bericht `AN_HAUPT-E.md` im Worktree.
- **F** Publish ohne Matchgrenze: Worktree `/home/nathanael/.worktrees/brain-f-publish`, Branch `feat/brain-build-publish-ohne-matchgrenze`. Bericht `.tasks/2026-10-07-f-publish/AN_HAUPT-F.md` dort. F liest Spielwerte über Es Leser `brain_storage::asset_mirror`. Gate-Stand prüfen.
- origin/main ist `f6f5cef6` oder neuer (parallele Arbeit, nicht zurückdrehen).

## Auftrag

1. **E integrieren:** auf aktuellen origin/main bringen (Rebase oder Merge im eigenen Worktree, Konflikte in `deadlock-brain/src/main.rs` sauber lösen), fmt, Clippy, Tests, Gate `gate_hook.py --review` bis ALLOW (bei BLOCK frischer nativer Fixer je Runde), `git push origin HEAD:main` als Einzelschritt.
2. **E live:** Release aus eigenem Worktree vom origin/main-SHA über den regulären Brain-Deploy-Weg (im Repo nachsehen, Graphify zuerst), Neustart, dann den ersten vollständigen Spiegel-Import für die aktuelle `client_version` ausführen (Assets beider Sprachen, Patch-Discovery, Builddaten). Beweis: `source_runs` mit `mirror_complete=true` und `load_mirrored_assets` liefert Helden und Items.
3. **F integrieren:** wie Schritt 1 auf den neuen main, Gate bis ALLOW, Push.
4. **F live:** Deploy wie oben, dann Warden-Build mit `--publish` veröffentlichen und die `hero_build_id` melden.
5. Branches und Worktrees von E, E-Fix1 und F nach `git merge-base --is-ancestor` mit geprüftem Exit-Code löschen.

## Grenzen

- Kein Force-Push, nie `git push origin main`, `git add` nur eigene Dateien, ein Git-Schritt je Bash-Aufruf mit literalen absoluten Pfaden.
- G (`a867ef50`, Worktree `brain-g-v2-20261007`) nicht anfassen; G mergt selbst.
- Keine Matchdaten speichern, keine Matchgrenze, keine neuen Pipelines. Rust only, Postgres only, keine Code-Kommentare, keine ENV-Konfiguration, Secrets nie im Klartext.
- Deutsch ohne Em-Dashes, echte Umlaute.

## Bericht

`AN_HAUPT-I.md` (neueste oben): Merge-SHAs, Gate-Antworten wörtlich, Testzahlen, Deploy-SHA, Importbeweis, `hero_build_id`. Mit Pflichtzeile `MERGEPROTOKOLL[MS-1]`. Fertig heißt gemergt, deployt, live geprüft, aufgeräumt; dann `python3 ~/Documents/tools/t3-thread.py settle --selbst`.
