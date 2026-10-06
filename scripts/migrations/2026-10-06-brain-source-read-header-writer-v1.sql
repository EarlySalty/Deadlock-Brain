BEGIN;

-- Nur der bestehende Writer darf die atomischen Headerprojektionen berechnen.
REVOKE ALL ON FUNCTION brain.source_read_header_v1(jsonb),
    brain.source_original_header_v1(jsonb) FROM PUBLIC;
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'brain_ingest') THEN
        GRANT EXECUTE ON FUNCTION brain.source_read_header_v1(jsonb),
            brain.source_original_header_v1(jsonb) TO brain_ingest;
    END IF;
END $$;

COMMIT;
