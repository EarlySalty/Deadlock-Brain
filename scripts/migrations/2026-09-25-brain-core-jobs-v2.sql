-- Explicit opt-in migration. Never invoked by an API constructor or read path.
BEGIN;
CREATE SCHEMA IF NOT EXISTS brain;
CREATE TABLE IF NOT EXISTS brain.source_jobs_v1 (
    source_id text PRIMARY KEY CHECK (length(source_id) > 0),
    owner text NOT NULL CHECK (length(owner) > 0),
    fence bigint NOT NULL CHECK (fence > 0),
    lease_until timestamptz NOT NULL,
    state text NOT NULL CHECK (state IN ('running', 'idle')),
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS brain.source_checkpoints_v1 (
    source_id text PRIMARY KEY,
    configuration text NOT NULL CHECK (length(configuration) > 0),
    generation bigint NOT NULL CHECK (generation > 0),
    checkpoint_json jsonb NOT NULL,
    batch_json jsonb NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT now()
);
CREATE TABLE IF NOT EXISTS brain.conversation_owners_v1 (
    conversation_id text PRIMARY KEY,
    actor_id text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS source_jobs_v1_expired_idx ON brain.source_jobs_v1 (lease_until) WHERE state = 'running';
COMMENT ON TABLE brain.source_checkpoints_v1 IS 'Records and checkpoint advance in the same fenced transaction; batch_json identifies an idempotent replay.';
COMMIT;
