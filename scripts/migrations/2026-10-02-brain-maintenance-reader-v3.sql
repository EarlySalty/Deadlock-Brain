-- Bestehende Dienstrollen erhalten nur die notwendige Registry-Sicht.
BEGIN;
DO $body$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_ingest') THEN
        GRANT SELECT,INSERT,UPDATE ON brain.maintenance_jobs_v1,brain.maintenance_sources_v1 TO brain_ingest;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_service') THEN
        GRANT SELECT ON brain.maintenance_sources_v1 TO brain_service;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_readonly') THEN
        GRANT SELECT ON brain.maintenance_sources_v1 TO brain_readonly;
    END IF;
END
$body$;
COMMIT;
