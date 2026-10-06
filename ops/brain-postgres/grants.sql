\set ON_ERROR_STOP on
\connect :"db"
BEGIN;
REVOKE ALL ON ALL TABLES IN SCHEMA brain FROM brain_ingest, brain_service, brain_readonly, brain_site;
REVOKE ALL ON ALL SEQUENCES IN SCHEMA brain FROM brain_ingest, brain_service, brain_readonly, brain_site;
REVOKE ALL ON ALL FUNCTIONS IN SCHEMA brain FROM PUBLIC;
GRANT USAGE ON SCHEMA brain TO brain_ingest, brain_service, brain_readonly, brain_site;

GRANT SELECT ON ALL TABLES IN SCHEMA brain TO brain_readonly;

GRANT SELECT ON brain.core_schema_version, brain.corpus_releases_v1,
    brain.source_record_revisions, brain.source_record_heads,
    brain.source_jobs_v1, brain.source_checkpoints_v1,
    brain.conversation_owners_v1 TO brain_ingest;
GRANT INSERT ON brain.source_record_revisions, brain.corpus_releases_v1 TO brain_ingest;
GRANT INSERT, UPDATE ON brain.source_record_heads, brain.source_jobs_v1,
    brain.source_checkpoints_v1 TO brain_ingest;

GRANT SELECT ON brain.core_schema_version, brain.corpus_releases_v1,
    brain.source_record_revisions, brain.source_record_heads,
    brain.source_jobs_v1, brain.source_checkpoints_v1,
    brain.conversation_owners_v1 TO brain_service;
GRANT INSERT ON brain.conversation_owners_v1 TO brain_service;

DO $$
DECLARE
  patch_table text;
  patch_sequence text;
BEGIN
  IF to_regclass('brain.site_comments_v1') IS NOT NULL THEN
    GRANT SELECT, INSERT ON brain.site_comments_v1 TO brain_site;
    GRANT USAGE ON SEQUENCE brain.site_comments_v1_id_seq TO brain_site;
    REVOKE ALL ON brain.site_comments_v1 FROM brain_readonly;
  END IF;
  IF to_regprocedure('brain.source_read_header_v1(jsonb)') IS NOT NULL THEN
    GRANT EXECUTE ON FUNCTION brain.source_read_header_v1(jsonb) TO brain_ingest;
  END IF;
  IF to_regprocedure('brain.source_original_header_v1(jsonb)') IS NOT NULL THEN
    GRANT EXECUTE ON FUNCTION brain.source_original_header_v1(jsonb) TO brain_ingest;
  END IF;
  IF to_regclass('patchnotes.changelog_posts') IS NOT NULL THEN
    GRANT USAGE ON SCHEMA patchnotes TO brain_ingest;
    GRANT SELECT ON patchnotes.changelog_posts TO brain_ingest;
  END IF;
  IF to_regclass('brain.source_runs') IS NOT NULL THEN
    GRANT SELECT (id), INSERT (source, status, started_at),
      UPDATE (status, finished_at, summary) ON brain.source_runs TO brain_ingest;
  END IF;
  IF to_regclass('brain.source_documents') IS NOT NULL THEN
    GRANT SELECT, INSERT ON brain.source_documents TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_snapshots') IS NOT NULL THEN
    GRANT SELECT, INSERT ON brain.entity_snapshots TO brain_ingest;
  END IF;
  IF to_regclass('brain.patch_events') IS NOT NULL THEN
    GRANT SELECT, INSERT ON brain.patch_events TO brain_ingest;
  END IF;
  FOREACH patch_table IN ARRAY ARRAY[
    'brain.source_runs', 'brain.source_documents',
    'brain.entity_snapshots', 'brain.patch_events'
  ] LOOP
    IF to_regclass(patch_table) IS NOT NULL THEN
      patch_sequence := pg_get_serial_sequence(patch_table, 'id');
      IF patch_sequence IS NOT NULL THEN
        EXECUTE format('GRANT USAGE ON SEQUENCE %s TO brain_ingest', patch_sequence);
      END IF;
    END IF;
  END LOOP;
  IF to_regclass('brain.entities') IS NOT NULL THEN
    GRANT SELECT ON brain.entities TO brain_ingest, brain_service;
  END IF;
  IF to_regclass('brain.entity_aliases') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_aliases TO brain_ingest, brain_service;
  END IF;
  IF to_regclass('brain.patch_changes') IS NOT NULL THEN
    GRANT SELECT ON brain.patch_changes TO brain_ingest, brain_service;
  END IF;
  IF to_regclass('brain.maintenance_sources_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.maintenance_sources_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT, UPDATE ON brain.maintenance_sources_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.maintenance_jobs_v1') IS NOT NULL THEN
    GRANT SELECT, INSERT, UPDATE ON brain.maintenance_jobs_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_profile_entities_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_profile_entities_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT, UPDATE ON brain.entity_profile_entities_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_profile_facts_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_profile_facts_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT, UPDATE ON brain.entity_profile_facts_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_semantic_projections_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_semantic_projections_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT ON brain.entity_semantic_projections_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_patch_intervals_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_patch_intervals_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT, UPDATE ON brain.entity_patch_intervals_v1 TO brain_ingest;
  END IF;
  IF to_regclass('brain.entity_derived_receipts_v1') IS NOT NULL THEN
    GRANT SELECT ON brain.entity_derived_receipts_v1 TO brain_service, brain_readonly;
    GRANT SELECT, INSERT ON brain.entity_derived_receipts_v1 TO brain_ingest;
  END IF;
  IF EXISTS (SELECT FROM pg_namespace WHERE nspname = 'brain_legacy') THEN
    EXECUTE 'REVOKE ALL ON SCHEMA brain_legacy FROM PUBLIC';
    EXECUTE 'GRANT USAGE ON SCHEMA brain_legacy TO brain_readonly';
    EXECUTE 'GRANT SELECT ON ALL TABLES IN SCHEMA brain_legacy TO brain_readonly';
  END IF;
END $$;
COMMIT;
