-- C5 scratch bootstrap ONLY, invoked after socket/data-directory/role checks.
-- Matches the existing SourceStore query contract. No production migration.
CREATE TABLE IF NOT EXISTS brain.source_runs (
    id bigserial PRIMARY KEY,
    source text NOT NULL,
    status text NOT NULL,
    started_at timestamptz NOT NULL,
    finished_at timestamptz,
    summary jsonb NOT NULL DEFAULT '{}'::jsonb
);
CREATE TABLE IF NOT EXISTS brain.source_documents (
    id bigserial PRIMARY KEY,
    source text NOT NULL,
    external_id text NOT NULL,
    title text,
    url text,
    content_type text NOT NULL,
    raw_path text NOT NULL,
    content_hash text NOT NULL,
    fetched_at timestamptz NOT NULL,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    UNIQUE (source, external_id, content_hash)
);
