# G-K-R2: deterministischer Buildschutz und Ausgabezweck im Toolpfad

status: Start nach verifizierter Provider-Rückgabe freigegeben, 07.10.2026

## 1. Ziel und Vertrag

Frischer nativer Fixer für genau die beiden verifizierten Kernel-BLOCKs aus `G/REVIEW.md`. Regulärer Gate auf `4f42c209..45f51d6f`, Modell gpt-6.1-sol: Toolpfad umgeht bestehenden deterministischen Buildschutz; AnswerPurpose geht verloren und Veröffentlichung wird auch bei InternalRead verlangt. Aktuelle Quellen-, Lese-, Modellweitergabe- und Publikationsrechte sind getrennte Verträge. Frische Ausführung, Cachetreffer und Flightfolger müssen denselben Zweck bewahren. Kein öffentliches Ergebnis aus bloß internen Rechten.

Bestehende Domain-/Buildvalidierung weiterverwenden. Nichtdomainiger Build darf nicht allein aufgrund gewöhnlicher Tools mit freier Modellbauempfehlung answered werden. Ein tatsächlich ausgeführter, validierter deterministischer BuildPlan darf im neuen Werkzeugweg nutzbar sein; dessen Schema, Szenario, Pin und Ergebnis müssen am echten vertrauenswürdigen Port geprüft werden. Nicht BuildPlan aus allen Sessions pauschal entfernen und nicht die ursprüngliche Schutzbedingung heimlich aufweichen. Bei fehlendem Vertrag konkreten begrenzten Bedarf melden.

AnswerPurpose bis Ausführung, Abhängigkeitsvalidierung, finaler Ausgabe und Wiederverwendung tragen. Providerweitergabe ist auch bei InternalRead weiter zu prüfen. ForPublication fordert zusätzlich aktuelle Veröffentlichung. Cache-/Flightzweckbindung und sämtliche unzitierten Eingaben berücksichtigen. Fehlerabrechnung mit observed/reserved, Budgets, ursprünglicher Abbruch und Deadline erhalten. Keine neuen Quellenfreigaben, Modelle, Timeouts oder ENV-Schalter.

## 2. Eigentum

Exklusiv `brain-kernel/src/{lib.rs,execution.rs,flight.rs,cache.rs,outcome.rs}` und unmittelbar betroffene Kerneltests. Provider, Reasoner, Sources, Serve, SQL, Loader und Manifeste bleiben unverändert. Nur wenn zwingend erforderlich: begrenzter kompatibler Typanschluss in `brain-contracts/src/tools.rs` nach ausdrücklicher Aktivierung durch Bereichsführung; zunächst keinen Vertrag ändern. Eigene neue Rohbelege und Rückgabe unter `G/pruefungen/g-k-r2/`. REGISTER, REVIEW, AN_HAUPT und TODO nicht bearbeiten. Keine globalen Formatierungen oder zusätzlichen Refactorings.

## 3. Arbeitsstand

Primärer Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`. Der Start wird erst nach tatsächlichem Abschluss des Providerfixers G-P-R2 erfolgen. Start-HEAD per git log feststellen und im Bericht binden; keine angenommene spätere SHA. Bereits committed: a6568629 Vertrag, 4f42c209 Provider, 45f51d6f Kernel. Reasoner-WIP gehört der Bereichsführung, nicht anfassen. Nach lokal geprüfter eigener Änderung sind genau eigene Kerneldateien gezielt zu stagen und als eigener Featurecommit zu sichern. Trailer `Co-authored-by: GPT 6.1 Sol <modell@local>`. Bereichsführung macht während dieses Laufs keine Git-Schreibschritte. Kein Push/main, Worktreewechsel, Release, Runtime, Produktions-DB oder Liveprovider. Keine Delegation und keine Hookumgehung. Keine Stellvertreteraktion für den abgewiesenen Testaufruf im Hilfsprüfbaum; normale Prüfungen im primären G-Worktree sind erlaubt.

## 4. Beweisziel

Compiler, kontrolliertes Format, striktes Kernel-Clippy und bestehende vollständige Kernelsuite über vorhandenen Buildslot, maximal drei Jobs und privates Debugtarget. Tests mit --include-ignored --test-threads=1, tatsächliche passed/failed/ignored und vorhandene Baseline 57/18 beziehungsweise ursprüngliche 34/18 aus `G/G-K-R1-NACHWEISE.md` nachweisen. Bestehende Rechteprüfungen nicht löschen oder abschwächen.

Gezielte Belege: Buildprofil ohne deterministischen BuildPlan darf mit nur Profil-/Servertoolbelegen nicht answered werden; gültiger deterministischer Weg bleibt nutzbar. InternalRead mit gültigem Lesen/Providerweitergabe aber fehlender Publikationsfreigabe funktioniert frisch, aus Cache und als Flightfolger. Dasselbe Material bei ForPublication wird abgewiesen. Nichtzitierte Abhängigkeit, Scope-/Actorwechsel und Rechteverlust vor finaler Ausgabe bleiben blockierend. Keine echte Wall-Clock in Zustandstests.

Nach gezieltem Featurecommit regulären Gate auf dem tatsächlichen Start-HEAD bis Fix-HEAD fahren und Exit/Gateurteil/SHA zurückgeben. Unveränderte Reviewer-Kette benutzen; sie urteilte in Runde 1 mit gpt-6.1-sol. Ausreichendes Prozessfenster für die konfigurierte Gatefrist, keine Zwei-Minuten-Grenze. Bei erneutem BLOCK Funde zurückgeben, nicht selbst in weiterer Runde fixen. Bereichsführung nimmt den gesamten ursprünglichen Kernelumfang mit Fix anschließend geordnet ab.

## 5. Routing

Auftraggeber Bereichsführung G, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine weiteren Worker, T3-Threads, ListAgents oder SendMessage. Statusproduzent Bereichsführung G. Nach etwa 20 Minuten eigene Wirkung prüfen, spätestens nach 30 Minuten. Rückgabe als Workflowresult mit Dateien, realen Prüf-/Gateaufrufen, SHA, Zahlen, Sourcefingerprints und offenen Grenzen. Gebaut, geprüft, reviewt, committed, gemergt und live getrennt. Rückfragen an Auftraggeber, nicht Nutzer.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
