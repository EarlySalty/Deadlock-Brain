BEGIN;

CREATE FUNCTION serve.hero_is_playable(hero_entity bigint) RETURNS boolean
LANGUAGE plpgsql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $playable$
DECLARE
    hero_object bigint;
    hero_revision text;
BEGIN
    SELECT f.raw_object_id,f.source_revision INTO hero_object,hero_revision
    FROM canon.entity e
    CROSS JOIN LATERAL (
        SELECT f.raw_object_id,f.source_revision FROM serve.current_fact f
        WHERE f.entity_id=e.id AND f.stat_key='p2_slot_1_hero' AND f.source='assets'
            AND f.value_text::text=e.key
        OFFSET 0
    ) f
    WHERE e.id=hero_entity AND e.kind='hero';
    IF hero_object IS NULL THEN
        RETURN false;
    END IF;
    IF hero_revision IS DISTINCT FROM (
        SELECT o.revision FROM canon.receipt r
        JOIN raw.object o ON o.id=r.raw_object_id
        WHERE r.extractor='v1:17:assets_hero_slots:1:1' AND r.locator_kind='raw_bytes' AND r.locator=''
            AND o.source='assets' AND o.path='heroes/en/observed/complete.snapshot'
            AND r.value_sha256=o.content_sha256
            AND EXISTS(SELECT 1 FROM serve.published_receipt p WHERE p.id=r.id)
        ORDER BY r.ingest_run_id DESC,r.id DESC
        LIMIT 1
    ) THEN
        RETURN false;
    END IF;
    RETURN EXISTS(
        SELECT 1 FROM (
            SELECT convert_from(o.bytes,'UTF8')::jsonb AS hero FROM raw.object o
            WHERE o.id=hero_object AND o.source='assets'
            OFFSET 0
        ) o
        WHERE o.hero->'player_selectable'='true'::jsonb
            AND o.hero->'disabled'='false'::jsonb
    );
END
$playable$;

REVOKE ALL ON FUNCTION serve.hero_is_playable(bigint) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION serve.hero_is_playable(bigint) TO brain_tool,brain_service;
DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.hero_is_playable(bigint) TO brain_site;
    END IF;
END
$site$;

COMMIT;
