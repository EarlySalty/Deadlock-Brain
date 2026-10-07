# K: Bestehende Maintenance-Suite prüfen

## Ziel und Vertrag

Derselbe K-Auftrag, Produzent teil-k, Versuch 1. Eigene Artefaktschicht ist gebaut, aber ein zusätzlicher bestehender Maintenance-Bibliothekslauf endete mit 56 passed, 4 failed, 0 ignored. Die vier Fehler nennen eine fehlende feste PostgreSQL-Testfixture. Log: `/tmp/brain-k-artifact-tests-final.log`. Keine Vorherbaseline belegt. Ursache empirisch prüfen und denselben bestehenden Lauf mit einer nachweislich isolierten vorhandenen Testmechanik abschließen. Keine Altfehlerbehauptung ohne genaue Vorhermessung.

Vor Suche Graphify fragen, danach genaue Fundstelle lesen. Bereits abgefragtes Symbol `postgres_runner_private_basis_reset_and_rejected_review_resume` hatte im globalen Graph keinen Treffer. Tatsächliche Fehlerorte: `rust/crates/brain-maintenance/src/integration/runner_tests.rs:269` und `:1035`.

## Eigentum und Arbeitsstand

Worktree `/home/nathanael/.worktrees/brain-k-ki-20261007`, Branch `feat/brain-k-ki-20261007`, HEAD `7cbb9fe6`. K-Artefaktmodule, Site, Migration und eigene Akten sind vorhandener eigener WIP. Nicht ändern. Keine Produktdateien schreiben, keine Testabschwächung, kein Commit, Push, Merge, Releasebuild oder Deploy. Du besitzt ausschließlich eigene temporäre Prüfartefakte und gegebenenfalls eine von dir gestartete isolierte Testfixture. Fremde Dienste, Testcluster, Locks und Buildprozesse bleiben unberührt. Bestehende Fixture nur verwenden, wenn ihre Isolation und Zuständigkeit bewiesen sind; keinen fremden Cluster stoppen oder verändern.

## Beweisziel

Exakte verwendete Befehle, Exit-Codes und Testzahlen melden. Vorhandene Compiler-Caches nutzen, echte freie Buildslots unter `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/build-slot-{1,2,3}.lock` halten, höchstens `--jobs 3`, moderne absolute Cargo-CLI `+1.97.1`. `SQLX_OFFLINE=true` ist nur Prüfkonfiguration. Der fehlgeschlagene Befehl war `cargo test -p brain-storage -p brain-maintenance --lib --test compare_artifact -- --include-ignored --test-threads=1`; vorhandene genaue vollständige Variante im eigenen Workertranscript ab219fb00896aa7ba. Keine Pipe oder nachgeschalteten Marker, die Test-Exits verdecken. Fehlende Fixture ist kein Codefehlerbeweis. Wenn isoliert nicht sicher herstellbar, genaue belegte Ursache statt fremder Eingriffe melden. Eigene gestartete Fixture nach Prüfung sauber beenden, andere nicht.

## Grenzen und Routing

NEVER read, print or write plaintext secrets.
MUST NOT send private user/community data to remote models.
Keine Produktdaten oder private Fragen lesen. Keine G-, I-, Q- oder H-Dateien schreiben. Keine Modell-, Provider- oder Timeoutänderung. Keine neuen Connectoren, Kommentare oder dauerhaften Skripte. Kein ListAgents, SendMessage oder neuer T3-Thread. Kein Reviewer, nur ausführender Prüffixer. Keine weitere Delegation.

Auftraggeber K-Session `988eeaea-28ee-424c-b362-e250610cde91`; Delegator `481426fe-b477-42b3-91c6-901811fcba1d`, Hauptorchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7`. Rückgabe direkt als native Task-Rückgabe. K schreibt den Fachbericht und integriert. Zentrale Register/TODO unverändert. Keine Rundenmeldungen, Rückgabe nur bei Abschluss oder echtem Blocker.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: /home/nathanael/.worktrees/brain-k-ki-20261007
