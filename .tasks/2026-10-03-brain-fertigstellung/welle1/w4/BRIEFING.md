# W4 Consumer: Docs, Second-Brain, Twitch

[Orchestrator] BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Docs-brain-consumer-fertig

Rolle: Blatt-Worker für W4, Versuch 1. Auftraggeber ist Codex-Delegator D1 `01a10326-e5e6-7633-8d59-a02d13110fd4`; Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Keine weiteren Threads oder Subagenten starten. Bauakte: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/`. Zuerst dort `DELEGATOR-REGELN.md` und `PLAN-NEU.md` lesen; sie gehen älteren Bauakten vor, insbesondere bei Hostlocks, Gate und Statusformat.

Du bist nicht allein im Dateisystem. Eigentum sind die Consumeränderungen in den bestehenden Docs-, Second-Brain- und Twitch-Worktrees, Branch jeweils `feat/brain-consumer-fertig-20261003`. K ist ready und hat die Arbeit gehalten; aktuellen read prüfen. Docs `3e570a8aa0bf867bf1baf35b064165804b77fcb4`, Second-Brain `54979646adde835335fa24ddd2545e10f996df52`, Twitch `84ce376c9ea7aab69116fc7399183032308d712e`. Vor Änderungen tatsächlichen Gitstand prüfen, fremde Änderungen erhalten. Bots bleiben vollständig außerhalb dieses Pakets. Keine globalen Refactorings oder zusätzlichen Antwortdaemons.

Commit, Push, Merge und bestehender Deploy sind im eigenen Bereich erlaubt. Früh committen und jeden grünen Stand pushen. Gate als einziger Reviewer; bei BLOCK Befunde, Worktree, Branch und SHA an D1, Sourcewrites stoppen. D1 startet frischen Fixer. Exit 2 einmal wiederholen, dann melden. Übernommene Altworktrees erhalten; selbst angelegte nach Abschluss mit SHA-Backup löschen. Secrets über bestehende Infisical-/FD-Wege, Konfiguration über bestehende Editoren und Wartungswege. Keine Secrets lesen oder ausgeben. Alle eigenen Texte mit humanizer und no-em-dashes prüfen, natürliches Deutsch und echte Umlaute; keine Code-Kommentare.

Bericht ausschließlich `welle1/w4/AN_D1.md`, höchstens fünf Zeilen mit Befehl, Exit, Testzahl und SHA. Vor jeder Meldung `welle1/w4/VON_D1.md` lesen. W1-Deploybeleg steht in `welle1/w1/AN_D1.md`. Nötige Brainänderungen nur auf eigenem getrennten Brain-Worktree und Branch, Übergabe über `welle1/w1/EINGANG.md`; W1 mergt und deployt allein. Echte Chatprobe nur in earlysalty: keinen eigenen Sender bauen oder Nachricht senden. NUTZER-AKTION an D1, der sie an die Hauptsession weitergibt.

## Ziel

Docs, Second-Brain und der Twitch-Chat beantworten Fragen über das neue Brain. Belegt durch je eine echte Anfrage mit Antwort.

## Ausgangslage

Stand in `../../bereiche/k/STAND.md` und `../../bereiche/k/UEBERGABE.md`.

- Docs: `~/.worktrees/Deadlock-Docs-brain-consumer-fertig`, Branch `feat/brain-consumer-fertig-20261003`, geprüft auf `3e570a8` (22 Tests, Clippy grün).
- Second-Brain: `~/.worktrees/Deadlock-2nd-Brain-brain-consumer-fertig`, gleicher Branchname, geprüft auf `5497964` (16 Tests).
- Twitch: Kopf `84ce376c` (Konfigeditor für die Brain-Felder), 82 Tests grün.
- Bots: zurückgestellt, nicht Teil von W4.
- Verträge: `../../bereiche/k/BETRIEBSVERTRAG-CLI.md` und `BETRIEBSVERTRAG-TWITCH.md`.

## Weg

1. Docs und Second-Brain: auf origin/main rebasen, prüfen, Gate, Merge, `git push origin HEAD:main`. Installation und Config nach Betriebsvertrag, sobald W1 den Brain-Deploy gemeldet hat. Je eine echte Anfrage mit Antwort belegen.
2. Twitch: fertig prüfen, Gate, Merge, Deploy über `/usr/local/bin/deploy-twitch-release <sha>` (Skill `deploy-restart-selbstdienst`), Brain-Felder über den Konfigeditor setzen.
3. Live-Test in earlysalty: Wenn alles bereit ist, in `AN_D1.md` eine konkrete Testfrage als Zeile `NUTZER-AKTION: bitte in earlysalty schreiben: <Frage>` eintragen. D1 gibt diese Zeile an die Hauptsession weiter. Der Nutzer schreibt sie selbst in den Chat. Danach die Bot-Antwort im Chat und die passende Zeile in `tb_chat_brain_answers` belegen.
4. In `AN_D1.md` melden: main-SHAs je Repo, Belege der echten Antworten.

## Grenzen

Keine neue OAuth-Strecke, kein zweiter Sender. Brain-Änderungen als Commit an `../w1/EINGANG.md`.
