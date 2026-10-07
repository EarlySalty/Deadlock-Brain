# Paket B, Fixer Runde 1: Game-Invite-Fix nach Gate-BLOCK

Rolle: Blatt-Worker (GPT 6.1 Sol), frischer Kontext. Du startest keine weiteren Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md` in diesem Ordner.

## Übernahme (nichts neu bauen)

- Repo Deadlock-Bots, Worktree `/home/nathanael/.worktrees/Deadlock-Bots-invite-fix`, Branch `fix/game-invite-spaete-antwort`, HEAD `cdbf32cb` (Basis `600b832a`).
- Ursache und Fixidee: `AN_HAUPT-B.md` in diesem Ordner.
- Mängelliste: `/home/nathanael/.worktrees/Deadlock-Bots-invite-fix/.tasks/2026-10-06-invite-fix/REVIEW.md` (zwei BLOCKING-Funde plus 22 zusätzliche `unwrap_used` in `invite_lounge_tests.rs`).

## Auftrag

1. Beide Funde am Code verifizieren und beheben (Live-Cursor nur im Live-Pfad, Uhr direkt vor jedem Fremdaufruf neu lesen, abgelaufene Claims ohne Versand abschließen), je mit Regression für beide Reihenfolgen bzw. Ablauf während DB-Wartezeit. Zwillinge in derselben Datei mitprüfen.
2. Die eigenen neuen `unwrap` in den Tests durch begründete `expect` ersetzen, keine Lints abschalten. Vorbestehende Diagnosen (`dl-central-db` `platform_connections.rs:30`) nicht anfassen.
3. Tests und Clippy für `dl-community` laufen lassen. Der Server hat wenig freien Speicher: Testcontainer begrenzen und danach sofort abbauen, bevor der Gate läuft.
4. Selbstprüfung mit `gate_hook.py --review`, gleiche Kette wie Runde 1 (geurteilt hat `gpt-6.1-sol`). Bei ALLOW: Merge nach main (`git push origin HEAD:main`, ein Git-Schritt je Aufruf), Release bauen im eigenen Worktree, Deploy über den in `AN_HAUPT-B.md` Punkt 5 belegten Weg (Release-Ordner unter `/opt/deadlock/bots/releases/<SHA>`, `current` atomar umhängen, bestehender Launcher), Neustart, Live-Beweis im Journal plus ein Testfall nur mit Testkonto. Danach Branch und Worktree löschen (`merge-base --is-ancestor` mit Exit-Code prüfen).
   Bei erneutem BLOCK: neue Mängelliste als `REVIEW-R2.md` ablegen, in `AN_HAUPT-B.md` melden und stoppen.

## Bericht

Oben in `AN_HAUPT-B.md` einen neuen Abschnitt „Fixer Runde 1“: Fix-Commit, Testzahlen, Gate-Antwort wörtlich, Merge-SHA, laufendes Release (`readlink /opt/deadlock/bots/current`), Live-Beleg. Danach `python3 ~/Documents/tools/t3-thread.py settle --selbst`.
