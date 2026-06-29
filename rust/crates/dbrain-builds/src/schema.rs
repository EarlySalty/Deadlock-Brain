use rusqlite::Connection;

pub const BUILD_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS item_catalog (
  item_id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  slot_type TEXT NOT NULL,
  tier INTEGER NOT NULL,
  defense_kind_json TEXT NOT NULL,
  damage_axis TEXT NOT NULL,
  properties_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS hero_catalog (
  hero_id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  base_health INTEGER NOT NULL,
  archetype TEXT NOT NULL,
  stats_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS hero_item_stats (
  hero_id INTEGER NOT NULL,
  item_id INTEGER NOT NULL,
  bracket TEXT NOT NULL,
  prevalence_builds INTEGER NOT NULL,
  wins INTEGER NOT NULL,
  losses INTEGER NOT NULL,
  matches INTEGER NOT NULL,
  players INTEGER NOT NULL,
  avg_buy_time_relative REAL,
  lift_pp REAL,
  patch_tag TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY(hero_id, item_id, bracket, patch_tag),
  FOREIGN KEY(item_id) REFERENCES item_catalog(item_id),
  FOREIGN KEY(hero_id) REFERENCES hero_catalog(hero_id)
);

CREATE INDEX IF NOT EXISTS idx_hero_item_stats_hero
  ON hero_item_stats(hero_id, bracket, patch_tag, prevalence_builds DESC, matches DESC);

CREATE TABLE IF NOT EXISTS hero_ability_orders (
  hero_id INTEGER NOT NULL,
  bracket TEXT NOT NULL,
  abilities_json TEXT NOT NULL,
  wins INTEGER NOT NULL,
  losses INTEGER NOT NULL,
  matches INTEGER NOT NULL,
  players INTEGER NOT NULL,
  patch_tag TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY(hero_id, bracket, patch_tag),
  FOREIGN KEY(hero_id) REFERENCES hero_catalog(hero_id)
);

CREATE TABLE IF NOT EXISTS hero_item_synergies (
  hero_id INTEGER NOT NULL,
  item_id INTEGER NOT NULL,
  with_item_id INTEGER NOT NULL,
  wins INTEGER NOT NULL,
  losses INTEGER NOT NULL,
  matches INTEGER NOT NULL,
  patch_tag TEXT NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY(hero_id, item_id, with_item_id, patch_tag),
  FOREIGN KEY(hero_id) REFERENCES hero_catalog(hero_id),
  FOREIGN KEY(item_id) REFERENCES item_catalog(item_id),
  FOREIGN KEY(with_item_id) REFERENCES item_catalog(item_id)
);

CREATE INDEX IF NOT EXISTS idx_hero_item_synergies_item
  ON hero_item_synergies(hero_id, item_id, patch_tag, matches DESC);
"#;

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(BUILD_SCHEMA)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_schema_creates_build_tables() {
        let conn = Connection::open_in_memory().expect("open sqlite");
        ensure_schema(&conn).expect("ensure schema");

        for table in [
            "item_catalog",
            "hero_catalog",
            "hero_item_stats",
            "hero_ability_orders",
            "hero_item_synergies",
        ] {
            let exists: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .expect("table lookup");
            assert_eq!(exists, 1, "missing {table}");
        }
    }
}
