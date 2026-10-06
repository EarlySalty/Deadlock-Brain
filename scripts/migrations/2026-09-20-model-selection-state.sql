-- Runtime verification state, not a second configuration store.
-- Apply explicitly through the approved PostgreSQL deployment path, not at startup.
BEGIN;
CREATE TABLE IF NOT EXISTS brain.model_selection_state (
    policy_fingerprint text PRIMARY KEY CHECK (policy_fingerprint ~ '^[0-9a-f]{64}$'),
    state jsonb NOT NULL CHECK (jsonb_typeof(state) = 'object'),
    updated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);
COMMENT ON TABLE brain.model_selection_state IS
    'Brain verified-model status only: policy digest, model, probe time, expiry and fixed error codes. No prompts, credentials or configuration values.';
COMMIT;
