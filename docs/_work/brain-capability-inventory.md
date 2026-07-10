# Brain Capability-Inventur + Q&A-Gap

Stand: 2026-06-25. Analyse read-only gegen Code und `data/deadlock_brain.sqlite3` via SQLite-URI `file:data/deadlock_brain.sqlite3?mode=ro`. Hinweis: `sqlite3` CLI fehlt in der Umgebung; DB-Zahlen wurden mit Python `sqlite3` im Read-only-Modus ermittelt.

## 1. CRATE-KARTE

### `deadlock-brain-core`

Gemeinsamer Kern fuer Pfade/Config, SQLite-Schema, HTTP-Client, Fireworks-Client und Datenmodelle; Feature-Logik liegt explizit in nachgelagerten Crates (`rust/crates/deadlock-brain-core/src/lib.rs:3-17`). Wichtige oeffentliche Einstiegspunkte: `config::load_settings`/Defaults (`rust/crates/deadlock-brain-core/src/config.rs:98-162`), `db::open_connection`/`apply_pragmas`/`now_epoch_seconds` (`rust/crates/deadlock-brain-core/src/db.rs:22-41`), `schema::ensure_schema`/`ensure_youtube_tables` (`rust/crates/deadlock-brain-core/src/schema.rs:701-708`), `HttpClient::new`/`get`/`get_json` (`rust/crates/deadlock-brain-core/src/http.rs:77-114`), `AiClient::chat`, `build_review_request`, `extract_ai_text` (`rust/crates/deadlock-brain-core/src/ai.rs:143-163`, `rust/crates/deadlock-brain-core/src/ai.rs:196-248`).

### `dbrain-sources`

Source-Ingestion fuer Assets API, Deadlock API, `deadlock-data`, Google Sheet, Patchnotes-DB, Statlocker und Wiki; es schreibt Source-Runs, Raw-Files, `source_documents` und `entity_snapshots` ueber den gemeinsamen Store (`rust/crates/dbrain-sources/src/lib.rs:3-29`, `rust/crates/dbrain-sources/src/store.rs:41-59`, `rust/crates/dbrain-sources/src/store.rs:82-147`). Oeffentliche Einstiegspunkte: `pull_assets`, `pull_match_metadata`, `pull_deadlock_data`, `pull_sheet`, `refresh_sheet`, `discover_sheet_tabs`, `pull_patchnotes`, `pull_statlocker`, `pull_wiki_page` (`rust/crates/dbrain-sources/src/lib.rs:19-29`); `pull_deadlock_data` markiert den Import als trusted und upsertet auch `entities`, `entity_aliases`, `hero_stat_profiles`, `hero_stat_values` direkt (`rust/crates/dbrain-sources/src/deadlock_data.rs:93-115`, `rust/crates/dbrain-sources/src/deadlock_data.rs:1225-1278`, `rust/crates/dbrain-sources/src/deadlock_data.rs:1397-1457`).

### `dbrain-normalize`

Normalisiert Snapshots in kanonische Entities/Aliasse, Patch-Events, Hero-Stats, Sheet-Tabellen, Lineage, Legacy-Entities und konservative Re-Resolution (`rust/crates/dbrain-normalize/src/lib.rs:3-20`). Oeffentliche Einstiegspunkte: `parse_patchnotes(_with_conn)`, `normalize_entities(_with_conn)`, `normalize_sheet_stats(_with_conn)`, `normalize_sheet_tabs(_with_conn)`, `resolve_gaps_with_conn`, `enrich_lineage(_with_conn)`, `enrich_legacy_entities(_with_conn)`, `normalize_alias`, `extract_lineage_candidates` (`rust/crates/dbrain-normalize/src/lib.rs:22-80`). Beispiel: `parse_patchnotes` liest Patchnote-Snapshots aus `entity_snapshots` und upsertet `patch_events` (`rust/crates/dbrain-normalize/src/patch.rs:119-153`, `rust/crates/dbrain-normalize/src/patch.rs:155-175`); Sheet-Normalisierung schreibt `hero_stat_profiles`/`hero_stat_values` bzw. `sheet_*` (`rust/crates/dbrain-normalize/src/sheet_stats.rs:14-24`, `rust/crates/dbrain-normalize/src/sheet_tabs.rs:106-130`).

### `dbrain-enrich`

Reichert `patch_events` deterministisch zu strukturierten Stat-/Value-Changes an und erzeugt LLM-gestuetzte Patch-Impact-/Meta-Notizen (`rust/crates/dbrain-enrich/src/lib.rs:3-25`). Oeffentliche Einstiegspunkte: `enrich_patch_events`, `enrich_patch_impact`, `enrich_meta_trends`, `build_patch_event_enrichments`, `enrich_patch_event`, `build_patch_impact_context`, `build_patch_impact_request`, `save_patch_impact_note`, `run_patch_impact_batch(_with_chat)`, `run_meta_trend_analysis(_with_chat)` (`rust/crates/dbrain-enrich/src/lib.rs:181-199`, `rust/crates/dbrain-enrich/src/lib.rs:509-578`, `rust/crates/dbrain-enrich/src/lib.rs:680-688`, `rust/crates/dbrain-enrich/src/lib.rs:752-780`). Die deterministische Enrichment-Schicht nutzt Regex-Patterns fuer `from/to`, `by amount`, `now grants`, Tier-Wechsel usw. (`rust/crates/dbrain-enrich/src/lib.rs:60-75`).

### `dbrain-retrieval`

Liefert SQL-/Alias-basiertes Retrieval fuer Status, Entity-Kontext, Timeline, Review-Kontext, Quality, Item-Kontext und persistente Analyse-Notizen (`rust/crates/dbrain-retrieval/src/lib.rs:3-33`). Oeffentliche Einstiegspunkte: `status`, `context`/`build_entity_context`, `timeline`/`build_entity_timeline`, `review`/`build_review_context`, `quality`/`run_quality_checks`, `item`/`build_item_context`, `analysis_save_review(_for_query)`, `analysis_run_ai`, `analysis_list`, `search_mechanic_notes`, `analyze_query` (`rust/crates/dbrain-retrieval/src/lib.rs:135-205`, `rust/crates/dbrain-retrieval/src/lib.rs:258-354`, `rust/crates/dbrain-retrieval/src/lib.rs:401-583`). Wichtig: `search_mechanic_notes` ist kein aktiver Vector-Retriever; wenn Tabellen fehlen, gibt es `[]`, sonst `todo!("Vektorsuche: spaetere Wave")` (`rust/crates/dbrain-retrieval/src/lib.rs:576-580`).

### `dbrain-learn`

Build- und Player-Lernworkflows plus deterministischer Hero-Build-Kontext/Suggestor; YouTube ist im Kommentar noch genannt, operativ aber im separaten `deadlock-brain-yt`-Binary (`rust/crates/dbrain-learn/src/lib.rs:3-26`). Oeffentliche Einstiegspunkte: `learn_import_steam_builds`, `learn_list_builds`, `build_learning_context`, `build_ai_build_learning_request`, `learn_analyze_build`, `learn_analyze_next`, `build_suggest`, `build_hero_build_context`, `build_match_coaching_context`, `build_player_match_decision_context`, `build_ai_player_match_decision_request`, `player_analyze_match`, `player_analyze_next`, `player_list_matches`, `player_match_context` (`rust/crates/dbrain-learn/src/lib.rs:14-26`). `build_hero_build_context` erzeugt einen deterministischen Build-Kontext aus Hero-Payload, Abilities, Wiki, Statlocker, Items und Review-Signalen (`rust/crates/dbrain-learn/src/build_optimizer.rs:82-155`); Fireworks wird erst in den `analyze-*` Pfaden genutzt (`rust/crates/dbrain-learn/src/build_learning.rs:377-412`, `rust/crates/dbrain-learn/src/build_learning.rs:414-424`, `rust/crates/dbrain-learn/src/player_decision_learning.rs:173-220`).

### `deadlock-brain` (CLI-bin)

Kein oeffentliches Library-API; operative Einstiegspunkte sind private `main()`, `run_from_cli()` und `run()` (`rust/crates/deadlock-brain/src/main.rs:735-860`). Die komplette CLI-Command-Struktur liegt in `Commands` und Nested-Enums (`rust/crates/deadlock-brain/src/main.rs:35-97`, `rust/crates/deadlock-brain/src/main.rs:177-187`, `rust/crates/deadlock-brain/src/main.rs:253-263`, `rust/crates/deadlock-brain/src/main.rs:323-339`, `rust/crates/deadlock-brain/src/main.rs:483-490`, `rust/crates/deadlock-brain/src/main.rs:539-549`, `rust/crates/deadlock-brain/src/main.rs:645-705`).

### `deadlock-brain-yt`

Separates YouTube-Ingestion-Binary mit Gemini-Web-Worker, Queue und Claim-Speicherung; CLI-Kommandos sind `gemini-login`, `ingest`, `claims`, `smoke` (`rust/crates/deadlock-brain-yt/src/main.rs:14-42`, `rust/crates/deadlock-brain-yt/src/main.rs:51-83`). Oeffentliche Moduleinstiege: `run_ingest` (`rust/crates/deadlock-brain-yt/src/loop_runner.rs:24-41`), `build_prompt`, `parse_model_claims`, `save_claims`, `query_claims` (`rust/crates/deadlock-brain-yt/src/claims.rs:25-29`, `rust/crates/deadlock-brain-yt/src/claims.rs:107-180`), `discover_youtube_videos`, `select_next_videos`, `mark_failed`, `mark_success` (`rust/crates/deadlock-brain-yt/src/queue.rs:101-138`, `rust/crates/deadlock-brain-yt/src/queue.rs:551-593`).

## 2. CLI-BEFEHLS-INVENTAR

Legende: **ASSEMBLY** = sammelt/normalisiert Daten oder baut Kontext ohne LLM-Antwort. **MODELL** = ruft ein Modell auf, sofern nicht `--dry-run`. **DEFERRED** = CLI gibt nur Hinweis auf anderes Binary aus.

| Befehl | Klasse | Was tut er | Modellaufruf | Tabellen read/write |
|---|---:|---|---|---|
| `status` | ASSEMBLY | Gibt Counts fuer Quellen, Snapshots, Patch-Events, Entities, Derived Data aus (`main.rs:757`, `dbrain-retrieval/src/lib.rs:135-199`). | Nein. | R: `source_documents`, `entity_snapshots`, `patch_events`, `entities`, optional `patch_event_enrichments`, `entity_lineage`, `legacy_entities`, `hero_stat_profiles`, `hero_stat_values`, `analysis_notes`, `build_learning_notes`, `player_match_decision_notes`. W: keine. |
| `context <query>` | ASSEMBLY | Baut Entity-Kontext: `best_match`, `aliases`, `lineage`, `patch_events`, `enrichments`, `sheet_stats`, `fallback` (`main.rs:758-769`, `dbrain-retrieval/src/lib.rs:205-255`). | Nein. | R: `entities`, `entity_aliases`, `entity_lineage`, `legacy_entities`, `patch_events`, `patch_event_enrichments`, `entity_snapshots`, dynamisch Sheet-/Stat-Tabellen (`dbrain-retrieval/src/lib.rs:664-760`, `874-1049`). W: keine. |
| `timeline <query>` | ASSEMBLY | Baut Patch-Timeline: gruppierte `patches`, `event_count`, `patch_count`, `impact_summary`, Enrichment-Meta, Fallback (`main.rs:771-783`, `dbrain-retrieval/src/lib.rs:262-339`). | Nein. | R: wie `context`, plus `patch_event_enrichments` fuer Impact-Klassifizierung. W: keine. |
| `review <query>` | ASSEMBLY | Baut KI-tauglichen Review-Kontext ohne Modell: `entity_summary`, `lineage`, `current_stat_hints`, `timeline_signals`, `open_questions`, `source_references`, `prompt_de`, `retrieval_meta` (`main.rs:785-799`, `dbrain-retrieval/src/lib.rs:354-399`). `--prompt-only` druckt nur `prompt_de` (`main.rs:791-793`). | Nein; kein `AiClient` im Pfad. | R: wie `context`. W: keine. |
| `quality` | ASSEMBLY | Lokale Datenqualitaetschecks (`main.rs:801-808`, `dbrain-retrieval/src/lib.rs:405-437`). | Nein. | R: `sqlite_master`, `source_documents`, `entity_snapshots`, `entities`, `entity_aliases`, `patch_events`, `patch_event_enrichments`, `entity_lineage`, `legacy_entities`, `hero_stat_profiles`. W: keine. |
| `lineage [query]` | ASSEMBLY | Listet Rename/Rework-Lineage (`main.rs:810-817`, `main.rs:1421-1461`). | Nein. | R: `entity_lineage`. W: keine. |
| `legacy [query]` | ASSEMBLY | Listet alte/entfernte Entities (`main.rs:819-826`, `main.rs:1464-1499`). | Nein. | R: `legacy_entities`. W: keine. |
| `build <hero>` | ASSEMBLY/HEURISTIK | Erzeugt deterministischen Hero-Build-Kontext/Vorschlag (`main.rs:828-840`, `dbrain-learn/src/build_optimizer.rs:82-155`). | Nein. | R: Review-Kontexttabellen, `entity_snapshots` fuer Hero/Ability/Item/Wiki/Statlocker (`dbrain-learn/src/build_optimizer.rs:885-950`, `1360-1455`). W: keine. |
| `item <query>` | ASSEMBLY | Zeigt strukturierte Item-Daten aus Assets-Payload, inklusive Slot/Tier/Cost/Properties/Archetypes (`main.rs:842-849`, `dbrain-retrieval/src/lib.rs:443-451`, `dbrain-retrieval/src/lib.rs:2873-2928`). | Nein. | R: `entities`, `entity_aliases`, `entity_snapshots`, plus minimale Kontextsuche. W: keine. |
| `learn import-steam-builds` | ASSEMBLY/IMPORT | Importiert Steam/GC Builds aus externer Steam-Bot-DB (`main.rs:865-880`, `dbrain-learn/src/build_learning.rs:58`, SQL ab `dbrain-learn/src/build_learning.rs:656`). | Nein. | R: externe `hero_build_sources`, Brain-Assets/Entities fuer Mapping. W: `learned_builds`. |
| `learn list-builds` | ASSEMBLY | Listet importierte Builds (`main.rs:882-893`, `dbrain-learn/src/build_learning.rs:214`). | Nein. | R: `learned_builds`. W: keine. |
| `learn analyze-build` | MODELL | Baut Build-Learning-Kontext und laesst Fireworks analysieren; `--dry-run` speichert nur Kontext (`main.rs:895-917`, `dbrain-learn/src/build_learning.rs:496-539`). | Ja, `AiClient::chat` bei nicht dry-run (`dbrain-learn/src/build_learning.rs:533-534`). | R: `learned_builds`, Build-/Review-Kontexttabellen. W: `build_learning_notes` (`dbrain-learn/src/build_learning.rs:577-607`). |
| `learn analyze-next` | MODELL | Analysiert naechste offene Builds (`main.rs:919-942`, `dbrain-learn/src/build_learning.rs:424-470`). | Ja, pro Build wie `analyze-build`, ausser `--dry-run`. | R: `learned_builds`, `build_learning_notes`, Kontexttabellen. W: `build_learning_notes`. |
| `player list-matches` | ASSEMBLY | Listet importierte Player-Matches (`main.rs:949-960`, `dbrain-learn/src/player_decision_learning.rs:44`). | Nein. | R: `entity_snapshots`/Statlocker-Match-Snapshots. W: keine. |
| `player match-context` | ASSEMBLY | Baut Fireworks-tauglichen Player-Match-Kontext ohne Modell (`main.rs:962-969`, `dbrain-learn/src/player_decision_learning.rs:93-97`). | Nein. | R: Statlocker/Deadlock-API Snapshots in `entity_snapshots`, Item-/Hero-Payloads, Build-Kontexttabellen. W: keine. |
| `player analyze-match` | MODELL | Analysiert ein Player-Match als Entscheidungsbeispiel (`main.rs:971-994`, `dbrain-learn/src/player_decision_learning.rs:220-228`). | Ja, `AiClient::chat` bei nicht dry-run (`dbrain-learn/src/player_decision_learning.rs:416-417`). | R: Player-Match-Kontexttabellen. W: `player_match_decision_notes`. |
| `player analyze-next` | MODELL | Analysiert naechste offene Player-Matches (`main.rs:996-1019`, `dbrain-learn/src/player_decision_learning.rs:231-245`). | Ja, ausser `--dry-run`. | R: `player_match_decision_notes`, Player-Match-Snapshots. W: `player_match_decision_notes`. |
| `youtube discover/import-transcripts/fetch-transcripts/transcribe-local/analyze-next/auto-learn/queue` | DEFERRED | In `deadlock-brain` nur Stub mit Hinweis auf `deadlock-brain-yt` (`main.rs:851-854`, `main.rs:1271-1289`). | Nein in diesem CLI. | R/W: keine fachlichen Tabellen im `deadlock-brain`-Pfad. Echte YouTube-Pipeline liegt in `deadlock-brain-yt`. |
| `analysis save-review` | ASSEMBLY + PERSIST | Speichert Review-Kontext optional mit fertigem `result_text` (`main.rs:1026-1040`, `dbrain-retrieval/src/lib.rs:474-497`). | Nein. | R: wie `review`. W: `analysis_notes` (`dbrain-retrieval/src/lib.rs:3101-3193`). |
| `analysis run-minimax` | MODELL | Baut Review-Kontext, baut Fireworks-Request, ruft Fireworks und speichert Ergebnis; `--dry-run` nur Request (`main.rs:1042-1064`, `dbrain-retrieval/src/lib.rs:499-550`). | Ja, `AiClient::chat` bei nicht dry-run (`dbrain-retrieval/src/lib.rs:526-528`). | R: wie `review`. W: `analysis_notes`. |
| `analysis list` | ASSEMBLY | Listet gespeicherte Analyse-Notizen (`main.rs:1066-1077`, `dbrain-retrieval/src/lib.rs:552-574`). | Nein. | R: `analysis_notes`. W: keine. |
| `pull assets` | IMPORT | Holt Assets API Endpoints (`main.rs:1085-1092`, `dbrain-sources/src/assets_api.rs:32-85`). | Nein. | R: HTTP/cache. W: Raw-Dateien, `source_runs`, `source_documents`, `entity_snapshots`. |
| `pull deadlock-data` | IMPORT + NORMALIZE | Holt trusted `deadlock-data` und parst danach Patchnotes (`main.rs:1094-1110`, `dbrain-sources/src/deadlock_data.rs:30-115`). | Nein. | R: Git/Files, vorhandene Entities/Aliasse. W: `source_runs`, `source_documents`, `entity_snapshots`, `entities`, `entity_aliases`, `hero_stat_profiles`, `hero_stat_values`, danach `patch_events`. |
| `pull patchnotes` | IMPORT | Importiert Patchnotes aus zentraler Bot-DB read-only (`main.rs:1112-1121`, `dbrain-sources/src/patchnotes_db.rs:32-49`). | Nein. | R: externe `changelog_posts`. W: `source_runs`, `source_documents`, `entity_snapshots`. |
| `pull statlocker` | IMPORT | Holt Statlocker WPA/Leaderboard/Player/Match/Build-Analyse (`main.rs:1123-1149`, `dbrain-sources/src/statlocker.rs:62-90`). | Nein. | R: HTTP/cache, teils vorhandene `entity_snapshots`. W: `source_runs`, `source_documents`, `entity_snapshots`. |
| `refresh-sheet` | IMPORT + NORMALIZE | Holt alle Sheet-Tabs und normalisiert Stats/Tabs (`main.rs:1153-1172`, `dbrain-sources/src/google_sheet.rs:48-67`). | Nein. | W: `source_runs`, `source_documents`, `entity_snapshots`, `hero_stat_profiles`, `hero_stat_values`, `sheet_hero_rankings`, `sheet_boons_ap`, `sheet_raw_heroes`, `sheet_heroes_stats`, `sheet_items`, `sheet_shop_bonuses`, `sheet_tab_rows`. |
| `normalize entities` | NORMALIZE | Baut `entities`/`entity_aliases` aus Snapshots (`main.rs:1174-1178`, `dbrain-normalize/src/entities.rs:56`). | Nein. | R: `entity_snapshots`, `source_documents`. W: `entities`, `entity_aliases`. |
| `normalize sheet-stats` | NORMALIZE | Baut `hero_stat_profiles`/`hero_stat_values` (`main.rs:1179-1181`, `dbrain-normalize/src/sheet_stats.rs:14-113`). | Nein. | R: `entity_snapshots`, `entities`, `entity_aliases`. W: `hero_stat_profiles`, `hero_stat_values`. |
| `normalize sheet-tabs` | NORMALIZE | Baut tabellarisierte Sheet-Daten (`main.rs:1182-1184`, `dbrain-normalize/src/sheet_tabs.rs:106-130`). | Nein. | R: `entity_snapshots`, `entities`, `entity_aliases`. W: `sheet_hero_rankings`, `sheet_boons_ap`, `sheet_raw_heroes`, `sheet_heroes_stats`, `sheet_items`, `sheet_shop_bonuses`, `sheet_tab_rows`. |
| `normalize resolve-gaps` | NORMALIZE | Re-resolvt offene Entity-Luecken und Hero-FKs; `--dry-run` ohne Writes (`main.rs:1185-1187`, `dbrain-normalize/src/resolve_gaps.rs:53`, SQL bei `resolve_gaps.rs:75-216`, `resolve_gaps.rs:282`). | Nein. | R: `patch_events`, `youtube_learning_claims`, `entities`, `entity_aliases`, Hero-FK-Tabellen. W: `patch_events`, `youtube_learning_claims`, `hero_stat_profiles`/Sheet-FK-Tabellen je nach Treffer. |
| `parse patchnotes` | NORMALIZE | Parst Patchnote-Snapshots zu `patch_events` (`main.rs:1191-1195`, `dbrain-normalize/src/patch.rs:119-153`). | Nein. | R: `entity_snapshots`, `entities`, `entity_aliases`. W: `patch_events`. |
| `enrich patch-events` | ENRICH | Baut deterministische Patch-Event-Enrichments (`main.rs:1201-1203`, `dbrain-enrich/src/lib.rs:198-230`). | Nein. | R: `patch_events`, `patch_event_enrichments`. W: `patch_event_enrichments`. |
| `enrich lineage` | ENRICH | Baut Rename-/Rework-Lineage (`main.rs:1204-1206`, `dbrain-normalize/src/lineage.rs:54`, SQL `lineage.rs:187-260`). | Nein. | R: `patch_events`. W: `entity_lineage`. |
| `enrich legacy-entities` | ENRICH | Baut alte/entfernte Entity-Namen (`main.rs:1207-1209`, `dbrain-normalize/src/legacy.rs:44-110`). | Nein. | R: `patch_events`, `entities`, `entity_aliases`, `entity_lineage`. W: `legacy_entities`. |
| `enrich patch-impact` | MODELL | Baut Patch-Impact-Kontext; bei `--dry-run` nur Prompt/Request; sonst Fireworks und `patch_impact_notes` (`main.rs:1210-1256`, `dbrain-enrich/src/lib.rs:509-578`, `dbrain-enrich/src/lib.rs:680-688`). | Ja, ausser `--dry-run`. | R: `patch_events`, `patch_event_enrichments`, Review-Kontexttabellen. W: `patch_impact_notes`. |
| `enrich meta-trends` | MODELL | Ruft Fireworks fuer aktuell hart codierte Mock-Shifts `Abrams`/`Infernus` auf (`main.rs:1213-1216`, `dbrain-enrich/src/lib.rs:752-780`). | Ja. | R: keine echten Meta-Inputtabellen im aktuellen Codepfad; W: `meta_trend_notes`. |

## 3. MODELL-ANBINDUNG

### Fireworks im Core

Fireworks ist als Blocking HTTP-Client im Core implementiert. Defaults: Modell `accounts/fireworks/models/deepseek-v4-flash`, OpenAI-kompatible Base URL `https://api.fireworks.ai/inference/v1` (`rust/crates/deadlock-brain-core/src/config.rs:11-13`). `Settings` enthalten API-Key, Base URL, Modell, Timeout, Tokens, Temperatur, Top-P und Token-Plan-Flag (`rust/crates/deadlock-brain-core/src/config.rs:29-36`); `load_settings` zieht `FIREWORK_API_KEY` oder `FIREWORKS_API_KEY` und waehlt die passende Base URL (`rust/crates/deadlock-brain-core/src/config.rs:115-161`). Der Client sendet `POST {base_url}/chat/completions` mit Bearer Auth (`rust/crates/deadlock-brain-core/src/ai.rs:147-192`).

Aktuelle Fireworks-Aufrufer:

- `analysis run-minimax`: `analysis_run_ai` baut Review-Kontext, `build_review_request`, `AiClient::chat`, speichert in `analysis_notes` (`rust/crates/dbrain-retrieval/src/lib.rs:499-550`).
- `learn analyze-build`/`learn analyze-next`: `run_single_build_learning_analysis` baut Build-Learning-Request, ruft `AiClient::chat`, speichert in `build_learning_notes` (`rust/crates/dbrain-learn/src/build_learning.rs:496-539`, `rust/crates/dbrain-learn/src/build_learning.rs:577-607`).
- `player analyze-match`/`player analyze-next`: ruft Fireworks und speichert `player_match_decision_notes` (`rust/crates/dbrain-learn/src/player_decision_learning.rs:173-220`, `rust/crates/dbrain-learn/src/player_decision_learning.rs:416-420`).
- `enrich patch-impact`: baut Patch-Impact-Request und ruft Fireworks im Single- oder Batch-Pfad (`rust/crates/deadlock-brain/src/main.rs:1220-1256`, `rust/crates/dbrain-enrich/src/lib.rs:680-688`).
- `enrich meta-trends`: ruft Fireworks, aber aktuell mit Mock-Shifts statt realer Meta-Daten (`rust/crates/dbrain-enrich/src/lib.rs:770-780`, `rust/crates/dbrain-enrich/src/lib.rs:793-815`).

Nicht angebunden: `context`, `timeline`, `review`, `build`, `item`, `quality`, `lineage`, `legacy` rufen kein Modell auf; `review` erzeugt nur den Prompt-Kontext (`rust/crates/deadlock-brain/src/main.rs:785-799`, `rust/crates/dbrain-retrieval/src/lib.rs:354-399`).

### YouTube/Gemini

Das separate Binary `deadlock-brain-yt` nutzt einen Gemini-Web-Worker, nicht Fireworks: `run_ingest` ruft `gemini::analyze_url`, parsed Claims und speichert sie (`rust/crates/deadlock-brain-yt/src/loop_runner.rs:35-47`). Im `deadlock-brain` Haupt-CLI ist `youtube ...` nur ein Deferred-Hinweis auf dieses Binary (`rust/crates/deadlock-brain/src/main.rs:1271-1277`).

### Vektorsuche/Embeddings

Rust-Schema laesst Vector-Tabellen bewusst aus: `VECTOR_TABLES_OMITTED` listet `vector_embeddings*` (`rust/crates/deadlock-brain-core/src/schema.rs:35-41`), und `open_connection_creates_regular_schema_without_vector_tables` erwartet deren Abwesenheit (`rust/crates/deadlock-brain-core/src/db.rs:49-74`). Im Rust-Retrieval ist `search_mechanic_notes` explizit inaktiv bzw. `todo!("Vektorsuche: spaetere Wave")` (`rust/crates/dbrain-retrieval/src/lib.rs:576-580`). Die Live-DB enthaelt zwar alte `vector_embeddings*`-Tabellen, aber `SELECT COUNT(*) FROM vector_embeddings` scheitert ohne `vec0`-Extension (`OperationalError: no such module: vec0`); `vector_embeddings_rowids` und `vector_embeddings_chunks` haben 0 Zeilen. Faktischer Stand: keine nutzbare Rust-Embedding-Suche.

## 4. DATENMODELL FÜR Q&A

Live-Zeilenzahlen aus `data/deadlock_brain.sqlite3`:

| Tabelle | Zeilen | Q&A-Nutzen | Wichtige Spalten/Beleg |
|---|---:|---|---|
| `entities` | 1.088 | Kanonische Hero/Item/Ability-Aufloesung; Entity-Typen: 39 `hero`, 175 `item`, 121 `item_special`, 255 `ability`, 154 `ability_internal`, 25 `hero_internal`, 319 `weapon_or_internal`. | Schema `id`, `entity_type`, `canonical_name`, `primary_external_id`, `source`, `metadata_json` (`rust/crates/deadlock-brain-core/src/schema.rs:80-92`). |
| `entity_aliases` | 6.381 | Alias-/Classname-/External-ID-Matching fuer Userfragen. | Schema `entity_id`, `alias`, `alias_norm`, `alias_kind`, `source`, `snapshot_id` (`rust/crates/deadlock-brain-core/src/schema.rs:94-107`). |
| `entity_snapshots` | 10.886 | Roh-/Card-Payloads fuer Heroes, Items, Abilities, Patchnotes, Sheet-Zeilen, Statlocker, YouTube. Beispiele: Hero-Snapshot hat `starting_stats`, `scaling_stats`, `items`; Deadlock-data Ability hat `Upgrades`; Sheet-Snapshots enthalten `values`. | Schema `source`, `entity_type`, `external_id`, `canonical_name`, `payload_json`, `source_document_id` (`rust/crates/deadlock-brain-core/src/schema.rs:129-141`). |
| `patch_events` | 12.523 | Patch-Historie: "Was hat Patch X an Hero Y geaendert?", Buff/Nerf/Rework-Linien. Breakdown: forum 11.353 Events, steam 1.075, other 95; Hero-Events 6.507, Item 2.997, General 2.790, Ability 144, Item-special 85. | Schema `patch_*`, `source_kind`, `posted_at`, `entity_type`, `entity_name`, `subject`, `change_type`, `normalized_line`, `old_value`, `new_value`, `confidence`, `event_hash` (`rust/crates/deadlock-brain-core/src/schema.rs:259-283`). |
| `patch_event_enrichments` | 3.109 | Strukturierte Stat-/Value-Aenderungen pro Patch-Event; hilfreich fuer "war das ein Buff/Nerf?" | Schema `patch_event_id`, `stat_name`, `old_value`, `new_value`, `unit`, `ability_name`, `secondary_entity_name`, `confidence`, `flags_json` (`rust/crates/deadlock-brain-core/src/schema.rs:242-257`). |
| `entity_lineage` | 208 | Rename/Rework-Beziehungen, damit alte Namen/Abilities noch gefunden werden. | Schema `relation_type`, source/target/owner Namen, `confidence`, `metadata_json` (`rust/crates/deadlock-brain-core/src/schema.rs:109-127`). |
| `legacy_entities` | 18 | Entfernte/alte Entities ausserhalb normaler Entities; hilft bei alten Patchnotes/Creator-Claims. | Schema `legacy_type`, `canonical_name`, `observed_entity_type`, first/last Patch-Event, `event_count`, `confidence`, `samples_json` (`rust/crates/deadlock-brain-core/src/schema.rs:198-217`). |
| `hero_stat_profiles` | 366 | Aktuelle/ historische Hero-Stat-Profile aus Sheet und deadlock-data. | Schema `snapshot_id`, `entity_id`, `hero_name`, `source`, `external_id`, `payload_hash`, `row_number` (`rust/crates/deadlock-brain-core/src/schema.rs:143-157`). |
| `hero_stat_values` | 10.918 | Atomare Hero-Stats: Base HP, Move Speed, DPS, Scaling usw.; gut fuer Faktenfragen und Vergleiche. | Schema `profile_id`, `entity_id`, `hero_name`, `stat_key`, `stat_label`, `numeric_value`, `raw_value` (`rust/crates/deadlock-brain-core/src/schema.rs:159-173`). |
| `youtube_learning_claims` | 1.087 | Creator-Claims zu Build, Mechanic, Matchup, Counterplay usw. Status: 401 `accepted`, 454 `needs_review`, 232 `unverified`. Claim-Typen: 453 `mechanic`, 203 `build`, 90 `macro`, 59 `meta`, 53 `matchup`, 43 `counterplay`, 16 `combo`, usw. | Schema hat `entity_type`, `entity_name`, `claim_type`, `claim_text`, `evidence_quote`, `model_confidence`, `verifier_confidence`, `status`, `verifier_json` (`rust/crates/deadlock-brain-core/src/schema.rs:672-695`). Live-DB hat **keine** `l2_verdict`/`l2_verifier_json`-Spalten; `verifier_json` enthaelt Keys wie `gpt_verifier`, `quote_found`, `reasons`, `status`. |
| `youtube_videos` / `youtube_transcripts` | 542 / 305 | Quellen fuer Creator-Claims und Transcript-gestuetzte Belege. | Schema `youtube_videos` (`rust/crates/deadlock-brain-core/src/schema.rs:639-657`), `youtube_transcripts` (`rust/crates/deadlock-brain-core/src/schema.rs:659-670`). |
| `learned_builds` | 76 | Importierte Top-/Steam-Builds als schwache Labels; beantworten "welche Builds existieren fuer Hero X?" | Schema `hero_name`, `quality_tier`, `quality_score`, `item_names_json`, `ability_order_json`, `details_json` (`rust/crates/deadlock-brain-core/src/schema.rs:175-196`). |
| `build_learning_notes` | 160 | Modellanalysen aus Build-Learning; Status: 152 `analysis_ready`, 8 `context_ready`. | Schema `learned_build_id`, `hero_name`, `prompt_text`, `result_text`, `insights_json`, `model`, `status` (`rust/crates/deadlock-brain-core/src/schema.rs:63-78`). |
| `player_match_decision_notes` | 4 | Match-/Build-Coaching-Lernnotizen fuer konkrete Player-Matches. | Schema `account_id`, `match_id`, `hero_id`, `hero_name`, `result_text`, `insights_json` (`rust/crates/deadlock-brain-core/src/schema.rs:305-320`). |
| `analysis_notes` | 3 | Persistierte Review-Kontexte oder Fireworks-Ergebnisse; aktuell alle `context_ready`, kein `result_text`. | Schema `query`, `context_kind`, `prompt_text`, `result_text`, `source_references_json`, `context_json` (`rust/crates/deadlock-brain-core/src/schema.rs:44-61`). |
| `patch_impact_notes` | 1 | LLM-Patchtrend pro Entity; nuetzlich, aber bisher praktisch leer. | Schema `entity_name`, `prompt_text`, `result_text`, `insights_json`, `patch_range_*`, `event_count` (`rust/crates/deadlock-brain-core/src/schema.rs:285-303`). |
| `meta_trend_notes` | 0 | Ziel fuer Meta-Trend-Analysen, derzeit keine nutzbaren Daten. | Schema `entity_name`, `trend_direction`, `winrate_delta`, `context_json`, `result_text` (`rust/crates/deadlock-brain-core/src/schema.rs:230-240`). |
| `sheet_hero_rankings` | 83 | Hero-Rollen/Profilwerte: Carry, CC, Engage, Mobility, Waveclear usw.; gut fuer "was kann Hero X?". | Schema `carry`, `crowd_control`, `engage`, `frontline`, `mobility`, `support`, `average_rank`, etc. (`rust/crates/deadlock-brain-core/src/schema.rs:335-352`). |
| `sheet_heroes_stats` | 294 | Tabellenform aktueller Hero-Stats: HP, Move Speed, Ammo, DPS, Scaling. | Schema breite Statspalten (`rust/crates/deadlock-brain-core/src/schema.rs:354-399`). |
| `sheet_raw_heroes` | 303 | Raw-Hero-Werte aus Sheet, teils andere Feldnamen/Quellen. | Schema `hero_id`, `disabled`, `move_speed`, `base_health`, `gun_growth`, `spirit_per_boon`, usw. (`rust/crates/deadlock-brain-core/src/schema.rs:413-431`). |
| `sheet_boons_ap` | 54 | Souls -> Boons/AP Progression; hilfreich fuer Economy-/Timingfragen. | Schema `souls`, `boons`, `ap`, `note` (`rust/crates/deadlock-brain-core/src/schema.rs:322-333`). |
| `sheet_shop_bonuses` | 41 | Shop-Bonus-Schwellen nach Souls fuer Weapon/Spirit/Vitality. | Schema `souls_cost`, `weapon`, `spirit`, `vitality`, `inc_from_prev_pct` (`rust/crates/deadlock-brain-core/src/schema.rs:433-442`). |
| `sheet_items` | 4.456 | Sheet-Item-Mapping Code/Game/Canonical Name. | Schema `item_id`, `code_name`, `game_name`, `canonical_name` (`rust/crates/deadlock-brain-core/src/schema.rs:401-411`). |
| `sheet_tab_rows` | 1.170 | Freiform-Sheet-Tabs; nuetzlich als Rohmaterial fuer Hidden Mechanics/Rankings, aber ohne semantische Normalisierung. | Schema `tab_name`, `gid`, `row_number`, `canonical_name`, `row_json` (`rust/crates/deadlock-brain-core/src/schema.rs:444-455`). |
| `mechanic_notes` | 0 | Ziel fuer Mechanik-Wissensbasis, derzeit leer. | Schema existiert (`rust/crates/deadlock-brain-core/src/schema.rs:219-228`), Retrieval-Vektorpfad inaktiv. |

## 5. RETRIEVAL-PFAD HEUTE

Heute ist Retrieval deterministisch und SQL-/Alias-basiert:

1. `build_entity_context` normalisiert Query, findet `best_match` ueber `entities` + `entity_aliases`, laedt Aliasse, Lineage/Legacy-Namen, Patch-Events, Enrichments und Sheet-Stats (`rust/crates/dbrain-retrieval/src/lib.rs:205-255`).
2. Entity-Matching bewertet exakte Canonical-/Alias-Treffer hoeher als LIKE-Treffer; unter `MIN_ENTITY_MATCH_SCORE=100` gibt es keinen kanonischen Treffer (`rust/crates/dbrain-retrieval/src/lib.rs:664-737`).
3. Patch-Events werden ueber Entity-/Alias-/Lineage-Namen oder Fallback `entity_name LIKE ?` selektiert (`rust/crates/dbrain-retrieval/src/lib.rs:874-966`).
4. Sheet-Stats kommen aus `entity_snapshots` und allen Tabellen, deren Name `sheet` oder `stat` enthaelt; Matching laeuft ueber bevorzugte Namensspalten oder LIKE (`rust/crates/dbrain-retrieval/src/lib.rs:1030-1173`).
5. `build_review_context` verdichtet den Kontext zu `entity_summary`, `current_stat_hints`, `timeline_signals`, `source_references`, `open_questions` und `prompt_de` (`rust/crates/dbrain-retrieval/src/lib.rs:354-399`, `rust/crates/dbrain-retrieval/src/lib.rs:1558-1691`, `rust/crates/dbrain-retrieval/src/lib.rs:1693-1759`, `rust/crates/dbrain-retrieval/src/lib.rs:1844-1849`).
6. Nur `analysis_run_ai` macht daraus einen LLM-Request und speichert eine Antwort (`rust/crates/dbrain-retrieval/src/lib.rs:499-550`).

Nahe an RAG ist bereits: Kontext-Assembler, Prompt, Quellenreferenzen, deduplizierte Entity/Alias-Aufloesung, Patch-Historie, Stat-Hints. Fehlend fuer echtes Q&A:

- Kein allgemeiner Befehl `ask "<Frage>"`, der Frage -> Intent -> Retrieval -> Trust/Gewichtung -> LLM-Antwort -> Quellen automatisch verbindet.
- `analyze_query` ist nur ein grober String-Matcher: findet bekannte Heroes/Items in der Frage, setzt sonst Default `hero_overview`; Intent-Fetch-Mapping ist teilweise vorbereitet, aber nicht mit Retrieval-Pipelines verdrahtet (`rust/crates/dbrain-retrieval/src/lib.rs:583-662`).
- Keine aktive Embedding-/Semantik-Suche; `search_mechanic_notes` ist TODO und `mechanic_notes` ist leer.
- Creator-Claims sind nicht in den normalen Review-/Context-Pfad eingebunden; kein Join/Gewichtung nach Claim-Typ, Status, Verifier-Confidence, Creator/Video/Datum.
- Live-DB hat keine L2-Claim-Spalten; die erwaehnten L2-Verdikte sind nicht persistiert.
- Quellen-/Trust-Modell ist uneinheitlich: `deadlock-data` ist als trusted markiert, aber Retrieval gewichtet nicht systematisch trusted Ground Truth gegen Sheet/Statlocker/Creator-Claims.
- Keine automatische Konfliktloesung zwischen Patch-Historie, aktuellem Snapshot, Creator-Claims und Statlocker.
- Build-Suggestor ist heuristisch, nicht Q&A-Dialog: keine natuerliche Antwort mit Belegen, keine Gegner-/Teamcomp-Retrievaltiefe ausser `vs_heroes`-Parameter im internen API.

## 6. KONKRETE Q&A-LÜCKEN-LISTE

| Nutzerfrage | Daten da | Fehlender Baustein |
|---|---|---|
| "Was hat der letzte Patch an Haze geaendert?" | `patch_events` 12.523, `patch_event_enrichments` 3.109, `timeline` gruppiert Events nach Patch. | `ask`-Befehl, der "letzter Patch" als Zeitfilter erkennt, Timeline selektiert, zusammenfasst und Quellen zitiert. |
| "Welche Items kontern Infernus?" | Item-Payloads in `entity_snapshots`, `sheet_items`, Creator-Claims `claim_type='counterplay'` 43, Build-Heuristik mit situational/counter Items. | Retrieval ueber Claims + Item-Archetypes + Hero-Mechanik; Trust-/Status-Filter; Antwortgenerator mit "belegt vs. wahrscheinlich". |
| "Bester Build fuer Abrams gegen Haze?" | `learned_builds` 76, `build_learning_notes` 160, `build_hero_build_context` kann `vs_heroes` intern beruecksichtigen, Items/Stats/Patches vorhanden. | CLI/API-Path fuer Gegnerparameter; Ranking ueber Build-Notes + Matchup-Claims + Itemsynergien; LLM-Antwort mit Begruendung/Unsicherheit. |
| "Ist Creator-Claim X korrekt?" | `youtube_learning_claims` 1.087 mit `status`, `verifier_confidence`, `verifier_json`; Patch-/Snapshot-Ground-Truth vorhanden. | Claim-by-ID/Claim-search Retrieval, L2-Verifikationspersistenz fehlt, automatischer Vergleich Claim -> trusted Snapshot/Patch/Stats -> Verdict. |
| "Welche Ability skaliert bei Kelvin mit Spirit?" | Deadlock-data Ability/Card Snapshots, Hero-bound abilities, `hero_stat_values` und raw payload keys wie `Upgrades`/Scaling-Felder. | Semantische Normalisierung von Ability-Scaling-Feldern in eigene Tabellen; Query-Parser fuer Hero+Ability+Stat; Antwort mit konkreten Payload-Feldern. |
| "Welche Heroes haben die beste Waveclear?" | `sheet_hero_rankings` 83 mit `wave_clear`, Hero-Entities/Aliasse. | Ranking-Query/Intent fuer Vergleichsfragen, Sortierung/Filter, LLM- oder templated Antwort mit Top-N und Quellen. |
| "Was sind aktuelle Meta-Trends?" | `sheet_hero_rankings`, Statlocker-Snapshots, Patch-Historie; `meta_trend_notes` Tabelle existiert. | Realer Meta-Trend-Assembler; aktueller `enrich meta-trends` nutzt Mock-Shifts und `meta_trend_notes` hat 0 Zeilen. |
| "Warum ist Item X auf Hero Y gut?" | Item-Payloads, Hero-Payloads/Abilities, Build-Suggestor-Scoring, Build-Learning-Notes. | Exponierter Explain-Pfad, der Item+Hero aufloest, Score-Begruendungen/Build-Notes/Claims zusammenfuehrt und Quellen ausgibt. |
| "Welche alten Namen/Reworks betreffen Hero/Ability Z?" | `entity_lineage` 208, `legacy_entities` 18, Patch-Events. | Antwortpfad existiert nur als Liste/Timeline; kein Frage-Intent, keine natuerliche Zusammenfassung mit "heute heisst das ...". |
| "Welche Mechanics gelten fuer Souls vor/nach Minute 10?" | YouTube-Claims haben Mechanikbeispiele; `sheet_boons_ap`/`sheet_shop_bonuses`; `mechanic_notes` leer. | Aktive Mechanik-KB und Suchindex; Claim-Trust-Filter; kein Vector-/Text-Retrieval fuer freie Mechanikfragen. |

## 7. VORHANDENE WIEDERVERWENDBARE BAUSTEINE

- Entity-/Alias-Resolver: `find_best_entity_match`, `load_aliases`, Lineage/Legacy-Lookup (`rust/crates/dbrain-retrieval/src/lib.rs:664-866`).
- Patch-Kontext-Assembler: `build_entity_context`, `build_entity_timeline`, `build_review_context` (`rust/crates/dbrain-retrieval/src/lib.rs:205-399`).
- Prompt- und Quellenreferenz-Geruest fuer LLM-Reviews: `prompt_de`, `source_references`, `compact_context_for_model`, `analysis_run_ai` (`rust/crates/dbrain-retrieval/src/lib.rs:1693-1759`, `rust/crates/dbrain-retrieval/src/lib.rs:1844-1849`, `rust/crates/dbrain-retrieval/src/lib.rs:3214-3231`, `rust/crates/dbrain-retrieval/src/lib.rs:499-550`).
- AiClient mit OpenAI-kompatiblem Fireworks-Modus (`rust/crates/deadlock-brain-core/src/ai.rs:147-192`).
- Trusted `deadlock-data` Ingest als Ground Truth inkl. Entity/Alias/Hero-Stats (`rust/crates/dbrain-sources/src/deadlock_data.rs:93-115`, `rust/crates/dbrain-sources/src/deadlock_data.rs:1225-1278`, `rust/crates/dbrain-sources/src/deadlock_data.rs:1397-1457`).
- Deterministischer Patch-Event-Enricher fuer Stat-/Value-Changes (`rust/crates/dbrain-enrich/src/lib.rs:198-264`, Regex-Patterns `rust/crates/dbrain-enrich/src/lib.rs:60-75`).
- Deterministischer Build-Context/Suggestor mit Hero/Ability/Item/Statlocker/Review-Signalen (`rust/crates/dbrain-learn/src/build_optimizer.rs:82-155`).
- Build-/Player-Learning-Notes als wiederverwendbare Modellanalysen (`build_learning_notes` 160, `player_match_decision_notes` 4).
- YouTube-Claim-Store mit Status und `verifier_json`; nutzbar, sobald Retrieval/Trust-Gewichtung angebunden wird (`rust/crates/deadlock-brain-yt/src/claims.rs:107-180`, Live-DB 1.087 Claims).

## Kurzfazit

Das Brain ist heute stark bei Daten-Ingestion, Normalisierung, Entity-Aufloesung, Patch-/Stat-Kontext und einigen spezialisierten Modellanalysen. Es beantwortet echte Nutzerfragen aber noch nicht als Produktpfad: Es gibt keinen allgemeinen Frage-Endpoint/-CLI-Befehl, keine aktive semantische Suche, keine systematische Trust-/Quellengewichtung und keine Einbindung von Creator-Claims in Review-Kontexte. `review`/`context`/`timeline` assemblieren Daten; nur `analysis run-minimax` erzeugt daraus eine LLM-Antwort, aber nur fuer den Review-Kontext und nicht fuer freie Q&A-Intents.
