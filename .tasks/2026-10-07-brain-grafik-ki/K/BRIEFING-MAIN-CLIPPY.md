# K: Enger Site-Test-Lintfix auf kombiniertem Mainstand

## Ziel und Vertrag

Bestehenden Clippyfehler cloned_ref_to_slice_refs in rust/crates/deadlock-brain/src/bin/site/compare_tests.rs:134 beheben. Aktueller kombinierter Brainstand 314bc590b2e4930a39c7b68c1bd36e0ce9f34282 hat Compiler und Format Exit 0, regulärer Gesamtgate gegen ca4d877f ALLOW. Breiter strikter Clippylauf scheitert an genau dieser eigenen Testzeile. Kein Produktverhalten, keine Assertion oder Voraussetzung ändern. Bestehende Ein-Element-Referenzschnittstelle über std::slice::from_ref wiederverwenden, keine Unterdrückung.

## Eigentum und Arbeitsstand

Ausschließlich compare_tests.rs im eigenen Worktree /home/nathanael/.worktrees/brain-k-ki-20261007, Branch feat/brain-k-ki-20261007. Ein neuer eigener Briefingtext, sonst kein Source-WIP. Kein Git-Schreiben, Commit, Push, Mainwechsel oder Deploy durch den Worker. K allein integriert. Keine andere Datei, keine neuen Code-Kommentare und keine globale Formatierung.

## Beweisziel

Zuerst Graphify, dann konkrete Fundstelle lesen. Minimalen unveränderten Testinhalt prüfen. Direkte Compiler-, Format- und strikte Clippyprüfung des Pakets deadlock-brain mit Cargo/Rust 1.97.1, absolutem eigenem Manifest, SQLX_OFFLINE=true für Compiler/Clippy, `--locked --offline --jobs 3` und `--all-targets`. Buildslot 3 per flock halten. Keine Tests oder Varianten der zuvor verweigerten Testkommandoform ausführen. Bei Schutzablehnung stoppen, keine andere Toolroute oder Settingsänderung. Resultate und tatsächlichen Diff zurückgeben; K committed den Minimalfix und führt den regulären Deltagate auf tatsächlichen SHAs aus. Mindestbeweis ist der Compiler, kein erfundener Testlauf.

## Routing

Nativer Worker, geerbtes Modell, high. Produzent teil-k, Versuch 1, Session 988eeaea-28ee-424c-b362-e250610cde91. Auftraggeber Delegator 481426fe-b477-42b3-91c6-901811fcba1d, Hauptorchestrator d3a1741e-82bc-4a48-865b-2845c663dca7. Keine zusätzlichen T3-Threads, Sessionkontakte oder zentralen Register-/TODO-Edits. Keine Secrets oder privaten Daten, keine Änderung von G-, Provider-, Modell-, Timeout- oder Datenschutzverträgen.
