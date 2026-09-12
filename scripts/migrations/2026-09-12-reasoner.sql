CREATE TABLE IF NOT EXISTS brain.reasoner_builds (
  hero_id bigint NOT NULL,
  patch_tag text NOT NULL,
  hero_name text NOT NULL,
  build jsonb NOT NULL,
  confidence text NOT NULL,
  used_ai boolean NOT NULL DEFAULT false,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag)
);

CREATE TABLE IF NOT EXISTS brain.reasoner_item_scores (
  hero_id bigint NOT NULL,
  patch_tag text NOT NULL,
  item_id bigint NOT NULL,
  combat_value double precision NOT NULL,
  per_slot_value double precision NOT NULL,
  per_soul_value double precision NOT NULL,
  purchase_bonus double precision NOT NULL,
  condition_factor double precision NOT NULL,
  active_value double precision NOT NULL,
  passive_value double precision NOT NULL,
  meta_support double precision NOT NULL,
  total double precision NOT NULL,
  confidence text NOT NULL,
  buy_phase text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag, item_id)
);

ALTER TABLE brain.reasoner_item_scores
  ADD COLUMN IF NOT EXISTS per_soul_value double precision,
  ADD COLUMN IF NOT EXISTS active_value double precision,
  ADD COLUMN IF NOT EXISTS passive_value double precision;

CREATE TABLE IF NOT EXISTS brain.reasoner_patch_deltas (
  hero_id bigint NOT NULL,
  patch_tag text NOT NULL,
  target_kind text NOT NULL,
  target_id bigint NOT NULL,
  mechanic text NOT NULL,
  sign smallint NOT NULL,
  magnitude double precision NOT NULL,
  note text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (hero_id, patch_tag, target_kind, target_id, mechanic)
);

CREATE TABLE IF NOT EXISTS brain.reasoner_backtests (
  run_id uuid NOT NULL DEFAULT gen_random_uuid(),
  hero_id bigint NOT NULL,
  patch_tag text NOT NULL,
  author text NOT NULL,
  core_coverage double precision NOT NULL,
  order_proximity double precision,
  switch_detected boolean,
  detail jsonb NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (run_id, hero_id, author)
);

CREATE UNIQUE INDEX IF NOT EXISTS reasoner_backtests_hero_patch_author
  ON brain.reasoner_backtests (hero_id, patch_tag, author);

ALTER TABLE brain.reasoner_backtests ALTER COLUMN order_proximity DROP NOT NULL;
