status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-z

[Orchestrator] Paket Z: Aufräumen, G5-Cutover, G6 Legacy-Ende, Abschluss

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig. Du bist Integrationsverantwortlicher für den Gesamtabschluss.

## Phase 1 sofort: Aufräumen und Cutover-Vorbereitung

1. **SHA-Backup** aller Remote-Branches, lokalen Branches und Worktrees von Deadlock-Brain nach `bereiche/z/BACKUP-<zeit>.txt` (Name, SHA, Worktree-Pfad) plus Git-Bundle unter `~/.local/share/deadlock-brain/branch-backup-20261003.bundle`.
2. **Bereinigen** (Deadlock-Brain, Stand 2026-10-03: 90 Worktrees, 59 ungemergte Remote-Branches): entfernen, was in origin/main enthalten (`git merge-base --is-ancestor`, Exit-Code prüfen) oder nachweislich überholt ist. Nicht anfassen: alles, was zu laufenden Sessions gehört (`ps`, `t3-thread.py list`), Worktrees und Branches des Wiki-Auftrags (`feat/brain-wiki-spielwissen-*`, `.tasks/2026-10-03-wiki-spielwissen`), die Branches der Pakete P/S/Q/R/K, der Hauptcheckout. Worktree vor `remove` auf ungesicherte Artefakte prüfen. Draft-PRs #3, #4, #5, #6, #9 mit kurzem Kommentar "überholt durch main <sha>" schließen, wenn ihr Inhalt in main steht oder überholt ist; #46 erst nach Rückmeldung von R. Unklares nicht löschen, sondern in `bereiche/z/UNKLAR.md` auflisten mit Vorschlag.
3. **Cutover-Runbook** `architecture/migration/CUTOVER_RUNBOOK.md`: welche Legacy-Writer, welche Units, Reihenfolge, Rückweg je Schritt, Messpunkte.

## Phase 2: G5-Cutover

Seit der gemeinsamen Integration durch Z gilt: P, S, Q und K liefern ihre lokal geprüften Eigenstände mit vollständigem SHA und offenen Betriebsverträgen als `phase: uebergeben`. Z darf und muss diese Stände gemeinsam integrieren und abnehmen, bevor sie live sind. Ein bereits abgeschlossener Live-Nachweis aller Pakete ist keine Startbedingung für diese Integration, weil deren Live-Prüfungen erst nach Zs Deployment möglich werden. Produktiver G5-Wechsel erst nach kombinierter Intent-Abnahme, Bug-/Security-Gate für den integrierten Stand, gesichertem frischem Datenbestand und geprüftem Rückweg. Nach dem Wechsel führen die Fachpakete ihre echten Live-Nachweise aus und melden erst dann `abgeschlossen`. R ist kein Muss für G5. Für das Wiki: `.tasks/2026-10-03-wiki-spielwissen/ENDE.md` oder dessen Abschlussbericht auf main. Wartest du, prüfst du die Dateien alle 20 Minuten und arbeitest in der Zwischenzeit am gemeinsamen Betriebsvertrag und Runbook. Ist der Wiki-Auftrag 24 Stunden nach Start dieses Pakets noch nicht fertig, schreibst du das in `AN_HAUPT.md`; die bereits beschlossene Ausnahme ist dann anhand der aktuellen Lage umzusetzen und ehrlich als fehlender Wiki-Pilot zu dokumentieren.

Dann: Cutover nach Runbook auf aktuellem origin/main, Release bauen (Host-Sperren), deployen, Dienste neu starten, Live-Messung gegen SLO aus Q, Consumer-Anfragen aus K erneut prüfen.

## Phase 3: G6 Legacy-Ende

Nach mindestens einem vollen Tageszyklus aller Timer ohne Fehler: Legacy-Writer und Python-Units stoppen und deaktivieren (Unit-Dateien als `.disabled` sichern), Legacy-Codepfade aus dem Repo entfernen, die nur noch Python-Legacy sind. Rückweg dokumentiert lassen.

## Phase 4: Abschluss

`STATUS.md` und `GATES.csv` mit SHA, Nachweisen und `approved_by: Hauptorchestrator 43a4886c im Auftrag des Nutzers` aktualisieren. `ABSCHLUSSBERICHT.md` in diesem Ordner: was live ist, Zahlen, Grenzen, bereinigte Branches/Worktrees (vorher/nachher), Rückweg. Die Akte selbst committen und pushen. Memory `brain-rust-integration-20260926` ist nicht deins, der Hauptorchestrator pflegt sie.

## Eigentum

`architecture/migration/` (STATUS, GATES, Runbook), `ops/`, Brain-Units unter `~/.config/systemd/user/`, Branch- und Worktree-Bereinigung in Deadlock-Brain. Code-Fixes, die beim Cutover auffallen, gibst du an einen frischen nativen Subagenten in deinem Worktree.

## Arbeitsstand

Worktree `~/.worktrees/brain-fertig-z`, Branch `feat/brain-fertig-z-20261003` von `origin/main`.

## Routing

Paket Z, Versuch 1, Produzent `teil-z`. Rest siehe GEMEINSAM.md.
