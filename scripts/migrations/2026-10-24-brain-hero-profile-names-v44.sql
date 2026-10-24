BEGIN;

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
    COALESCE(abilities.value,jsonb_build_object(
        'available',false,'facts','[]'::jsonb,'links','[]'::jsonb,
        'truncated',false,'omitted_count',0
    )) AS abilities
FROM serve.entity_identity e
JOIN canon.receipt identity_receipt ON identity_receipt.id=e.receipt_id
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
        JOIN canon.receipt receipt ON receipt.id=f.receipt_id
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
        JOIN canon.receipt receipt ON receipt.id=f.receipt_id
            AND receipt.raw_object_id=identity_receipt.raw_object_id
            AND receipt.ingest_run_id=identity_receipt.ingest_run_id
            AND receipt.extractor=identity_receipt.extractor
        WHERE f.entity_id=e.entity_id AND f.valid_from_run=receipt.ingest_run_id
            AND (f.stat_key LIKE 'weapon.%' OR f.stat_key LIKE 'weapon\_%' ESCAPE '\')
        ORDER BY f.stat_key LIMIT 32
    ) f
) weapon ON true
LEFT JOIN LATERAL (
    SELECT jsonb_build_object(
        'available',true,
        'facts',COALESCE(jsonb_agg(r.fact ORDER BY r.ord) FILTER(WHERE r.kept),'[]'::jsonb),
        'links','[]'::jsonb,
        'truncated',count(*) FILTER(WHERE NOT r.kept)>0,
        'omitted_count',count(*) FILTER(WHERE NOT r.kept)
    ) AS value
    FROM (
        SELECT q.fact,q.ord,sum(octet_length(regexp_replace(q.fact::text,'<[^>]*>','','g'))+190) OVER(ORDER BY q.ord)<=7500 AS kept
        FROM (
            SELECT row_number() OVER(ORDER BY x.prio,x.slot,x.stat_key,x.fact::text) AS ord,x.fact
            FROM (
                SELECT DISTINCT ON (s.slot,c.stat_key,c.fact->'condition'->'upgrade_tier',c.fact->'condition'->'upgrade_entry')
                    s.slot,c.stat_key,
                    CASE c.stat_key
                        WHEN 'name_de' THEN 0
                        WHEN 'name_en' THEN 0
                        WHEN 'ability_description_desc' THEN 1
                        WHEN 'ability_description_t1_desc' THEN 2
                        WHEN 'ability_description_t2_desc' THEN 3
                        WHEN 'ability_description_t3_desc' THEN 4
                        ELSE 5 END AS prio,
                    jsonb_strip_nulls(jsonb_build_object(
                        'stat_key',c.stat_key,'value',c.fact->'value','label',c.fact->'label',
                        'unit',NULLIF(c.fact->>'unit',''),'condition',c.fact->'condition',
                        'entity_id',s.to_entity_id,'slot',s.slot,
                        'citation',c.fact->'citation','source_revision',c.fact->'source_revision'
                    ))||jsonb_build_object('patch',NULL) AS fact
                FROM serve.p2_hero_slot s
                CROSS JOIN LATERAL (
                    SELECT f.stat_key,f.fact FROM serve.p2_fact f
                    WHERE f.entity_id=s.to_entity_id
                        AND (f.stat_key IN('name_de','name_en','ability_description_desc',
                                'ability_description_t1_desc','ability_description_t2_desc','ability_description_t3_desc')
                            OR (NULLIF(f.fact->>'label','') IS NOT NULL
                                AND f.fact->'value' NOT IN('0'::jsonb,'""'::jsonb,'false'::jsonb,'null'::jsonb,'"0m"'::jsonb)))
                    OFFSET 0
                ) c
                WHERE s.from_entity_id=e.entity_id AND c.fact->>'value'<>''
                ORDER BY s.slot,c.stat_key,c.fact->'condition'->'upgrade_tier',c.fact->'condition'->'upgrade_entry',
                    CASE c.fact->'condition'->>'lang' WHEN 'de' THEN 0 WHEN 'en' THEN 1 ELSE 2 END
            ) x
        ) q
    ) r
    HAVING count(*)>0
) abilities ON true
WHERE e.kind='hero';

COMMIT;
