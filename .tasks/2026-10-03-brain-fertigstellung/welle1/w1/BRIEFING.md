# W1 Brain-Integration

[Orchestrator] BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-welle1

Rolle: Blatt-Worker für W1, Versuch 1. Auftraggeber ist Codex-Delegator D1 `01a10326-e5e6-7633-8d59-a02d13110fd4`; Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Keine weiteren Threads oder Subagenten starten. Zuerst `../../DELEGATOR-REGELN.md` und `../../PLAN-NEU.md` lesen. Die Delegatorregeln und diese Anpassungen gehen älteren Bauakten vor.

Du bist nicht allein im Dateisystem. Eigener Schreibbereich ist der neue Brain-Worktree mit Branch `integrate/brain-welle1`; nur du mergst und deploytest Brain. Fremde Änderungen erhalten. Im Hauptbaum nur `welle1/w1/AN_D1.md` und eigene Übergabeartefakte schreiben. Bericht höchstens fünf Zeilen, Befehl, Exit, Testzahl und SHA nennen; vorher `welle1/w1/VON_D1.md` lesen. D1 berichtet der Hauptsession. `EINGANG.md` lesen, fremde Einträge erhalten. Alle eigenen Texte mit humanizer und no-em-dashes prüfen, natürliches Deutsch und echte Umlaute; keine Code-Kommentare.

Vor jeder Übernahme alten Threadstatus mit `t3-thread.py read` prüfen. Worktrees von P, Q und Z bleiben bis zum Statuswechsel von running unangetastet. Fehlende STAND.md oder noch laufende Eigentümer an D1 melden. Sichere Vorbereitung im eigenen neuen Worktree ist erlaubt. Keine alten Threads anschreiben. Früh committen und jeden grünen Stand pushen. Eigenen Branch und Worktree nach Abschluss erst mit SHA-Backup löschen, alte Worktrees erhalten.

Einziger Reviewer ist `gate_hook.py`. Bei BLOCK Befunde, Worktree, Branch und SHA an D1 melden und Sourcewrites stoppen; D1 startet einen frischen Fixer. Bei Exit 2 einmal wiederholen, danach melden. Commit, Push, Merge und bestehender Deploy sind im Paket erlaubt. Es zählt nur das Welle-1-Ziel, keine Zusatzhärtung oder Refactorings.

## Ziel

Brain-CLI und brain-serve laufen vom selben neuen Release auf origin/main, mit Patchnotes-Kandidaten, HTTP-Build-Publish und C9-Transport. Du bist der einzige, der ins Brain-Repo nach main merged und Brain deployt.

## Ausgangslage

- Repo `/home/nathanael/repos/Deadlock-Brain`, origin/main ist `f7b02cc` (enthält den Forum-Merge #62).
- Live: CLI `/opt/deadlock-brain/current` zeigt auf `be2aa6bd…`, brain-serve auf `/opt/deadlock-brain/maintenance-releases/f7b02cc…`.
- Zu übernehmende Branches (alle auf Basis `511a347b`, 14 bis 16 Commits hinter main). Jeweils zuerst `bereiche/<x>/STAND.md` lesen:
  - Z `feat/brain-fertig-z-20261003` (Worktree `~/.worktrees/brain-fertig-z`): Release-Installer mit SHA-Bindung (`b86353a`) und gemeinsamer Kandidaten-Aktivierungsadapter (eventuell als wip-Commit). Betriebsvertrag `bereiche/z/BETRIEBSVERTRAG.md`.
  - S `feat/brain-fertig-s-20261003`: `9a6f3d5`, `148e1a5`, `5a831bf` (HTTP-Build-Publish mit Wiederaufnahme).
  - Q `feat/brain-fertig-q-20261003`: nur `e48c189` und `5c220a8` (C9). Q's uncommittete oder wip-Retention-/Writer-Arbeit ist Welle 2 und bleibt draußen.
  - P `feat/brain-patchnotes-rust-sync-20261003`: `1a5b2b3` (Patchnotes-Kandidat).

## Weg

1. Neuer Worktree `~/.worktrees/brain-welle1` mit Branch `integrate/brain-welle1` von frischem origin/main.
2. Die Commits oben per Cherry-pick übernehmen, Konflikte selbst lösen. Wip-Commits von Z nur übernehmen, wenn sie für Installer oder Aktivierung nötig sind, und dann fertig bauen.
3. Prüfen nach `HOSTPROBE.md`: fmt, clippy und Tests der betroffenen Crates. Rote Tests fixen.
4. `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo ~/.worktrees/brain-welle1 --base <origin/main-SHA> --head <HEAD-SHA>`. Bei BLOCK an D1 übergeben und Sourcewrites stoppen; ein frischer Fixer übernimmt.
5. Merge nach main, `git push origin HEAD:main`.
6. Release über den Installer aus Z bauen und installieren, sodass CLI und brain-serve auf den neuen SHA zeigen. Bestehende Config-Sperre, Journal und Rollback aus `brain-maintenance` nutzen, keine Hand-Edits an JSON oder DB. Migrationen nur neu anlegen, nie angewandte ändern.
7. Live prüfen: brain-serve antwortet auf eine echte Anfrage, `deadlock-brain --version` bzw. der Release-SHA stimmt, die Patchnotes-Route liefert Daten.
8. In `AN_D1.md` melden: neuer main-SHA, Release-SHA, Live-Beleg. Dann arbeitest du `EINGANG.md` ab: dort legen W2, W3 und W4 Brain-Commits ab, die sie brauchen. Jeden Eingang wieder über Gate, Merge und Deploy bringen. Bleibe für die Integration bis zur Freigabe durch D1 verfügbar.

## Grenzen

Kein Forum-, Wiki-, Replay-, Serverguide- oder Retention-Code in Welle 1. `brain-forum-initial.service` nicht anfassen. Alte Worktrees und Branches erst löschen, wenn die Hauptsession das freigibt.
