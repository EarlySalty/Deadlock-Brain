# Paket G: Register

status: aktiv, 07.10.2026

## Bereichsführung

- Auftrag: `.tasks/2026-10-06-brain-abschluss/BRIEFING-G.md`.
- Haupt-Orchestrator: `3fcd8f71-443e-48ae-825c-527eb52fbe56`; Kommunikation nur über die Akte.
- G-Thread laut Steuerung: `a867ef50`; native Teil-Orchestrator-Session, Modell `gpt-6.1-sol[1m]`.
- UltraCode: vom SessionStart-Hook aktiv bestätigt; tatsächlicher Workflowstart wird unten erfasst. Keine neue Hauptsession oder T3-Threads.
- Worktree: `/home/nathanael/.worktrees/brain-g-v2-20261007`.
- Branch: `feat/brain-v2-g-20261007`.
- Start-HEAD: `bfda408cb988722ddceadb56bca5b72e12d12731`; frischer sauberer Worktree von `origin/main` nach Fetch.
- Statusproduzent: Bereichsführung G. Zentralen Versuch und Ereigniskanal hat die Hauptsession noch nicht gesondert vergeben; keine erfundene Versuchszahl.
- Wirkung: Featurearbeit und lokale Prüfungen; kein Main-Push, Release-Build, Install, Neustart oder Tick während des Release-Holds.

## Native Worker

| Worker | Eigentum | Start | Zustand |
| --- | --- | --- | --- |
| sheet-modell | Nur `G/SHEET-MODELL.md` und `G/sheet/` in der gemeinsamen Aufgabenakte; keine Produktdateien | Workflow `brain-g-sheet-model`, Task `w6krhelmo`, Run `wf_f775f9c0-69b`, 07.10.2026 | aktiv |
| bestand:mechanik | Nur `G/BESTAND-MECHANIK.md`; Produktcode nur lesen | Workflow `brain-g-bestand`, Task `wyqxc1iva`, Run `wf_4e39a761-389` | aktiv |
| bestand:antwort | Nur `G/BESTAND-ANTWORT.md`; Produktcode nur lesen | Derselbe Bestandsworkflow | aktiv |
| bestand:daten-abbau | Nur `G/BESTAND-DATEN.md`; Produktcode und fremde Artefakte nur lesen | Derselbe Bestandsworkflow | aktiv |
| baseline | Nur `G/BASELINE.md` und `G/pruefungen/baseline/`; unveränderte Bestandssuites lokal prüfen | Workflow `brain-g-baseline`, Task `wih3iu8uu`, Run `wf_ccd9af54-f71` | aktiv |

Workflowstart wurde vom Werkzeug bestätigt. Native Session-ID `030a7b6f-d25c-482d-b66c-68185cd05dbb`. UltraCode-Verfügbarkeit ist durch den tatsächlichen Workflowstart belegt; Worker-Effort ausdrücklich `xhigh`. Transkript: `/home/nathanael/.claude/projects/-home-nathanael-repos-Deadlock-Brain/030a7b6f-d25c-482d-b66c-68185cd05dbb/subagents/workflows/wf_f775f9c0-69b/`.

## Verträge

E besitzt API-Spiegel und Patchimport. F besitzt Publish-Regel, Planer und Confidence. A schließt bisherige Steckbrief-Freischaltung ab. G beschreibt Leseschnitt, gemeinsam genutzte Mechanik, Steckbrief-Ansicht und Antwortwerkzeuge vor Implementierung in `G/PLAN.md`. Kein Worker delegiert weiter. Gemeinsame Produktdateien haben nur einen Schreiber.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
