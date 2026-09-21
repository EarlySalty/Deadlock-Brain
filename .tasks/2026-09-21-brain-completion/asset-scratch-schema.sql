-- Isolated contract fixture, never a production migration.
CREATE SCHEMA brain;
CREATE TABLE brain.source_runs (
    id BIGSERIAL PRIMARY KEY, source TEXT NOT NULL, status TEXT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL, finished_at TIMESTAMPTZ, summary JSONB
);
CREATE TABLE brain.source_documents (
    id BIGSERIAL PRIMARY KEY, source TEXT NOT NULL, external_id TEXT NOT NULL,
    title TEXT, url TEXT, content_type TEXT NOT NULL, raw_path TEXT NOT NULL,
    content_hash TEXT NOT NULL, fetched_at TIMESTAMPTZ NOT NULL, metadata JSONB NOT NULL,
    UNIQUE(source, external_id, content_hash)
);
CREATE TABLE brain.entity_snapshots (
    id BIGSERIAL PRIMARY KEY, source TEXT NOT NULL, entity_type TEXT NOT NULL,
    external_id TEXT NOT NULL, canonical_name TEXT, payload_hash TEXT NOT NULL,
    payload JSONB NOT NULL, fetched_at TIMESTAMPTZ NOT NULL,
    source_document_id BIGINT REFERENCES brain.source_documents(id),
    UNIQUE(source, entity_type, external_id, payload_hash)
);
CREATE TABLE brain.item_catalog (
    item_id BIGINT PRIMARY KEY, name TEXT NOT NULL, slot_type TEXT NOT NULL,
    tier BIGINT NOT NULL, defense_kind JSONB NOT NULL, damage_axis TEXT NOT NULL,
    properties JSONB NOT NULL, updated_at TIMESTAMPTZ NOT NULL
);
