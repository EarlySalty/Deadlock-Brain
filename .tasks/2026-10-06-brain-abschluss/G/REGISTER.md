# Paket G: Register

status: aktiv, 07.10.2026

## Bereichsführung

- Auftrag: gemeinsame Akte `.tasks/2026-10-06-brain-abschluss/BRIEFING-G.md`.
- Haupt-Orchestrator: `3fcd8f71-443e-48ae-825c-527eb52fbe56`; Kommunikation nur über die Akte.
- G-Thread laut Steuerung: `a867ef50`; native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Modell `gpt-6.1-sol[1m]`.
- UltraCode: SessionStart-Hook bestätigt aktive Verfügbarkeit; tatsächliche Workflowstarts sind unten belegt, Effort `xhigh`.
- Worktree: `/home/nathanael/.worktrees/brain-g-v2-20261007`; Branch `feat/brain-v2-g-20261007`.
- Start-HEAD: `bfda408cb988722ddceadb56bca5b72e12d12731`; sauber von frisch geholtem `origin/main`.
- Statusproduzent: Bereichsführung G. Zentralen Versuch und Ereigniskanal hat die Hauptsession noch nicht gesondert vergeben; keine erfundene Versuchszahl.
- Wirkung: Featurearbeit und lokale Prüfungen; kein Main-Push, Release-Build, Install, Neustart oder Tick während des Release-Holds.

## Native Worker

| Worker | Eigentum | Start | Zustand |
| --- | --- | --- | --- |
| sheet-modell | Nur `G/SHEET-MODELL.md` und `G/sheet/` in der gemeinsamen Aufgabenakte; keine Produktdateien | Workflow `brain-g-sheet-model`, Task `w6krhelmo`, Run `wf_f775f9c0-69b` | aktiv |
| bestand:mechanik | Nur `G/BESTAND-MECHANIK.md`; Produktcode nur lesen | Workflow `brain-g-bestand`, Task `wyqxc1iva`, Run `wf_4e39a761-389` | aktiv |
| bestand:antwort | Nur `G/BESTAND-ANTWORT.md`; Produktcode nur lesen | Derselbe Bestandsworkflow | aktiv |
| bestand:daten-abbau | Nur `G/BESTAND-DATEN.md`; Produktcode und fremde Artefakte nur lesen | Derselbe Bestandsworkflow | aktiv |
| baseline | Nur `G/BASELINE.md` und `G/pruefungen/baseline/`; unveränderte Bestandssuites lokal prüfen | Workflow `brain-g-baseline`, Task `wih3iu8uu`, Run `wf_ccd9af54-f71` | aktiv |

Die drei Workflowstarts wurden vom Werkzeug bestätigt. Transkriptpfade wurden vom Werkzeug unter der nativen Session ausgegeben; der Blattordner des Sheetworkflows war beim ersten direkten Zugriff noch nicht vorhanden. Nicht als lesbarer Ergebnisnachweis ausgeben. Maßgeblich sind Fertignachricht, Artefakt und echte Workerwirkung.

## Eigentumswechsel der Berichtskopie

Nach dem Worktree-Eintritt verweigert das Harness Elternsession-Schreibzugriffe auf die gemeinsame Checkout-Akte. Diese Fassung und `AN_HAUPT-G.md` werden deshalb im eigenen Worktree geführt. Zuvor gestartete Worker haben unverändert ihre ursprünglichen, getrennten Schreibbereiche in der gemeinsamen Akte. Kein alternativer Schreibweg und keine Delegation einer verweigerten Operation. Zur Integration werden ihre abgeschlossenen Artefakte in die Featureakte übernommen. TODO bleibt unangetastet.

Ein Abschluss-Hook hält den frisch angelegten Branch für erledigt, weil er noch nur den Main-Ausgangsstand enthält. Die laufenden Worker benötigen ihn tatsächlich. Kein Cleanup während aktiver Arbeit, keine behauptete Fertigstellung.

## Verträge

E besitzt API-Spiegel und Patchimport. F besitzt Publish-Regel, Planer und Confidence. A schließt bisherige Steckbrief-Freischaltung ab. G beschreibt Leseschnitt, gemeinsam genutzte Mechanik, Steckbrief-Ansicht und Antwortwerkzeuge vor Implementierung in `G/PLAN.md`. Kein Worker delegiert weiter. Jede Produktdatei bekommt genau einen Schreiber.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
