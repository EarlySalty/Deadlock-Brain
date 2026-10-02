-- Unveränderliche Enqueue-Anfrage getrennt von später freigegebenen effektiven Jobrechten.
BEGIN;
ALTER TABLE brain.maintenance_jobs_v1 ADD COLUMN IF NOT EXISTS enqueue_spec_json jsonb;
UPDATE brain.maintenance_jobs_v1 SET enqueue_spec_json=spec_json WHERE enqueue_spec_json IS NULL;
ALTER TABLE brain.maintenance_jobs_v1 ALTER COLUMN enqueue_spec_json SET NOT NULL;
COMMENT ON COLUMN brain.maintenance_jobs_v1.enqueue_spec_json IS 'Unveränderliche Enqueue-Anfrage für Replayprüfung. spec_json trägt die aktuell autorisierte Dokumentpolicy.';
COMMIT;
