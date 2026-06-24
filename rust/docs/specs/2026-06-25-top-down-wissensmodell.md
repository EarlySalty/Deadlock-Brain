# Top-Down-Wissensmodell und Kategorisierungs-Luecken

Datum: 2026-06-25  
DB: `data/deadlock_brain.sqlite3` read-only per Python `sqlite3` URI `mode=ro`  
Scope: Analyse und Designvorschlag. Keine Migration, keine Live-DB-Writes, keine Aenderung an `src/deadlock_brain/`.

## Kurzfazit

Die DB hat bereits den richtigen Kern fuer ein Top-Down-Modell: `entities` ist der kanonische Entity-Katalog, `entity_aliases` der Resolver, `entity_snapshots` die Rohfakten, und Patch-/Stats-/Build-/Claim-Tabellen haengen lose ueber `entity_id`, `entity_name` oder Aliase daran.

Die wichtigste Korrektur zur Ausgangszahl: Die ~1.649 "ohne Kategorie" sind in der aktuellen DB `patch_events.section IS NULL`, nicht "ohne Hero/Item-Zuordnung". Von diesen 1.649 Zeilen sind 1.000 bereits `hero`, 354 `item`, 2 `ability` und 293 `general`. Die eigentlichen Patch-Events ohne Top-Down-Entity sind 417 `entity_type='general'`.

Das Ziel sollte additiv sein: keine flachen Tabellen ersetzen, sondern Views plus wenige Bridge-Tabellen darueberlegen. Das haelt die Pipeline stabil und macht die Wissenskarte pro Hero/Item abrufbar.

## Ist-Struktur und Key-Fluss

### Normalisierungsfluss

1. Quellen landen in `source_documents`.
2. Extrahierte Rohobjekte landen in `entity_snapshots`.
3. `entity_normalizer.py` baut daraus `entities` und `entity_aliases`.
4. `patch_parser.py` liest `entity_snapshots.entity_type='patchnote'` und erzeugt `patch_events`.
5. `patch_event_enricher.py` erzeugt je `patch_events.id` genau ein `patch_event_enrichments`-Objekt. `confidence=0` und `stat_name=NULL` bedeutet "nicht semantisch geparst".
6. `lineage.py` und `legacy_entities.py` extrahieren Rename/Rework/Legacy-Hinweise aus `patch_events`.
7. `sheet_normalizer.py` und `sheet_tabs.py` normalisieren Google-Sheet-Snapshots in Hero-Stats, Rankings, Items und freie Tabs.
8. `build_learning.py` speichert Builds in `learned_builds` und Analysen in `build_learning_notes`.
9. `youtube_learning.py` speichert Videos/Transkripte/Claims; Claims haben keinen FK zu `entities`, sondern nur `entity_type`/`entity_name` plus `verifier_json`.
10. `retrieval.py` und `review_context.py` fuehren Entity, Aliase, Lineage, Patch-Events, Enrichments und Sheet-Stats zur aktuellen Review-Sicht zusammen.

### Aktuelle Kardinalitaeten

| Bereich | Anzahl |
|---|---:|
| Tabellen | 34 |
| `entities` | 833 |
| `entity_aliases` | 3.532 |
| `patch_events` | 3.109 |
| `patch_event_enrichments` | 3.109 |
| `entity_lineage` | 208 |
| `hero_stat_profiles` | 302 |
| `hero_stat_values` | 7.319 |
| `sheet_items` | 4.456 |
| `learned_builds` | 76 |
| `build_learning_notes` | 160 |
| `youtube_learning_claims` | 1.087 |
| `mechanic_notes` | 0 |

Entity-Verteilung:

| `entity_type` | Anzahl | Top-Down-Rolle |
|---|---:|---|
| `hero` | 38 | primaere Hero-Knoten |
| `item` | 155 | primaere Item-Knoten |
| `item_special` | 17 | Item-nahe Spezialobjekte, aktuell separat |
| `ability` | 176 | Child-Knoten unter Hero |
| `hero_internal` | 23 | interne/nicht aktive Heroes |
| `ability_internal` | 105 | interne Abilities |
| `weapon_or_internal` | 319 | Engine-/Weapon-/Internal-Objekte |

## Vollstaendige Tabellenzuordnung ins Top-Down-Modell

Diese Tabelle ist die Ziel-Mapping-Spezifikation. "Mappt nach" meint: wo die bestehende Tabelle im Top-Down-Modell gelesen werden soll. Es ist kein Umbauvorschlag fuer die Live-DB.

| Tabelle | Spalten | Mappt nach |
|---|---|---|
| `source_documents` | `id`, `source`, `external_id`, `title`, `url`, `content_type`, `raw_path`, `content_hash`, `fetched_at`, `metadata_json` | Game -> Source/Audit. Quelle fuer Snapshots, Patchnotes, YouTube-Transkripte. |
| `source_runs` | `id`, `source`, `status`, `started_at`, `finished_at`, `summary_json` | Game -> Ingestion Runs/Audit. |
| `entity_snapshots` | `id`, `source`, `entity_type`, `external_id`, `canonical_name`, `payload_hash`, `payload_json`, `fetched_at`, `source_document_id` | Game -> Rohfakten. Fuer Heroes/Items/Abilities/API/Sheet/Patchnote/YouTube-Snapshots. |
| `entities` | `id`, `entity_type`, `canonical_name`, `primary_external_id`, `source`, `first_snapshot_id`, `metadata_json`, `created_at`, `updated_at` | Kanonische Knowledge-Nodes. `hero` und `item` sind primaer; `ability` ist Hero-Child; interne Typen bleiben Debug/Resolver. |
| `entity_aliases` | `id`, `entity_id`, `alias`, `alias_norm`, `alias_kind`, `source`, `external_id`, `snapshot_id`, `created_at` | Resolver fuer alle eingehenden Namen. Muss fuer Top-Down-Views immer vor direktem String-Vergleich genutzt werden. |
| `entity_lineage` | `id`, `patch_event_id`, `relation_type`, `source_entity_type`, `source_name`, `source_name_norm`, `target_entity_type`, `target_name`, `target_name_norm`, `owner_entity_type`, `owner_name`, `owner_name_norm`, `confidence`, `metadata_json`, `created_at`, `updated_at` | Rename/Rework-Kanten. In Top-Down: Hero/Item/Ability History und Alias-Erweiterung fuer historische Namen. |
| `legacy_entities` | `id`, `legacy_type`, `canonical_name`, `name_norm`, `observed_entity_type`, `first_patch_event_id`, `last_patch_event_id`, `first_seen_at`, `last_seen_at`, `event_count`, `confidence`, `status`, `samples_json`, `created_at`, `updated_at` | Historische oder entfernte Namen, die nicht in `entities` sind. In Top-Down als `legacy_alias`/Review-Queue nutzen, nicht blind als echte Entity. |
| `patch_events` | `id`, `patch_snapshot_id`, `patch_external_id`, `patch_title`, `patch_url`, `source_kind`, `posted_at`, `line_index`, `section`, `entity_type`, `entity_name`, `subject`, `change_type`, `raw_line`, `normalized_line`, `old_value`, `new_value`, `confidence`, `metadata_json`, `event_hash`, `created_at` | Hero/Item/Ability/GameConcept -> Patch-History. `entity_type/entity_name` ist Parser-Sicht, muss ueber Alias/Lineage auf Entity oder Concept aufgeloest werden. |
| `patch_event_enrichments` | `id`, `patch_event_id`, `stat_name`, `old_value`, `new_value`, `unit`, `ability_name`, `secondary_entity_name`, `confidence`, `flags_json`, `created_at`, `updated_at` | Strukturierte Patch-Changes. Unter Hero/Item: Stat-Deltas, Ability-Deltas, sekundaire Entities. `confidence=0` = nicht verstanden. |
| `patch_impact_notes` | `id`, `entity_type`, `entity_name`, `entity_id`, `context_hash`, `prompt_version`, `prompt_text`, `result_text`, `insights_json`, `model`, `status`, `patch_range_start`, `patch_range_end`, `event_count`, `created_at`, `updated_at` | AI-abgeleitete Patch-Auswirkung pro Entity. In Top-Down als abgeleitete Summary, nicht als Quelle. |
| `hero_stat_profiles` | `id`, `snapshot_id`, `entity_id`, `hero_name`, `source`, `external_id`, `payload_hash`, `row_number`, `created_at`, `updated_at` | Hero -> Stats Snapshot/Profile. Primaerer EAV-Einstieg fuer Sheet-Stats. |
| `hero_stat_values` | `id`, `profile_id`, `entity_id`, `hero_name`, `stat_key`, `stat_label`, `numeric_value`, `raw_value`, `created_at`, `updated_at` | Hero -> Stats+Scaling EAV. Fuer `stat_key` wie `hp_gain`, `dmg_gain`, `spirit_gain`, `total_bullet_ratio`, `total_spirit_ratio`, `aggregate_growth`, `dps_growth_increase`, `hp_growth_increase`. |
| `sheet_heroes_stats` | `id`, `snapshot_id`, `entity_id`, `hero_name`, `alt_fire_type`, `hero_labs`, `base_hp`, `base_move_speed`, `base_sprint`, `base_stamina`, `base_regen`, `base_ammo`, `pellets`, `alt_fire_pellets`, `base_bullet_dmg`, `base_fire_rate`, `base_dps`, `max_gun_dps`, `max_gun_damage`, `dpm`, `max_dpm`, `falloff_range_min`, `falloff_range_max`, `hp_gain`, `dmg_gain`, `spirit_gain`, `spirit_bonus`, `spirit_bonus_2`, `spirit_ratio`, `spirit_ratio_2`, `spirit_scaling`, `spirit_scaling_2`, `aggregate_growth_pct`, `dps_growth_pct`, `hp_growth_pct`, `melee_ratio`, `total_bullet_ratio`, `total_spirit_ratio`, `max_level_hp`, `payload_hash`, `created_at`, `updated_at` | Hero -> Stats+Scaling Wide View. Achtung: `spirit_scaling` und `spirit_scaling_2` sind aktuell 0-mal non-null; Scaling steckt praktisch in Ratio/Gain-Spalten und EAV-Rohwerten. |
| `sheet_raw_heroes` | `id`, `snapshot_id`, `entity_id`, `hero_name`, `hero_id`, `disabled`, `move_speed`, `sprint_speed`, `crouch_speed`, `move_accel`, `light_melee_dmg`, `heavy_melee_dmg`, `max_hp`, `base_stamina`, `stam_regen`, `hp_regen`, `base_health`, `gun_growth`, `alt_gun_growth`, `hp_per_boon`, `melee_gain`, `spirit_per_boon`, `payload_hash`, `created_at`, `updated_at` | Hero -> Raw API/Sheet Stats. Disabled/unreleased Heroes bleiben Review-/Hidden-Daten. |
| `sheet_hero_rankings` | `id`, `snapshot_id`, `entity_id`, `hero_name`, `carry`, `crowd_control`, `disengage`, `early`, `engage`, `frontline`, `late`, `mid`, `mid_contest`, `mobility`, `nuke_phys`, `nuke_spirit`, `pick`, `poke`, `support`, `wave_clear`, `sustain_dps_phys`, `sustain_dps_spirit`, `average_rank`, `payload_hash`, `created_at`, `updated_at` | Hero -> Role/Meta/Ranking Profil. |
| `sheet_boons_ap` | `id`, `snapshot_id`, `souls`, `boons`, `ap`, `note`, `created_at`, `updated_at` | Game -> Economy/Progression. Nicht Hero-spezifisch. |
| `sheet_shop_bonuses` | `id`, `snapshot_id`, `souls_cost`, `weapon`, `spirit`, `vitality`, `inc_from_prev_pct`, `created_at`, `updated_at` | Game -> Shop/Economy Scaling. Nicht Hero-spezifisch. |
| `sheet_items` | `id`, `snapshot_id`, `item_id`, `code_name`, `game_name`, `canonical_name`, `created_at`, `updated_at` | Item/Ability -> Sheet Item Facts. Es gibt keinen `entity_id`; Aufloesung laeuft ueber `item_id`, `code_name`, `game_name`, `canonical_name` gegen Aliase. |
| `sheet_tab_rows` | `id`, `snapshot_id`, `tab_name`, `gid`, `row_number`, `canonical_name`, `row_json`, `created_at`, `updated_at` | Freie Sheet-Wissensbereiche: `Hidden Mechanics`, `damage calculator`, `haze`, `Damage Comparison`, `scratchpad`, `hero query`, `ttk`. Muss per Tab-spezifischer Semantik auf Hero/Item/Concept gemappt werden. |
| `learned_builds` | `id`, `source`, `source_build_id`, `hero_id`, `hero_name`, `language`, `source_rank`, `quality_tier`, `quality_score`, `name`, `author_account_id`, `description`, `tags_json`, `details_json`, `item_names_json`, `ability_order_json`, `source_metadata_json`, `imported_at`, `updated_at` | Hero -> Builds. Items/Ability-Order als Listen, aktuell stringbasiert aber in der DB voll aufloesbar. |
| `build_learning_notes` | `id`, `learned_build_id`, `hero_name`, `source`, `context_hash`, `prompt_version`, `prompt_text`, `result_text`, `insights_json`, `model`, `status`, `created_at`, `updated_at` | Hero -> Build-Learning/Summary. Abgeleitete Notizen, nicht kanonische Fakten. |
| `youtube_feed_sources` | `feed_key`, `source_type`, `url`, `handle`, `playlist_id`, `channel_id`, `title`, `enabled`, `metadata_json`, `created_at`, `updated_at` | Game -> YouTube Source Registry. |
| `youtube_videos` | `video_id`, `feed_key`, `channel_id`, `channel_title`, `title`, `url`, `published_at`, `description`, `metadata_json`, `transcript_status`, `learning_status`, `discovered_at`, `updated_at` | Game -> Video Source. |
| `youtube_transcripts` | `video_id`, `language`, `source_kind`, `transcript_text`, `content_hash`, `source_document_id`, `imported_at`, `updated_at` | Game -> Evidence Text. |
| `youtube_learning_claims` | `id`, `video_id`, `claim_hash`, `claim_index`, `entity_type`, `entity_name`, `claim_type`, `claim_text`, `evidence_quote`, `timestamp_seconds`, `model_confidence`, `verifier_confidence`, `status`, `model`, `prompt_version`, `prompt_text`, `model_response_text`, `provider_metadata_json`, `verifier_json`, `created_at`, `updated_at` | Hero/Item/Ability/GameConcept -> Claims. Aktuell kein FK; Top-Down braucht Claim-Entity-Bridge. |
| `analysis_notes` | `id`, `query`, `entity_type`, `entity_name`, `context_kind`, `context_hash`, `prompt_version`, `prompt_text`, `result_text`, `model`, `confidence`, `status`, `source_references_json`, `context_json`, `created_at`, `updated_at` | Entity -> Review/Analysis Cache. Abgeleitet. |
| `player_match_decision_notes` | `id`, `account_id`, `match_id`, `hero_id`, `hero_name`, `context_hash`, `prompt_version`, `prompt_text`, `result_text`, `insights_json`, `model`, `status`, `created_at`, `updated_at` | Hero -> Match/Coaching Notes. Stringbasiert ueber `hero_name`, optional `hero_id`. |
| `mechanic_notes` | `id`, `title`, `content`, `source`, `category`, `rowid`, `created_at` | Mechaniken/Tricks. Aktuell leer; spaeter Hero/Item/Concept-Zuordnung ueber Bridge. |
| `vector_embeddings` | virtual `vec0(embedding float[384])` | Suchindex fuer `mechanic_notes`, kein Domain-Fakt. |
| `vector_embeddings_chunks` | `chunk_id`, `size`, `validity`, `rowids` | sqlite-vec intern, nicht ins Top-Down-Modell mappen. |
| `vector_embeddings_info` | `key`, `value` | sqlite-vec intern/metainfo. |
| `vector_embeddings_rowids` | `rowid`, `id`, `chunk_id`, `chunk_offset` | sqlite-vec intern. |
| `vector_embeddings_vector_chunks00` | `rowid`, `vectors` | sqlite-vec intern. |
| `sqlite_sequence` | interne Autoincrement-Sequenzen | Kein Domain-Modell. |

## Kategorisierungs-Luecken-Report

### 1. Patch-Events ohne `section`

`section IS NULL` ist die Quelle der genannten ~1.649. Das ist eine Abschnitts-/Patchnote-Struktur-Luecke, nicht automatisch eine Entity-Luecke.

| `entity_type` bei `section IS NULL` | Anzahl |
|---|---:|
| `hero` | 1.000 |
| `item` | 354 |
| `general` | 293 |
| `ability` | 2 |
| **Summe** | **1.649** |

Ursache:

- `patch_parser.detect_section()` akzeptiert nur bekannte Headings wie `General`, `Items`, `Heroes`, `Map`, `UI`, `Audio`, `Misc`, `Street Brawl`.
- Viele Forum-Patches sind nur Bullet-Listen ohne solche Headings.
- Das ist fuer Top-Down nicht kritisch, solange `entity_type/entity_name` gesetzt ist. Es ist kritisch bei den 293 `general` ohne Section.

Beispiele mit `section IS NULL`, aber trotzdem sauber zugeordnet:

| Typ | Beispiel |
|---|---|
| Hero | `Calico: Gloom Bombs cooldown increased from 10s to 12s` -> `hero/Calico` |
| Item | `Mystic Shot: Base damage reduced from 55 to 45` -> `item/Mystic Shot` |
| Ability | `Ice Path: Fixed Ice Path being overly sticky for allies` -> `ability/Ice Path` |

### 2. Patch-Events ohne Top-Down-Entity (`entity_type='general'`)

Aktuell: 417 von 3.109 Patch-Events.

| Muster | Anzahl | Warum der Parser nicht weiterkommt | Beispiele |
|---|---:|---|---|
| `global_objective_economy` | 148 | Zeilen beschreiben Game-Systeme statt Hero/Item: Souls, Urn, Trooper, Guardian, Walker, Patron, Rejuv, Shrines. Es fehlt ein `game_concepts`-Knoten. | `Breakable souls rescaled...`; `Trooper resistance near base reduced...`; `Patron 1st death time...` |
| `unresolved_subject` | 145 | `Subject:` vorhanden, aber der Aliasindex kann es nicht eindeutig als Entity mappen, oder der Parser hat alten Gruppen-Kontext weitergetragen. | `Doorman: Call Bell...`; `Curse: Cooldown...`; `The Magnificent Sinclair: Rabbit Hex...` |
| `global_all_hero_or_stat_rule` | 83 | Globale Werte/Regeln ohne Einzel-Owner, z.B. all heroes, global spirit, respawn, movement rule. | `Sprint increased by +0.5 for all heroes`; `Spirit Power scaling globally reduced by -7%`; Jump-Pad-Stun-Regel |
| `other_general` | 19 | Restfaelle: Server/Experimental/Hero-Labs/ambigue Kurzzeilen. | `Server performance improvements`; `Golden Statue Buffs adjustments` |
| `map_movement_structure` | 11 | Map-/Movement-/Struktur-Regeln statt Entity. | Wall Jump, ziplines, rope, camera, spawn area |
| `bugfix_no_owner` | 7 | Bugfix ohne sicheren Besitzer. | `Fixed High-Velocity Mag affecting some abilities` |
| `ui_settings_audio` | 4 | UI/Settings/Audio/Build-Feature. | `Added an option...`; `Changed default shop music...` |

Wichtige konkrete Ursachen:

- Fehlender Kurzalias: `The Doorman` hat Aliase `the doorman` und `hero doorman`, aber nicht `doorman`. Dadurch landen 28 `Doorman`-Subjects unter `general`, obwohl der Hero existiert.
- Historische Namen/Renames: `Backstabber` ist heute `Stalker`; `Spellslinger Headshots` ist `Spirit Rend`; solche Namen sind teilweise nur in `entity_lineage`/`legacy_entities` und nicht als direkte Alias-Aufloesung im Parser.
- Gruppen-Kontext-Leak: Im Patch `12-16-2025 Update` werden freie Objective-/Trooper-Zeilen unter dem vorherigen Subject `Tankbuster` gespeichert, obwohl die Raw-Lines `- Trooper bounty...`, `- Shrines health...`, `- Walker Rocket Barrage...` sind. Das ist kein echter Tankbuster-Patch.
- Sammelzeilen: `Increased bullet damage growth by 10% for the following heroes: Mo & Krill, Apollo` wird als Subject genommen; nachfolgende freie Game-System-Zeilen erben dieses Subject.
- Game-Mode-/Concept-Zeilen: `Street Brawl` ist kein Hero/Item, sondern ein Mode/Concept. Es braucht einen Concept-Knoten.

Top `general`-Subjects:

| Subject | Anzahl | Kommentar |
|---|---:|---|
| `Doorman` | 28 | Alias-Luecke zu `The Doorman` |
| `Tankbuster` | 18 | Kontext-Leak, nicht echte Item-Zuordnung |
| `Increased bullet damage growth by 10% for the following heroes` | 8 | Sammel-/Gruppenzeile |
| `Street Brawl` | 8 | Game-Mode/Concept |
| `The Magnificent Sinclair` | 8 | Historischer Hero-Name/Alias zu `Sinclair` |
| `Silence Glyph` | 5 | Historisches/umbenanntes Item |
| `Spellslinger Headshots` | 5 | Historischer Item-Name zu `Spirit Rend` |

### 3. Patch-Enrichment-Luecken

`patch_event_enrichments` hat 3.109 Zeilen, aber 336 haben `confidence=0` bzw. `flags_json=['unparsed']`.

| Muster | Anzahl |
|---|---:|
| alle Enrichments `confidence=0`/`unparsed` | 336 |
| davon `hero/removed` | 107 |
| davon `hero/changed` | 55 |
| davon `general/changed` | 37 |
| davon `hero/buff` | 27 |
| davon `general/removed` | 22 |
| davon `general/buff` | 18 |
| davon `item/removed` | 18 |
| davon `item/changed` | 13 |

Ursache:

- Enricher erkennt hauptsaechlich `from ... to ...`, `by N`, `now grants`, `no longer grants`, `now has/costs/scales`, `moved`, `upgrades from`, generische `fixed/added/now`.
- Lange funktionale Saetze, Nicht-Standard-Formen und mehrteilige Effekte bleiben unparsed.
- Viele `removed`/`changed`-Hero-Zeilen sind semantisch wichtig, aber keine einfache Stat-Differenz.

Beispiele:

- `Ava no longer replenishes stamina on usage`
- `Can no longer target an ethereal shifted ally with things like Rescue Beam, Viscous Cube, etc`
- `Shoulder Charge T3 changed to -18s Cooldown`
- `Sprint increased by +0.5 for all heroes`
- `Weapon tree investment bonus increased by 6-12% (...)`

### 4. General-Events mit bekannten Namen im Text

`quality.py` findet 73 `general`-Events, die bekannte lokale Namen im Text enthalten. Beispiele:

- `Curse: Cooldown reduced from 50s to 45s` matched nur ueber Legacy/Lineage, nicht als aktives Item.
- `Doorman: Call Bell explosion damage...` enthaelt bekannte Ability `Call Bell`, aber Hero-Subject wurde nicht erkannt.
- `Kelvin Beam: Arctic Beam DPS reduced...` enthaelt Hero `Kelvin` und Ability `Arctic Beam`.
- `Golden Statue Buffs adjustments` ist Legacy/Concept.

Das ist eine gute Review-Queue fuer `patch_event_entities`.

### 5. Stats- und Sheet-Luecken

Hero-Stats sind fuer aktive Heroes grundsaetzlich vorhanden: alle 38 `hero`-Entities haben mindestens eine Stats-Quelle. Die Luecken liegen in ungematchten Sheet-Zeilen und unfertigen/alten Namen.

| Tabelle | Rows | `entity_id IS NULL` | Beispiele |
|---|---:|---:|---|
| `hero_stat_profiles` | 302 | 4 | `Doorman`, `Priest`, 2 Asset-API-Error-Zeilen |
| `hero_stat_values` | 7.319 | 42 | Werte aus den 4 ungematchten Profilen |
| `sheet_heroes_stats` | 294 | 2 | `Priest` |
| `sheet_raw_heroes` | 303 | 73 | disabled/unreleased Heroes wie `Airheart`, `Boho`, `Bomber`, plus Asset-API-Error-Zeilen |
| `sheet_hero_rankings` | 83 | 9 | Tier-Header wie `S (4.0+)`, `A (3.7 - 4.0)`, plus `Doorman` |

Scaling:

- `hero_stat_values` enthaelt relevante Keys wie `hp_gain`, `dmg_gain`, `spirit_gain`, `total_bullet_ratio`, `total_spirit_ratio`, `aggregate_growth`, `dps_growth_increase`, `hp_growth_increase`, `gun_growth`, `alt_gun_growth`, `melee_gain`.
- `sheet_heroes_stats.spirit_scaling` und `spirit_scaling_2` sind aktuell 0-mal non-null. Die Scaling-Information ist eher in Ratio-/Gain-Spalten und Rohwerten vorhanden.

### 6. YouTube-Claim-Luecken

| Kennzahl | Anzahl |
|---|---:|
| `youtube_learning_claims` | 1.087 |
| `accepted` | 397 |
| `needs_review` | 429 |
| `unverified` | 261 |
| explizit `entity_not_resolved` in `verifier_json.reasons` | 287 |
| Claims ohne `entity_name` | 107 |

Haeufigste `entity_not_resolved`-Cluster:

| `entity_type` | `claim_type` | Anzahl |
|---|---|---:|
| `ability` | `mechanic` | 125 |
| `item` | `build` | 48 |
| `ability` | `build` | 16 |
| `hero` | `mechanic` | 14 |
| `item` | `mechanic` | 13 |
| `ability` | `timing` | 11 |
| `ability` | `counterplay` | 8 |

Beispiele:

- `Abrams Ability 3`
- `Point Blank / Close Quarters`
- `Dynamo (Black Hole)`
- `Doorman Bell - Sichtbarkeit`
- `Luggage Cart T3 - Waende`
- `Toxic Bolts`

Ursache:

- Claim-Entity-Aufloesung ist name-only (`entities.canonical_name` oder `entity_aliases.alias_norm`) und kann zusammengesetzte Namen, Slash-Alternativen, "Hero (Ability)"-Formen, deutsch/englische Beschreibungszusatze und Ability-Nummern nicht normalisieren.
- 261 `unverified`-Claims haben `entity_type=NULL`; viele sind Game-Concepts wie Souls, Trooper-Minions, Dschungel-Camps, Tower/Tunnel. Diese sollten nicht als fehlgeschlagene Hero-/Item-Claims gewertet werden, sondern brauchen Concept-Kategorien.

### 7. Legacy- und Lineage-Luecken

`entity_lineage`: 208 Kanten.

| Relation | Anzahl |
|---|---:|
| `rework` | 198 |
| `rename` | 9 |
| `replaced_by` | 1 |

`legacy_entities`: 18, davon 5 `suspect_parser_subject`.

Suspekte Beispiele:

- `Trooper Spawn Rate from`
- `Removed global 40% healing reduction (balance note`
- `Added new T1 Spirit Item`
- `Improved the following Enhanced Items`
- `The following abilities have -15% radius in Street Brawl`

Diese duerfen nicht automatisch als echte Items in Top-Down eingebaut werden.

### 8. Orphans und Builds

FK-/Orphan-Checks sind sauber:

| Check | Anzahl |
|---|---:|
| `PRAGMA foreign_key_check` | 0 |
| Enrichments ohne Patch-Event | 0 |
| Lineage ohne Patch-Event | 0 |
| Hero-Stat-Values ohne Profile | 0 |
| YouTube-Claims ohne Video | 0 |
| Build-Learning-Notes ohne Build | 0 |

Build-Import ist gut aufloesbar:

| Kennzahl | Anzahl |
|---|---:|
| `learned_builds` | 76 |
| Builds mit aufloesbarem Hero | 76 |
| Build-Item-Referenzen | 2.963 |
| unaufloesbare Build-Item-Referenzen | 0 |

Restluecke: Build-Hero-Coverage ist ungleich verteilt. `Celeste` und `Viscous` haben aktuell 0 Builds, `Infernus` hat 10.

## Zielmodell: Game -> Hero/Item -> Wissensbereiche

### Wurzel: Game

Game ist die DB plus globale Konzepte:

- Sources/Audit: `source_documents`, `source_runs`
- Canonical Registry: `entities`, `entity_aliases`, `entity_lineage`, `legacy_entities`
- Global Concepts: Souls, Urn, Troopers, Guardians, Walkers, Patron, Shrines, Rejuv, Map/Movement, UI, Street Brawl, Shop/Economy
- Global Stats/Economy: `sheet_boons_ap`, `sheet_shop_bonuses`

### Hero-Knoten

Ein Hero-Profil sollte enthalten:

- Identity: `entities` row, aliases, metadata, lineage.
- Stats + Scaling: `hero_stat_profiles`, `hero_stat_values`, `sheet_heroes_stats`, `sheet_raw_heroes`, `sheet_hero_rankings`.
- Abilities: `entities.entity_type='ability'` ueber neue Relationship oder abgeleitete Classname/Hero-ID-Regel.
- Tricks/Mechaniken: `youtube_learning_claims` mit `claim_type in ('mechanic','combo','timing')`, `mechanic_notes`, `sheet_tab_rows` aus `Hidden Mechanics`.
- Patch-History: `patch_events` + `patch_event_enrichments`; historische Namen ueber `entity_lineage` und `legacy_entities`.
- Builds: `learned_builds`, `build_learning_notes`.
- Matchups/Counterplay: `youtube_learning_claims` mit `claim_type in ('matchup','counterplay')`, `player_match_decision_notes`.
- Gaps: nicht aufgeloeste Claims, General-Patch-Events mit Hero-/Ability-Namen, fehlende Stats/Builds.

### Item-Knoten

Ein Item-Profil sollte enthalten:

- Identity: `entities` row, aliases, metadata (`item_slot_type`, `item_tier`, `cost` aus `metadata_json`).
- Sheet-Facts: `sheet_items` ueber code/name/id gegen Aliase.
- Patch-History: `patch_events` + `patch_event_enrichments`, inklusive historischen Item-Namen ueber `entity_lineage`/`legacy_entities`.
- Builds: Builds, in denen das Item in `item_names_json` vorkommt.
- Claims/Mechaniken/Counterplay: `youtube_learning_claims` und spaeter `mechanic_notes`.
- Gaps: ungepruefte Legacy-Items, alte Namen, Slash-Claims.

## Additiver Schema-/View-Vorschlag

Nicht destruktiv, in dieser Reihenfolge.

### 1. Game Concepts

Problem: 148+ globale Objective/Economy-Zeilen und 261 alte unverified Claims sind keine Heroes/Items.

Vorschlag:

```sql
CREATE TABLE game_concepts (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  concept_type TEXT NOT NULL,          -- economy|objective|map|movement|ui|mode|system
  canonical_name TEXT NOT NULL,
  name_norm TEXT NOT NULL,
  description TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(concept_type, name_norm)
);

CREATE TABLE game_concept_aliases (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  concept_id INTEGER NOT NULL,
  alias TEXT NOT NULL,
  alias_norm TEXT NOT NULL,
  alias_kind TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  UNIQUE(concept_id, alias_norm, alias_kind),
  FOREIGN KEY(concept_id) REFERENCES game_concepts(id) ON DELETE CASCADE
);
```

Start-Concepts: `Souls`, `Soul Urn`, `Troopers`, `Guardians`, `Walkers`, `Patron`, `Shrines`, `Rejuv`, `Jungle Camps`, `Zipline`, `Jump Pad`, `Street Brawl`, `Hero Labs`, `Shop/Economy`, `UI/Settings`.

### 2. Entity Relationships

Problem: Abilities sind eigene Entities, aber nicht explizit unter Heroes verknuepft.

```sql
CREATE TABLE entity_relationships (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  parent_entity_id INTEGER NOT NULL,
  child_entity_id INTEGER NOT NULL,
  relationship_type TEXT NOT NULL,     -- has_ability|has_weapon|replaced_by|related
  source TEXT NOT NULL,
  confidence REAL NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(parent_entity_id, child_entity_id, relationship_type, source),
  FOREIGN KEY(parent_entity_id) REFERENCES entities(id) ON DELETE CASCADE,
  FOREIGN KEY(child_entity_id) REFERENCES entities(id) ON DELETE CASCADE
);
```

Ableitung initial:

- `ability.metadata_json.hero` oder `entity_snapshots.payload_json.hero`
- `class_name`-Pattern `ability_<hero_token>_*`, `citadel_ability_<hero_token>_*`
- Hero class alias `hero_<hero_token>`

### 3. Patch Event Bridge

Problem: `patch_events` kann nur einen `entity_type/entity_name` tragen. Sammelzeilen, Concept-Zeilen und Body-Matches brauchen mehrere Zuordnungen.

```sql
CREATE TABLE patch_event_entities (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  patch_event_id INTEGER NOT NULL,
  entity_id INTEGER,
  concept_id INTEGER,
  role TEXT NOT NULL,                  -- primary|mentioned|owner|secondary|global
  resolver TEXT NOT NULL,              -- parser_subject|alias_scan|lineage|concept_rule|manual_review
  confidence REAL NOT NULL,
  reason TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  CHECK ((entity_id IS NOT NULL) <> (concept_id IS NOT NULL)),
  FOREIGN KEY(patch_event_id) REFERENCES patch_events(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE CASCADE,
  FOREIGN KEY(concept_id) REFERENCES game_concepts(id) ON DELETE CASCADE
);
```

Damit koennen z.B. `Street Brawl: McGinnis Heavy Barrage...` gleichzeitig `concept=Street Brawl`, `entity=McGinnis`, `ability=Heavy Barrage` tragen.

### 4. Claim Bridge

Problem: YouTube-Claims haben keinen FK und 287 explizit nicht aufgeloeste Entity-Namen.

```sql
CREATE TABLE claim_entities (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  claim_id INTEGER NOT NULL,
  entity_id INTEGER,
  concept_id INTEGER,
  role TEXT NOT NULL,                  -- primary|mentioned|countered_by|build_item|matchup_side
  resolver TEXT NOT NULL,
  confidence REAL NOT NULL,
  status TEXT NOT NULL,                -- resolved|needs_review|rejected
  reason TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  CHECK ((entity_id IS NOT NULL) <> (concept_id IS NOT NULL)),
  FOREIGN KEY(claim_id) REFERENCES youtube_learning_claims(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE CASCADE,
  FOREIGN KEY(concept_id) REFERENCES game_concepts(id) ON DELETE CASCADE
);
```

Resolver-Regeln:

- Split Slash: `Point Blank / Close Quarters` -> zwei Item-Kandidaten.
- Parentheses: `Dynamo (Black Hole)` -> Hero `Dynamo` + Ability/claim text review fuer Ability.
- Ability number: `Abrams Ability 3` -> Hero `Abrams`, Ability-Slot-Resolver aus assets.
- Hyphen descriptors: `Doorman Bell - Sichtbarkeit` -> Hero `The Doorman` + Ability `Call Bell`, Rest als mechanic topic.
- Concept fallback: Souls, Troopers, Jungle Camps, Tower/Tunnel -> `game_concepts`.

### 5. Views fuer Top-Down-Abruf

View-Skizzen:

```sql
CREATE VIEW v_public_entities AS
SELECT id, entity_type, canonical_name, primary_external_id, source, metadata_json
FROM entities
WHERE entity_type IN ('hero','item','item_special','ability');
```

```sql
CREATE VIEW v_hero_stats_current AS
SELECT
  e.id AS hero_entity_id,
  e.canonical_name AS hero_name,
  hsp.id AS profile_id,
  hsp.snapshot_id,
  hsv.stat_key,
  hsv.stat_label,
  hsv.numeric_value,
  hsv.raw_value,
  hsp.source,
  hsp.external_id,
  hsp.row_number,
  hsp.updated_at
FROM entities e
JOIN hero_stat_profiles hsp ON hsp.entity_id=e.id
JOIN hero_stat_values hsv ON hsv.profile_id=hsp.id
WHERE e.entity_type='hero';
```

```sql
CREATE VIEW v_entity_patch_changes AS
SELECT
  pe.id AS patch_event_id,
  pee.id AS enrichment_id,
  pe.entity_type,
  pe.entity_name,
  pe.patch_title,
  pe.patch_url,
  pe.posted_at,
  pe.section,
  pe.change_type,
  pe.normalized_line,
  pe.confidence AS parser_confidence,
  pee.stat_name,
  pee.old_value,
  pee.new_value,
  pee.unit,
  pee.ability_name,
  pee.secondary_entity_name,
  pee.confidence AS enrichment_confidence,
  pee.flags_json
FROM patch_events pe
LEFT JOIN patch_event_enrichments pee ON pee.patch_event_id=pe.id;
```

Nach Bridge-Ergaenzung:

```sql
CREATE VIEW v_topdown_patch_changes AS
SELECT
  pee.entity_id,
  pee.concept_id,
  pee.role,
  pe.id AS patch_event_id,
  pe.patch_title,
  pe.posted_at,
  pe.change_type,
  pe.normalized_line,
  enr.stat_name,
  enr.old_value,
  enr.new_value,
  enr.unit,
  enr.ability_name,
  enr.confidence AS enrichment_confidence
FROM patch_event_entities pee
JOIN patch_events pe ON pe.id=pee.patch_event_id
LEFT JOIN patch_event_enrichments enr ON enr.patch_event_id=pe.id;
```

## Pro-Hero-Profilkarte

Ziel: ein deterministischer Abruf fuer `hero_entity_id` oder Hero-Name.

```text
HeroProfileCard
  identity:
    entity_id, canonical_name, primary_external_id, aliases, metadata
  stats:
    latest_profile, stat_values, raw_hero_stats, ranking
  scaling:
    hp_gain, dmg_gain, spirit_gain, gun_growth, alt_gun_growth,
    melee_gain, total_bullet_ratio, total_spirit_ratio,
    aggregate_growth_pct, dps_growth_pct, hp_growth_pct
  abilities:
    ability entities, aliases, ability-specific patch changes
  patch_history:
    recent events, structured stat deltas, low-confidence/unparsed flags,
    lineage/rename/rework links
  builds:
    learned_builds, build_learning_notes, item_names, ability_order
  claims:
    accepted/needs_review mechanics, timing, build, matchup, counterplay
  matchups_counterplay:
    claim_type matchup/counterplay plus player_match_decision_notes
  gaps:
    missing stats, unresolved claims, general events with known names,
    unparsed enrichments
```

Abruf-Skizze:

```sql
-- Input: :hero_entity_id
SELECT * FROM entities WHERE id=:hero_entity_id AND entity_type='hero';
SELECT * FROM entity_aliases WHERE entity_id=:hero_entity_id;
SELECT * FROM v_hero_stats_current WHERE hero_entity_id=:hero_entity_id;
SELECT * FROM sheet_heroes_stats WHERE entity_id=:hero_entity_id ORDER BY updated_at DESC;
SELECT * FROM sheet_raw_heroes WHERE entity_id=:hero_entity_id ORDER BY updated_at DESC;
SELECT * FROM sheet_hero_rankings WHERE entity_id=:hero_entity_id ORDER BY updated_at DESC;
SELECT * FROM entity_relationships WHERE parent_entity_id=:hero_entity_id;
SELECT * FROM v_topdown_patch_changes WHERE entity_id=:hero_entity_id ORDER BY posted_at DESC;
SELECT * FROM learned_builds WHERE lower(hero_name)=lower(:hero_name) ORDER BY quality_score DESC;
SELECT * FROM claim_entities WHERE entity_id=:hero_entity_id AND status <> 'rejected';
```

Ohne neue Bridge-Tabellen kann die aktuelle Retrieval-Logik schon eine Vorstufe bauen (`review_context.build_review_context`), aber sie bleibt string-/aliasbasiert und verliert Multi-Entity/Concept-Faelle.

## Empfohlene Reihenfolge

1. Nur Report-Skript und Spec reviewen.
2. `game_concepts` + Aliase als erstes additiv einfuehren.
3. `entity_relationships` fuer Hero -> Ability aufbauen und testen.
4. `patch_event_entities` read-only aus vorhandenen Daten befuellen, zuerst mit `status/confidence` und Review-Queue, nicht als harte Wahrheit.
5. `claim_entities` mit Resolver-Regeln fuer Slash/Parentheses/Ability-Nummer/Concepts.
6. Views fuer `HeroProfileCard` und `ItemProfileCard`.
7. Danach erst Parser-Regeln anpassen: Doorman-Kurzalias, Gruppen-Kontext-Reset, Sammelzeilen-Splitting.

## Offene Risiken

- `section IS NULL` darf nicht als fehlende Entity missverstanden werden. Sonst wuerde man 1.356 bereits brauchbare Hero/Item-Patch-Zeilen falsch priorisieren.
- Parser-Rebuild kann Event-IDs/Event-Hashes verschieben, wenn Regeln geaendert werden. Bridges sollten daher `patch_event_id` plus `event_hash`/line metadata speichern oder rebuildbar sein.
- `legacy_entities` enthaelt 5 suspekte Parser-Subjects. Diese duerfen nur mit Review/Confidence genutzt werden.
- YouTube-Claims brauchen eine eigene Concept-Schicht; sonst werden Game-Macro-Claims als "unresolved entity" fehlbewertet.
- `sheet_items` hat keinen `entity_id`. Fuer stabile Item-Views ist entweder eine View ueber Aliase oder ein additiver Link-Cache noetig.

## Reproduzierbarer Report

Das Skript `scripts/read_only_top_down_gap_report.py` erzeugt die zentralen Zahlen aus der Live-DB, ohne zu schreiben:

```bash
python3 scripts/read_only_top_down_gap_report.py --db data/deadlock_brain.sqlite3
```

