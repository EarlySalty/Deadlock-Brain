CREATE SCHEMA brain;
CREATE SCHEMA patchnotes;
CREATE TABLE brain.youtube_videos(video_id text PRIMARY KEY, metadata jsonb DEFAULT '{}');
CREATE TABLE brain.youtube_transcripts(video_id text PRIMARY KEY, source_kind text NOT NULL);
CREATE TABLE patchnotes.changelog_posts(id bigint PRIMARY KEY, title text, url text, posted_at timestamptz, raw_content text);
CREATE TABLE brain.patch_events(
    id bigserial PRIMARY KEY, event_hash text UNIQUE NOT NULL, patch_external_id text,
    patch_title text, patch_url text, posted_at timestamptz, source_kind text,
    entity_type text, entity_name text, subject text, change_type text,
    raw_line text, old_value text, new_value text, created_at timestamptz DEFAULT now(), metadata jsonb DEFAULT '{}'
);
CREATE TABLE brain.knowledge_events(
    event_hash text PRIMARY KEY, event_source text, observed_at timestamptz,
    occurred_at timestamptz, effective_from timestamptz, metadata jsonb DEFAULT '{}'
);
INSERT INTO patchnotes.changelog_posts VALUES(1,'Synthetic fixture','https://example.test/patch','2020-01-01T00:00:00Z','Synthetic value changed');
INSERT INTO brain.patch_events(event_hash,patch_external_id,patch_title,patch_url,posted_at,source_kind,entity_type,entity_name,raw_line,old_value,new_value,created_at)
VALUES('e1','patch_1','Synthetic fixture','https://example.test/patch','2020-01-01T00:00:00Z','steam','mechanic','Synthetic','Synthetic 35 to 40','35','40','2020-02-01T00:00:00Z');
