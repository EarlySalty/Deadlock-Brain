# W2 Patchnotes in Rust

[Orchestrator] BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/patchnotes-rust-fertig

Rolle: Blatt-Worker für W2, Versuch 1. Auftraggeber ist Codex-Delegator D1 `01a10326-e5e6-7633-8d59-a02d13110fd4`; Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Keine weiteren Threads oder Subagenten starten. Die Bauakte liegt unter `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`. Zuerst dort `DELEGATOR-REGELN.md` und `PLAN-NEU.md` lesen; diese Regeln gehen älteren Bauakten vor.

Du bist nicht allein im Dateisystem. Eigentum: Rust-Patchnotes und nötiger Betrieb im bestehenden Worktree, Branch `feat/patchnotes-rust-fertig-20261003`. Bestehende und fremde Arbeit erhalten, keine Refactorings oder globale Formatierung. P hält laut STAND.md HEAD `2538b41f9a738a979068e4767e3684c0baef0d8e`; der Runtime-Fix danach ist noch ungeprüft. Prüfe den tatsächlichen Gitstand vor Änderungen. Vor Übernahme `t3-thread.py read --thread 09ce79b1-d5ed-4ae8-b77a-1fa54fdfcb1a`: bei running Worktree nicht anfassen, D1 melden. Alten Thread nicht anschreiben.

Commit, Push, Merge und bestehender Deploy sind erlaubt. Früh committen und jeden grünen Stand pushen. Gate als einziger Reviewer; bei BLOCK Befunde mit Branch und SHA an D1, Sourcewrites stoppen, frischer Fixer folgt. Exit 2 einmal wiederholen, dann melden. Keine Community-Ankündigung ohne Nutzer-Go. Trockenlauf darf Perplexity aufrufen. Produktive Units erst nach belegtem Rust-Lauf umstellen, echte Veröffentlichung nur im bereits freigegebenen Betrieb; keine manuell erzeugte Ankündigung. Alte Dateien als `.disabled` sichern.

Bericht ausschließlich in `welle1/w2/AN_D1.md` der Bauakte, höchstens fünf Zeilen mit Befehl, Exit, Testzahl und SHA. Vorher `welle1/w2/VON_D1.md` lesen. Brainänderungen nur in eigenem getrennten Brain-Worktree, Übergabe an W1 über `welle1/w1/EINGANG.md`, niemals Brain-main oder Brain-Deploy ändern. Eigene Worktrees nach Abschluss mit SHA-Backup löschen, übernommene Altworktrees erhalten. Alle eigenen Texte mit humanizer und no-em-dashes prüfen, natürliches Deutsch und echte Umlaute; keine Code-Kommentare.

## Ziel

Der Patchnotes-Bot läuft produktiv in Rust und ersetzt den Python-Pfad. Neue Patchnotes werden erkannt, über den bestehenden Weg veröffentlicht (Perplexity sonar-pro mit altem Prompt, Bilder aus der Quelle, siehe Memory `patchnotes-bot-perplexity-und-bilder`) und als Kandidat an das Brain geliefert.

## Ausgangslage

- Repo `Deadlock--Patchnotes-Bot`, Worktree `~/.worktrees/patchnotes-rust-fertig`, Branch `feat/patchnotes-rust-fertig-20261003`, 32 Commits vor origin/main.
- Stand und offene Punkte in `../../bereiche/p/STAND.md` und `../../bereiche/p/AN_HAUPT.md` (Status 14). Bekannt: ein Bot-Test rot (`ingestion_is_atomic_and_historical_previews_remain_suppressed`), Clippy in Bot und Storage rot, Runtime und DevFeed noch nicht abgeschlossen.
- Der Brain-Anteil (`1a5b2b3`) wird von W1 übernommen, nicht von dir.

## Weg

1. Im bestehenden Worktree weiterarbeiten, auf aktuellen origin/main rebasen.
2. Roten Test und Clippy fixen, Runtime und DevFeed fertig machen. Prüfen nach `HOSTPROBE.md`.
3. Einen echten Lauf gegen die echte Quelle ohne Veröffentlichung (Trockenlauf) zeigen: erkannte letzte Patchnotes, erzeugter Text. Ein echter Perplexity-Aufruf ist erlaubt, Veröffentlichung im Trockenlauf nicht.
4. Gate (`gate_hook.py --review`), Merge nach main, `git push origin HEAD:main`.
5. Deploy über den bestehenden Weg, die alten Python-Units auf das Rust-Binary umstellen (Altdatei als `.disabled` sichern), Dienst neu starten, Journal prüfen.
6. In `AN_D1.md` melden: main-SHA, aktive Unit, Beleg aus dem Trockenlauf und vom ersten echten Lauf.

## Grenzen

Kein Modellwechsel, kein zweiter LLM-Connector. Brauchst du eine Änderung im Brain-Repo, Commit auf eigenem Brain-Branch pushen und in `../w1/EINGANG.md` eintragen.
