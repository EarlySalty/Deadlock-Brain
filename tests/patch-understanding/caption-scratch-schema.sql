-- Scratch-Schema fuer den echten Caption-Integrationstest
-- (deadlock-brain-yt::transcripts::tests::save_transcript_is_idempotent_and_sets_ready_pg).
-- Reduzierte, aber getreue Obermenge der Spalten, die der reale Rust-Pfad und
-- die Testhelfer beruehren. Nur fuer eine leere Wegwerf-Testdatenbank, niemals
-- Produktion. Die Patch-Tabellen bleiben leer; sie existieren nur, damit die
-- monolithische Migration 2026-09-18-patch-evidence.sql sauber durchlaeuft.
CREATE SCHEMA brain;
CREATE SCHEMA patchnotes;

CREATE TABLE brain.youtube_feed_sources(
    feed_key text PRIMARY KEY,
    source_type text NOT NULL,
    url text,
    enabled int NOT NULL DEFAULT 1,
    metadata jsonb NOT NULL DEFAULT '{}',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE brain.youtube_videos(
    video_id text PRIMARY KEY,
    feed_key text,
    channel_title text,
    title text,
    url text,
    published_at timestamptz,
    discovered_at timestamptz NOT NULL DEFAULT now(),
    transcript_status text NOT NULL DEFAULT 'missing',
    learning_status text,
    metadata jsonb NOT NULL DEFAULT '{}',
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE brain.youtube_transcripts(
    video_id text PRIMARY KEY,
    language text,
    source_kind text NOT NULL,
    transcript_text text,
    content_hash text,
    source_document_id bigint,
    imported_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE brain.youtube_learning_claims(
    claim_hash text PRIMARY KEY,
    video_id text
);
CREATE TABLE brain.youtube_transcript_claim_attempts(
    video_id text
);

-- Von der Migration referenzierte Patch-Tabellen (leer).
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
