status: aktiv
Datum: 2026-09-29

# Paket C: sicherer Harness und belastbare Regressionen

Gemeinsamer Vertrag /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md. Sol, einziger Thread C, keine Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a, Bump-up wie im Vertrag. Keine Code-Kommentare.

Worktree /home/nathanael/.worktrees/brain-pre-g5-harness-20260929
Branch fix/pre-g5-harness-20260929
Basis 305df2d36ec7b5d0513d6c0769051b41538d6a1b, clean.

Wörtliche Referenzen: „Testpfad ermöglichen, ohne gegen Workspace-Sicherheitsregeln zu verstoßen.“ „Der alte 18/18-Nachweis zählt NICHT automatisch für neue Fact-/Match-Fixes.“ „Kein Budget erhöhen, um Tests grün zu bekommen.“

Bestätigte vorhandene Nachweise in architecture/migration/FINAL_LOCAL_INTEGRATION_REVIEW.md Abschnitte 4 bis 6 nennen:
- scripts/test_brain_serve.sh, test_brain_storage_upgrade.sh, test_brain_core_postgres.sh, test_wiki_runtime.sh
- scripts/run_isolated_load.sh über run_isolated_pilot.sh, run_isolated_serve_checks.sh
- architecture/migration/s12/check-completion.sh
- 18 Prozess-E2E-Fälle und 600 Requests je 8/16/32 Worker bei Pool 4.
Diese historischen Belege beziehen sich auf alte SHAs. Harness wiederverwenden, keine zweite Testwelt bauen. Graphify zuerst mit globalem Graphen, im frischen Worktree ist kein lokaler graphify-out vorhanden. VORCHECK.md entsteht beim Orchestrator und kann weitere Fundstellen liefern, nicht darauf warten.

Aufgabe:
1. Vorhandenen Harness auf secret-sicheren isolierten Wegwerf-Postgres-Pfad bringen. Unix Socket/Peer, eingeschränkte Config 600/700, keine produktiven Credentials, kein Passwort in Environment/Logs/Repo. Keine neue Secret-Infrastruktur. Keine Python-Features/Skripte; neue ausführbare Logik Rust, vorhandene Verwaltungs-/Buildwerkzeuge nur im erlaubten Rahmen nutzen.
2. Regressionsnachweise Fakten aus Auftrag Punkt 3 komplett, Assets Starting Stats Punkt 4 (V1 bestehend fünf starting_stats-Felder) sowie normaler Wiki-Store/Release, Legacy-Import/Tombstone/Revoke. Vorhandene Implementierung nicht neu erfinden. Tatsächliche kleine Root-Cause-Fixes in diesen Nicht-Source-Pfaden erlaubt, größere Abweichung genau melden. Keine Defaults/Budgets lockern.
3. E2E/Last-Runner so ausführbar machen, dass Orchestrator auf final integriertem Code-Head exakt erneut messen kann. Vorläufig auf eigenem Head validieren, dann PR abgeben; Finale Messung wird nach A und C gemeinsam veranlasst. Je 600 answered bei 8/16/32, Poolpeak <=4, null too many clients/unauthorized_evidence, keine unerklärten 429/503, bounded wait und Recovery. Nicht max_connections erhöhen.
4. Passende fmt/clippy/test/build-Gates aus Nutzerauftrag. Ignorierte DB-Tests ausdrücklich ausweisen und gezielt mit Testinstanz ausführen. Keine Tests abschwächen.

Exklusiver Scope: scripts/test_* und scripts/run_isolated_* für Brain-Testbetrieb, zugehörige bestehende Rust-Pilot-/Harness-Programme und Testdateien in brain-core/brain-storage/brain-serve/domain/kernel/retrieval/wiki/quality/legacy. NICHT brain-feeds oder dbrain-sources anfassen, sie gehören A. Keine Root-Exports/Cargo.lock ohne zwingenden Bedarf verändern; falls nötig nur gezielt und im Bericht markieren, Orchestrator integriert Konflikt. Keine Architektur-Statusdoku (F). Keine Replay-validation.rs aus PR #46 übernehmen, dessen Integration wird separat geprüft.

Isolierte Instanz an eigenem temporären Unix-Socket, keine vorhandene 5446-Instanz neu starten oder verändern. Testdaten nur selbst erstellt/Fixtures, keine Communitydaten extern. Vor Release-Build vorhandenen hostweiten Lock respektieren, eigene Worktree-Binaries, keine fremden Prozesse stoppen. Build-Jobs begrenzen, keine globalen ENV-/Cargo-Configänderungen.

Commit/Push eigener Branch und PR nach migration/rust-integration erlaubt. Nie selber mergen oder produktiv starten. Vor Abgabe gate_hook.py --review für eigene Änderung, unabhängiger Review folgt. Bericht .tasks/2026-09-29-technical-closeout/C-REPORT.md mit konkreten SHA/Befehlen/Exitcodes und wiederholbaren finalen Befehlen. Technische Lücken lösen, Betreiberfreigaben nicht erfinden.
