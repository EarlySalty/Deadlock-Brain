\set ON_ERROR_STOP on
\connect brain
BEGIN;
REVOKE ALL ON ALL TABLES IN SCHEMA brain FROM brain_ingest, brain_service, brain_readonly;
REVOKE ALL ON ALL SEQUENCES IN SCHEMA brain FROM brain_ingest, brain_service, brain_readonly;
REVOKE ALL ON ALL FUNCTIONS IN SCHEMA brain FROM PUBLIC;
GRANT USAGE ON SCHEMA brain TO brain_ingest, brain_service, brain_readonly;

GRANT SELECT ON ALL TABLES IN SCHEMA brain TO brain_readonly;

GRANT SELECT ON brain.core_schema_version, brain.corpus_releases_v1,
    brain.source_record_revisions, brain.source_record_heads,
    brain.source_jobs_v1, brain.source_checkpoints_v1 TO brain_ingest;
GRANT INSERT ON brain.source_record_revisions, brain.corpus_releases_v1 TO brain_ingest;
GRANT INSERT, UPDATE ON brain.source_record_heads, brain.source_jobs_v1,
    brain.source_checkpoints_v1 TO brain_ingest;

GRANT SELECT ON brain.core_schema_version, brain.corpus_releases_v1,
    brain.source_record_revisions, brain.source_record_heads,
    brain.source_jobs_v1, brain.source_checkpoints_v1,
    brain.conversation_owners_v1 TO brain_service;
GRANT INSERT ON brain.conversation_owners_v1 TO brain_service;

DO $$
BEGIN
  IF EXISTS (SELECT FROM pg_namespace WHERE nspname = 'brain_legacy') THEN
    EXECUTE 'REVOKE ALL ON SCHEMA brain_legacy FROM PUBLIC';
    EXECUTE 'GRANT USAGE ON SCHEMA brain_legacy TO brain_readonly';
    EXECUTE 'GRANT SELECT ON ALL TABLES IN SCHEMA brain_legacy TO brain_readonly';
  END IF;
END $$;
COMMIT;
