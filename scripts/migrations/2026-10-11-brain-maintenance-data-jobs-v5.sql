BEGIN;
CREATE TABLE IF NOT EXISTS brain.maintenance_data_jobs_v1 (
    id text PRIMARY KEY CHECK (id IN ('assets_build_data','sheet_sync')),
    fence bigint NOT NULL DEFAULT 0 CHECK (fence >= 0),
    lease_until timestamptz,
    available_at timestamptz NOT NULL DEFAULT now(),
    last_started_at timestamptz,
    last_success_at timestamptz,
    last_finished_at timestamptz,
    last_error_code text,
    result_json jsonb,
    updated_at timestamptz NOT NULL DEFAULT now()
);
DO $$
DECLARE
    entity_table text;
    entity_sequence text;
BEGIN
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'brain_ingest') THEN
        GRANT SELECT, INSERT, UPDATE ON brain.maintenance_data_jobs_v1 TO brain_ingest;
        IF to_regclass('brain.entities') IS NOT NULL THEN
            GRANT SELECT, INSERT, UPDATE ON brain.entities TO brain_ingest;
        END IF;
        IF to_regclass('brain.entity_aliases') IS NOT NULL THEN
            GRANT SELECT, INSERT ON brain.entity_aliases TO brain_ingest;
        END IF;
        FOREACH entity_table IN ARRAY ARRAY['brain.entities', 'brain.entity_aliases'] LOOP
            SELECT pg_get_serial_sequence(entity_table, 'id') INTO entity_sequence
            FROM pg_attribute WHERE attrelid = to_regclass(entity_table)
                AND attname = 'id' AND attnum > 0 AND NOT attisdropped;
            IF entity_sequence IS NOT NULL THEN
                EXECUTE format('GRANT USAGE ON SEQUENCE %s TO brain_ingest', entity_sequence);
            END IF;
        END LOOP;
    END IF;
    IF EXISTS (SELECT FROM pg_roles WHERE rolname = 'brain_readonly') THEN
        GRANT SELECT ON brain.maintenance_data_jobs_v1 TO brain_readonly;
    END IF;
END $$;
COMMIT;
