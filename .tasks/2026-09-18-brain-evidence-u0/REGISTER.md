# Register: Brain-Evidenz U0 und Primärdatenlauf U1

Intent: externe ChatGPT-Session mit Nutzerübergabe vom 18. September 2026, keine T3-Intent-ID vorhanden.

| Paket | Thread | Modell | Status | Worktree | Letzte Meldung |
|---|---|---|---|---|---|
| U0: zwei Evidenzkorrekturen und Tests | Opus-4.8-Worker (diese Session) | opus48 gemäß lokalen Bauvorgaben | Umgesetzt, getestet, Feature-Branch gepusht; wartet auf externen Review/Merge | `/home/nathanael/.worktrees/brain-evidence-u0-20260918` | Folgemigration + Assertions + CI + Doku; TESTNACHWEIS[TW-1] 33 passed, 4 ignored, Baseline 0 rot; siehe REPORT.md |
| U1: echter unabhängiger Referenzlauf | Diese ChatGPT-Session | Kein neuer Modell-/Providerweg | Betriebsprüfung | Derselbe Feature-Stand nur lesend | Keine Creator-Eingabe, kein Videoimport |

Freigabepunkt: Worker testet, committet und pusht ausschließlich den Feature-Branch. Keine Produktionsmigration, kein Merge und kein Deploy durch den Worker. Orchestrator prüft Diff, Tests, Betriebszugang und Release-Gate. Andere Brain-Worktrees bleiben unangetastet.
