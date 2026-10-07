status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/Deadlock-Docs-brain-consumer-fertig

# Worker: übernommenen Docs-Consumer prüfen

## Ziel und Vertrag

GEMEINSAM.md, AUFTRAG.md und BRIEFING-K.md gelten. Lies SCOUT-docs.md in diesem Ordner. Der sichere typisierte Docs-Adapter ist bereits vollständig übernommen. Nicht erneut bauen oder cherry-picken. Rust ist die Produktivsprache, kein neuer Python-Code und keine neuen Code-Kommentare. Graphify zuerst bei Codefragen. Das Repository ist ein Korpus plus on-demand CLI, kein neuer Antwortdaemon. docs.public bleibt fest, private Operatornutzung ausgeschlossen. Q/Z richten die passende Serverfreigabe, den öffentlichen Docs-Release und das Anfragejournal ein.

## Eigentum

Nur tools/brain-adapter/ und vorhandener zugehöriger .gitignore-Eintrag in diesem Worktree. Keine Community-Hilfen, Audit-/Writerverträge oder Korpusdeployänderungen. Keine neue Unit oder neue Installationsmechanik. Wenn die bestehende CLI vor dem Betrieb eine minimale Ergänzung benötigt, diese im eigenen Bereich vornehmen. Offene Konfigurations- und Deploydetails konkret berichten, nicht durch einen lokalen Modellpfad ersetzen. Keine eigenen Bug-/Security-Reviews neben dem Merge-Gate.

## Arbeitsstand

Branch feat/brain-consumer-fertig-20261003, HEAD 3e570a8aa0bf867bf1baf35b064165804b77fcb4, sauber. Vollständiger Baum identisch mit sicherem Sol-Präfix 14455aebb48db6aef82dfd94d173ebdf25a6cf0e. Fünfzehn Adaptercommits übernommen. Fremder main-Branch 67bb24706fa37fbdccb6b99845f06c45d1448d0e ist divergent und bleibt unangetastet. Basis für jede eigene Integration ist frisch gefetchtes origin/main, zunächst 4b072aee3127564def674f4b53d9be5d1d42cfdf. Eigene Ergänzungen committen erlaubt; main-Merge, Push und Deploy erfolgen durch teil-k nach Abnahme. Keine weiteren Agenten oder T3-Threads.

## Beweisziel

Bestehende fmt-, clippy- und betroffene Adaptertests tatsächlich ausführen. Vor cargo test rolle-test-waechter laden. Rustup-Toolchain /home/nathanael/.cargo/bin verwenden, nicht den alten System-Cargo 1.75. Hostlocks zwingend zuerst /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, dann /tmp/deadlock-cargo-release.lock. HOSTPROBE.md anwenden; aktive Compiler außerhalb der Locks nicht beenden. Bei Exit 75 unter gehaltenen Sperren 30 Sekunden später erneut prüfen. Maximal zwei Jobs, keine parallelen Compiler. Bestehende zentrale Build-Ablage und check.sh verwenden, Caches nicht löschen. Kein Release vor Merge.

Liefere tatsächliche Prüfexits, Testanzahl und Source-SHA. Für den späteren Live-Beweis dokumentieren: vorhandener CLI-Befehl, harmlose öffentliche Casual-Lane-Frage aus SCOUT-docs.md, erforderlicher vertrauenswürdiger FD5-Starter, fehlende normale TOML und nicht geheime Feldliste. Keine Secrets abrufen oder speichern, keine Produktivdaten ändern. Gate und unabhängige Intent-Abnahme organisiert teil-k nach deinem Abschluss.

## Routing

Auftraggeber teil-k, Hauptorchestrator 43a4886c-e135-484b-838a-0512d224a634. Paket K, Versuch 1. Statusereignisse allein teil-k; TODO.md und zentrales REGISTER.md nicht schreiben. Ergebnis bevorzugt BAU-DOCS.md in diesem Ordner, sonst vollständige kompakte Rückgabe für teil-k. Nachweise, Gitkopf und echte Blocker nennen. Deutsche Texte mit echten Umlauten, humanizer und no-em-dashes anwenden.
