CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;
CREATE INDEX alias_trigram ON canon.alias USING gin(alias_norm public.gin_trgm_ops);

CREATE VIEW serve.entity_identity AS
SELECT e.id AS entity_id,e.kind,e.key,e.receipt_id,e.source_revision,
    jsonb_build_object(
        'entity_id',e.id,'kind',e.kind,'key',e.key,
        'citation',e.receipt_id::text,'source_revision',e.source_revision,'patch',NULL,
        'name_en',names.name_en,'name_de',names.name_de,
        'facts',COALESCE(names.facts,'[]'::jsonb),'truncated',false,'omitted_count',0
    ) AS identity
FROM serve.current_entity e
LEFT JOIN LATERAL (
    SELECT max(f.value_text) FILTER(WHERE f.stat_key='name_en') AS name_en,
        max(f.value_text) FILTER(WHERE f.stat_key='name_de') AS name_de,
        jsonb_agg(jsonb_build_object(
            'stat_key',f.stat_key,'value',f.value_text,'condition','{}'::jsonb,
            'citation',f.receipt_id::text,'source_revision',f.source_revision,'patch',NULL
        ) ORDER BY f.stat_key) AS facts
    FROM (
        SELECT stat_key,value_text,receipt_id,source_revision
        FROM serve.current_fact WHERE entity_id=e.id
            AND stat_key IN('name_en','name_de') AND value_text IS NOT NULL
        ORDER BY stat_key LIMIT 2
    ) f
) names ON true;

CREATE VIEW serve.hero_profile AS
SELECT e.entity_id,e.identity,
    jsonb_build_object(
        'facts',COALESCE(stats.facts,'[]'::jsonb),
        'truncated',COALESCE(stats.total,0)>32,
        'omitted_count',greatest(COALESCE(stats.total,0)-32,0)
    ) AS stats,
    jsonb_build_object(
        'facts',COALESCE(weapon.facts,'[]'::jsonb),
        'truncated',COALESCE(weapon.total,0)>32,
        'omitted_count',greatest(COALESCE(weapon.total,0)-32,0)
    ) AS weapon,
    jsonb_build_object(
        'available',false,'facts','[]'::jsonb,'links','[]'::jsonb,
        'truncated',false,'omitted_count',0
    ) AS abilities
FROM serve.entity_identity e
LEFT JOIN LATERAL (
    SELECT max(f.total) AS total,jsonb_agg(jsonb_build_object(
        'stat_key',f.stat_key,
        'value',COALESCE(to_jsonb(f.value_num),to_jsonb(f.value_text),to_jsonb(f.value_bool)),
        'condition','{}'::jsonb,'citation',f.receipt_id::text,
        'source_revision',f.source_revision,'patch',NULL
    ) ORDER BY f.stat_key) AS facts
    FROM (
        SELECT stat_key,value_num,value_text,value_bool,receipt_id,source_revision,
            count(*) OVER() AS total
        FROM serve.current_fact WHERE entity_id=e.entity_id
            AND stat_key NOT IN('name_en','name_de')
            AND stat_key NOT LIKE 'weapon.%' AND stat_key NOT LIKE 'weapon\_%' ESCAPE '\'
        ORDER BY stat_key LIMIT 32
    ) f
) stats ON true
LEFT JOIN LATERAL (
    SELECT max(f.total) AS total,jsonb_agg(jsonb_build_object(
        'stat_key',f.stat_key,
        'value',COALESCE(to_jsonb(f.value_num),to_jsonb(f.value_text),to_jsonb(f.value_bool)),
        'condition','{}'::jsonb,'citation',f.receipt_id::text,
        'source_revision',f.source_revision,'patch',NULL
    ) ORDER BY f.stat_key) AS facts
    FROM (
        SELECT stat_key,value_num,value_text,value_bool,receipt_id,source_revision,
            count(*) OVER() AS total
        FROM serve.current_fact WHERE entity_id=e.entity_id
            AND (stat_key LIKE 'weapon.%' OR stat_key LIKE 'weapon\_%' ESCAPE '\')
        ORDER BY stat_key LIMIT 32
    ) f
) weapon ON true
WHERE e.kind='hero';
