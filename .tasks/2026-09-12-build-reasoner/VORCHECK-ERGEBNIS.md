# Vorcheck-Ergebnis: build-reasoner

Datum: 2026-09-12. Lesend, kein Code geändert, kein Branch erzeugt. Vorgehen wie
vorgesehen: zuerst `graphify query`/`graphify explain` im Repo-Root, danach
Nachlesen der Fundstellen mit `sed -n`. DB-Zugang über den bestehenden Weg des
Repos (`scripts/run_sheet_sync_with_infisical.sh:14-22`: Infisical-Config,
`scripts/export_infisical_env.py`, Ergebnis `DEADLOCK_CENTRAL_DSN`). Secrets
wurden nicht ausgegeben. Verbunden wurde mit psql gegen die Datenbank `deadlock`,
die die Schemata `brain`, `tierlist` und `steam` enthält. Der Steam-Bot wurde
lesend unter `/home/nathanael/repos/Deadlock-Steam-Bot` geöffnet.

Wichtigster Befund vorab: Der Referenzfall Warden ist in zwei zentralen
Datenquellen nicht abgedeckt. `brain.patch_events` enthält null Warden-Erwähnungen
nach dem 30.06.2026, obwohl Patches bis 22.08.2026 erfasst sind. Und
`brain.hero_item_stats`/`brain.hero_item_synergies` enthalten nur drei Helden
(Ivy, Graves, Bebop), Warden fehlt komplett. Für Patch-Delta und Backtest mit
Warden als Pflichtfall muss das Paket deshalb eigene Wege eröffnen, Details bei
Punkt 4 und 6.

## 1) Payload-Felder der Helden- und Item-Snapshots

Tabelle `brain.entity_snapshots`, Spalten: id, legacy_sqlite_id, source,
entity_type, external_id, canonical_name, payload_hash, payload jsonb,
fetched_at, source_document_id, imported_at (information_schema). Bestand:
deadlock_assets_api hero 216 Zeilen und item_or_ability 2030, deadlock_data hero
351, item 1575, item_card 2511, ability 2655, dazu u.a. patchnote_varianten,
sheet_rows, forum, youtube, statlocker.

Held Warden, Quelle deadlock_assets_api, entity_type hero: neueste Zeile
id=36470, external_id=25, fetched_at 2026-07-06. Pfad zu den gefragten Feldern:

- Tier/Archetyp: `payload->hero_type` = "brawler", `payload->gun_tag` =
  "Heavy Hitter", `payload->complexity`.
- Basis-Stats: `payload->starting_stats` (z.B. max_health mit value),
  `payload->level_info`, `payload->scaling_stats` (EFireRate und
  ERoundsPerSecond mit scale und scaling_stat ETechPower).
- Fähigkeiten: `payload->items->signature1..4` = class_name der vier
  Signaturen (ability_warden_crowd_control, high_alert, lock_down,
  riot_protocol).
- Kaufboni: `payload->purchase_bonuses` ist ein Objekt mit den Schlüsseln
  weapon, vitality, spirit; je Schlüssel ein Array von
  `{tier, value, value_type}`, z.B. Warden weapon Tier 1 bis 5 =
  4/8/13/18/23 mit value_type MODIFIER_VALUE_WEAPON_DAMAGE_INCREASE. Das ist
  der Kaufbonus je Slot und Tier aus den Rechenregeln.
- Shop-Boni: `payload->cost_bonuses`, Array über Gold-Schwellen 800 bis 28800
  mit `{gold_threshold, bonus, percent_on_graph}`.
- Slot-Info: `payload->item_slot_info`, `payload->shop_stat_display`,
  `payload->stats_display`, `payload->item_draft_bucketing`.
- Hinweis: `purchase_bonuses` steht nur im Helden-Payload, nicht im
  Item-Payload. Die Item-Payloads der vier Beispiele haben diesen Schlüssel
  nicht.

Items, Quelle deadlock_assets_api, entity_type item_or_ability, je neueste
Zeile: Veil Walker id=13183 (tier 3, cost 3200, activation passive,
class_name upgrade_veil_walker, 28 Properties), Mercurial Magnum id=13178
(tier 4, cost 6400, upgrade_ethereal_bullets, 20 Properties), Siphon Bullets
id=13091 (tier 4, cost 6400, upgrade_siphon_bullets, 23 Properties),
Quicksilver Reload id=13177 (tier 2, cost 1600, upgrade_quick_silver,
18 Properties). Pfad zu Stats und Mechanik im Payload:

- `payload->item_tier`, `payload->cost`, `payload->activation`
  (passive/active), `payload->class_name`, `payload->shopable`.
- Stats: `payload->upgrades->property_upgrades` (Zuwachs durch Imbue, z.B.
  Veil Walker HealOnVeil 300, SpiritPower 25) und `payload->properties` je
  Property mit label, value, postfix, css_class, tooltip_section
  (passive/innate), `scale_function` (stat_scale, subclass_name,
  specific_stat_scale_type) und `provided_property_type`. Cooldown steht als
  Property AbilityCooldown mit value und postfix "s" (Veil Walker 15.0).
- Bedingungstext (Bedingung, Wann-wirkt): die Assets-Payload hat keinen
  Freitext. Der steht im Snapshot-Typ `deadlock_data.item_card`:
  Veil Walker Description "Walking through a cosmic veil grants you Stealth,
  Heal and increased Move Speed", Info2.Type Passive mit DescKey
  #upgrade_veil_walker_desc, Info2.Cooldown 15.0, Other mit
  RevealOnDamageDuration 0.5. Item-Card-Pfads: `payload->Tier`, `->Cost`,
  `->Slot` (Armor/Tech/Weapon-Sprache des Spiels), `->Info1` und `->Info2`
  (Type Innate/Passive, Main/Alt, DescKey, Cooldown), `->Other`,
  `->IsImbue`, `->Upgrades` (Imbue-Zuwachs), `->Components` (Baum),
  `->Description` (HTML-Text), `->ShopFilters`.
- Imbue-Items: Mercurial Magnum IsImbue=true und Quicksilver Reload
  IsImbue=true, beide Desc "Your imbued ability charges up over time with
  BonusSpiritDamage, BonusFireRate, and reloads bullets..." (Proc-Cooldown
  und Tickrate stecken in den Properties des Assets-Payload). Siphon Bullets
  IsImbue=false, Info2DescKey #upgrade_siphon_bullets_desc_passive2,
  Info2.Cooldown 1.2, Main HealthStealPctHero 2.5, Description leer.
- Relationale Kopie: `brain.item_catalog` (item_id, name, slot_type, tier,
  defense_kind, damage_axis, properties jsonb, updated_at). Einträge:
  Mercurial Magnum item_id 3919289022 spirit t4, Quicksilver Reload
  84321454 spirit t2, Siphon Bullets 1282141666 vitality t4, Veil Walker
  865958998 vitality t3. Die item_id entspricht der GC-ability_id und passt
  damit direkt zu BuildSpecMod.ability_id. item_catalog hat kein
  Kostenfeld, Kosten stehen nur in den Snapshots (assets cost, item_card
  Cost). Slot-Sprache unterscheidet sich: item_catalog spirit/vitality/
  weapon, item_card Tech/Armor/Weapon.

Lesende Wiederverwendung im Code: `load_entity_payload`
rust/crates/dbrain-learn/src/build_optimizer.rs:1572 (holt den neuesten
Assets-Payload je canonical_name), `load_hero_abilities` build_optimizer.rs:1125
(findet Ability-Payloads per LIKE auf class_name im payload-Text),
`load_wiki_summary` build_optimizer.rs:1606.

Risiko: Prod-DB, nur lesend anfassen. Der Warden-Assets-Payload ist vom
06.07.2026, die item_cards vom 20.08.2026, die Patchnoten-Ingestion lief zuletzt
am 22.08.2026. Für den Patch-Bezug muss der Vorcheck der Snapshot-Zeitstempel
in jede Rechnung einbauen.

## 2) Helden-Kontext in build_optimizer.rs

Rust/crates/dbrain-learn/src/build_optimizer.rs, 3161 Zeilen. Die Zeilenangabe
82-155 aus der Bestandsaufnahme trägt heute Konstanten und Structs
(z.B. SPIRIT_DOMINANT_WEAPON_CORE_MALUS Zeile 27, SITUATIONAL_ITEMS Zeile 30,
WEAPON_AFFINITY_ITEMS mit Quicksilver Reload Zeile 72). Die eigentlichen
Funktionen:

Deterministisch, rein auf übergebenen Payloads (wiederverwendbar für den
Helden-Modell-Kern):

- `infer_hero_needs(hero_payload: &Value, abilities: &[Value]) -> Value`,
  build_optimizer.rs:1185. Baut needs, ability_role_tags, scaling_stats,
  priority_scaling_stats, damage_profile, lane_priorities,
  build_implications, decision_framework.
- `infer_ability_role_tags(text: &str, props: &Value) -> Vec<String>`,
  build_optimizer.rs:1246.
- `key_ability_properties(props: &Value) -> Value`, build_optimizer.rs:1296.
- `ability_scaling_stats(props: &Value) -> BTreeSet<String>`,
  build_optimizer.rs:1328.
- `ability_damage_profile(props: &Value) -> Value`, build_optimizer.rs:1339
  (zählt damage, spirit_scaling, weapon_stat, control, survival Quellen).
- `hero_damage_profile(abilities, ability_tags, role, gun_tag) -> Value`,
  build_optimizer.rs:1385. Liefert `damage_plan` mit weapon/spirit/hybrid/
  utility über Schwellschwellen.
- `priority_scaling_stats(needs, damage_profile) -> Vec<String>`,
  build_optimizer.rs:1424.
- `gameplan_summary` build_optimizer.rs:1479, `build_implications`
  build_optimizer.rs:1506, `build_plan` build_optimizer.rs:1538,
  `lane_priorities` build_optimizer.rs:1497,
  `hero_decision_framework` build_optimizer.rs:1562.
- `economy_summary(hero_payload)` build_optimizer.rs:903 und
  `hero_summary(...)` build_optimizer.rs:944.
- `classify_item_archetypes(item: &Value) -> Vec<String>`,
  build_optimizer.rs:997.
- Scoring deterministisch auf Kontextbasis: `score_item` Zeile 358,
  `score_properties` Zeile 557, `property_hint_score` Zeile 593,
  `hero_need_bonus` Zeile 664, `score_archetypes` Zeile 734,
  `patch_synergy_bonus` Zeile 825, `statlocker_wpa_bonus` Zeile 854.

DB-gebunden (async, nicht rein deterministisch):

- Einstieg: `build_suggest(pool, BuildSuggestOptions)` build_optimizer.rs:189
  und `build_hero_build_context(pool, query, vs_heroes, limit_events)`
  build_optimizer.rs:199. Bricht mit Fehler ab, wenn die Entity kein Held ist
  (build_optimizer.rs:206-213).
- Loader: `load_public_items` 1084, `load_hero_abilities` 1125,
  `load_entity_payload` 1572, `load_wiki_summary` 1606,
  `load_statlocker_wpa_signals` 1650, `load_build_learning_signals` 1699,
  `load_learned_build_item_profile` 1739, `build_review_context` 2222,
  `find_best_entity_match` 2273, `load_aliases` 2377, `load_patch_events`
  2403, `load_enrichments` 2468.

Befund: Der ganze Zahlenpfad ist heute schon KI-frei und deterministisch pro
DB-Stand. Ein neuer Reasoner kann genau diese Trennung übernehmen: Loader plus
reine Rechenfunktionen. "Deterministisch" heißt hier deterministisch je
Snapshot-Stand, nicht zeitlich unveränderlich.

Risiko: Viele harte Schwellen und Item-Listen als Konstanten (Zeilen 18 bis
131). Wer die Rechenregeln aus dem Skill überführt, muss entscheiden, was von
diesen Konstanten ersetzt wird.

## 3) dbrain-builds und der Weg zum Steam-Bot

Strukturen in rust/crates/dbrain-builds/src/spec.rs:

- `BuildSpecPayload` spec.rs:98: hero_id i64, name, description, language i64,
  mod_categories Vec<BuildSpecCategory>, ability_order
  Option<Vec<AbilityOrderEntry>>.
- `BuildSpecCategory` spec.rs:109: name, optional, mods Vec<BuildSpecMod>.
- `BuildSpecMod` spec.rs:116: ability_id i64, annotation String. (Kein
  imbue_target_ability_id, kein sell_priority, kein required_flex_slots.)
- `AbilityOrderEntry` spec.rs:122: ability_id, currency_type, delta.
- `AssemblyResult` spec.rs:129: payload plus warnings Vec<String>.
- `assemble_payload(hero_id, llm: LlmBuildSpec, name_to_id:
  &BTreeMap<String,i64>, ability_order: Option<&[AbilityOrderEntry]>) ->
  AssemblyResult`, spec.rs:303. Unbekannte Item-Namen gehen in warnings und
  werden verworfen.
- Vorstufen: `load_corpus_build_by_hero_id` spec.rs:134,
  `build_candidate_set` spec.rs:157, `build_spec_user_prompt` spec.rs:241,
  `parse_llm_spec_text` spec.rs:268, `ability_order_from_corpus` spec.rs:285.

Sync in rust/crates/dbrain-builds/src/sync.rs:

- `sync_build_data(pool, BuildDataSyncOptions) -> BuildDataSyncSummary`,
  sync.rs:70 (Item- und Hero-Katalog von der API, dann je Held).
- `sync_one_hero(pool, api, hero_id, options) -> HeroBuildDataSyncSummary`,
  sync.rs:95. Ruft build_item_stats, item_stats, hero_stats, je Lift-Kandidat
  hero_stats_with_item, ability_order_stats, item_permutation_stats und
  schreibt via upsert_hero_item_stats, upsert_ability_order, upsert_synergies.
  Ratebremsung über analytics_pause sync.rs:612.
- `upsert_hero_item_stats(pool, hero_id, prevalence, item_stats, catalog_ids,
  lift_map) -> usize`, sync.rs:451. Schreibt brain.hero_item_stats mit
  bracket BRACKET_BADGE_80 ("badge80") und patch_tag PATCH_TAG_CURRENT
  ("current"), Konfliktlösung auf (hero_id, item_id, bracket, patch_tag).
- Heutiges Ranking: `composite_score(row, max_builds)` in
  rust/crates/dbrain-builds/src/engine.rs:332, Prävalenz 50 Prozent,
  Winrate 30 Prozent, Lift 20 Prozent, drei starre Pfade. Das ist der Punkt,
  an dem der Reasoner ansetzt.

Weg zum Steam-Bot, Stand heute:

1. Im Brain erzeugt `deadlock-brain build-spec` das Payload-JSON:
   rust/crates/deadlock-brain/src/main.rs:1313 `run_build_spec` lädt
   build_context und Corpus, ruft das LLM, assembled und gibt mit
   `print_json(&assembled.payload)` (main.rs:1377) nur nach stdout aus. Das
   Brain reiht nichts in den Bot ein.
2. Im Steam-Bot ist die Warteschlange die Tabelle `steam.steam_tasks` (type,
   payload, status 'PENDING'), Insert über
   `insert_publish_task_with_executor` in
   rust/crates/steam-persistence/src/builds.rs:478, SQL builds.rs:598.
   `publish_task_payload` builds.rs:499 baut
   {origin_hero_build_id, target_language, target_name, target_description,
   minimal, update}.
3. Handler: `BUILD_PUBLISH` registriert in
   rust/crates/steam-core/src/task/mod.rs:123, Handler
   rust/crates/steam-core/src/task/handlers/builds/publish.rs:50. Dieser Pfad
   ist der Klon-Pfad: Er braucht einen Eintrag in tierlist.hero_build_sources
   (publish.rs:80 ff. get_hero_build_source) und klont einen fremden Build.
   Für eigene Builds ist `BUILD_PUBLISH_ORIGINAL` da, registriert in
   task/mod.rs:125, Handler rust/crates/steam-core/src/task/handlers/builds/
   publish_original.rs. Args-Struktur publish_original.rs:19: hero_id,
   hero_build_id, version, name, description, language, mod_categories und
   ability_order; Mods erlauben ability_id, annotation, required_flex_slots,
   imbue_target_ability_id (publish_original.rs:54) und sell_priority
   (publish_original.rs:57). AbilityOrder als {ability_id, currency_type,
   delta} publish_original.rs:62. Der Bot hängt fixe Community-Zeilen an die
   Beschreibung (publish_original.rs:68).
4. Befund: Für BUILD_PUBLISH_ORIGINAL existiert heute nur der Handler, aber
   kein Produzent. Weder ein Cog noch Code im Bot noch im Brain fügt solche
   Tasks ein (Suche nach BUILD_PUBLISH_ORIGINAL im Bot-Repo trifft nur
   Registrierung und Handler). Das Publish-Paket muss also den Enqueue in
   steam.steam_tasks bauen oder den manuellen SQL-Weg dokumentieren, und das
   Payload-Schema um imbue_target_ability_id und sell_priority erweitern,
   die der Bot bereits versteht, BuildSpecMod aber noch nicht führt.

Risiko: steam.steam_tasks ist die Live-Queue des Bots; Test-Tasks müssen
sofort erkennbar und löschbar sein. tierlist.hero_build_sources ist
Voraussetzung des Klon-Pfads und für eigene Builds der falsche Weg.

## 4) brain.patch_events und patch_event_enrichments

Spalten patch_events: id, legacy_sqlite_id, patch_snapshot_id,
patch_external_id, patch_title, patch_url, source_kind, posted_at, line_index,
section, entity_type, entity_name, subject, change_type, raw_line,
normalized_line, old_value, new_value, confidence, metadata jsonb, event_hash,
created_at, imported_at.

- Held- oder Item-Bezug: ausschließlich über entity_type ("hero"/"item"/...)
  und entity_name als Text. Es gibt kein entity_id im Event. Alias-Auflösung
  läuft über `brain.entities` (id, entity_type, canonical_name,
  primary_external_id) und `brain.entity_aliases` (entity_id, alias,
  alias_norm, alias_kind, source). Genau so macht es der bestehende Lese-Pfad
  `load_patch_events` build_optimizer.rs:2403: Namensmenge aus canonical_name
  plus Alias-Kinds canonical/snapshot_name/class_name_short, dann
  `WHERE lower(entity_name) = ANY($1) ORDER BY patch_snapshot_id DESC,
  line_index LIMIT 500`. Item-Bezüge innerhalb einer Zeile stehen zusätzlich
  in den Enrichments.
- Patch-Stand: patch_title, patch_external_id, patch_url, posted_at,
  source_kind, patch_snapshot_id (Fremdschlüssel Richtung
  brain.source_documents). Einen Patch-Tag oder eine Versionsnummer gibt es
  in patch_events nicht. Die Item-Stat-Tabellen nutzen stattdessen den
  Pseudo-Tag 'current' ohne Datum, siehe Punkt 6.

patch_event_enrichments Spalten: id, patch_event_id, stat_name, old_value,
new_value, unit, ability_name, secondary_entity_name, confidence, flags
jsonb, created_at, updated_at. Item-Bezug über secondary_entity_name bzw.
ability_name, kein entity_id.

Die letzten drei Patches mit Warden-Events, mit Zeilen:

1. "Minor Update - 06-30-2026", posted_at 2026-06-30, 6 Events. Zeilen
   u.a.: entity_type hero, subject Willpower, change_type buff, "Willpower
   T3 increased from +2.5 spirit power scaling to +2.7"; change_type nerf,
   "Willpower T2 increased from -22s Cooldown to -24s"; nerf "Bullet damage
   per boon reduced from 0.34 to 0.28".
2. "06-30-2026 Update", posted_at 2026-06-30, 3 Events, gleiche Zeilen der
   großen Fassung.
3. "Minor Update - 06-11-2026", posted_at 2026-06-12, 36 Events.

Risiko, groß und für den Auftrag zentral: Nach dem 30.06.2026 gibt es null
Warden-Treffer in patch_events, obwohl die Ingestion bis zum 22.08.2026 lief
(Patches 07-28 mit 109 Events, 08-12 mit 99, 08-22 mit 12; dort sind Helden
und Items sauber als entity_name erfasst, zum Beispiel Doorman, Haze,
Spirit Burn). Auch die Suche nach "nightshift" in Titel und Zeilen trifft
nichts. Der Nightshift-Patch des Referenzfalls ist in dieser Tabelle für
Warden also nicht vorhanden, entweder weil er nach dem 22.08. liegt und die
Ingestion stillsteht oder weil die Warden-Änderungen unter anderen Entity-
Namen laufen. Vor dem Patch-Delta-Paket muss geklärt werden, woher das Delta
für Warden kommen soll (Ingestion nachziehen, zweites Lesen der
patchnote_snapshots, oder Delta aus deadlock_data changelogs
(changelogs_hotfixes, je 9 Zeilen) ableiten). Zusätzlich ist der Pseudo-Bezug
patch_tag 'current' ohne Datum für "Backtest je Patch-Stand" unbrauchbar.

## 5) tierlist.hero_build_sources, watched_build_authors, meta_builds, meta_items

Alle vier in der Datenbank deadlock, Schema tierlist (Steam-Bot-DB).

- `tierlist.hero_build_sources`, 107 Zeilen. Spalten: hero_build_id,
  origin_build_id, author_account_id, hero_id, language, version bigint,
  name, description, tags jsonb, details jsonb, published_at,
  last_updated_at, fetched_at, last_seen_at. details enthält
  `modCategories` als Array von {name, optional, mods: [{abilityId,
  annotation, ...}]} und `abilityOrder` als {currencyChanges: [{abilityId,
  currencyType, delta, annotation}]}. Item-Reihenfolge innerhalb einer
  Kategorie ist also erhalten, Kategorien auch. Die Kategorie-Namen sind
  überwiegend GC-Loc-Keys: #Citadel_HeroBuilds_EarlyGame (35),
  MidGame (25), LateGame (19), Early (19), Late (13), Mid (12), Lane (6),
  dazu Freitext wie "Core" (6), "pick 1" (5), "Situational" (5), "Early
  Core" (5). "Kern" und "Situational" sind also nur bei Autoren lesbar, die
  sie so benennen, sonst steckt die Phasensprache in den Loc-Keys
  EarlyGame/MidGame/LateGame/Lane. Patch-Bezug je Build: nur die
  Spiel-Versionsnummer `version` und die Zeitstempel published_at,
  last_updated_at, last_seen_at. Ein Mapping version zu Patch-Datum oder
  Patch-Tag existiert nicht. last_seen_at geht bis 2026-09-12, die Quelle
  lebt also. Warden (hero_id 25) hat drei Builds: 357243 "twitch.tv/oses7"
  ver126, 419580 "Metro Hypercarry Warden" ver156, 323228 "JONJON" ver1.
  "Lightbringer" taucht in vier Namen/Beschreibungen auf, aber nicht als
  beobachteter Autor und nicht als Warden-Build.
- `tierlist.watched_build_authors`, 10 Zeilen. Spalten: author_account_id,
  notes, is_active, priority, last_checked_at, last_checked_status,
  last_checked_message, created_at. Lightbringer ist nicht dabei. Beobachtet
  werden: Sanya Sniper 906011648, Cosmetical 29715818, ABL 888853854, Piggy
  1144439143, Deathy 87624911, Heresy 477467839, JonJon69 244109796,
  Amerikanec 298155885, AverageJonas 74963221, sowie 91484677 ("Initial
  author provided by user"), alle is_active true.
- `tierlist.meta_builds`, 5 Zeilen, Testdaten: ids b1 bis b5, hero_id h21
  bis h25, Autor "Deathy", je 2 Items und 10 ability_order-Einträge,
  created_at 2026-04-07. Spalten: id text, hero_id text, name, author_id,
  author_name, description, ability_order jsonb, items jsonb, upvotes,
  downvotes, status, created_at. Kein Patch-Bezug.
- `tierlist.meta_items`, 0 Zeilen. Spalten: id, name, type, stats jsonb,
  image_url.
- Daneben existieren im Schema noch meta_heroes, meta_tier_lists,
  meta_tier_history, meta_votes, hero_build_clones,
  tierlist_hero_meta u.a. (Tabellenliste information_schema).

Risiko: Für den Backtest ist hero_build_sources die einzige echte Quelle mit
Item-Reihenfolge, Kategorien und Annotationen. meta_builds und meta_items
liefern nichts. Ohne version-zu-Patch-Mapping ist "je Patch-Stand" nur über
last_seen_at/published_at näherbar. Der Lightbringer-Warden-Referenzfall
liegt nur als Screenshot vor, nicht als Datenzeile.

## 6) brain.hero_item_stats und brain.hero_item_synergies

- `brain.hero_item_stats`: hero_id, item_id, bracket, prevalence_builds,
  wins, losses, matches, players, avg_buy_time_relative, lift_pp, patch_tag,
  updated_at. 500 Zeilen, bracket ausschließlich 'badge80', patch_tag
  ausschließlich 'current', jüngster updated_at 2026-07-15
  15:54:19. Nur drei Helden sind abgedeckt: Ivy 173 Zeilen (15.07.), Graves
  170 (08.07.), Bebop 157 (08.07.). Warden: null Zeilen. Stichprobe Ivy:
  Greater Expansion prev=1167, wr 56,2 Prozent, lift_pp 2,9; Transcendent
  Cooldown prev=1125, wr 60,5, lift 7,2.
- `brain.hero_item_synergies`: hero_id, item_id, with_item_id, wins, losses,
  matches, patch_tag, updated_at. 7452 Zeilen, jüngster updated_at
  2026-07-15, ebenfalls nur drei Helden, Warden ohne Zeilen.

Risiko: Der item-spezifische Teil des Reasoners hat für Warden keine
Winrate-, Verbreitungs- oder Synergie-Basis. `sync_one_hero` (Punkt 3) muss
für Warden gegen die Analytics-API laufen (Auth über User-Agent,
Ratebremsung eingebaut). patch_tag 'current' ohne Datum macht
Patch-Schnitte unmöglich; das Paket sollte ein echtes Patch-Feld
nachziehen oder die updated_at-Historie nutzen.

## 7) Fireworks-Client in deadlock-brain-core/src/ai.rs

- Konfiguration: `AiConfig` ai.rs:10 mit api_key (privat), base_url, model,
  timeout_seconds, max_completion_tokens, temperature, top_p,
  use_token_plan; `AiConfig::from_settings` ai.rs:41. Konfigurationsquellen
  in rust/crates/deadlock-brain-core/src/config.rs (ai_model, ai_base_url,
  ai_api_key, ai_use_token_plan config.rs:33).
- Aufruf-Signaturen: `AiClient::chat(&self, request: &ChatCompletionRequest)
  -> Result<Value>` ai.rs:141 und `AiClient::chat_value(&self,
  request_payload: &Value) -> Result<Value>` ai.rs:148; intern
  `call_openai_compatible` ai.rs:151, POST auf {base_url}/chat/completions
  mit Bearer-Token, blocking reqwest. `ChatCompletionRequest` ai.rs:93 mit
  model, messages, max_tokens (seriell "max_tokens"), temperature, top_p,
  stream=false. Blocking-Client, im Binary teils auf Blocking-Thread
  ausgelagert (main.rs:1057 ff.).
- Denken: Es gibt keinen Request-Parameter, der Reasoning abschaltet. Heute
  wird nur die Antwort gewaschen: `strip_thinking` ai.rs:239 entfernt
  `<think>`-Blöcke, `extract_ai_text` ai.rs:186 liest choices[0].message.
  content bzw. content-Arrays (type text, überspringt type thinking). Das
  Feld use_token_plan wird durchgereicht, taucht aber im Request nicht auf.
- JSON-Ausgabe erzwingen: Nein. Es gibt kein response_format oder
  json_schema im Request. JSON wird per Prompt erbeten und hinterher
  geparst, zum Beispiel parse_llm_spec_text spec.rs:268 für build-spec, oder
  im enrich-Crate. Für die Agentenrollen des Reasoners heißt das: Entweder
  beim Prompt-und-Parser-Weg bleiben oder ChatCompletionRequest um ein
  response_format-Feld erweitern (Punkt im Paket KI-Schnittstelle, sqlx
  nicht betroffen).
- Modell: Nur über Settings, kein Modellname im Code außer einem Test
  (ai.rs:305, deepseek-v4-flash als Beispielwert). Passt zum Auftrag.

Risiko: Fireworks-Key kommt aus Settings bzw. Infisical (run_build_spec
prüft api_key_present main.rs:1330). Ohne Key müssen alle Reason-Pfade ohne
KI laufen können, das Fertig-Kriterium verlangt genau das.

## 8) CLI-Struktur in deadlock-brain/src/main.rs

- clap mit derive: `struct Cli` main.rs:29-34 (#[derive(Parser)] mit
  #[command(subcommand)] command: Commands). `enum Commands` main.rs:36-133,
  jede Variante mit about-Text auf Deutsch. Top-Level-Varianten ohne
  Unterbefehl sind Unit- oder Args-Varianten (Status, Context(EntityContextArgs),
  Build(BuildArgs), BuildSpec(BuildSpecArgs)), Gruppen mit Unterbefehlen
  folgen dem Muster `Wiki { #[command(subcommand)] target: WikiCommands }`
  (main.rs:82-86). Eigene Subcommand-Enums: WikiCommands main.rs:212,
  LearnCommands main.rs:294, PlayerCommands main.rs:381, AnalysisCommands
  main.rs:549, PullCommands main.rs:615, NormalizeCommands main.rs:814,
  ParseCommands main.rs:862, EnrichCommands main.rs:896, dazu PgCommands und
  InsightCommands.
- Dispatch: `async fn run(cli: Cli)` main.rs:1045. Sonderbehandlung für Pg
  und Insights (spawn_blocking, main.rs:1049-1074), danach PgPool
  (DEADLOCK_CENTRAL_DSN) ab main.rs:1087 und das große match über Commands,
  das je Variante die run_-Funktionen ruft (z.B. Commands::BuildSpec =>
  run_build_spec main.rs:1204, run_build_spec main.rs:1313; run_learn
  main.rs:1400).
- Ein neues Top-Level `reason` mit Unterbefehlen (build, patch-impact,
  backtest) kommt also als Viertes Muster dazu: enum-Variante
  `Reason { #[command(subcommand)] target: ReasonCommands }` im Commands-
  Enum (nach BuildSpec/Item, vor Learn), ein neues `enum ReasonCommands`
  mit Args-Structs, ein `run_reason`-Dispatcher und match-Arme. Betroffene
  Stellen in main.rs: etwa 5 bis 8 (Enum-Variante, ReasonCommands, Args,
  Dispatch-Arm, run-Funktion, eventuell Pretty-Printer). Kein sqlx-Offline-
  Einfluss, sofern die Reason-Logik in einem eigenen Modul/Crate lebt.

## 9) Tests: Mechanik und Baseline

Mechanik:

- Lauf: `cargo test --workspace` in rust/, Toolchain 1.97.1 über
  ~/.cargo/bin/cargo (/usr/bin/cargo ist zu alt, wie im Auftrag vermerkt).
- Compile-geprüfte sqlx-Makros laufen offline gegen den Cache rust/.sqlx/
  (SQLX_OFFLINE-Build ohne DB grün), dokumentiert in docs/sqlx-offline-
  cache.md:1-8. Neue oder geänderte query!-Makros erfordern
  `cargo sqlx prepare` gegen eine Wegwerf-Postgres, sonst schlägt der
  Offline-Build mit "no cached data" fehl.
- DB-Tests sind markiert mit `#[ignore = "benoetigt Scratch-Postgres via
  DEADLOCK_CENTRAL_DSN"]` und steigen über test_pool()-Helper ein, die ohne
  DSN sauber überspringen: rust/crates/dbrain-builds/src/util.rs:86,
  rust/crates/dbrain-learn/src/lib.rs:80, rust/crates/dbrain-normalize/src/
  tests.rs:24, dbrain-retrieval/src/lib.rs:6356, dbrain-enrich/src/lib.rs:
  1702, dbrain-sources/src/store.rs:571, deadlock-brain-yt/src/testutil.rs:13.
  Schreib-Lese-Roundtrips nutzen synthetische Hoch-IDs und räumen auf
  (Beispiel sync.rs-Tests, TEST_HERO_ID 990018). Live-DB ist für Tests
  tabu, die Doku sieht Wegwerf-Docker-Postgres vor.

Baseline vom 12.09.2026, Lauf ohne DSN (Kompilation gegrün, Laufzeit unter
einer Minute): 181 bestanden, 0 fehlgeschlagen, 37 ignoriert, Doc-Tests 0.
Je Crate bestanden/ignoriert: dbrain_builds 7/5, dbrain_enrich 3/3,
dbrain_learn 22/5, dbrain_normalize 13/7, dbrain_retrieval 21/12,
dbrain_sources 55/5, deadlock_brain (main.rs) 38/0, deadlock_brain_core
12/0, deadlock_brain_yt 10/5. Heute ist also nichts rot; die 37 ignorierten
sind die DB-Tests, die erst mit Scratch-DSN laufen.

Risiko: Die DB-abhängigen Tests prüfen genau die Schreibpfade, die das
Publish-Paket erweitert. Ohne Scratch-Postgres im Paket bleibt die Prüfung
dieser Pfade also nur Compile-Ebene.

## Zusammenfassung: Wiederverwendbares und Fehlendes

Wiederverwendbar, Position genau benannt:

- Helden-Zahlenpfad: build_optimizer.rs, deterministische Funktionen Punkt 2,
  plus Loader und Kontext-Orchestrierung build_hero_build_context
  build_optimizer.rs:199.
- Item-/Hero-Grunddaten: brain.entity_snapshots (Hero-Payload mit
  purchase_bonuses und cost_bonuses, item_card mit Bedingungstext, Imbue und
  Cooldown), brain.item_catalog mit GC-kompatiblen item_ids.
- Publish-Payload und Assemblierung: spec.rs:98 und spec.rs:303, Publish-Gegenseite
  im Bot unterstützt bereits imbue und sell_priority.
- Steam-Bot-Weg: steam.steam_tasks plus Handler BUILD_PUBLISH_ORIGINAL
  publish_original.rs:19, Registrierung task/mod.rs:125.
- KI-Baustein: AiClient.chat/chat_value ai.rs:141/148, Antwortwaschung
  strip_thinking ai.rs:239.
- CLI-Gerüst: Commands-Enum main.rs:36, Dispatch main.rs:1045.
- Patch-Timeline-Lesung mit Alias-Auflösung: build_optimizer.rs:2403, 2468.

Fehlend oder lückenhaft (Risiken mit Handlungsbedarf im Paketschnitt):

1. Warden-Patch-Delta nach 30.06.2026 existiert in patch_events nicht
   (Punkt 4). Klärung der Ingestion oder zweite Quelle nötig, sonst ist der
   Pflichtfall des Backtests ohne Basis.
2. hero_item_stats und hero_item_synergies fehlen für Warden und 35 weitere
   Helden (Punkt 6), sync_one_hero muss je Held laufen.
3. Kein Produzent für BUILD_PUBLISH_ORIGINAL, BuildSpecMod ohne imbue- und
   sell_priority-Felder (Punkt 3).
4. patch_tag 'current' ohne Datum und kein version-zu-Patch-Mapping in
   hero_build_sources (Punkte 4, 5, 6): Backtest "je Patch-Stand" braucht
   ein echtes Patch-Feld oder ein Mapping.
5. Kein JSON-Erzwingungsmechanismus im Fireworks-Request, Reasoning-Abschaltung
   nur als Antwortwaschung (Punkt 7).
6. meta_builds/meta_items sind leer bzw. Testdaten, Lightbringer ist weder
   Autor noch Build in der DB (Punkt 5), der Referenzfall existiert nur als
   Screenshot.

## Geschätzte Zahl der betroffenen Stellen und Repos

- Repos: 2. Deadlock-Brain (alle Bauarbeiten) und Deadlock-Steam-Bot (rein
  lesend, der Publish-Weg wird über die bestehende Schnittstelle genutzt,
  Änderungen am Bot sind laut Auftrag tabu und nach diesem Vorcheck auch
  nicht nötig: Handler und Payload-Felder reichen bereits).
- Datenbank: 1 Postgres `deadlock` mit den Schemata brain, tierlist, steam.
  Nur lesend im Vorcheck; schreibend im Projekt voraussichtlich brain
  (neue Reasoner-Tabellen, patch_tag-Erweiterung) und steam.steam_tasks
  (Publish-Enqueue).
- Betroffene Stellen im Brain, Schätzung: 12 bis 20 Dateien. Davon neu
  Reasoner-Modul 6 bis 10 Dateien (Helden-Modell, Item-Modell, Mechanik,
  Patch-Delta, Meta-Signale, Composer, Backtest), main.rs 1 Datei mit 5 bis
  8 Stellen, dbrain-builds spec.rs 1 Datei (imbue, sell_priority,
  optionaler Kostenpfad), Publish-Anbindung 1 bis 2 Dateien, sqlx-Cache
  Nachziehung falls neue compile-geprüfte Queries dazukommen, plus Tests.
- Ausdrücklich nicht im Code anzufassen: Steam-Bot, Twitch- und Discord-Bot,
  Ingest-Pfade, Modellnamen außerhalb der Settings.
