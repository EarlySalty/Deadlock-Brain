CREATE SCHEMA IF NOT EXISTS brain;

CREATE TABLE IF NOT EXISTS brain.population_player_matches (
  match_id bigint NOT NULL,
  account_id bigint NOT NULL,
  hero_id bigint NOT NULL,
  team smallint NOT NULL,
  won boolean NOT NULL,
  average_badge integer,
  duration_s integer,
  start_time timestamptz,
  items bigint[] NOT NULL,
  buy_times_s integer[] NOT NULL,
  sold_times_s integer[] NOT NULL,
  net_worth_rank smallint[] NOT NULL,
  imbue_targets bigint[] NOT NULL,
  ability_points bigint[] NOT NULL,
  ability_times_s integer[] NOT NULL,
  fetched_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (match_id, account_id)
);

CREATE INDEX IF NOT EXISTS population_player_matches_hero_idx
  ON brain.population_player_matches (hero_id);

CREATE TABLE IF NOT EXISTS brain.population_sync_runs (
  run_id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  started_at timestamptz NOT NULL DEFAULT now(),
  finished_at timestamptz,
  requested_matches integer NOT NULL,
  hero_filter bigint,
  since_unix bigint,
  window_low_match_id bigint,
  window_high_match_id bigint,
  matches_seen integer NOT NULL DEFAULT 0,
  player_matches_inserted integer NOT NULL DEFAULT 0,
  player_matches_skipped integer NOT NULL DEFAULT 0,
  duration_ms bigint NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS brain.population_hero_buckets (
  hero_id bigint NOT NULL,
  bucket text NOT NULL,
  players_raw integer NOT NULL,
  players_weighted double precision NOT NULL,
  ability_order_followers integer NOT NULL DEFAULT 0,
  ability_order_players integer NOT NULL DEFAULT 0,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, bucket)
);

CREATE TABLE IF NOT EXISTS brain.population_item_stats (
  hero_id bigint NOT NULL,
  bucket text NOT NULL,
  item_id bigint NOT NULL,
  buyers integer NOT NULL,
  prevalence_raw double precision NOT NULL,
  prevalence_weighted double precision NOT NULL,
  median_position double precision NOT NULL,
  median_buy_time_s double precision NOT NULL,
  sell_rate double precision NOT NULL,
  is_staple boolean NOT NULL,
  next_item_id bigint,
  next_item_share double precision NOT NULL DEFAULT 0,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, bucket, item_id)
);

CREATE TABLE IF NOT EXISTS brain.population_imbue_stats (
  hero_id bigint NOT NULL,
  bucket text NOT NULL,
  item_id bigint NOT NULL,
  target_ability_id bigint NOT NULL,
  target_count integer NOT NULL,
  total_imbues integer NOT NULL,
  is_split boolean NOT NULL,
  is_thin boolean NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, bucket, item_id)
);

CREATE TABLE IF NOT EXISTS brain.population_ability_order (
  hero_id bigint NOT NULL,
  bucket text NOT NULL,
  position smallint NOT NULL,
  ability_id bigint NOT NULL,
  followers integer NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, bucket, position)
);
