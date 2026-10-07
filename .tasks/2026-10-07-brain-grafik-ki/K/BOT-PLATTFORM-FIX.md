# K: Plattformbezogene Guidefähigkeit

Abgeschlossener frischer nativer Fixer a6f53254e217b2724 hat ausschließlich die eigenen isolierten bot_tasks.rs und bot_tasks_contract.rs geändert. Kein G-Dateischreiber, Modulexport, Wirefeld, Provideranschluss oder Runtimeeingriff.

validate_for akzeptiert Available für PublicGuideEndAnswer jetzt nur bei Discord. capability_availability erhält Plattformkontext und liefert Available nur für Discord plus Approved. Titel und persönliche Hilfe bleiben Unavailable. Zwei zusätzliche Matrixfälle sichern beide Plattformen sowie alle vorhandenen Fähigkeiten ab; vorhandene Aufrufer aktualisiert.

## Prüfung

Tatsächlich gehaltener Buildslot1, bestehender zentraler Targetcache /home/nathanael/.cache/deadlock-brain-shared-flash-target und sccache. Moderne absolute Cargo-CLI, `--locked --jobs 3`, scoped `-p brain-contracts --test bot_tasks_contract`. Tests mit `--include-ignored`, Clippy mit `-- -D warnings`. Formatcheck nur beide eigene Dateien via rustfmt Edition2021/skip_children. Worker meldet finale Exits jeweils0; Haupt-K prüfte tatsächliche Logmarker.

Finaler Test /tmp/k-bot-platform-fix-tests-final-20261007.log: 20 passed, 0 failed, 0 ignored, 0 filtered. Compiler-/Clippylogs /tmp/k-bot-platform-fix-{check,clippy}-final-20261007.log enthalten Finished ohne Fehler. Formatlog /tmp/k-bot-platform-fix-fmt-final-20261007.log. Keine behauptete unveränderte Baseline oder vollständige Reposuite.

TESTNACHWEIS[TW-1]: 20 passed, 0 ignored | Baseline: nicht erhoben

Noch unexportiert und unverdrahtet. Haupt-K sichert den gezielten Sourcecheckpoint und fährt den regulären Gate. Zentralantwortpriorität bleibt gesonderter laufender Auftrag.
