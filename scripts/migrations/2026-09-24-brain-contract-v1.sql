BEGIN;

CREATE SCHEMA IF NOT EXISTS brain;

CREATE TABLE IF NOT EXISTS brain.source_record_revisions (
    source_id text NOT NULL,
    logical_id text NOT NULL,
    revision bigint NOT NULL CHECK (revision > 0),
    content_hash text NOT NULL,
    tombstone boolean NOT NULL DEFAULT false,
    record_json jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (source_id, logical_id, revision)
);

CREATE TABLE IF NOT EXISTS brain.source_record_heads (
    source_id text NOT NULL,
    logical_id text NOT NULL,
    revision bigint NOT NULL CHECK (revision > 0),
    content_hash text NOT NULL,
    tombstone boolean NOT NULL DEFAULT false,
    record_json jsonb NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (source_id, logical_id)
);

CREATE TABLE IF NOT EXISTS brain.corpus_releases_v1 (
    release_id text PRIMARY KEY,
    knowledge_version text NOT NULL,
    patch text NOT NULL,
    release_json jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

COMMIT;
