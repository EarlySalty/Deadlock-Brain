BEGIN;
CREATE OR REPLACE VIEW serve.entity_identity AS
SELECT e.id AS entity_id,e.kind,e.key,e.receipt_id,e.source_revision,
    jsonb_build_object(
        'entity_id',e.id,'kind',e.kind,'key',e.key,
        'citation',e.receipt_id::text,'source_revision',e.source_revision,'patch',NULL,
        'name_en',names.name_en,'name_de',names.name_de,
        'facts',COALESCE(names.facts,'[]'::jsonb),'truncated',false,'omitted_count',0
    ) AS identity
FROM serve.current_entity e
JOIN serve.published_receipt identity_receipt ON identity_receipt.id=e.receipt_id
LEFT JOIN LATERAL (
    SELECT max(f.value_text) FILTER(WHERE f.stat_key='name_en') AS name_en,
        max(f.value_text) FILTER(WHERE f.stat_key='name_de') AS name_de,
        jsonb_agg(jsonb_build_object(
            'stat_key',f.stat_key,'value',f.value_text,'condition','{}'::jsonb,
            'citation',f.receipt_id::text,'source_revision',f.source_revision,'patch',NULL
        ) ORDER BY f.stat_key) AS facts
    FROM (
        SELECT f.stat_key,f.value_text,f.receipt_id,f.source_revision
        FROM serve.current_fact f
        JOIN serve.published_receipt receipt ON receipt.id=f.receipt_id
            AND receipt.raw_object_id=identity_receipt.raw_object_id
            AND receipt.ingest_run_id=identity_receipt.ingest_run_id
            AND receipt.extractor=identity_receipt.extractor
        WHERE f.entity_id=e.id AND f.valid_from_run=receipt.ingest_run_id
            AND f.stat_key IN('name_en','name_de') AND f.value_text IS NOT NULL
        ORDER BY f.stat_key LIMIT 2
    ) f
) names ON true;

CREATE OR REPLACE VIEW serve.hero_profile AS
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
JOIN serve.published_receipt identity_receipt ON identity_receipt.id=e.receipt_id
LEFT JOIN LATERAL (
    SELECT max(f.total) AS total,jsonb_agg(jsonb_build_object(
        'stat_key',f.stat_key,
        'value',COALESCE(to_jsonb(f.value_num),to_jsonb(f.value_text),to_jsonb(f.value_bool)),
        'condition','{}'::jsonb,'citation',f.receipt_id::text,
        'source_revision',f.source_revision,'patch',NULL
    ) ORDER BY f.stat_key) AS facts
    FROM (
        SELECT f.stat_key,f.value_num,f.value_text,f.value_bool,f.receipt_id,f.source_revision,
            count(*) OVER() AS total
        FROM serve.current_fact f
        JOIN serve.published_receipt receipt ON receipt.id=f.receipt_id
            AND receipt.raw_object_id=identity_receipt.raw_object_id
            AND receipt.ingest_run_id=identity_receipt.ingest_run_id
            AND receipt.extractor=identity_receipt.extractor
        WHERE f.entity_id=e.entity_id AND f.valid_from_run=receipt.ingest_run_id
            AND f.stat_key NOT IN('name_en','name_de')
            AND f.stat_key NOT LIKE 'weapon.%' AND f.stat_key NOT LIKE 'weapon\_%' ESCAPE '\'
        ORDER BY f.stat_key LIMIT 32
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
        SELECT f.stat_key,f.value_num,f.value_text,f.value_bool,f.receipt_id,f.source_revision,
            count(*) OVER() AS total
        FROM serve.current_fact f
        JOIN serve.published_receipt receipt ON receipt.id=f.receipt_id
            AND receipt.raw_object_id=identity_receipt.raw_object_id
            AND receipt.ingest_run_id=identity_receipt.ingest_run_id
            AND receipt.extractor=identity_receipt.extractor
        WHERE f.entity_id=e.entity_id AND f.valid_from_run=receipt.ingest_run_id
            AND (f.stat_key LIKE 'weapon.%' OR f.stat_key LIKE 'weapon\_%' ESCAPE '\')
        ORDER BY f.stat_key LIMIT 32
    ) f
) weapon ON true
WHERE e.kind='hero';
COMMIT;
