BEGIN;
SET LOCAL lock_timeout = '5s';

ALTER TABLE brain.youtube_transcript_evidence
    ADD COLUMN IF NOT EXISTS timing_status text NOT NULL DEFAULT 'missing'
        CHECK (timing_status IN ('available','partial','missing')),
    ADD COLUMN IF NOT EXISTS parser_version text NOT NULL DEFAULT 'json3_timed_v1';

CREATE TABLE IF NOT EXISTS brain.patch_reference_documents (
    id bigserial PRIMARY KEY,
    source_url text NOT NULL,
    content_hash text NOT NULL,
    document jsonb NOT NULL,
    imported_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(source_url,content_hash)
);

CREATE TABLE IF NOT EXISTS brain.patch_insight_runs (
    id bigserial PRIMARY KEY,
    patch_url text NOT NULL,
    patch_date date NOT NULL,
    context_hash text NOT NULL,
    output_hash text NOT NULL,
    prompt_version text NOT NULL,
    status text NOT NULL CHECK (status IN ('hypothesis_draft','stale','rejected')),
    result jsonb NOT NULL,
    warnings jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE(context_hash,output_hash,prompt_version)
);
CREATE INDEX IF NOT EXISTS patch_insight_runs_lookup
    ON brain.patch_insight_runs(patch_url,created_at DESC,id DESC);
COMMIT;
