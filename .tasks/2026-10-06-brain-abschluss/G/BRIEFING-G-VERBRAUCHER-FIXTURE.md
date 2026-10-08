# G: vorhandenes Verbraucherfixture an vollständigen Turnvertrag anpassen

status: vorbereitet, 07.10.2026

## 1. Ziel und Vertrag

Frischer nativer Fixer für einen bestätigten eigenen bestehenden Testbruch. Lies G/VERBRAUCHER-R3.md. Der große Panel-/Dokufall in brain-serve/src/discord_live.rs bestand in der unveränderten Baseline und scheitert aktuell mit invalid chat schema. Seine Loopbackantwort an Zeile 785 enthält keinen finish_reason, obwohl der konkrete kompatible Parser transport.rs:594 dieses Feld für den vollständigen Turn verlangt.

Genau das bestehende Antwortfixture um den normalen Abschlussgrund stop ergänzen. Produktionsparser, Turnprüfung und Budgets unverändert. Kein neues Verhalten, keine Budgeterhöhung, keine Fixtureverkleinerung oder Abschwächung. Der gesamte vorhandene Fall muss weiter beide Panelgrößen, Dokubeibehaltung, Packgrenze, Freigaben und tatsächliche zwei Netzrunden prüfen.

## 2. Eigentum

Ausschließlich die JSON-Loopbackantwort im Test grosse_live_panels_und_doku_passen_gemeinsam_ins_providerbudget in `rust/crates/brain-serve/src/discord_live.rs`. Keine Änderung an Produktteilen derselben Datei, anderen Tests, Kernel, Provider, Contracts, Reasoner, Manifeste, Konfiguration oder Runtime. Eigentum ist bereits vor Bearbeitung in AN_HAUPT-G gemeldet. G-K-R4 besitzt getrennt Kernel und seine neue Serve-Testdatei; nichts davon anfassen.

Eigene Rohbelege unter G/pruefungen/g-verbraucher-fixture/. Keine REGISTER-/PLAN-/REVIEW-/AN_HAUPT-/TODO-Änderung. Keine Kommentare oder globale Formatierung. Vor Bestandssuche code-suche/Graphify, danach gezielt lesen.

## 3. Arbeitsstand

Primärer Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Featurebranch `feat/brain-v2-g-20261007`, letzter vor deinem Start belegter committed HEAD b352472f. G-K-R4 ist aktiver disjunkter Schreiber mit exklusivem Featurecommitrecht. Deshalb keinerlei Git-Schreibschritt durch dich, auch kein Staging oder Commit. Bereichsführung sichert deinen geprüften Einzeilenfix erst nach tatsächlichen Rückgaben. Kein Push/main, Worktreewechsel, Hilfsbaum-Test, Deploy, Dienstneustart, Produktions-DB, Infisical-Liveabruf oder externe Modellprobe.

## 4. Beweisziel

Kontrolliertes rustfmt --check, passender Compiler und striktes Clippy über vorhandenen Cargo-Slot mit höchstens drei Jobs und eigenem Debugtarget. Betroffenen tatsächlich vorhandenen Fall mit --include-ignored --test-threads=1 ausführen, Exit unverdeckt, echte passed/failed/ignored/filtered-Zahlen. Vorhermessung aus consumers.log: ein ausgeführter Zielcase rot; Originalbaseline derselbe Zielcase bestanden. Kein zusätzlicher Rotlauf vorgeschrieben.

Nur der bereits vorhandene Loopback-Fall ist deine Testwirkung. Bestehende private/Livefälle nicht starten, da Voraussetzungen fehlen und echte Community-Daten weder im Bericht noch bei externen Modellen landen dürfen. Untertest ist keine vollständige Serve-Abnahme. Nach Abschluss Quellfingerprint und unveränderten engen Diff melden. Kein eigener Reviewer, Gate auf dem späteren eigenen Commit führt Bereichsführung aus; bei BLOCK folgt frischer Fixerkontext.

## 5. Routing

Auftraggeber Bereichsführung G, Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Haupt-Orchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`. Keine weiteren Worker/T3-Threads, ListAgents oder SendMessage. Statusproduzent Bereichsführung. Wache nach 25 Minuten, spätestens 30. Rückgabe mit exaktem Diff, Befehlen/Exits, Testzahlen und Quellbindung. Kein Commit, Review-ALLOW, Main oder Live behaupten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
