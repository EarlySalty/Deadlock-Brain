\set ON_ERROR_STOP on
BEGIN;
DO $role$
BEGIN
    IF NOT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        CREATE ROLE brain_site LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS NOINHERIT CONNECTION LIMIT 4;
    ELSIF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site'
        AND (NOT rolcanlogin OR rolsuper OR rolcreatedb OR rolcreaterole OR rolreplication OR rolbypassrls OR rolinherit))
        OR EXISTS(SELECT 1 FROM pg_auth_members WHERE member=(SELECT oid FROM pg_roles WHERE rolname='brain_site')) THEN
        RAISE EXCEPTION 'brain_site ist keine isolierte Login-Rolle';
    END IF;
END
$role$;
ALTER ROLE brain_site SET default_transaction_read_only=on;
ALTER ROLE brain_site SET statement_timeout='500ms';
GRANT CONNECT ON DATABASE brain TO brain_site;
DO $revoke$
DECLARE schema_name text;
BEGIN
    FOR schema_name IN SELECT nspname FROM pg_namespace WHERE nspname IN('brain','canon','raw','ops','serve','docs') LOOP
        EXECUTE format('REVOKE ALL ON SCHEMA %I FROM brain_site',schema_name);
        EXECUTE format('REVOKE ALL ON ALL TABLES IN SCHEMA %I FROM brain_site',schema_name);
        EXECUTE format('REVOKE ALL ON ALL SEQUENCES IN SCHEMA %I FROM brain_site',schema_name);
        EXECUTE format('REVOKE ALL ON ALL FUNCTIONS IN SCHEMA %I FROM brain_site',schema_name);
    END LOOP;
END
$revoke$;
DO $serve$
BEGIN
    IF to_regclass('serve.hero_profile') IS NULL OR to_regprocedure('serve.receipt_is_published(bigint)') IS NULL THEN
        RAISE EXCEPTION 'brain_site benötigt zuerst die Spielwerte-Migrationen';
    END IF;
    GRANT USAGE ON SCHEMA serve TO brain_site;
    GRANT SELECT ON serve.entity_identity,serve.hero_profile,serve.item_card,serve.ability_card,serve.current_fact,serve.p2_fact,serve.published_receipt TO brain_site;
    GRANT EXECUTE ON FUNCTION serve.receipt_is_published(bigint) TO brain_site;
    IF to_regprocedure('serve.read_compare_artifact_v1(text)') IS NOT NULL AND to_regprocedure('serve.read_compare_artifact_v2(text)') IS NOT NULL THEN
        GRANT EXECUTE ON FUNCTION serve.read_compare_artifact_v1(text), serve.read_compare_artifact_v2(text) TO brain_site;
    END IF;
    IF to_regclass('serve.site_doc_section') IS NOT NULL THEN
        GRANT SELECT ON serve.site_doc_section TO brain_site;
        GRANT EXECUTE ON FUNCTION serve.doc_meta(text),serve.doc_tsv(text) TO brain_site;
    END IF;
    IF to_regprocedure('serve.site_doc_page(text)') IS NOT NULL THEN
        GRANT EXECUTE ON FUNCTION serve.site_doc_page(text) TO brain_site;
    END IF;
END
$serve$;
COMMIT;
