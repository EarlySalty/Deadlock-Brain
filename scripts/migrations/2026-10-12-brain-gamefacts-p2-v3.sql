BEGIN;
CREATE INDEX relation_to_entity ON canon.relation(to_entity,kind,from_entity);
CREATE INDEX p2_component_lookup ON canon.fact(entity_id,stat_key,valid_from_run DESC)
WHERE stat_key LIKE 'component\_item\_%' ESCAPE '\';

CREATE VIEW serve.p2_fact AS
SELECT selected.id AS entity_id,projected.stat_key,projected.fact
FROM serve.current_entity selected
CROSS JOIN LATERAL (
WITH facts AS MATERIALIZED (
    SELECT * FROM serve.current_fact WHERE entity_id=selected.id
), qualified AS (
    SELECT f.entity_id,substr(f.stat_key,23) AS stat_key,f.receipt_id,
        a.attributes-'unit'-'source_language'-'label'-'numeric' AS condition,
        a.attributes->>'source_language' AS language,1 AS priority,
        jsonb_build_object(
            'entity_id',f.entity_id,'stat_key',substr(f.stat_key,23),
            'value',CASE WHEN a.attributes->'numeric'='true'::jsonb
                AND pg_input_is_valid(f.value_text,'double precision')
                THEN to_jsonb(f.value_text::double precision)
                ELSE COALESCE(to_jsonb(f.value_num),to_jsonb(f.value_text),to_jsonb(f.value_bool)) END,
            'condition',a.attributes-'unit'-'source_language'-'label'-'numeric',
            'unit',a.attributes->'unit','label',a.attributes->'label',
            'qualifiers',a.receipts,'citation',f.receipt_id::text,
            'source',f.source,'source_revision',f.source_revision,'patch',NULL
        ) AS fact
    FROM facts f
    JOIN facts n ON n.entity_id=f.entity_id
        AND n.stat_key='p2_q_'||substr(f.stat_key,6,16)||'_count'
        AND n.raw_object_id=f.raw_object_id AND n.valid_from_run=f.valid_from_run
        AND n.source='assets'
    CROSS JOIN LATERAL (
        SELECT count(*) AS total,COALESCE(jsonb_object_agg(q.attr,q.value),'{}'::jsonb) AS attributes,
            COALESCE(jsonb_agg(jsonb_build_object(
                'key',q.attr,'value',q.value,'citation',q.receipt_id::text,
                'source_revision',q.source_revision,'patch',NULL
            ) ORDER BY q.attr),'[]'::jsonb) AS receipts
        FROM (
            SELECT substr(c.stat_key,23) AS attr,c.receipt_id,c.source_revision,
                CASE WHEN substr(c.stat_key,23) IN('upgrade_tier','upgrade_entry')
                    AND c.value_text ~ '^[0-9]{1,20}$' THEN to_jsonb(c.value_text::numeric)
                    ELSE COALESCE(to_jsonb(c.value_num),to_jsonb(c.value_text),to_jsonb(c.value_bool)) END AS value
            FROM facts c
            WHERE c.entity_id=f.entity_id AND c.raw_object_id=f.raw_object_id
                AND c.valid_from_run=f.valid_from_run AND c.source='assets'
                AND left(c.stat_key,22)='p2_q_'||substr(f.stat_key,6,16)||'_'
                AND c.stat_key<>n.stat_key
            ORDER BY c.stat_key LIMIT {{ROWS}}
        ) q
    ) a
    JOIN facts root ON root.entity_id=f.entity_id
        AND root.stat_key='identity_key_'||(a.attributes->>'source_language')
        AND root.raw_object_id=f.raw_object_id AND root.valid_from_run=f.valid_from_run
    WHERE f.source='assets' AND f.stat_key ~ '^p2_v_[0-9a-f]{16}_[a-z][a-z0-9_]*$'
        AND n.value_text=a.total::text
), rows AS (
    SELECT * FROM qualified
    UNION ALL
    SELECT f.entity_id,f.stat_key,f.receipt_id,'{}'::jsonb,NULL,
        CASE WHEN f.source IN('deadlock-data','gametracking') AND f.value_num IS NOT NULL THEN 0 ELSE 2 END,
        jsonb_build_object(
            'entity_id',f.entity_id,'stat_key',f.stat_key,
            'value',COALESCE(to_jsonb(f.value_num),to_jsonb(f.value_text),to_jsonb(f.value_bool)),
            'condition','{}'::jsonb,'citation',f.receipt_id::text,'source',f.source,
            'source_revision',f.source_revision,'patch',NULL
        )
    FROM facts f
    WHERE f.stat_key NOT LIKE 'p2\_%' ESCAPE '\'
        AND f.stat_key NOT LIKE 'component\_item\_%' ESCAPE '\'
), ranked AS (
    SELECT *,min(priority) OVER(PARTITION BY entity_id,stat_key,condition) AS chosen FROM rows
)
SELECT DISTINCT ON(entity_id,stat_key,condition) entity_id,stat_key,fact
FROM ranked WHERE priority=chosen
ORDER BY entity_id,stat_key,condition,language='en' DESC NULLS LAST,receipt_id DESC
) projected
WHERE selected.kind IN('hero','ability','item');

CREATE VIEW serve.p2_component AS
SELECT DISTINCT ON(c.entity_id,c.value_text)
    c.entity_id,c.stat_key,c.value_text,c.receipt_id,c.source_revision
FROM serve.current_fact c
WHERE c.stat_key LIKE 'component\_item\_%' ESCAPE '\' AND c.value_text IS NOT NULL
    AND (
        EXISTS(SELECT 1 FROM serve.current_fact root WHERE root.entity_id=c.entity_id
            AND root.stat_key='identity_key_'||split_part(c.stat_key,'_',3)
            AND root.raw_object_id=c.raw_object_id AND root.valid_from_run=c.valid_from_run)
        OR (c.stat_key !~ '^component_item_(en|de)_[0-9]+$'
            AND NOT EXISTS(SELECT 1 FROM serve.current_fact root WHERE root.entity_id=c.entity_id
                AND root.stat_key IN('identity_key_en','identity_key_de')))
    )
ORDER BY c.entity_id,c.value_text,c.stat_key LIKE 'component\_item\_en\_%' ESCAPE '\' DESC,c.receipt_id DESC;

CREATE VIEW serve.entity_card AS
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
                        THEN r.to_entity ELSE r.from_entity END LIMIT 1
                ) t
                WHERE r.from_entity=e.entity_id OR r.to_entity=e.entity_id
            ) x
        ) q
    ) r
) l
WHERE e.kind IN('item','ability');

CREATE VIEW serve.item_card AS
SELECT entity_id,card FROM serve.entity_card WHERE kind='item';

CREATE VIEW serve.ability_card AS
SELECT entity_id,card||jsonb_build_object('hero_selection',jsonb_build_object(
    'available',false,'reason','hero_slot_not_published'
)) AS card
FROM serve.entity_card WHERE kind='ability';

GRANT SELECT ON serve.p2_fact,serve.p2_component,serve.entity_card,serve.item_card,serve.ability_card TO brain_tool,brain_service;
COMMIT;
