\set ON_ERROR_STOP on
BEGIN;
CREATE ROLE brain_site LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT CONNECTION LIMIT 4;
GRANT CONNECT ON DATABASE brain TO brain_site;
GRANT USAGE ON SCHEMA brain TO brain_site;
DO $grants$
BEGIN
    IF to_regclass('brain.site_comments_v1') IS NOT NULL THEN
        GRANT SELECT ON brain.site_comments_v1 TO brain_site;
    END IF;
    IF to_regprocedure('brain.read_compare_artifact_v1(text)') IS NOT NULL THEN
        GRANT EXECUTE ON FUNCTION brain.read_compare_artifact_v1(text) TO brain_site;
    END IF;
    IF to_regprocedure('brain.read_site_profile_bindings_v1(text,text)') IS NOT NULL THEN
        GRANT EXECUTE ON FUNCTION brain.read_site_profile_bindings_v1(text,text) TO brain_site;
    END IF;
    IF to_regprocedure('brain.append_site_comment_v1(text,text,text)') IS NOT NULL THEN
        GRANT EXECUTE ON FUNCTION brain.append_site_comment_v1(text,text,text) TO brain_site;
    END IF;
END
$grants$;
COMMIT;
