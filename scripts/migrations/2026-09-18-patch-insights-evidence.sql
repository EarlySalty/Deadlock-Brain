BEGIN;
SET LOCAL lock_timeout = '5s';

CREATE TABLE IF NOT EXISTS brain.youtube_transcript_evidence (
    video_id text NOT NULL REFERENCES brain.youtube_videos(video_id) ON DELETE CASCADE,
    evidence_hash text NOT NULL,
    transcript_hash text NOT NULL,
    language text NOT NULL,
    source_kind text NOT NULL,
    raw_caption jsonb NOT NULL,
    segments jsonb NOT NULL CHECK (jsonb_typeof(segments) = 'array'),
    timing_status text NOT NULL CHECK (timing_status IN ('available','partial','missing')),
    parser_version text NOT NULL,
    imported_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY(video_id,evidence_hash)
);
CREATE INDEX IF NOT EXISTS youtube_transcript_evidence_lookup
    ON brain.youtube_transcript_evidence(video_id,transcript_hash,imported_at DESC);

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
