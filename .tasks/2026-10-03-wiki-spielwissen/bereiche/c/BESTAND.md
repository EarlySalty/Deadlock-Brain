status: erledigt
Datum: 2026-10-03
Belegstand: origin/main 511a347b653beba13c2bf130f4bead7a7196cc2a

# Native Bestandsrecherche C

Lesender Worker `a881d910ea2e58ff4` hat die aktuelle Datenhaltung und den produktiven Lesepfad geprüft. Keine Änderungen, Builds, Imports oder Neustarts durch die Recherche. Sol high auf Proxy 18768 bestätigt.

## Vorhandene Datenhaltung

`brain_storage::PgStore` in `rust/crates/brain-storage/src/lib.rs:156` und `:165` nutzt `brain.source_record_revisions`, `brain.source_record_heads` und `brain.corpus_releases_v1`. Revisionen sind unveränderlich. Wiederholungen ergeben Unchanged, gleiche Revision bei abweichendem Inhalt RevisionConflict; ältere Revisionen werden ignoriert (`:194-248`). `SourceRecordV2` in `brain-contracts/src/lib.rs:169` verlangt positive numerische Store-Revisionen bis i64::MAX. Originale Stringrevisionen dürfen nicht als erfundene Zahlen ausgegeben werden.

Gemeinsame Herkunft und Rechte existieren bereits: `OriginArtifact` und `SourcePolicy` in `brain-contracts/src/source.rs:104-132`. Lizenz, Sichtbarkeit, Veröffentlichung, Anbieterweitergabe und Rohdatenspeicherrechte sind getrennt. Die Ursprungsversion bleibt im OriginArtifact; eine gegebenenfalls lokal zugeteilte Store-Revision muss ausdrücklich davon unterschieden werden.

`DocumentStorePort` wird in `brain-storage/src/pg_jobs.rs:98` implementiert. `pg_release.rs:234` stellt `commit_batches_and_publish_checked` bereit; der Maintenance-Pfad (`:244`) erhält die unberührten Release-Pins. Ein Batch erlaubt höchstens 10.000 Records und genau einen Record je logischer ID (`brain-contracts/src/store.rs:89-123`).

## Vorhandene Quellenpfade

`external::SourceIr::from_text` (`dbrain-sources/src/external.rs:174`) erhält exakte UTF-8-Rohbytes und Quarantäne für ungültige Inhalte. `SourceStore::persist_ir` (`store.rs:131`) speichert Rohdaten und Herkunft im vorhandenen Quellenbestand. Allein SourceStore genügt nicht für Versionen und Konflikte, weil dieser Pfad nach Inhalt dedupliziert.

`wiki_corpus.rs:139` übernimmt gepinnte Wiki-Seiten und Revisionen mit Lizenz und Antwort-Hash. Vollständige erfolgreiche Läufe veröffentlichen die genaue Snapshot-Mitgliederliste (`:194-219`). `wiki_runtime::stage_into_store` (`:566`) zeigt die Brücke auf gemeinsame SourceRecordV2-Records. ScratchWikiStore (`:375`) ist nur Pilot, kein Produktionsziel.

## Produktiver Brain-Zugriff

`brain-serve/src/service.rs:79`, `:145-174` und `:242` nutzen LocalPgReader und ReleaseRetriever und verlangen den ausdrücklich gewählten Release samt Wissensversion. HTTP-Endpunkte `/v1/retrieve` und `/v1/answer` sind in `brain-api/src/http.rs:21` verankert. Neue Datenbankrecords ohne ausgewählten CorpusRelease sind nicht produktiv erreichbar.

Der separate Markdown-Wiki-Lesepfad filtert bisher deadlock_data und deadlock_wiki (`dbrain-retrieval/src/game_wiki.rs:28`, `:698-751`). Er ersetzt nicht den Core-Release-Lesepfad.

## Runtime und Migration

Aktiver `brain-serve.service` arbeitet unter `/opt/deadlock-brain/current` und startet das installierte Rust-Binary mit `/etc/deadlock-brain/brain-serve.json` und vorhandener Infisical-Konfiguration (`~/.config/systemd/user/brain-serve.service:10-12`). Parent-C hat zusätzlich PID und exakten aktuellen Release-SHA separat in LIVE_BASELINE.md belegt. Ein Deploy-Wrapper ist noch nicht verifiziert.

Schema-Prüfung und Owner-Migration sind getrennt (`brain-storage/src/schema.rs:76-96`, `src/bin/brain-migrate.rs:1-17`). Bereits angewandte Migrationen bleiben unverändert.

Der beauftragte historische Ausgangsstand 2734c2d enthält die heutigen Core-Crates und Runtime-Pfade nicht. Der aktuelle Main-Integrationsworktree ist deshalb die produktive Grundlage. Fremde 21 Ausgangsbranch-Commits werden nicht integriert.
