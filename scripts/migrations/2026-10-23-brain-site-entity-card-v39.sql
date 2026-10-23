BEGIN;

CREATE FUNCTION serve.site_entity_id(requested_key text)
RETURNS bigint
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $id$
    SELECT k.id FROM canon.entity k WHERE k.key=requested_key AND k.kind IN('hero','item','ability')
$id$;
REVOKE ALL ON FUNCTION serve.site_entity_id(text) FROM PUBLIC;

CREATE FUNCTION serve.site_entity_card(requested_key text)
RETURNS TABLE(entity_id bigint,kind text,card jsonb)
LANGUAGE plpgsql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $card$
DECLARE target bigint;
BEGIN
    SELECT k.id INTO target FROM canon.entity k
    WHERE k.key=requested_key AND k.kind IN('item','ability');
    IF target IS NULL THEN
        RETURN;
    END IF;
    RETURN QUERY
    SELECT e.entity_id,e.kind::text,
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
        )
    FROM (
        SELECT i.entity_id,i.kind,i.key,i.identity FROM serve.entity_identity i
        WHERE i.entity_id=target AND i.kind IN('item','ability')
        OFFSET 0
    ) e
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
                    CROSS JOIN LATERAL (
                        SELECT a.entity_id FROM serve.current_alias a
                        WHERE a.alias_norm=c.value_text AND a.lang='und'
                        OFFSET 0
                    ) a
                    CROSS JOIN LATERAL (
                        SELECT i.entity_id,i.identity FROM serve.entity_identity i
                        WHERE i.entity_id=a.entity_id AND i.key=c.value_text AND i.kind='item' LIMIT 1
                    ) t
                    WHERE c.entity_id=e.entity_id
                        AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
                    UNION ALL
                    SELECT 0,c.stat_key,'upgrades_to',e.entity_id,t.entity_id,t.entity_id,
                        c.receipt_id,c.source_revision,t.identity
                    FROM (
                        SELECT DISTINCT candidate.entity_id FROM canon.fact candidate
                        WHERE candidate.stat_key COLLATE "C">='component_item_' COLLATE "C"
                            AND candidate.stat_key COLLATE "C"<'component_item`' COLLATE "C"
                            AND candidate.value_text::text=e.key
                    ) candidate
                    CROSS JOIN LATERAL (
                        SELECT c.entity_id,c.stat_key,c.receipt_id,c.source_revision FROM serve.p2_component c
                        WHERE c.entity_id=candidate.entity_id AND c.value_text=e.key
                            AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
                        OFFSET 0
                    ) c
                    CROSS JOIN LATERAL (
                        SELECT i.entity_id,i.identity FROM serve.entity_identity i
                        WHERE i.entity_id=c.entity_id AND i.kind='item' LIMIT 1
                    ) t
                    WHERE e.kind='item'
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
    ) l;
END
$card$;
REVOKE ALL ON FUNCTION serve.site_entity_card(text) FROM PUBLIC;
DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.site_entity_id(text),serve.site_entity_card(text) TO brain_site;
    END IF;
END
$site$;

COMMIT;
