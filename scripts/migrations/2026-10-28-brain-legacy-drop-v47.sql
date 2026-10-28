BEGIN;
SET LOCAL lock_timeout='5000ms';
DO $legacy_drop$
DECLARE
    statement text;
BEGIN
    FOR statement IN
        SELECT format('DROP FUNCTION %I.%I(%s)', n.nspname, p.proname, pg_get_function_identity_arguments(p.oid))
        FROM pg_proc p JOIN pg_namespace n ON n.oid=p.pronamespace
        WHERE n.nspname='brain' AND p.proname IN ('append_site_comment_v1','read_site_profile_bindings_v1')
    LOOP
        EXECUTE statement;
    END LOOP;
    IF EXISTS (
        SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        WHERE n.nspname='brain' AND c.relname='patch_changes' AND c.relkind='v'
    ) THEN
        EXECUTE 'DROP VIEW brain.patch_changes';
    END IF;
    SELECT 'DROP TABLE ' || string_agg(format('%I.%I', n.nspname, c.relname), ', ')
    INTO statement
    FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
    WHERE n.nspname='brain' AND c.relkind IN ('r','p') AND c.relname IN (
        'entity_derived_receipts_v1','entity_patch_intervals_v1','entity_semantic_projections_v1',
        'entity_profile_facts_v1','entity_profile_entities_v1','entity_snapshots','entity_aliases',
        'entities','patch_event_enrichments','patch_events','source_documents','source_runs',
        'maintenance_jobs_v1','maintenance_sources_v1','site_comments_v1'
    );
    IF statement IS NOT NULL THEN
        EXECUTE statement;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_namespace WHERE nspname='brain_legacy') THEN
        EXECUTE 'DROP SCHEMA brain_legacy CASCADE';
    END IF;
END
$legacy_drop$;
COMMIT;
