CREATE SCHEMA brain;
CREATE SCHEMA tierlist;
CREATE TABLE brain.entities (id bigint, entity_type text, canonical_name text, primary_external_id text);
CREATE TABLE brain.entity_aliases (entity_id bigint, alias text, alias_kind text);
CREATE TABLE brain.entity_snapshots (id bigint, source text, entity_type text, canonical_name text, external_id text, payload jsonb, fetched_at timestamptz);
CREATE TABLE brain.hero_catalog (hero_id bigint, name text);
CREATE TABLE brain.item_catalog (item_id bigint, name text, slot_type text, tier bigint, defense_kind jsonb, damage_axis text);
CREATE TABLE brain.hero_item_stats (hero_id bigint, item_id bigint, bracket text, patch_tag text, prevalence_builds bigint, wins bigint, losses bigint, matches bigint, avg_buy_time_relative float8, lift_pp float8);
CREATE TABLE brain.patch_events (id bigint, entity_name text, patch_external_id text, posted_at timestamptz, raw_line text, entity_type text, change_type text);
CREATE TABLE brain.patch_event_enrichments (patch_event_id bigint, secondary_entity_name text);
CREATE TABLE brain.hero_ability_orders (hero_id bigint, abilities jsonb, matches bigint, updated_at timestamptz);
CREATE TABLE tierlist.hero_build_sources (hero_id bigint, author_account_id bigint, version bigint, details jsonb, published_at timestamptz, last_updated_at timestamptz);
CREATE TABLE tierlist.watched_build_authors (author_account_id bigint, priority bigint);
INSERT INTO brain.entities VALUES (25, 'hero', 'Warden', '25');
INSERT INTO brain.hero_catalog VALUES (25, 'Warden');
INSERT INTO brain.entity_snapshots VALUES
  (1, 'deadlock_assets_api', 'hero', 'Warden', '25', '{"id":25,"name":"Warden","hero_type":"brawler","starting_stats":{"max_health":{"value":815}},"weapon_info":{"bullet_damage":20,"shots_per_second":4,"clip_size":20,"reload_duration":2}}', now()),
  (2, 'deadlock_assets_api', 'item_or_ability', 'Fixture Item', '100', '{"id":100,"name":"Fixture Item","type":"upgrade","shopable":true,"item_slot_type":"spirit","item_tier":2,"cost":1250,"properties":{"TechPower":{"value":10}}}', now());
INSERT INTO brain.item_catalog VALUES (100, 'Fixture Item', 'spirit', 2, '[]', 'spirit');
INSERT INTO brain.patch_events VALUES (1, 'Warden', 'patch-1', '2026-09-12T10:00:00Z', 'Base health increased from 800 to 815', 'hero', 'buff');
INSERT INTO brain.hero_ability_orders VALUES (25, '[10,10,20,10,10]', 100, now());
INSERT INTO tierlist.hero_build_sources VALUES (25, 1, 1, '{"author":"Fixture Author","categories":[]}', now(), now());
