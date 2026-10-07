# W3 Steam-Publish

[Orchestrator] BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/steam-publish-fertig

Rolle: Blatt-Worker für W3, Versuch 1. Auftraggeber ist Codex-Delegator D1 `01a10326-e5e6-7633-8d59-a02d13110fd4`; Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Keine weiteren Threads oder Subagenten starten. Bauakte: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`. Zuerst dort `DELEGATOR-REGELN.md` und `PLAN-NEU.md` lesen. Diese Anpassungen gehen älteren Bauakten vor.

Du bist nicht allein im Dateisystem. Eigentum: Steam-Publish und nötiger Betrieb im bestehenden Steam-Worktree, Branch `feat/steam-publish-fertig-20261003`. S2 ist nach read um 21:06 Uhr stopped. STAND.md hat Vorrang: Steam-HEAD `4a5c4ff3a38e9a4bf7fe2dbf8de882b00700233d`, Brain-HEAD `633d15e252a6b5a1cb635a3d06dcfcfceed6e02c`. Beide sind WIP, gepusht und sauber. Vor Änderungen tatsächlichen Gitstand prüfen, fremde Arbeit erhalten.

Brain-WIP nur lesen und den für Publish nötigen Commit an W1 über `welle1/w1/EINGANG.md` übergeben; W1 entscheidet über Übernahme und prüft ihn. Nötige neue Brain-Fixes in einem eigenen getrennten Worktree und Branch erstellen. Keine Änderung an W1s Worktree, kein Brain-main-Merge oder Deploy. Die im alten Briefing vorgeschlagene direkte Änderung auf S' Brain-Branch entfällt.

Commit, Push, Merge und bestehender Steam-Deploy sind erlaubt. Früh committen und jeden grünen Stand pushen. Einziger Reviewer ist das Gate. Bei BLOCK Befunde, Branch und SHA an D1, Sourcewrites stoppen; D1 startet frischen Fixer. Bei Exit 2 einmal wiederholen, dann melden. Übernommene Altworktrees erhalten, selbst angelegte nach Abschluss mit SHA-Backup löschen. Keine weiteren Tests als für diesen Auftrag nötig. Alle eigenen Texte mit humanizer und no-em-dashes prüfen, natürliches Deutsch und echte Umlaute; keine Code-Kommentare.

Bericht ausschließlich `welle1/w3/AN_D1.md` in der Bauakte, höchstens fünf Zeilen mit Befehl, Exit, Testzahl und SHA. Vor jeder Meldung `welle1/w3/VON_D1.md` lesen. Brain-Deploybeleg steht in `welle1/w1/AN_D1.md`; erst danach tatsächlich publizieren. Kein eigener Chat-Sender oder Community-Ankündigung.

## Ziel

`deadlock-brain reason build --publish` veröffentlicht einen Build über den HTTP-Publish-Endpunkt des Steam-Bots, und wir haben eine echte `hero_build_id`.

## Ausgangslage

- Steam-Repo Worktree `~/.worktrees/steam-publish-fertig`, Branch `feat/steam-publish-fertig-20261003`, Commit `9aec0cc` (GC-Drosselung sichtbar) plus minimaler Lockfile-Diff. Der Publish-Endpunkt selbst ist schon auf Steam-main `4c56217` und läuft auf `127.0.0.1:8783`.
- Brain-Seite (`9a6f3d5`, `148e1a5`, `5a831bf`) wird von W1 übernommen. Stand in `../../bereiche/s/STAND.md`.
- Erinnerung aus dem alten S-Auftrag: Wiederaufnahme gespeicherter Anfragen, sichtbares anhaltendes 429 und Fehlerexit bei BLOCKED. Prüfe lesend, ob `5a831bf` und der gesicherte WIP das abdecken. Nötige Fixes auf eigenem getrennten Brain-Branch committen, pushen und in `../w1/EINGANG.md` eintragen.

## Weg

1. Steam-Branch auf origin/main rebasen, prüfen nach `HOSTPROBE.md`, Gate, Merge, `git push origin HEAD:main`, Deploy über den bestehenden Steam-Weg (Binary `steam-bot` bzw. steam-core), Health prüfen.
2. Warten, bis W1 in seiner `AN_D1.md` den Brain-Deploy meldet.
3. Einen echten Build mit `deadlock-brain reason build ... --publish` veröffentlichen und die `hero_build_id` aus der Antwort bzw. aus `steam.steam_tasks` (`BUILD_PUBLISH_ORIGINAL`) belegen.
4. In `AN_D1.md` melden: Steam-main-SHA, Held, `hero_build_id`.

## Grenzen

Valve-Drosselung (Code 5) ist kein Fehler zum Wegwiederholen: Backoff, sichtbar melden.
