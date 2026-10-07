# G0-R1: Werkzeugvertrag stabilisieren

## Verbindliche Wiederaufnahme

Der ursprüngliche Fixworkflow `wf_563c24f7-221` wurde beim Ende der vorigen Session gestoppt, ohne Abschlussmeldung. Sämtliche bereits vorhandenen Vertragsänderungen bleiben erhalten. Erst aktuelle Typen, Diff und bisherige Prüflogs untersuchen, danach den vorhandenen Fix vervollständigen und verifizieren; nicht neu implementieren oder auf eine frühere Vertragsform zurücksetzen. Es gibt keinen beauftragten parallelen Vertragsschreiber. G-M wird getrennt im Reasoner fortgesetzt. HEAD `f129c91a`, Main-/Runtime-Hold erneut bestätigt. Frühere rote Prüfungen sind historische Befunde und beweisen nicht automatisch den jetzigen Zustand. Für den Abschluss zählen neue tatsächliche Prüfungen und ein präziser stabiler Typvertrag.

## 1. Ziel und Vertrag

Führe die vorhandenen G0-Änderungen zu einem einzigen kompatiblen und verifizierten Brain-Werkzeugvertrag zusammen. Der ursprüngliche Workflow `wgid0g5cj` ist abgeschlossen und meldete konkurrierende Änderungen. Die durch eine native Nachricht gestartete Fortsetzung `a3a054b7281fbde9e` wurde mit bestätigtem TaskStop beendet. Es ist kein anderer G0-Schreiber beauftragt; G-M schreibt ausschließlich Reasoner. Keine fremde Session als Ursache behaupten. Vorhandenen Stand und beide G0-Rückgaben prüfen, nicht neu beginnen oder zurücksetzen.

Grundlage: `G/BRIEFING-G0.md`, `G/PLAN.md` C3/C5 und Abschnitt 8, aktuelle Produktdateien. Erst vollständige Bestandssicht über Graphify und gezieltes Lesen; danach die widersprüchlichen Typen/Felder/Tests zusammenführen. Einmalige endgültige Formen für Tooldefinition, validierten Aufruf, Ergebnis, Modelblock, Final-/ToolCalls-Turn, Usage, typisierte Unteranfrage und Tool-Abhängigkeiten. ToolCall-Validierung muss mit Definitionen und geschlossenen sieben Toolnamen funktionieren. Zuordnung von Ergebnissen und Call-IDs prüfen. Authorisierten Kontext, Release, Version und Deadline besitzt der Server, nicht ein Modellargument.

Erhalte den sicheren kompatiblen `AnswerProviderPort::answer_turn`-Default: vorhandener Textport ist nur bei einer echten Textanfrage ohne Werkzeugdefinitionen/Historie zulässig, andernfalls fail closed. Bestehende Verbraucher dürfen nicht brechen. Vollständige Darstellung und konservative Eingabezählung aus derselben gemeinsamen Repräsentation; kein zweiter Zähler. ToolExecutionPort und Quellen-/Publikations-/Cache-Neuprüfung behalten typisierte konkrete Unteranfragen. Kein SQL, E-Werteleser, Dispatcher oder Runtimeumbau in diesem Paket.

## 2. Eigentum

Exklusiv `rust/crates/brain-contracts/src/{lib.rs,provider_input.rs,tools.rs}` und direkt zugehörige Cratetests. Keine Reasoner-/Provider-/Kernel-/Storage-/Serve-Dateien, Manifeste, Lockdatei oder globale Formatierung. Optional eigener knapper `G/G0-VERTRAG.md` mit endgültigen Signaturen. Register, AN_HAUPT und TODO bleiben bei Bereichsführung. Rust only, keine neuen Code-Kommentare, ENV-Schalter, Modelle oder festen Timeouts. Vorhandene Änderungen erhalten und fachlich zusammenführen, keine Schutzumgehung oder Codekopie aus fremden Worktrees.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, dokumentierter gepushter HEAD `f129c91a`. Uncommittierte G0-Produktänderungen sind aktuell nicht baubar; nicht committen oder pushen. G-M besitzt seine getrennten Reasoneränderungen. Kein weiterer Agent/Workflow/T3-Thread und kein Git, Deploy, Releasebuild, DB-Schreiben oder Runtimeeingriff. Keine Nachricht an laufende Agenten; Bereichsführung startet selbst notwendige Folgepakete nach deiner Fertignachricht.

## 4. Beweisziel

Die bisherigen G0-Prüfungen waren rot: fmt Exit 1, Clippy/Test Exit 101, 0 ausgeführte Tests. Dies ist ein eigener Vertrags-/Compilerkonflikt, nicht durch die 38 Baselinefehler anderer Pakete erklärt. Beseitige ihn vollständig. Prüfe die eigenen drei Dateien formatiert, `cargo clippy --package brain-contracts --all-targets --locked --offline --jobs 2 -- -D warnings`, `cargo test --package brain-contracts --locked --offline --jobs 2 -- --include-ignored --test-threads=1` und passende Verbraucherkompilierung. Vorhandenen Buildslot `/tmp/deadlock-cargo-release.lock` und eigenen Debug-Target verwenden; kein Compiler-Rennen.

Volle Logs, unverdeckte Exits und passed/failed/ignored zurückgeben. Testfälle müssen tatsächliche Block-/Call-/Ergebniszuordnung, unbekannte Felder/Namen, Duplikate, abschließende Antworten und vollständige Eingabezählung prüfen, nicht Nutzertextwortlaut. Gate ist einziger Reviewer; Bereichsführung fährt es auf dem verifizierten Paketcommit. Bei anderem aktiven Schreiber konkrete Beobachtung sofort zurückgeben, nichts überschreiben.

## 5. Routing und Rückgabe

Auftraggeber native G-Bereichsführung, Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Worker produziert Status für G0-R1. Erste Wache 20 Minuten. Rückgabe: endgültige Typen/Funktionen, sichere Defaults, genaue Dateiliste, Befehle/Exits/Testzahlen, vorhandene kompatible Verbraucher und kleinster G-P/G-K-Anschluss. Keine Nutzerfrage oder Sessionkoordination, keine weitere Architekturauswahl. Deutsche Texte mit echten Umlauten, keine Gedankenstriche.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
