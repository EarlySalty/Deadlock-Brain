# Auftrag: Deadlock Brain abschließen (06.10.2026)

Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56` (Delegator, baut nichts selbst).

## Nutzerwunsch (sinngemäß)

1. Feststellen, wie weit der Brain-Ausbau wirklich ist. Es wurde viel gemacht, aber gefühlt nur die Hälfte fertig, vieles geht nicht.
2. Die offenen Teile parallel mit GPT 6.1 Sol fertig bauen.
3. Game Invites (Playtest-Einladungen) funktionieren falsch: das Modell reagiert rund 2 Stunden später auf eine Anfrage und sagt „du wurdest schon eingeladen“. Das ist Blödsinn und muss an der Ursache behoben werden.
4. Aufräumen: 106 Remote-Branches, Draft-PRs, rund 120 Worktrees. Niemand weiß, was davon fertig ist.

## Pakete (parallel, getrennte Schreibpfade)

| Paket | Inhalt | Briefing |
|---|---|---|
| A | Bestandsaufnahme und Fertigbau Brain (Teil-Orchestrator) | `BRIEFING-A.md` |
| B | Game-Invite-Bug (Blatt-Worker, ggf. repoübergreifend) | `BRIEFING-B.md` |
| C | Branches, PRs, Worktrees aufräumen (Blatt-Worker) | `BRIEFING-C.md` |

Kopplung: C löscht nur, was nachweislich in `origin/main` steckt oder durch einen gemergten Stand ersetzt ist. Alles mit eigenem, nicht gemergtem Inhalt kommt in `C/OFFEN.md` und wird von A bewertet, nicht von C gelöscht. A und B arbeiten nur in eigenen, frischen Worktrees.

## Vorgeschichte (lesen statt neu erfinden)

- `.tasks/2026-10-03-brain-fertigstellung/` (KOPF.md, UEBERGABE-KOPF-GROK.md, PLAN-NEU.md, TODO.md, welle1/)
- `.tasks/2026-10-04-spielwissen-steckbriefe/`, `.tasks/2026-10-05-nebenfehler/`
- Fremde Threads aus früheren Läufen (D1b `5efe9f27`, D5 `6b670366`, Kopf-Grok `31575951` u. a.) nicht anschreiben, nicht settlen, nicht fortsetzen. Ihre Worktrees und Branches gelten als Artefakte, die A übernimmt.

## Allgemeine Regeln für alle Pakete

- Antworten und Berichte auf Deutsch, echte Umlaute, keine Gedankenstriche.
- Produktiver Code nur Rust. Keine Code-Kommentare.
- Codebase-Fragen zuerst über Graphify (Skill `code-suche`), dann grep.
- Nichts doppelt bauen: vorhandene Branches und Worktrees prüfen und weiterverwenden.
- Abschluss je Änderung: `gate_hook.py --review` selbst gegen die eigene Arbeit, dann Merge nach main über den lokalen Merge-Gate-Hook (`git push origin HEAD:main`, ein Git-Schritt je Bash-Aufruf, literale Pfade), Deploy über den bestehenden Weg (`brain-release install` bzw. Bots-Deploy), Neustart, Live-Beweis, Branch und Worktree löschen.
- Nie force-pushen. Vor jeder Löschung `git merge-base --is-ancestor` mit geprüftem Exit-Code. Vor Massenlöschung SHA-Backup schreiben.
- Keine Session-zu-Session-Koordination außer über die Akte hier. Bericht an den Haupt-Orchestrator als `AN_HAUPT-<Paket>.md` in diesem Ordner (neueste Einträge oben), kurz.
- Wer fertig ist (gemergt, deployt, live geprüft, aufgeräumt, berichtet): `python3 ~/Documents/tools/t3-thread.py settle --selbst`.
