# G-K-R4: tatsächliche Belege an konkrete Provider weitergeben

status: vorbereitet, 07.10.2026

## 1. Ziel und Vertrag

Du bist ein frischer nativer Fixer, weder ursprünglicher Implementierer noch G-K-R1/R2/R3. Gemeinsamer regulärer Gate `a6568629..b352472f` hat mit Exit 1 geurteilt: `[gpt-6.1-sol] BLOCK: The tool loop cannot complete with either concrete provider.` Mängelliste `G/REVIEW.md`, Log `/tmp/brain-g-provider-kernel-gesamt-gate-20261007.log`.

Bereichsführung bestätigte execution.rs:187/194/210: der Kernel sammelt tatsächliche evidence aus Werkzeugabhängigkeiten, reicht aber an beide grounded_turn_input_ceiling-Aufrufe und answer_turn_accounted leere Slices weiter. Konkrete Provider verweigern dadurch Folgerunden mit Toolresult-Beleg-IDs; der Kernelmock verdeckt dies. Alle drei Stellen gemeinsam korrigieren. Vollständige angesammelte, geprüfte Belege weitergeben, einschließlich unzitierter Abhängigkeiten. Tatsächliche Providerautorisierung und finale Zitatprüfung erhalten. Keine Scope-/Actor-/Purpose-/Pin-/Build-/Deadline-/Accountingprüfung abschwächen. Vollständige wiederholte UTF-8-Modelleingabe zählen; keine bytes/4-Schätzung.

Nach Graphify gezielt betroffene Stellen lesen. Worktree hat keinen eigenen Graph; bekannte Repoalternative `/home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json`, ansonsten globaler Graph. Neue Symbole fehlen teilweise. Keine volle Neuextraktion. Neuer Connector, Modell, Vertragsform, ENV-Feld, Timeout oder Rechenkern ist nicht beauftragt.

## 2. Eigentum

Exklusiv Kernel-execution.rs und unmittelbare vorhandene Kerneltests (lib.rs bei Bedarf). Zusätzlich eine konkrete Providerintegration im bestehenden Serve-Testbereich, bevorzugt neue `rust/crates/brain-serve/tests/tool_evidence_loop.rs`. Serve besitzt bereits brain-kernel/brain-providers/reqwest/tokio-Abhängigkeiten; keinen neuen Manifestpfad aufbauen. Beide realen Providerklassen gegen HTTP-Loopback prüfen. Werkzeugport und Datenquelle dürfen für Zustandsgrenzen kontrollierte Fakes sein; gefürchtete Providerautorisierung, Serialisierung, Antwortparser und Folgerundentransport müssen echt laufen. Das ist kein echter Luna-/Spiegel-/Buildbeweis.

Providerproduktdateien, Serve-Produktdateien einschließlich discord_live.rs, Contracts, Reasoner, Sources, SQL, Manifeste, Runtime und Konfiguration bleiben unverändert. Keine globalen Formatierungen, Kommentare oder Refactorings. Eigene Rohbelege `G/pruefungen/g-k-r4/`. REGISTER, REVIEW, AN_HAUPT und TODO nicht bearbeiten. Die Verbraucherschema-/Infisicalbefunde stehen getrennt, kein Scope-Zuwachs.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, tatsächlicher Start-HEAD `b352472fbe75429a221cefe5a59133f41bd35520`. Vorherige Fixer tatsächlich abgeschlossen. Uncommittierter Reasoner und Akte bleiben unangetastet. Genau eigene Kernel-/Serve-Testdateien dürfen nach verifizierten lokalen Prüfungen gezielt gestaged und als eigener Featurecommit gesichert werden. Trailer `Co-authored-by: GPT 6.1 Sol <modell@local>`. Bereichsführung führt während deines Laufs keine Git-Schreibschritte aus. Kein Push/main, Worktreewechsel, Release, Runtime, Produktions-DB oder Liveprovider. Kein Stellvertreter für den vor Ausführung abgewiesenen Hilfsbaum-Test. Kontextmodus-Lesewerkzeug meldete eine Projektwurzelbegrenzung; keinen solchen abgewiesenen Aufruf über Sandbox-Code ersetzen. Normale zugelassene Dateiwerkzeuge und primäre G-Prüfungen verwenden.

## 4. Beweisziel

Kontrolliertes Format, Compiler und striktes Clippy für Kernel, Provider und Serve über bestehenden Buildslot, höchstens drei Cargo-Jobs, eigener Debugtarget. Bestehende Suites mit --include-ignored --test-threads=1, unverdeckten Exits und echten passed/failed/ignored/filtered-Zahlen. Ausgang Kernel 71 passed/18 gleiche failed, Provider 44/0. Serveverbraucher 414 passed/14 failed wurde bisher ohne passende Vorher-Baseline gemessen; keine alte Fehlerklassifikation erfinden.

Belegen: tatsächlicher Toolturn, Ergebnis mit freigegebenem Beleg, Folgerunde und finale belegte Antwort über beide konkreten Provider. Zweite Runden müssen dieselben angesammelten Belege für Eingabezählung und Autorisierung erhalten. Unzitierten Beleg mitzählen/prüfen, knappes Eingabebudget vor Fremdaufruf abweisen, Rechteverlust nicht durch Belegweitergabe kaschieren. Vorhandene konservative Fehlerabrechnung und ursprüngliche Deadline bleiben. Kein Fake für die gerade defekte Providergrenze. Kein echter Wall-Clock-Zustandstest.

Nach gezieltem Featurecommit regulären Gate auf b352472f..Fix-HEAD fahren. Unveränderte Reviewer-Kette, erste Runde gpt-6.1-sol. Ausreichendes Prozessfenster, keine neue Reviewerinstanz neben dem Gate. Bei BLOCK Ergebnis zurückgeben; kein Selbstfix in weiterer Runde. Bereichsführung prüft anschließend erneut den gesamten ursprünglichen Provider-/Kernelumfang einschließlich aller Fixes. Delta-ALLOW ist kein Gesamt-G-ALLOW.

## 5. Routing

Auftraggeber Bereichsführung G, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine weitere Delegation, T3-Threads, ListAgents oder SendMessage. Statusproduzent Bereichsführung G. Wache nach 20 Minuten, spätestens 30. Rückgabe mit Dateien, tatsächlichen Befehlen/Exits, Quellfingerprints, SHA, Zahlen und Grenzen. Gebaut, geprüft, reviewt, committed, gemergt und live getrennt. Rückfragen an Auftraggeber, nicht Nutzer.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
