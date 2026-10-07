# Paket D: Gebauten Code aus den Archiv-Tags ernten

Rolle: Blatt-Worker (GPT 6.1 Sol). Keine weiteren Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`.

## Hintergrund

Paket C hat 105 Branches auf 11 reduziert. Gelöschte Stände liegen als Tags `archiv/<name>` auf origin (`C/ARCHIV-TAGS.md`, `C/OFFEN.md`, `C/BACKUP-SHAS.txt`); die Draft-PRs #3, #4, #5, #6, #9 wurden dadurch automatisch geschlossen. In `C/OFFEN.md` hat A 44 Stände mit „übernehmen“ markiert, teils mit fehlender Funktion auf main (z. B. generische Spielstilplanung, Sicherheitsregressionen, Rechtefix mit Kernübergaben, MCP-Einstieg, typisierter TOML-Vertrag, Caption-/Patch-Review). Erhalten heißt bisher nur archiviert, nicht integriert. Der Nutzer will gebauten Code nicht verlieren.

## Auftrag

1. Für jeden „übernehmen“-Stand und jeden der fünf PR-Heads am Code prüfen (nicht an Commit-Texten): Was davon fehlt auf aktuellem `origin/main` wirklich als Funktion oder Test? Ergebnis `D/ERNTE.md`: Tag/PR, fehlender Inhalt in einem Satz, Nutzen für das eine Brain (hoch/mittel/niedrig/keiner), Konflikt mit laufender A-Arbeit (`A/STAND.md`, `A/REGISTER.md`), Empfehlung (portieren/schon ersetzt/nur Belegdoku, nicht portieren).
2. Danach `AN_HAUPT-D.md` mit Kurzfazit (wie viele Stände echten fehlenden Code tragen) und abwarten, bis der Haupt-Orchestrator die Portierliste freigibt (`VON_HAUPT.md`, Abschnitt Paket D). Bis dahin keine Produktänderung.
3. Freigegebene Portierungen einzeln: eigener Worktree `~/.worktrees/brain-ernte-<thema>` von aktuellem main, Eigenanteil per Cherry-pick oder Neuaufsetzen (alte tote Basis nie voll mergen), Tests, Gate-Selbstprüfung, Merge, `brain-release install`, Live-Beleg, aufräumen. Nichts doppelt zu A bauen; bei Überschneidung A den Vortritt lassen und in `AN_HAUPT-D.md` melden.
