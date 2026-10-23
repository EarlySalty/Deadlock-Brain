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
        ORDER BY f.receipt_id DESC
        LIMIT 1
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
            AND (o.hero->'prerelease_only' IS NULL OR o.hero->'prerelease_only'='false'::jsonb)
            AND (o.hero->'assigned_players_only' IS NULL OR o.hero->'assigned_players_only'='false'::jsonb)
            AND (o.hero->'in_hero_labs' IS NULL OR o.hero->'in_hero_labs'='false'::jsonb)
            AND (o.hero->'InHeroLabs' IS NULL OR o.hero->'InHeroLabs'='false'::jsonb)
    );
END
$playable$;

CREATE FUNCTION serve.hero_assets_game_id(hero_entity bigint)
RETURNS TABLE(receipt_id bigint,source_revision text,game_id text)
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $assets$
    SELECT f.receipt_id::bigint,f.source_revision::text,o.hero->>'id'
    FROM canon.entity e
    CROSS JOIN LATERAL (
        SELECT f.receipt_id,f.raw_object_id,f.source_revision FROM serve.current_fact f
        WHERE f.entity_id=e.id AND f.stat_key='p2_slot_1_hero' AND f.source='assets'
            AND f.value_text::text=e.key
        ORDER BY f.receipt_id DESC
        LIMIT 1
    ) f
    CROSS JOIN LATERAL (
        SELECT convert_from(o.bytes,'UTF8')::jsonb AS hero FROM raw.object o
        WHERE o.id=f.raw_object_id AND o.source='assets'
        OFFSET 0
    ) o
    WHERE e.id=hero_entity AND e.kind='hero' AND jsonb_typeof(o.hero->'id')='number'
        AND o.hero->>'id' ~ '^[1-9][0-9]{0,18}$'
$assets$;

CREATE FUNCTION serve.hero_game_identity(hero_entity bigint) RETURNS jsonb
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $identity$
    SELECT jsonb_build_object(
        'stat_key','identity_game_id','value',a.game_id,'condition','{}'::jsonb,
        'citation',a.receipt_id::text,'source_revision',a.source_revision,'patch',NULL
    )
    FROM (SELECT * FROM serve.hero_assets_game_id(hero_entity) OFFSET 0) a
    WHERE serve.hero_is_playable(hero_entity)
    LIMIT 1
$identity$;

CREATE FUNCTION serve.hero_game_id_is_playable(game_id text) RETURNS boolean
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $game$
    SELECT EXISTS(
        SELECT 1 FROM (
            SELECT e.id FROM canon.entity e
            CROSS JOIN LATERAL serve.hero_assets_game_id(e.id) a
            WHERE e.kind='hero' AND a.game_id=hero_game_id_is_playable.game_id
            OFFSET 0
        ) h
        WHERE serve.hero_is_playable(h.id)
    )
$game$;

CREATE OR REPLACE VIEW serve.entity_card AS
SELECT e.entity_id,e.kind,
    jsonb_build_object(
        'identity',e.identity,
        'facts',jsonb_build_object(
            'facts',COALESCE(f.rows,'[]'::jsonb),
            'truncated',COALESCE(f.total,0)>f.shown,
            'omitted_count',COALESCE(f.total,0)-f.shown
        ),
        'links',jsonb_build_object(
            'links',COALESCE(l.rows,'[]'::jsonb),
            'truncated',COALESCE(l.total,0)>l.shown,
            'omitted_count',COALESCE(l.total,0)-l.shown
        )
    ) AS card
FROM serve.entity_identity e
CROSS JOIN LATERAL (
    SELECT max(r.total) AS total,count(*) FILTER(WHERE r.kept) AS shown,
        jsonb_agg(r.fact ORDER BY r.ord) FILTER(WHERE r.kept) AS rows
    FROM (
        SELECT q.fact,q.ord,count(*) OVER() AS total,
            q.ord<=32 AND sum(q.result_bytes) OVER(ORDER BY q.ord)<=10240 AS kept
        FROM (
            SELECT row_number() OVER(
                    ORDER BY CASE
                        WHEN c.stat_key='item_cost' THEN 0
                        WHEN c.stat_key LIKE '%\_description\_%' ESCAPE '\'
                            AND c.fact->>'value'<>'' THEN 1
                        WHEN c.stat_key LIKE 'identity\_%' ESCAPE '\' THEN 5
                        WHEN NULLIF(c.fact->>'label','') IS NOT NULL
                            AND c.fact->'value' NOT IN('0'::jsonb,'""'::jsonb,'false'::jsonb,'null'::jsonb) THEN
                            CASE WHEN c.fact->'condition'->>'usage_flag_0'='ConditionallyApplied'
                                OR c.fact->'condition'->>'scale_class' LIKE '%damage%' THEN 2 ELSE 3 END
                        ELSE 4 END,c.stat_key,c.fact::text
                ) AS ord,
                c.fact,octet_length(c.fact::text)+187-octet_length(c.fact->>'citation')
                    +COALESCE((SELECT sum(187-octet_length(q->>'citation'))
                        FROM jsonb_array_elements(c.fact->'qualifiers') q),0) AS result_bytes
            FROM serve.p2_fact c
            WHERE c.entity_id=e.entity_id
        ) q
    ) r
) f
CROSS JOIN LATERAL (
    SELECT max(r.total) AS total,count(*) FILTER(WHERE r.kept) AS shown,
        jsonb_agg(r.link ORDER BY r.ord) FILTER(WHERE r.kept) AS rows
    FROM (
        SELECT q.link,q.ord,count(*) OVER() AS total,
            q.ord<=16 AND sum(octet_length(q.link::text)) OVER(ORDER BY q.ord)<=2560 AS kept
        FROM (
            SELECT row_number() OVER(ORDER BY x.grp,length(x.sort),x.sort,x.receipt_id) AS ord,
                jsonb_build_object(
                    'entity_id',x.entity_id,'kind',x.kind,
                    'from_entity_id',x.from_id,'to_entity_id',x.to_id,
                    'citation',x.receipt_id::text,'source_revision',x.source_revision,
                    'patch',NULL,'identity',x.identity
                ) AS link
            FROM (
                SELECT 0 AS grp,c.stat_key AS sort,'component_of' AS kind,
                    t.entity_id AS from_id,c.entity_id AS to_id,t.entity_id,
                    c.receipt_id,c.source_revision,t.identity
                FROM serve.p2_component c
                JOIN serve.current_alias a ON a.alias_norm=c.value_text AND a.lang='und'
                CROSS JOIN LATERAL (
                    SELECT entity_id,identity FROM serve.entity_identity
                    WHERE entity_id=a.entity_id AND key=c.value_text AND kind='item' LIMIT 1
                ) t
                WHERE c.entity_id=e.entity_id
                    AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
                UNION ALL
                SELECT 0,c.stat_key,'upgrades_to',e.entity_id,t.entity_id,t.entity_id,
                    c.receipt_id,c.source_revision,t.identity
                FROM serve.p2_component c
                CROSS JOIN LATERAL (
                    SELECT entity_id,identity FROM serve.entity_identity
                    WHERE entity_id=c.entity_id AND kind='item' LIMIT 1
                ) t
                WHERE c.value_text=e.key AND e.kind='item'
                    AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
                UNION ALL
                SELECT 1,'',r.kind,r.from_entity,r.to_entity,t.entity_id,
                    r.receipt_id,r.source_revision,t.identity
                FROM serve.current_relation r
                CROSS JOIN LATERAL (
                    SELECT i.entity_id,i.identity FROM serve.entity_identity i
                    WHERE i.entity_id=CASE WHEN r.from_entity=e.entity_id
                        THEN r.to_entity ELSE r.from_entity END
                        AND (i.kind<>'hero' OR serve.hero_is_playable(i.entity_id)) LIMIT 1
                ) t
                WHERE r.from_entity=e.entity_id OR r.to_entity=e.entity_id
            ) x
        ) q
    ) r
) l
WHERE e.kind IN('item','ability');

REVOKE ALL ON FUNCTION serve.hero_is_playable(bigint),serve.hero_assets_game_id(bigint),
    serve.hero_game_identity(bigint),serve.hero_game_id_is_playable(text) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION serve.hero_is_playable(bigint),serve.hero_game_identity(bigint),
    serve.hero_game_id_is_playable(text) TO brain_tool,brain_service;
DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.hero_is_playable(bigint) TO brain_site;
    END IF;
END
$site$;

COMMIT;
