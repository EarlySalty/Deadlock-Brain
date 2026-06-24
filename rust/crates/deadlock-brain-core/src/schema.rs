use rusqlite::Connection;

pub const REGULAR_TABLES: &[&str] = &[
    "analysis_notes",
    "build_learning_notes",
    "entities",
    "entity_aliases",
    "entity_lineage",
    "entity_snapshots",
    "hero_stat_profiles",
    "hero_stat_values",
    "learned_builds",
    "legacy_entities",
    "mechanic_notes",
    "meta_trend_notes",
    "patch_event_enrichments",
    "patch_events",
    "patch_impact_notes",
    "player_match_decision_notes",
    "sheet_boons_ap",
    "sheet_hero_rankings",
    "sheet_heroes_stats",
    "sheet_items",
    "sheet_raw_heroes",
    "sheet_shop_bonuses",
    "sheet_tab_rows",
    "source_documents",
    "source_runs",
    "youtube_feed_sources",
    "youtube_learning_claims",
    "youtube_transcripts",
    "youtube_videos",
];

pub const VECTOR_TABLES_OMITTED: &[&str] = &[
    "vector_embeddings",
    "vector_embeddings_chunks",
    "vector_embeddings_info",
    "vector_embeddings_rowids",
    "vector_embeddings_vector_chunks00",
];

pub const REGULAR_SCHEMA_WITHOUT_YOUTUBE: &str = r#"
CREATE TABLE IF NOT EXISTS analysis_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  query TEXT NOT NULL,
  entity_type TEXT,
  entity_name TEXT,
  context_kind TEXT NOT NULL,
  context_hash TEXT NOT NULL,
  prompt_version TEXT NOT NULL,
  prompt_text TEXT NOT NULL,
  result_text TEXT,
  model TEXT,
  confidence REAL,
  status TEXT NOT NULL,
  source_references_json TEXT NOT NULL DEFAULT '[]',
  context_json TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS build_learning_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  learned_build_id INTEGER,
  hero_name TEXT,
  source TEXT NOT NULL,
  context_hash TEXT NOT NULL,
  prompt_version TEXT NOT NULL,
  prompt_text TEXT NOT NULL,
  result_text TEXT,
  insights_json TEXT NOT NULL DEFAULT '{}',
  model TEXT,
  status TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY(learned_build_id) REFERENCES learned_builds(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS entities (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_type TEXT NOT NULL,
  canonical_name TEXT NOT NULL,
  primary_external_id TEXT,
  source TEXT NOT NULL,
  first_snapshot_id INTEGER,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(entity_type, canonical_name),
  FOREIGN KEY(first_snapshot_id) REFERENCES entity_snapshots(id)
);

CREATE TABLE IF NOT EXISTS entity_aliases (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_id INTEGER NOT NULL,
  alias TEXT NOT NULL,
  alias_norm TEXT NOT NULL,
  alias_kind TEXT NOT NULL,
  source TEXT NOT NULL,
  external_id TEXT,
  snapshot_id INTEGER,
  created_at INTEGER NOT NULL,
  UNIQUE(entity_id, alias_norm, alias_kind),
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE CASCADE,
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id)
);

CREATE TABLE IF NOT EXISTS entity_lineage (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  patch_event_id INTEGER NOT NULL,
  relation_type TEXT NOT NULL,
  source_entity_type TEXT,
  source_name TEXT NOT NULL,
  source_name_norm TEXT NOT NULL,
  target_entity_type TEXT,
  target_name TEXT,
  target_name_norm TEXT,
  owner_entity_type TEXT,
  owner_name TEXT,
  owner_name_norm TEXT,
  confidence REAL NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY(patch_event_id) REFERENCES patch_events(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS entity_snapshots (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  entity_type TEXT NOT NULL,
  external_id TEXT NOT NULL,
  canonical_name TEXT,
  payload_hash TEXT NOT NULL,
  payload_json TEXT NOT NULL,
  fetched_at INTEGER NOT NULL,
  source_document_id INTEGER,
  UNIQUE(source, entity_type, external_id, payload_hash),
  FOREIGN KEY(source_document_id) REFERENCES source_documents(id)
);

CREATE TABLE IF NOT EXISTS hero_stat_profiles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  entity_id INTEGER,
  hero_name TEXT NOT NULL,
  source TEXT NOT NULL,
  external_id TEXT NOT NULL,
  payload_hash TEXT NOT NULL,
  row_number INTEGER,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS hero_stat_values (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  profile_id INTEGER NOT NULL,
  entity_id INTEGER,
  hero_name TEXT NOT NULL,
  stat_key TEXT NOT NULL,
  stat_label TEXT NOT NULL,
  numeric_value REAL,
  raw_value TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(profile_id, stat_key),
  FOREIGN KEY(profile_id) REFERENCES hero_stat_profiles(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS learned_builds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  source_build_id TEXT NOT NULL,
  hero_id INTEGER,
  hero_name TEXT,
  language INTEGER,
  source_rank INTEGER,
  quality_tier TEXT NOT NULL,
  quality_score REAL NOT NULL,
  name TEXT,
  author_account_id TEXT,
  description TEXT,
  tags_json TEXT NOT NULL DEFAULT '[]',
  details_json TEXT NOT NULL DEFAULT '{}',
  item_names_json TEXT NOT NULL DEFAULT '[]',
  ability_order_json TEXT NOT NULL DEFAULT '[]',
  source_metadata_json TEXT NOT NULL DEFAULT '{}',
  imported_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(source, source_build_id)
);

CREATE TABLE IF NOT EXISTS legacy_entities (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  legacy_type TEXT NOT NULL,
  canonical_name TEXT NOT NULL,
  name_norm TEXT NOT NULL,
  observed_entity_type TEXT NOT NULL,
  first_patch_event_id INTEGER,
  last_patch_event_id INTEGER,
  first_seen_at TEXT,
  last_seen_at TEXT,
  event_count INTEGER NOT NULL,
  confidence REAL NOT NULL,
  status TEXT NOT NULL,
  samples_json TEXT NOT NULL DEFAULT '[]',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(legacy_type, name_norm),
  FOREIGN KEY(first_patch_event_id) REFERENCES patch_events(id) ON DELETE SET NULL,
  FOREIGN KEY(last_patch_event_id) REFERENCES patch_events(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS mechanic_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  source TEXT NOT NULL,
  category TEXT NOT NULL,
  rowid INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  UNIQUE(title)
);

CREATE TABLE IF NOT EXISTS meta_trend_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_name TEXT NOT NULL,
  trend_direction TEXT NOT NULL,
  winrate_delta REAL NOT NULL,
  context_json TEXT NOT NULL DEFAULT '{}',
  result_text TEXT,
  status TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS patch_event_enrichments (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  patch_event_id INTEGER NOT NULL,
  stat_name TEXT,
  old_value TEXT,
  new_value TEXT,
  unit TEXT,
  ability_name TEXT,
  secondary_entity_name TEXT,
  confidence REAL NOT NULL,
  flags_json TEXT NOT NULL DEFAULT '[]',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(patch_event_id),
  FOREIGN KEY(patch_event_id) REFERENCES patch_events(id)
);

CREATE TABLE IF NOT EXISTS patch_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  patch_snapshot_id INTEGER NOT NULL,
  patch_external_id TEXT NOT NULL,
  patch_title TEXT,
  patch_url TEXT,
  source_kind TEXT NOT NULL,
  posted_at TEXT,
  line_index INTEGER NOT NULL,
  section TEXT,
  entity_type TEXT NOT NULL,
  entity_name TEXT,
  subject TEXT,
  change_type TEXT NOT NULL,
  raw_line TEXT NOT NULL,
  normalized_line TEXT NOT NULL,
  old_value TEXT,
  new_value TEXT,
  confidence REAL NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  event_hash TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  UNIQUE(event_hash),
  FOREIGN KEY(patch_snapshot_id) REFERENCES entity_snapshots(id)
);

CREATE TABLE IF NOT EXISTS patch_impact_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  entity_type TEXT NOT NULL,
  entity_name TEXT NOT NULL,
  entity_id INTEGER,
  context_hash TEXT NOT NULL,
  prompt_version TEXT NOT NULL DEFAULT 'patch_impact_de_v1',
  prompt_text TEXT NOT NULL,
  result_text TEXT,
  insights_json TEXT NOT NULL DEFAULT '{}',
  model TEXT,
  status TEXT NOT NULL,
  patch_range_start TEXT,
  patch_range_end TEXT,
  event_count INTEGER,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS player_match_decision_notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  account_id TEXT NOT NULL,
  match_id TEXT NOT NULL,
  hero_id TEXT,
  hero_name TEXT,
  context_hash TEXT NOT NULL,
  prompt_version TEXT NOT NULL,
  prompt_text TEXT NOT NULL,
  result_text TEXT,
  insights_json TEXT NOT NULL DEFAULT '{}',
  model TEXT,
  status TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sheet_boons_ap (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  souls INTEGER NOT NULL,
  boons INTEGER,
  ap INTEGER,
  note TEXT,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sheet_hero_rankings (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  entity_id INTEGER,
  hero_name TEXT NOT NULL,
  carry REAL, crowd_control REAL, disengage REAL, early REAL,
  engage REAL, frontline REAL, late REAL, mid REAL, mid_contest REAL,
  mobility REAL, nuke_phys REAL, nuke_spirit REAL, pick REAL,
  poke REAL, support REAL, wave_clear REAL,
  sustain_dps_phys REAL, sustain_dps_spirit REAL,
  average_rank REAL,
  payload_hash TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS sheet_heroes_stats (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  entity_id INTEGER,
  hero_name TEXT NOT NULL,
  alt_fire_type TEXT,
  hero_labs TEXT,
  base_hp REAL,
  base_move_speed REAL,
  base_sprint REAL,
  base_stamina REAL,
  base_regen REAL,
  base_ammo REAL,
  pellets REAL,
  alt_fire_pellets REAL,
  base_bullet_dmg REAL,
  base_fire_rate REAL,
  base_dps REAL,
  max_gun_dps REAL,
  max_gun_damage REAL,
  dpm REAL,
  max_dpm REAL,
  falloff_range_min REAL,
  falloff_range_max REAL,
  hp_gain REAL,
  dmg_gain REAL,
  spirit_gain REAL,
  spirit_bonus REAL,
  spirit_bonus_2 REAL,
  spirit_ratio REAL,
  spirit_ratio_2 REAL,
  spirit_scaling REAL,
  spirit_scaling_2 REAL,
  aggregate_growth_pct REAL,
  dps_growth_pct REAL,
  hp_growth_pct REAL,
  melee_ratio REAL,
  total_bullet_ratio REAL,
  total_spirit_ratio REAL,
  max_level_hp REAL,
  payload_hash TEXT NOT NULL,
  created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS sheet_items (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  item_id INTEGER,
  code_name TEXT NOT NULL,
  game_name TEXT NOT NULL,
  canonical_name TEXT NOT NULL,
  created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sheet_raw_heroes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  entity_id INTEGER,
  hero_name TEXT NOT NULL,
  hero_id INTEGER,
  disabled INTEGER NOT NULL DEFAULT 0,
  move_speed REAL, sprint_speed REAL, crouch_speed REAL, move_accel REAL,
  light_melee_dmg REAL, heavy_melee_dmg REAL, max_hp REAL,
  base_stamina REAL, stam_regen REAL, hp_regen REAL, base_health REAL,
  gun_growth REAL, alt_gun_growth REAL, hp_per_boon REAL,
  melee_gain REAL, spirit_per_boon REAL,
  payload_hash TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
  FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS sheet_shop_bonuses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  souls_cost INTEGER NOT NULL,
  weapon INTEGER, spirit INTEGER, vitality INTEGER,
  inc_from_prev_pct REAL,
  created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sheet_tab_rows (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  snapshot_id INTEGER NOT NULL,
  tab_name TEXT NOT NULL,
  gid TEXT NOT NULL,
  row_number INTEGER,
  canonical_name TEXT,
  row_json TEXT NOT NULL,
  created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
  UNIQUE(snapshot_id),
  FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS source_documents (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  external_id TEXT NOT NULL,
  title TEXT,
  url TEXT,
  content_type TEXT NOT NULL,
  raw_path TEXT NOT NULL,
  content_hash TEXT NOT NULL,
  fetched_at INTEGER NOT NULL,
  metadata_json TEXT NOT NULL DEFAULT '{}',
  UNIQUE(source, external_id, content_hash)
);

CREATE TABLE IF NOT EXISTS source_runs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  source TEXT NOT NULL,
  status TEXT NOT NULL,
  started_at INTEGER NOT NULL,
  finished_at INTEGER,
  summary_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_analysis_notes_entity
  ON analysis_notes(entity_type, entity_name);

CREATE INDEX IF NOT EXISTS idx_analysis_notes_query
  ON analysis_notes(query, created_at);

CREATE UNIQUE INDEX IF NOT EXISTS idx_analysis_notes_unique
  ON analysis_notes(query, context_hash, prompt_version, COALESCE(model, ''), status);

CREATE INDEX IF NOT EXISTS idx_build_learning_notes_hero
  ON build_learning_notes(hero_name, updated_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS idx_build_learning_notes_unique
  ON build_learning_notes(learned_build_id, context_hash, prompt_version, COALESCE(model, ''), status);

CREATE INDEX IF NOT EXISTS idx_entities_type_name
  ON entities(entity_type, canonical_name);

CREATE INDEX IF NOT EXISTS idx_entity_aliases_entity
  ON entity_aliases(entity_id);

CREATE INDEX IF NOT EXISTS idx_entity_aliases_norm
  ON entity_aliases(alias_norm);

CREATE INDEX IF NOT EXISTS idx_entity_lineage_owner
  ON entity_lineage(owner_name_norm);

CREATE INDEX IF NOT EXISTS idx_entity_lineage_source
  ON entity_lineage(source_name_norm);

CREATE INDEX IF NOT EXISTS idx_entity_lineage_target
  ON entity_lineage(target_name_norm);

CREATE UNIQUE INDEX IF NOT EXISTS idx_entity_lineage_unique
  ON entity_lineage(
    patch_event_id,
    relation_type,
    source_name_norm,
    COALESCE(target_name_norm, ''),
    COALESCE(owner_name_norm, '')
  );

CREATE INDEX IF NOT EXISTS idx_entity_snapshots_entity
  ON entity_snapshots(entity_type, canonical_name);

CREATE INDEX IF NOT EXISTS idx_hero_stat_profiles_entity
  ON hero_stat_profiles(entity_id);

CREATE INDEX IF NOT EXISTS idx_hero_stat_profiles_hero
  ON hero_stat_profiles(hero_name);

CREATE INDEX IF NOT EXISTS idx_hero_stat_values_entity_key
  ON hero_stat_values(entity_id, stat_key);

CREATE INDEX IF NOT EXISTS idx_hero_stat_values_key
  ON hero_stat_values(stat_key);

CREATE INDEX IF NOT EXISTS idx_learned_builds_hero
  ON learned_builds(hero_name, quality_score DESC);

CREATE INDEX IF NOT EXISTS idx_learned_builds_quality
  ON learned_builds(quality_tier, quality_score DESC);

CREATE INDEX IF NOT EXISTS idx_legacy_entities_name
  ON legacy_entities(name_norm);

CREATE INDEX IF NOT EXISTS idx_legacy_entities_type
  ON legacy_entities(legacy_type, status);

CREATE INDEX IF NOT EXISTS idx_mtn_entity_name ON meta_trend_notes(entity_name);

CREATE INDEX IF NOT EXISTS idx_patch_event_enrichments_secondary_entity
  ON patch_event_enrichments(secondary_entity_name);

CREATE INDEX IF NOT EXISTS idx_patch_event_enrichments_stat
  ON patch_event_enrichments(stat_name);

CREATE INDEX IF NOT EXISTS idx_patch_events_entity
  ON patch_events(entity_type, entity_name);

CREATE INDEX IF NOT EXISTS idx_patch_events_patch
  ON patch_events(patch_snapshot_id, line_index);

CREATE INDEX IF NOT EXISTS idx_patch_events_source_kind
  ON patch_events(source_kind);

CREATE INDEX IF NOT EXISTS idx_pin_entity_name ON patch_impact_notes(entity_name);

CREATE INDEX IF NOT EXISTS idx_pin_status ON patch_impact_notes(status);

CREATE UNIQUE INDEX IF NOT EXISTS idx_pin_unique
  ON patch_impact_notes(entity_name, context_hash, prompt_version, COALESCE(model, ''));

CREATE INDEX IF NOT EXISTS idx_player_match_decision_notes_lookup
  ON player_match_decision_notes(account_id, match_id, updated_at DESC);

CREATE UNIQUE INDEX IF NOT EXISTS idx_player_match_decision_notes_unique
  ON player_match_decision_notes(account_id, match_id, context_hash, prompt_version, COALESCE(model, ''), status);

CREATE INDEX IF NOT EXISTS idx_sba_souls ON sheet_boons_ap(souls);

CREATE INDEX IF NOT EXISTS idx_sheet_boons_ap_souls
  ON sheet_boons_ap(souls);

CREATE INDEX IF NOT EXISTS idx_sheet_hero_rankings_entity
  ON sheet_hero_rankings(entity_id);

CREATE INDEX IF NOT EXISTS idx_sheet_hero_rankings_name
  ON sheet_hero_rankings(hero_name);

CREATE INDEX IF NOT EXISTS idx_sheet_raw_heroes_entity
  ON sheet_raw_heroes(entity_id);

CREATE INDEX IF NOT EXISTS idx_sheet_raw_heroes_name
  ON sheet_raw_heroes(hero_name);

CREATE INDEX IF NOT EXISTS idx_shr_entity ON sheet_hero_rankings(entity_id);

CREATE INDEX IF NOT EXISTS idx_shr_name ON sheet_hero_rankings(hero_name);

CREATE INDEX IF NOT EXISTS idx_shs_entity ON sheet_heroes_stats(entity_id);

CREATE INDEX IF NOT EXISTS idx_shs_name ON sheet_heroes_stats(hero_name);

CREATE INDEX IF NOT EXISTS idx_si_canonical ON sheet_items(canonical_name);

CREATE INDEX IF NOT EXISTS idx_si_code ON sheet_items(code_name);

CREATE INDEX IF NOT EXISTS idx_si_game ON sheet_items(game_name);

CREATE INDEX IF NOT EXISTS idx_source_documents_source
  ON source_documents(source, external_id);

CREATE INDEX IF NOT EXISTS idx_srh_entity ON sheet_raw_heroes(entity_id);

CREATE INDEX IF NOT EXISTS idx_srh_name ON sheet_raw_heroes(hero_name);

CREATE INDEX IF NOT EXISTS idx_ssb_souls ON sheet_shop_bonuses(souls_cost);

CREATE INDEX IF NOT EXISTS idx_str_canonical ON sheet_tab_rows(canonical_name);

CREATE INDEX IF NOT EXISTS idx_str_tab ON sheet_tab_rows(tab_name);
"#;

pub const YOUTUBE_SCHEMA: &str = r#"
        CREATE TABLE IF NOT EXISTS youtube_feed_sources (
          feed_key TEXT PRIMARY KEY,
          source_type TEXT NOT NULL,
          url TEXT NOT NULL,
          handle TEXT,
          playlist_id TEXT,
          channel_id TEXT,
          title TEXT,
          enabled INTEGER NOT NULL DEFAULT 1,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS youtube_videos (
          video_id TEXT PRIMARY KEY,
          feed_key TEXT NOT NULL,
          channel_id TEXT,
          channel_title TEXT,
          title TEXT NOT NULL,
          url TEXT NOT NULL,
          published_at TEXT,
          description TEXT,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          transcript_status TEXT NOT NULL DEFAULT 'missing',
          learning_status TEXT NOT NULL DEFAULT 'queued',
          discovered_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(feed_key) REFERENCES youtube_feed_sources(feed_key)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_videos_learning
          ON youtube_videos(learning_status, transcript_status, published_at);

        CREATE TABLE IF NOT EXISTS youtube_transcripts (
          video_id TEXT PRIMARY KEY,
          language TEXT,
          source_kind TEXT NOT NULL,
          transcript_text TEXT NOT NULL,
          content_hash TEXT NOT NULL,
          source_document_id INTEGER,
          imported_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id),
          FOREIGN KEY(source_document_id) REFERENCES source_documents(id)
        );

        CREATE TABLE IF NOT EXISTS youtube_learning_claims (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          video_id TEXT NOT NULL,
          claim_hash TEXT NOT NULL UNIQUE,
          claim_index INTEGER NOT NULL,
          entity_type TEXT,
          entity_name TEXT,
          claim_type TEXT NOT NULL,
          claim_text TEXT NOT NULL,
          evidence_quote TEXT NOT NULL,
          timestamp_seconds REAL,
          model_confidence REAL NOT NULL,
          verifier_confidence REAL NOT NULL,
          status TEXT NOT NULL,
          model TEXT,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          model_response_text TEXT NOT NULL,
          provider_metadata_json TEXT NOT NULL DEFAULT '{}',
          verifier_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_learning_claims_video
          ON youtube_learning_claims(video_id, status);
"#;

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(REGULAR_SCHEMA_WITHOUT_YOUTUBE)?;
    ensure_youtube_tables(conn)
}

pub fn ensure_youtube_tables(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(YOUTUBE_SCHEMA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_schema_creates_all_regular_tables() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        ensure_schema(&conn).expect("ensure schema");

        let tables = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .expect("prepare")
            .query_map([], |row| row.get::<_, String>(0))
            .expect("query")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect");

        for table in REGULAR_TABLES {
            assert!(tables.iter().any(|actual| actual == table), "missing {table}");
        }
        for table in VECTOR_TABLES_OMITTED {
            assert!(
                !tables.iter().any(|actual| actual == table),
                "vector table should be omitted: {table}"
            );
        }
    }
}
