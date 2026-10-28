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
    IF EXISTS (SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
        WHERE n.nspname='brain' AND c.relname='site_comments_v1' AND c.relkind IN ('r','p')) THEN
        EXECUTE 'DROP TABLE brain.site_comments_v1';
    END IF;
    IF EXISTS (SELECT 1 FROM pg_namespace WHERE nspname='brain_legacy') THEN
        EXECUTE 'DROP SCHEMA brain_legacy CASCADE';
    END IF;
END
$legacy_drop$;
COMMIT;
