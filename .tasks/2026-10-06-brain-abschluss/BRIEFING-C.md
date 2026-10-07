# Paket C: Branches, PRs und Worktrees von Deadlock-Brain aufräumen

Rolle: Blatt-Worker (GPT 6.1 Sol). Du erledigst genau diesen Auftrag, startest keine weiteren Orchestratoren oder T3-Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Erst `AUFTRAG.md` in diesem Ordner lesen.

## Lage

Repo `EarlySalty/Deadlock-Brain` (Checkout `~/repos/Deadlock-Brain`, nicht umschalten, er ist dreckig und gehört niemandem von uns): 106 Remote-Branches, 5 Draft-PRs (#3, #4, #5, #6, #9), rund 120 Worktrees (`git worktree list`). Der Nutzer weiß nicht, was davon fertig ist.

## Vorgehen

1. SHA-Backup aller Remote-Branches und Worktree-HEADs nach `C/BACKUP-SHAS.txt` (Name, SHA, Datum letzter Commit), committen und pushen auf einen eigenen Branch `chore/brain-aufraeumen-20261006` aus eigenem Worktree, damit das Backup auf origin liegt.
2. Jeden Branch und jeden Worktree klassifizieren in `C/INVENTAR.md`:
   - `gemergt`: `git merge-base --is-ancestor <sha> origin/main` mit Exit 0, oder Patch-identisch per `git cherry origin/main <branch>` ohne `+`-Zeilen.
   - `ersetzt`: Inhalt nachweislich in anderer Form auf main (Beleg nennen).
   - `offen`: eigener, nicht gemergter Inhalt.
   - Worktrees zusätzlich: uncommitteter Stand ja/nein, läuft darin gerade ein Prozess (`lsof +D` bzw. `fuser`), Thread-Zuordnung aus den Akten.
3. Aufräumen nur, was sicher ist: `gemergt` und belegt `ersetzt` remote löschen (`git push origin --delete <branch>`, einzeln, nie force), passende Worktrees ohne uncommitteten Stand und ohne laufenden Prozess per `git worktree remove` entfernen, `git worktree prune`. Worktrees mit uncommittetem Stand vorher als `wip:`-Commit auf ihrem Branch sichern und pushen, dann wie `offen` behandeln.
4. PRs: gemergte oder ersetzte Draft-PRs mit kurzem Kommentar (Deutsch, worin der Inhalt steckt) schließen. Offene PRs bleiben.
5. Alle `offen`-Einträge nach `C/OFFEN.md` (Branch, SHA, Inhalt in einem Satz, Diffgröße gegen main, letzter Commit). Paket A entscheidet dort je Zeile „übernehmen“ oder „verwerfen“. Zeilen mit „verwerfen“ löschst du danach (das Backup bleibt). Prüfe `C/OFFEN.md` dafür bis zum Ende deines Laufs alle 30 Minuten; nach 3 Stunden ohne Entscheidung den Rest als offen im Bericht lassen.

Nicht anfassen: `main`, Branches und Worktrees der Pakete A und B (Branches `fix/game-invite-*`, alles, was in `A/STAND.md` als in Arbeit steht), fremder laufender Arbeit.

## Abschluss

Bericht in `AN_HAUPT-C.md`: vorher/nachher (Branches, PRs, Worktrees), was gelöscht wurde, was offen blieb und warum. Backup-Branch bleibt auf origin.
