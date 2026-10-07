status: blockiert
Datum: 2026-10-03

# Q8: Retentionspfad und verbleibende Vertragsgrenzen

Erhaltener HEAD: `5c220a8f047eb980d953d9f9f285b34739b5ed88`. Kein Commit, keine Migration, kein Datenbankschreiben, kein Deploy und kein Anbieteraufruf.

## Gelieferter Entwurf

Der abgeschlossene ursprüngliche Q8-Workflow liefert `source_retention.rs`, `pg_source_retention.rs`, `pg_source_retention.sql` und `tests/source_retention.rs` unter `rust/crates/brain-storage/`. Anfrage-/Berichtsclient und unnummerierter SQL-Installationsvertrag sind vorhanden. Der SQL-Pfad begrenzt sich auf `google-sheet/` und `youtube-core/`, erhält aktuelle Köpfe und blockiert die Löschung releasegepinnter Revisionen sowie Inhalte in Checkpoint-Batchkopien. Eine eigene Tabelle für Ablösezeitpunkte hält die Frist von zwölf Kalendermonaten auch dann fest, wenn Zwischenrevisionen entfernt werden. Bestätigte Tombstones machen ältere Revisionen unmittelbar für die Bereinigung fällig.

Parallel entstanden `pg_q_retention.rs`, `pg_q_retention_types.rs`, `pg_q_retention.sql` und `tests/q_retention.rs` durch den zusätzlich aufgenommenen lokalen Agentkontext. Beide Entwürfe bleiben erhalten; keiner wird als zweiter Produktivpfad exportiert. Vor Integration ist genau ein Pfad zu konsolidieren. `brain-storage/src/lib.rs` wurde nicht verdrahtet. Der echte PgStore-Adapter ist deshalb unkompiliert, die PostgreSQL-Ausführung ungeprüft.

## Geprüfter Bestand

Vorhandene Komponenten: PgStore-Quellensperren, eingezäunte Checkpoint-Commits, unveränderliche Releasepublikation, historische/aktuelle Rechteprüfung in `CorpusSnapshot::authorized` einschließlich Tombstones sowie PgStore- und LocalPgReader-Snapshots. Kernmigrationen und Grants wurden gelesen. Legacy-`source_documents`, Rohdateien, Entity-Snapshots und normalisierte Sheettabellen implementieren keine normale Kern-Retention; ihre Bereinigung bleibt gesondert zu verbinden. Kein angeblicher vollständiger Datenschutzbeweis durch bloße Kernmetadaten.

## Noch erforderliche Integration

1. Einen Entwurf konsolidieren und dessen echten PgStore-Adapter kompilieren. SQL erst als neue, eindeutig nummerierte Owner-Migration integrieren; `origin/main` wurde von Q danach frisch auf `358ed4ee07d315d4d71dd0438f9446f5b6a22d49` geprüft. Vorgesehen: `source_retention_supersessions_v1`, eng begrenzte SECURITY-DEFINER-Funktion unter `brain_migrate`, nur EXECUTE für `brain_ingest`, keine allgemeinen DELETE-Rechte.
2. Sichere Verschwinden-Tombstones und Kandidaten über den bestehenden Writer-/Leasepfad committen. Frische Vollständigkeitsprüfung und sichere Leermengen ergänzt Q7.
3. Veraltete betroffene Releases nach Prüfung aller aktiven Bindungen ausdrücklich stilllegen. Der Entwurf löscht gepinnte Inhalte bewusst nicht. Bestehende Rohkopien in `batch_json` brauchen einen geprüften Replay-Digest-Vertrag, der Idempotenz erhält.
4. Über Zs gemeinsame interne Aktivierung alte Anfragen/Reader leeren, internen Grant und Operatorrelease atomar austauschen, Snapshots/Caches ersetzen und Rechte prüfen. Erst danach sicher bereinigen. Öffentliche Bindungen erhalten. Legacy-Materialisierungen, Rohdateien und Backups gesondert berücksichtigen.
5. `require_history_retention()` bleibt blockierend, bis echte PostgreSQL- und Live-Wirkung nachgewiesen sind. Eine Retentionskonfiguration oder reine Fixtureprüfung reicht nicht zur Timerfreigabe.

## Tatsächliche Prüfungen

Rust 1.97.1, `--locked --offline --jobs 2`, beide Hostlocks und frische Compilerprobe laut Worker. `cargo test -p brain-storage --test source_retention --test ir_contracts -- --include-ignored`: Exit 0, sieben bestanden, null fehlgeschlagen, null ignoriert. Striktes Clippy für den Integrationstest sowie gezieltes Rustfmt: Exit 0. Logs: `.q-retention-tests.log`, `.q-retention-clippy.log` im Q-Worktree. Der Anfrageclient wurde über den Testharness kompiliert, der nicht exportierte PgStore-Adapter nicht.

`BRAIN_CORE_TEST_PG_SOCKET` fehlt. Keine PostgreSQL-Suite und keine SQL-Löschung ausgeführt. Lokale Tests belegen weder Speicherung noch ausgelieferte Aufbewahrungswirkung.

TESTNACHWEIS[TW-1]: 7 passed, 0 ignored | Baseline: nicht erhoben rot
