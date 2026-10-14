BEGIN;
CREATE INDEX p2_hero_source_objects ON raw.object(ingest_run_id DESC,id DESC)
WHERE source='assets' AND path LIKE 'heroes/en/%';
CREATE INDEX p2_slot_lookup ON canon.fact(entity_id,stat_key,valid_from_run DESC)
WHERE stat_key LIKE 'p2_slot_%';

CREATE OR REPLACE VIEW serve.p2_fact AS
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
    CROSS JOIN LATERAL (
        SELECT * FROM facts n WHERE n.entity_id=f.entity_id
            AND n.stat_key='p2_q_'||substr(f.stat_key,6,16)||'_count'
            AND n.raw_object_id=f.raw_object_id AND n.valid_from_run=f.valid_from_run
            AND n.source='assets' LIMIT 1
    ) n
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
    CROSS JOIN LATERAL (
        SELECT * FROM facts root WHERE root.entity_id=f.entity_id
            AND root.stat_key='identity_key_'||(a.attributes->>'source_language')
            AND root.raw_object_id=f.raw_object_id AND root.valid_from_run=f.valid_from_run LIMIT 1
    ) root
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

CREATE VIEW serve.p2_hero_slot_source AS
SELECT o.revision
FROM raw.object o
CROSS JOIN LATERAL (
    SELECT GREATEST(o.ingest_run_id,
        COALESCE((SELECT max(r.ingest_run_id) FROM canon.receipt r WHERE r.raw_object_id=o.id),0),
        COALESCE((SELECT max(j.run_id) FROM ops.ingest_rejection j WHERE j.raw_object_id=o.id),0)) AS observed_run
) observed
WHERE o.source='assets' AND o.path LIKE 'heroes/en/observed/%.snapshot'
ORDER BY observed.observed_run DESC,o.id DESC LIMIT 1;

CREATE VIEW serve.p2_hero_slot AS
SELECT selected.id AS from_entity_id,ability.entity_id AS to_entity_id,
    position.slot AS slot,
    jsonb_build_object(
        'entity_id',hero.entity_id,'kind','has_ability',
        'from_entity_id',hero.entity_id,'to_entity_id',ability.entity_id,
        'slot',position.slot,'identity',hero.identity,
        'citation',r.receipt_id::text,'source_revision',r.source_revision,'patch',NULL,
        'qualifiers',jsonb_build_array(jsonb_build_object(
            'key','slot','value',position.slot,
            'citation',slot.receipt_id::text,'source_revision',slot.source_revision,'patch',NULL
        ))
    ) AS link
FROM serve.current_entity selected
CROSS JOIN LATERAL (
    SELECT entity_id,key,kind,identity FROM serve.entity_identity WHERE entity_id=selected.id LIMIT 1
) hero
CROSS JOIN generate_series(1,4) AS position(slot)
CROSS JOIN LATERAL (
    SELECT * FROM serve.current_fact WHERE entity_id=selected.id
        AND stat_key='p2_slot_'||position.slot||'_slot' AND source='assets' LIMIT 1
) slot
CROSS JOIN LATERAL (
    SELECT * FROM serve.current_fact WHERE entity_id=selected.id
        AND stat_key='p2_slot_'||position.slot||'_target' LIMIT 1
) target
CROSS JOIN LATERAL (
    SELECT * FROM serve.current_fact WHERE entity_id=selected.id
        AND stat_key='p2_slot_'||position.slot||'_hero' LIMIT 1
) root
CROSS JOIN LATERAL (
    SELECT entity_id FROM serve.current_alias WHERE alias_norm=target.value_text AND lang='und' LIMIT 2
) ability_alias
CROSS JOIN LATERAL (
    SELECT id FROM serve.current_entity WHERE id=ability_alias.entity_id
        AND key=target.value_text AND kind='ability' LIMIT 1
) ability_entity
CROSS JOIN LATERAL (
    SELECT entity_id,key,identity FROM serve.entity_identity WHERE entity_id=ability_entity.id LIMIT 1
) ability
CROSS JOIN LATERAL (
    SELECT * FROM serve.current_relation WHERE from_entity=selected.id AND to_entity=ability.entity_id
        AND kind='has_ability' AND source='assets' LIMIT 1
) r
CROSS JOIN LATERAL (
    SELECT * FROM serve.published_receipt WHERE id=r.receipt_id LIMIT 1
) p
CROSS JOIN LATERAL (
    SELECT * FROM serve.published_receipt WHERE id=slot.receipt_id LIMIT 1
) q
JOIN serve.p2_hero_slot_source source ON source.revision=r.source_revision
WHERE selected.kind='hero' AND hero.kind='hero'
    AND p.ingest_run_id=slot.valid_from_run AND p.locator='/items/signature'||position.slot
    AND q.locator=p.locator AND q.value_sha256=p.value_sha256
    AND slot.value_text=position.slot::text
    AND target.raw_object_id=slot.raw_object_id AND target.valid_from_run=slot.valid_from_run
    AND root.value_text=hero.key AND root.raw_object_id=slot.raw_object_id
    AND root.valid_from_run=slot.valid_from_run AND r.raw_object_id=slot.raw_object_id;

CREATE VIEW ops.p2_hero_slot_rejection AS
SELECT j.id,j.run_id,j.raw_object_id,j.code,
    CASE WHEN j.code='sql_write' AND object.value IS NOT NULL AND NOT EXISTS(
        SELECT 1 FROM canon.entity e WHERE e.key=translate(lower(btrim(object.value->>'class_name',chr(9)||chr(10)||chr(11)||chr(12)||chr(13)||' ')),'- ','__')
    ) THEN 'Heldenreferenz ist nicht kanonisch veröffentlicht'
    WHEN j.code='sql_write' AND object.value IS NOT NULL AND EXISTS(
        SELECT 1 FROM generate_series(1,4) s WHERE NOT EXISTS(
            SELECT 1 FROM canon.entity e WHERE e.key=translate(lower(btrim(object.value->'items'->>('signature'||s),chr(9)||chr(10)||chr(11)||chr(12)||chr(13)||' ')),'- ','__')
        )
    ) THEN 'Fähigkeitsreferenz ist nicht kanonisch veröffentlicht'
    ELSE j.detail END AS reason
FROM ops.ingest_rejection j JOIN raw.object o ON o.id=j.raw_object_id
CROSS JOIN LATERAL (
    SELECT CASE WHEN j.code='sql_write' AND o.path NOT LIKE '%.snapshot'
        THEN convert_from(o.bytes,'UTF8')::jsonb ELSE NULL END AS value
) object
WHERE o.source='assets' AND j.extractor LIKE '%assets_hero_slots%';
GRANT SELECT ON serve.p2_hero_slot_source,serve.p2_hero_slot TO brain_tool,brain_service;
GRANT SELECT ON ops.p2_hero_slot_rejection TO brain_ingest;
COMMIT;
