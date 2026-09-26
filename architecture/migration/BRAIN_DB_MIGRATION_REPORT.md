# Brain-Datenmigration aus DL-Main: Bericht

Stand: 26.09.2026. Kopie in die eigene Brain-Instanz, kein Cutover. In DL-Main wurde nichts gelöscht, geändert oder umgestellt.

## Quelle und Ziel

| | Quelle | Ziel |
|---|---|---|
| Instanz | DL-Main `postgresql@16-main`, PostgreSQL 16.14, Port 5432 | `deadlock-brain-postgresql`, PostgreSQL 16.14, Socket `/run/deadlock-brain-postgresql`, Port 5446 |
| Datenbank | `deadlock` (4,5 GB gesamt) | `brain` |
| Schema | `brain` (Owner `deadlock`, Superuser) | `brain_legacy` (Owner `brain_migrate`), daneben das leere Kernschema `brain` v2 |
| Leserolle | `postgres`, ein `REPEATABLE READ READ ONLY`-Snapshot, exportiert und an `pg_dump --snapshot` übergeben | Import als `brain_migrate` |
| Werkzeug | `ops/brain-postgres/legacy-import.sh` | Artefakte `/var/backups/deadlock-brain/postgresql/legacy-import-20260926T025758Z/` (0700) |

Hashes der Quelle wurden im selben Snapshot wie der Dump berechnet; laufende Writer (Patchnotes-Sync alle 5 Minuten) verfälschen den Vergleich deshalb nicht.

## Was in DL-Main Brain gehört

Bestand am 26.09.2026 04:35 CEST in DB `deadlock`:

- Schema `brain`: 55 Tabellen und die View `patch_changes`. Keine Funktionen, keine Trigger, keine Hypertables, keine Fremdschlüssel über Schemagrenzen.
- Schema `knowledge`: 3 Tabellen (`chunk_embeddings`, `index_generations`, `active_index`, rund 9,5 MB, pgvector). Angelegt von Deadlock-Bots (`dl-central-db/migrations/2026091802_knowledge_dense.sql`), geschrieben von `dl-knowledge` mit Rolle `dl_knowledge_dml`. **Gehört nicht Brain**, nicht migriert.
- Schema `patchnotes`: gehört dem Patchnotes-Bot, nicht migriert.
- Alle Brain-Objekte gehören der Rolle `deadlock` (Superuser, zugleich DL-Main-Owner). Es gab bisher keine eigene Brain-Rolle.
- 34 Leerlaufverbindungen der Rolle `deadlock` auf DB `deadlock`; eine Zuordnung zu Brain ist ohne `application_name` nicht möglich.

Aktive Brain-Writer (systemd, unverändert gelassen): `deadlock-brain-patchnotes-sync.timer` (5 min), `deadlock-brain-sheet-sync.timer` (4 h), `deadlock-brain-wiki-refresh.timer`, `deadlock-brain-youtube-learning.timer` (derzeit `failed`), `deadlock-brain-build-data.timer` (derzeit `failed`), `dl-brain-feeder.timer` (Deadlock-Bots), dazu `deadlock-brain-site.service` (lesend, Port 8087).

## Migrierte Datenklassen

49 Tabellen, 653 476 Zeilen, Dumpgröße 128 MB (custom format). Für jede Tabelle sind Zeilenzahl und `md5` über alle Zeilen (Textform, sortiert, `TimeZone=UTC`) in Quelle und Ziel identisch (`HASHES_EQUAL tables=49`). Die Zeilenform enthält alle Spalten, also auch IDs, Zeitstempel, Quell-URLs und Provenienzfelder der Alttabellen.

| Klasse | Tabellen (Zeilen) |
|---|---|
| Entitäten und Aliase | `entities` (905), `entity_aliases` (3 778), `entity_lineage` (810), `legacy_entities` (68), `entity_snapshots` (37 123), `current_entity_state` (5 804), `hero_catalog` (38), `item_catalog` (251) |
| Patches | `patch_events` (32 821), `patch_event_enrichments` (32 821), `patch_impact_notes` (883), `knowledge_events` (26 685), `reasoner_patch_deltas` (540) |
| Quellen und Läufe | `source_documents` (7 308), `source_runs` (558), `forum_claims` (831), `insight_records` (51), `meta_trend_notes` (124) |
| Helden-, Item- und Populationsstatistik | `hero_item_stats` (18 982), `hero_item_synergies` (282 132), `hero_stat_values` (9 779), `hero_stat_profiles` (402), `hero_ability_orders` (114), `population_player_matches` (167 414), `population_item_stats` (12 781), `population_ability_order` (1 305), `population_imbue_stats` (748), `population_hero_buckets` (87), `population_sync_runs` (4), `player_match_decision_notes` (8) |
| Reasoner und Lernen | `reasoner_builds` (1), `reasoner_item_scores` (173), `reasoner_backtests` (4), `learned_builds` (0), `build_learning_notes` (0), `analysis_notes` (0), `mechanic_notes` (0) |
| Google-Sheet-Spiegel | `sheet_items` (4 478), `sheet_tab_rows` (2 410), `sheet_raw_heroes` (419), `sheet_heroes_stats` (396), `sheet_boons_ap` (90), `sheet_hero_rankings` (83), `sheet_shop_bonuses` (41) |
| YouTube | `youtube_videos` (195), `youtube_transcripts` (18), `youtube_feed_sources` (13), `youtube_learning_claims` (0), `youtube_transcript_claim_attempts` (0) |

Anpassung beim Import: Spaltendefaults `public.gen_random_uuid()` (pgcrypto in DL-Main) wurden auf das eingebaute `pg_catalog.gen_random_uuid()` umgestellt, damit die Brain-Instanz ohne `pgcrypto` auskommt. Daten sind davon nicht betroffen. Grants und Owner der Quelle wurden bewusst nicht übernommen (`--no-owner --no-acl`); im Ziel liest nur `brain_readonly`.

## Bewusst nicht migriert

| Objekt | Grund |
|---|---|
| View `brain.patch_changes` | liest `patchnotes.changelog_posts` (Patchnotes-Bot). Definition gesichert als `patch_changes.viewdef.sql`; braucht einen Patchnotes-Feed |
| `brain.feeder_runs`, `brain.plan_items`, `brain.plan_items_verworfen`, `brain.plan_runs` (30 Zeilen) | Werden ausschließlich von Deadlock-Bots geschrieben (`dl-brain-feeder`, `dl-dashboard`); Second-Brain-Planung, keine Brain-Wissensdaten |
| Schema `knowledge` | gehört `dl-knowledge` in Deadlock-Bots |
| Community-, Discord-, Twitch-, Steam- und Nutzertabellen | nicht Brain |

## Abhängigkeiten zu DL-Main, die einen Vertrag brauchen

| Stelle | Richtung | Heute | Nötiger Vertrag |
|---|---|---|---|
| `rust/crates/deadlock-brain/src/pg_patchnotes.rs`, `rust/crates/dbrain-sources/src/patchnotes_db.rs`, `dbrain-sources/src/store.rs`, View `patch_changes` | Brain liest `patchnotes.changelog_posts` | direkter SQL-Zugriff mit DL-Main-Credential | Patchnotes-Feed als Source (Export oder API des Patchnotes-Bots mit Post-ID, Titel, URL, Zeitpunkt, Rohtext, Hash) |
| `rust/crates/dbrain-reasoner/src/publish.rs`, `rust/crates/deadlock-brain/src/main.rs` | Brain schreibt `steam.steam_tasks` (`BUILD_PUBLISH_ORIGINAL`) und liest Status/`hero_build_id` | direkter SQL-Zugriff | Build-Publish-API des Steam-Bots (Auftrag anlegen, Status abfragen, idempotent über Auftrags-ID) |
| Deadlock-Bots `rust/crates/dl-brain/src/api_ingest.rs` | Bots schreiben `brain.entity_snapshots`, `current_entity_state`, `hero_catalog`, `item_catalog`, `source_runs` | Fremd-Writer im Brain-Schema | Deadlock-API-Ingest nach Brain verlegen oder als Feed an Brain liefern; kein Schreibzugriff fremder Dienste auf die Brain-Instanz |
| Deadlock-Bots `dl-bot`, `dl-brain-community`, `dl-brain-feeder`, `dl-dashboard`, MCP `dl-brain` | lesen `brain.*` (u. a. `patch_events`, `plan_*`, `entity_snapshots`, `patch_changes`) | direkter SQL-Zugriff | über `brain-serve`/BrainClient (C9) oder einen Lese-Export; die `plan_*`-Tabellen bleiben bei Deadlock-Bots |
| Match-, Meta- und Analysedaten | Brain liest Deadlock API | HTTP | bleibt so: Deadlock API ist Provider, kein eigenes ClickHouse und keine lokale Kopie des Analysebestands |

Keine dieser Stellen wurde mit einem zweiten Credential an DL-Main angebunden.

## Vergleich Kernmodell (Records, Revisionen, ACL, Tombstones, Releases, Checkpoints)

DL-Main enthält **keine** Tabellen des neuen Kernmodells (`source_record_revisions`, `source_record_heads`, `corpus_releases_v1`, `source_jobs_v1`, `source_checkpoints_v1`, `conversation_owners_v1`, `core_schema_version`). Es gibt dort also keine Releases, Checkpoints, Tombstones oder Knowledge-Versionen, die zu vergleichen wären. Im Ziel ist das Kernschema leer und auf Version 2 geprüft.

Die Kernsemantik wurde stattdessen an echten Pilotdokumenten in `brain_pilot` und per Backup/Restore geprüft: 20 Revisionen, 18 Heads, 1 Tombstone, 2 private Heads nach Revoke, Release `pilot-r1@pilot-knowledge-v1`, 3 Checkpoints, 279 Conversation-Owner; nach Restore in eine Wegwerf-Instanz identisch inklusive ACLs.

Offen: Es gibt keinen Konverter von den Alttabellen in `SourceRecordV2`. Welche Alttabellen als Quellen in den Kern-Store übernommen werden (und mit welcher Provenienz, Sichtbarkeit und Gültigkeit), ist eine eigene Aufgabe vor dem Cutover.

## Abweichungen

Keine. Zeilenzahlen und Inhaltshashes aller 49 migrierten Tabellen stimmen überein. Das Ergebnis betrifft den Snapshot vom 26.09.2026 02:57 UTC; spätere Writes in DL-Main sind nicht enthalten, weil die Writer bewusst nicht umgestellt wurden.
