BEGIN;
CREATE TABLE IF NOT EXISTS brain.response_deviations_v1 (
    audit_id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    request_id text NOT NULL CHECK (length(request_id) BETWEEN 1 AND 1024),
    model text NOT NULL CHECK (length(model) BETWEEN 1 AND 1024),
    check_json jsonb NOT NULL CHECK (jsonb_typeof(check_json) = 'object'),
    raw_output bytea NOT NULL CHECK (octet_length(raw_output) <= 16777216),
    raw_output_complete boolean NOT NULL,
    source_ids jsonb NOT NULL CHECK (jsonb_typeof(source_ids) = 'array'),
    evidence_ids jsonb NOT NULL CHECK (jsonb_typeof(evidence_ids) = 'array'),
    disposition text NOT NULL CHECK (disposition IN ('rejected', 'unchecked_returned')),
    identifiers_redacted boolean NOT NULL
);
CREATE INDEX IF NOT EXISTS response_deviations_request_v1 ON brain.response_deviations_v1(request_id, recorded_at);
REVOKE ALL ON brain.response_deviations_v1 FROM PUBLIC;
DO $$
DECLARE role_name text;
BEGIN
    FOREACH role_name IN ARRAY ARRAY['brain_service', 'brain_readonly', 'brain_ingest'] LOOP
        IF EXISTS (SELECT FROM pg_roles WHERE rolname = role_name) THEN
            EXECUTE format('REVOKE ALL ON brain.response_deviations_v1 FROM %I', role_name);
        END IF;
    END LOOP;
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'brain_service') THEN
        GRANT INSERT ON brain.response_deviations_v1 TO brain_service;
        GRANT SELECT(audit_id) ON brain.response_deviations_v1 TO brain_service;
    END IF;
END $$;
COMMIT;
