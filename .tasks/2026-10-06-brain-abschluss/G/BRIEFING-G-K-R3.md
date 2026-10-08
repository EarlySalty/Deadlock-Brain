# G-K-R3: gebundene deterministische Buildprüfung

status: beauftragt, 07.10.2026

## 1. Ziel und Vertrag

Du bist ein weiterer frischer nativer Fixer, nicht der ursprüngliche Kernelimplementierer oder G-K-R2. Offener bestätigter BLOCK aus `G/REVIEW.md`: nichtleere Toolsession umgeht den vorhandenen deterministischen Buildschutz und erlaubt freie Modellbauempfehlungen allein mit gewöhnlichen Toolbelegen. G-K-R2 hat nur AnswerPurpose behoben und diesen Rest korrekt offen gelassen. Lies `G/G-K-R2-NACHWEISE.md`.

Jetzt ausdrücklich aktiviert: kompatibler typisierter Prüfanschluss am vorhandenen vertrauenswürdigen ToolExecutionPort in brain-contracts/src/tools.rs. Prüfung erhält das tatsächliche BuildPlan-Ergebnis, die wirkliche typisierte Anfrage, Anfragekontext/Pin und ursprüngliche Deadline. Ohne echte Implementierung sicher ablehnen. Bestehende Signaturen und Ergebnisfelder kompatibel halten. Keine frei erfundene JSON-Ergebnisform, kein bloßes vom Modell behauptetes valid=true und kein neuer Planer. G-V wird den Anschluss später über Fs bestehenden reinen Buildvertrag und seine realen Evaluationsbelege implementieren.

Kernel darf Buildprofil nur mit tatsächlich ausgeführtem und durch diesen Anschluss geprüftem deterministischem BuildPlan-Ergebnis beantworten; gewöhnliche Profile/Serverbelege oder bloße Tooldefinition reichen nicht. Bereits gültige deterministische Domain-Buildwege erhalten. Nachweis an tatsächliches Ergebnis, Anfrage und Pin binden, bei Cache/Flight dieselbe Prüfung wiederholen. Das Ergebnis selbst muss dem validierenden Port wieder vorliegen, nicht nur einzelne Evidence-IDs. Scope/Actor/Purpose, alle zitierten und unzitierten Quellen sowie Quellenrevisionen bleiben geschützt. Keine synthetischen Quellenrechte oder Publish-ID. G-K-R2s Zweckfix erhalten.

Bei jeder Frist-, Rechte-, Source-, Validierungs- oder Portfehlergrenze bleiben beobachtete und reservierte Usage erhalten. Kein frisches Budget, keine zweite Ledger-/Providerstrecke, keine neuen Modelle, ENV-Felder oder festen Timeouts. Ein echter fehlender Planer-/Buildvertrag bleibt ehrlich gesperrt statt mit einem erfundenen Resultatschema ersetzt zu werden.

## 2. Eigentum

Exklusiv brain-contracts/src/tools.rs und notwendige direkte Vertragstests sowie brain-kernel/src/{lib.rs,execution.rs,flight.rs,cache.rs,outcome.rs} mit unmittelbaren Kerneltests. Bestehende PortError-/Accountingformen aus contracts/lib.rs unverändert konsumieren. Nur bei tatsächlich zwingendem Exportbedarf minimaler lib.rs-Export nachhalten; keine semantische Änderung an Accounting. Provider, Reasoner, Sources, Serve, SQL, Loader, Manifeste, Runtime und Konfiguration bleiben unverändert. Keine globalen Formatierungen, Kommentare oder Refactorings. Eigene Rohbelege unter G/pruefungen/g-k-r3/. REGISTER, REVIEW, AN_HAUPT und TODO nicht bearbeiten.

## 3. Arbeitsstand

Primärer Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, tatsächlicher Start-HEAD `3242fb36ed847dc06ef71d92c71108614b5d493f`. Providerfixer und Zweckfixer tatsächlich abgeschlossen. Keine anderen Produktwriter aktiv. Fertiger Reasoner-WIP und Dokumente gehören Bereichsführung und bleiben unangetastet. Nach vollständiger lokaler Prüfung dürfen genau eigene Vertrags-/Kerneldateien gezielt gestaged und als eigener Featurecommit gesichert werden. Trailer `Co-authored-by: GPT 6.1 Sol <modell@local>`. Bereichsführung führt während dieses Laufs keine Git-Schreibschritte aus. Kein Push/main, Worktreewechsel, Release, Runtime, Produktions-DB oder Liveprovider. Keine Stellvertreteraktion für den abgewiesenen zusätzlichen Testaufruf im Hilfsprüfbaum. Normale primäre G-Prüfungen sind dein Auftrag.

## 4. Beweisziel

Compiler, kontrolliertes Format, striktes Clippy für Contracts/Kernel und betroffene Verbraucher sowie vorhandene vollständige Suites über vorhandenen Buildslot, höchstens drei Cargo-Jobs, private Debugtargets. Tests --include-ignored --test-threads=1 mit unverdecktem Exit und passed/failed/ignored. Kernelreferenz 61 passed/18 gleiche failed, contractsreferenz 73 passed/0 failed aus G-K-R1. Neue Tests sind nicht allgemein Pflicht, der konkrete offene Sicherheitsbefund braucht aber nachvollziehbare Grenzbelege.

Abnahme: freie Buildfinale ohne geprüften Plan trotz ordentlicher anderer Toolbelege abweisen; alte Ports ohne Prüfanschluss sicher ablehnen; an vertrauenswürdigem Prüfport akzeptierter tatsächlicher BuildPlan bleibt nutzbar; geändertes Resultat/Anfrage/Pin oder verweigerte erneute Prüfung sperrt Cache/Flight; aktuelle Quellenrechte, Abbruch und volle kumulierte Fehlerabrechnung bleiben erhalten. Fakes für Kernelzustände sind zulässig, aber keine echte F/G-V-Buildabnahme behaupten. Keine echte Wall-Clock in Zustandstests.

Nach gezieltem Featurecommit regulären Gate auf `3242fb36` bis tatsächlichem Fix-HEAD fahren. Unveränderte Reviewer-Kette, Runde 1 urteilte mit gpt-6.1-sol. Ausreichendes Prozessfenster für konfigurierte Gatefrist. Bei erneutem BLOCK Funde zurückgeben, kein Selbstfix in weiterer Runde. Bereichsführung prüft anschließend den gesamten ursprünglichen Provider-/Kernelumfang mit beiden Fixes; dein Delta-ALLOW ist kein Gesamt-G-ALLOW.

## 5. Routing

Auftraggeber Bereichsführung G, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine weitere Delegation, T3-Threads, ListAgents oder SendMessage. Statusproduzent Bereichsführung G. Eigene Wirkung nach etwa 20 Minuten, spätestens 30 Minuten prüfen. Workflowrückgabe mit Dateien, realen Prüf-/Gatebefehlen und Exits, Sourcefingerprints, Commit-SHA, Zahlen und offenen Grenzen. Gebaut, geprüft, reviewt, committed, gemergt und live getrennt. Rückfragen an Auftraggeber, nicht Nutzer.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
