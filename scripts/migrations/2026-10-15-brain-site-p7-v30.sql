BEGIN;
DO $site$
BEGIN
    IF to_regclass('serve.doc_section') IS NOT NULL THEN
        EXECUTE 'CREATE VIEW serve.site_doc_section WITH (security_barrier=true) AS SELECT * FROM serve.doc_section WHERE scope=''public''';
    END IF;
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT USAGE ON SCHEMA serve TO brain_site;
        GRANT SELECT ON serve.entity_identity,serve.hero_profile,serve.item_card,serve.ability_card,serve.current_fact,serve.p2_fact,serve.published_receipt TO brain_site;
        GRANT EXECUTE ON FUNCTION serve.receipt_is_published(bigint) TO brain_site;
        IF to_regclass('serve.site_doc_section') IS NOT NULL THEN
            GRANT SELECT ON serve.site_doc_section TO brain_site;
            GRANT EXECUTE ON FUNCTION serve.doc_meta(text),serve.doc_tsv(text) TO brain_site;
        END IF;
    END IF;
END
$site$;
COMMIT;
