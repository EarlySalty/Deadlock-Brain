BEGIN;
ALTER TABLE brain.maintenance_jobs_v1 DROP CONSTRAINT IF EXISTS maintenance_jobs_v1_status_check;
ALTER TABLE brain.maintenance_jobs_v1 ADD CONSTRAINT maintenance_jobs_v1_status_check
CHECK (status IN ('planned','discovered_policy_pending','source_review','author','reviewer','publish','activated','no_change','failed','superseded'));
COMMIT;
