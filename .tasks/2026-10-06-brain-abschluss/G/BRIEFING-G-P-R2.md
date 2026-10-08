# G-P-R2: transienten Status trotz fehlerhaftem Diagnosekörper wiederholen

status: beauftragt, 07.10.2026

## 1. Ziel und Vertrag

Du bist ein frischer nativer Fixer für genau den verifizierten BLOCK aus `G/REVIEW.md`, Providerrunde 1. Originale Implementierer sind abgeschlossen. Gate `[gpt-6.1-sol]` auf `a6568629..4f42c209` beanstandet transport.rs:272: bounded read im 429/5xx-Zweig beendet wegen `?` Retries bei zu großen, abgeschnittenen oder zeitlich gescheiterten Fehlerkörpern. Befund am bestehenden Code erneut prüfen, Zwillinge in beiden vorhandenen Wireformen suchen und den berechtigten Kern beheben.

Vorhandenen Retry-/Charge-/UsageAccounting-Pfad weiterverwenden. Diagnosekörper weiter begrenzt lesen; sein Fehler hebt einen transienten HTTP-Status nicht auf. Nur bei tatsächlich verbleibender ursprünglicher Frist, kumuliertem Budget und konfigurierten Retries erneut senden. Beobachtete Usage und konservative Reservierung jedes Versuchs erhalten, unlesbare/fehlende Usage nicht als gemessene Null ausgeben. Keine zweite Abrechnung, kein neues Budget, keine neue Konstante oder Konfiguration. Falls Spec den Retry wegen ausgeschöpfter Reservierung begrenzt, das sauber als solche Grenze nachweisen.

## 2. Eigentum

Exklusiv `rust/crates/brain-providers/src/transport.rs` und direkt betroffene Tests in `rust/crates/brain-providers/tests/faults.rs`; nur bei belegtem Bedarf vorhandene Provider-lib/hardening ergänzen. Keine Vertrags-/Kernel-/Reasoner-/Sources-/Serve-/Manifeständerungen. Keine globalen Formatläufe oder Refactorings. Eigene neue Bericht-/Rohbelege unter `G/pruefungen/g-p-r2/`. REGISTER, REVIEW, AN_HAUPT und TODO bleiben bei Bereichsführung beziehungsweise Aufgabenstand.

## 3. Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, HEAD `45f51d6f875a930b9fa61fe2bce7f0dab85e7d80`. Contracts a6568629, Provider 4f42c209, Kernel 45f51d6f committed; Reasoner-WIP bleibt unangetastet. Keine anderen Produktwriter aktiv; Bereichsführung paketiert vorhandenen Reasoner getrennt. Du darfst nach vollständiger lokaler Prüfung ausschließlich die eigenen Providerdateien gezielt stagen und als eigenen Featurecommit sichern. Commit-Trailer `Co-authored-by: GPT 6.1 Sol <modell@local>`. Während deines Laufs führt die Bereichsführung keine Git-Schreibschritte aus; keine gemeinsame Stagingkonkurrenz. Kein Push, main, Worktreewechsel, Release, Runtime, DB oder Liveprovider. Schutz-Hooks nicht umgehen. Der zusätzliche isolierte Testaufruf im Hilfsprüfworktree wurde abgewiesen; führe keine solche Stellvertreteraktion aus. Normale Prüfungen auf dem freigegebenen primären G-Worktree sind weiterhin deine Aufgabe.

## 4. Beweisziel

Compiler, eigene Formatprüfung, striktes Provider-Clippy und vorhandene vollständige Provider-Suite mit --include-ignored --test-threads=1 über vorhandenen Buildslot, höchstens drei Cargo-Jobs, privates Debugtarget. Belege für beide Wireformen: 503 mit übergroßem Diagnosekörper und anschließendem gültigem Erfolg; abgeschnittener Fehlerkörper; eingehaltene ursprüngliche Deadline und kumuliertes Accounting. Keine echte Wall-Clock für nicht-I/O-Zustandsprüfungen, keine Nutzerdaten oder Secrets.

Nach vollständiger lokaler Prüfung und eigenem Featurecommit regulär `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-g-v2-20261007 --base 45f51d6f --head HEAD` auf genau deinen Fix fahren. Tatsächlicher Prozess darf bis zur unveränderten konfigurierten Gatefrist laufen; kein Zwei-Minuten-Hintergrundfenster. Gateantwort und Commit-SHA im Bericht nennen. Kein Hook- oder Reviewerwechsel. Bei erneutem BLOCK nicht selbst weiterfixen: Funde zurückgeben, weiterer frischer Fixer übernimmt. Bereichsführung prüft anschließend den ursprünglichen Providerumfang mit aufgenommenem Fix; dein Delta-ALLOW allein ist keine vollständige G-Abnahme.

## 5. Routing

Auftraggeber Bereichsführung G, native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Keine weiteren Worker, T3-Threads, ListAgents oder SendMessage. Fachrückgabe an diese Workflow-Rückgabe mit Dateien, tatsächlich ausgeführten Befehlen/Exits, passed/failed/ignored, Source-SHA und offenen Grenzen. Gebaut, lokal geprüft, gategeprüft, committed, gemergt und live getrennt melden. Statusproduzent bleibt Bereichsführung G. Nach etwa 20 Minuten eigene Wirkung prüfen, spätestens nach 30 Minuten. Rückfragen gehen an Auftraggeber, nicht Nutzer.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
