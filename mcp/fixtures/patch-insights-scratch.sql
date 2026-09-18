CREATE SCHEMA brain;
CREATE TABLE brain.youtube_feed_sources (
  feed_key text PRIMARY KEY, source_type text, url text, enabled integer,
  metadata jsonb, created_at timestamptz, updated_at timestamptz
);
CREATE TABLE brain.youtube_videos (
  video_id text PRIMARY KEY, feed_key text, channel_title text, title text, url text,
  published_at timestamptz, metadata jsonb, transcript_status text, learning_status text,
  discovered_at timestamptz, updated_at timestamptz
);
CREATE TABLE brain.youtube_transcripts (
  video_id text PRIMARY KEY REFERENCES brain.youtube_videos(video_id), language text,
  source_kind text, transcript_text text, content_hash text, source_document_id bigint,
  imported_at timestamptz, updated_at timestamptz
);
CREATE TABLE brain.entities (id bigserial PRIMARY KEY, canonical_name text);
CREATE TABLE brain.entity_aliases (entity_id bigint REFERENCES brain.entities(id), alias text);
CREATE TABLE brain.patch_changes (
  patch_title text, patch_date date, entity_type text, entity_name text,
  ability_name text, stat_name text, old_value text, new_value text,
  change_type text, numeric_direction text, raw_line text, patch_url text, confidence float8
);
CREATE TABLE brain.patch_events (patch_url text, raw_line text, created_at timestamptz);
INSERT INTO brain.entities(id,canonical_name) VALUES (1,'Fixture Hero');
INSERT INTO brain.entity_aliases(entity_id,alias) VALUES(1,'Fixture Alias');
INSERT INTO brain.patch_changes VALUES
('Synthetic patch one','2026-09-01','hero','Fixture Hero','Fixture Ability','cooldown','10s','12s','numeric','increase','Cooldown increased from 10s to 12s','https://example.invalid/patch-one',1),
('Synthetic patch two','2026-09-02','hero','Fixture Alias','Fixture Ability','cooldown','12s','9s','numeric','decrease','Cooldown reduced from 12s to 9s','https://example.invalid/patch-two',1);
INSERT INTO brain.patch_events VALUES
('https://example.invalid/patch-one','Cooldown increased from 10s to 12s','2026-09-03T10:00:00Z'),
('https://example.invalid/patch-two','Cooldown reduced from 12s to 9s','2026-09-04T10:00:00Z');
