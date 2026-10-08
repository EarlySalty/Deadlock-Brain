# G-M: gelieferte Reasonerarbeit als verifizierte Checkpoints sichern

status: startbereit auf dbce14ae nach tatsächlichem Abschluss beider Fixer und gemeinsamem ALLOW, 07.10.2026

Verbindliche Übernahme durch Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Haupt-Orchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`. `PAKETE.md` in `.tasks/2026-10-07-brain-fertigstellung-astra/` tatsächlich gelesen. I und K benötigen den gesicherten G-Rechenkern unabhängig von Main. Den gerade parallel und ohne Git-Schreibrecht vorbereiteten kleinen bestehenden Verbraucherfixturefix zuerst geprüft sichern, dann den Rechenkernfeaturevertrag liefern. Keine zyklische Main-Abhängigkeit und keinen fremden WIP kopieren.

## 1. Ziel und Vertrag

Du paketierst die bereits fertig gelieferte Reasonerarbeit. Kein Neubau, keine Umbenennung, kein Rechen-/Planerrefactoring. Lies G/CHECKPOINTS.md und G/G-M-0645-NACHWEISE.md. Alle Originalfixtures unverändert erhalten; bekannte Daten-/Mechaniklücken bleiben ehrlich. Vier DB-Fixturefehler wurden mit vorher 287 passed/4 failed und zuletzt 321 passed/4 failed belegt; die drei ausgeschlossenen Produktionsfälle sind kein grüner Lauf.

Die statischen Grenzen F1/F2/S1/S2/S3/S4 wurden von einem lesenden Architekturworker bestimmt. S1/S2/S3-HOLDBACK.patch sind exakt selektierte Originalhunks und wurden dry-run geprüft. Ihre Anwendung ausschließlich im Index hält folgende Änderungen zurück, ohne gelieferte Arbeitsquellen zu verändern. Zwischenstände benötigen tatsächlichen isolierten Compiler und striktes Clippy, nicht den Gesamt-WIP als Ersatz.

## 2. Eigentum

Ausschließlich gelieferte eigene Reasonerdateien types.rs, data.rs, mechanics.rs, progression.rs, combat.rs, lib.rs, planner.rs, calculation.rs, calculation_tests.rs und drei Fixtures unter testdata/calculation. Keine neuen Produktänderungen, Kommentarzeilen, Formatänderungen oder Fixturebearbeitung. Exaktes Staging nach CHECKPOINTS.md. Eigenes enges Befehls-/Prüfprotokoll unter G/pruefungen/g-m-checkpoints/. Keine REGISTER-/PLAN-/REVIEW-/AN_HAUPT-/TODO-Änderungen.

Provider/Kernel/Contracts/Serve, E/F-Lader und fremder Stand bleiben unverändert. Der Planertest enthält bereits die beauftragte korrigierte zweite Waffenbonusfixture; keine Planerproduktänderung hinzufügen. Vor Beginn zwölf gelieferte Dateien gegen den vorhandenen Bereichsfingerprint prüfen und nachher deren Arbeitsquellbytes erhalten.

## 3. Arbeitsstand

Primärworktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`. Start-SHA wird durch Bereichsführung im tatsächlichen Startauftrag genannt; nicht die Vorbereitungs-SHA als aktuellen Stand behaupten. Vorherige Produktwriter müssen abgeschlossen sein. Worktree besitzt weiteren eigenen Akten-WIP, der nicht gestaged wird. Vorhandener eigener isolierter Prüfworktree `/home/nathanael/.worktrees/brain-g-checkpoints-20261007` steht bisher detached auf a6568629; Eigentümer Bereichsführung. Sein committed Compiler war zugelassen, zusätzliche Tests wurden vor Ausführung abgewiesen. Keine Tests in diesem Hilfsbaum starten, keinen anderen Toolweg oder Worker dafür verwenden. Compiler/Clippy dort ausschließlich im zugelassenen Weg, falls die Befehlsprüfung blockiert, zurückmelden statt umgehen.

Gezielte eigene Featurecommits je Gruppe erlaubt, Git-Schritte einzeln mit literalen absoluten Pfaden, kein add -A. Trailer ausschließlich `Co-authored-by: GPT 6.1 Sol <modell@local>`. Keine zusätzliche Claude-Code-Attribution, kein anderer Modelltrailer. Nach dem jeweiligen tatsächlich bestandenen Gruppengate darf der geprüfte eigene Feature-HEAD sofort nach `origin/feat/brain-v2-g-20261007` gepusht werden; unreviewte Folgestände nicht pushen. Insbesondere den S3-Rechenkernvertrag gesichert bereitstellen, ohne auf Main oder I/F-Builds zu warten. Bereichsführung führt während deines Laufs keine Git-Schreibschritte aus. Kein Main, Release, Deploy, Neustart, Live oder Produktions-DB. Fremde Worktrees nicht ändern, keine fremden Sicherungen oder Stashes. Vorhandenen eigenen Hilfsbaum nicht löschen.

## 4. Beweisziel

Reihenfolge genau F1, F2, S1, S2, S3, S4. F1/F2 Originaldaten als getrennte Checkpoints; S1 nur Fundament, S2 vorhandene Simulation samt Testfixture, S3 öffentlicher Rechenkern, S4 gelieferte Rechenfälle. Vor jedem Commit staged diff auf eigenes Eigentum und korrekte Holdbacks prüfen. Vollständige Arbeitsquellen nicht temporär rücksetzen.

Pro committed Zwischenstand isoliert cargo check und striktes cargo clippy mit all-targets über bestehenden Buildslot und höchstens drei Jobs. Vorhandenen eigenen Prüfbaum auf den konkreten eigenen Commit setzen, Prüfquelle gegen Commit bestätigen. Keine WIP-Prüfung als Zwischenbeweis ausgeben. Bei nicht kompilierender Grenze zurückmelden, keinen Funktionsneubau oder Scopewechsel. Normale letzte primäre Reasonerprüfung mit `--include-ignored --test-threads=1` und unverdeckten Exits kann vorhandenen isolierten PostgreSQL-Verwaltungsweg benutzen; kein Hilfsbaum-Test. Reale Suitezahlen/ausgeschlossene Fälle präzise nennen.

Jede Gruppe nach Commit regulär `gate_hook.py --review` auf unmittelbar vorherigen HEAD bis Gruppen-SHA prüfen. Erste Runde gpt-6.1-sol, unveränderte Kette und ausreichendes Prozessfenster. Bei BLOCK stoppen und tatsächlichen Stand/Funde zurückgeben; nächste Fixrunde bekommt frischen Kontext. Mehrere Commits bedeuten keinen automatisch kleineren kumulierten Maingate. Keine Zwischenmerges zur Größen- oder Abschluss-Hookumgehung. Rohlogs vor Übergabe auf Geheimnisse prüfen, nicht pauschal stagen.

## 5. Routing

Auftraggeber Bereichsführung G, Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Haupt-Orchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`. Keine weiteren Worker/T3-Threads, ListAgents oder SendMessage. Statusproduzent Bereichsführung. Wache nach 25 Minuten, spätestens 30. Rückgabe mit exakten Gruppen-SHAs, Befehlen/Exits, Gates, Quellbindung und offenen Grenzen. Kein Gesamt-G-ALLOW oder Liveabschluss aus Zwischenbeweisen ableiten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
