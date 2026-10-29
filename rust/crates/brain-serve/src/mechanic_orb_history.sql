WITH facts AS MATERIALIZED (
    SELECT c.patch_id,c.label,c.released_at,c.extractor,c.raw_object_id,
        c.ingest_run_id,f.id,f.receipt_id,f.value_text::text AS body
    FROM serve.p4_patch_catalog c CROSS JOIN LATERAL (
        SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
            AND f.stat_key COLLATE "C" >= 'patch_change_r' AND f.stat_key COLLATE "C" < 'patch_change_s'
            AND f.stat_key ~ '^patch_change_r[0-9a-f]{64}_[0-9]+$'
        UNION ALL
        SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
            AND f.stat_key COLLATE "C" >= 'patch_line_r' AND f.stat_key COLLATE "C" < 'patch_line_s'
            AND f.stat_key ~ '^patch_line_r[0-9a-f]{64}_[0-9]+_[0-9]+$'
        UNION ALL
        SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
            AND f.stat_key COLLATE "C" >= 'patch_note_' AND f.stat_key COLLATE "C" < 'patch_note`'
            AND f.stat_key ~ '^patch_note_[0-9]+_[0-9]+$'
    ) f
    WHERE f.raw_object_id=c.raw_object_id AND f.valid_from_run=c.ingest_run_id
        AND strpos(lower(f.value_text::text),'orb')>0
), lines AS (
    SELECT f.*,r.source_revision,x.seg,x.ord FROM facts f
    JOIN serve.published_receipt r ON r.id=f.receipt_id
        AND r.extractor=f.extractor AND r.raw_object_id=f.raw_object_id
        AND r.ingest_run_id=f.ingest_run_id
        AND r.locator_kind='json_pointer' AND r.locator='/raw_content'
    CROSS JOIN LATERAL (
        SELECT CASE WHEN pg_input_is_valid(f.body,'jsonb')
            THEN f.body::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
    ) payload
    CROSS JOIN LATERAL unnest(regexp_split_to_array(payload.value->>'line_text','\s+-\s+'))
        WITH ORDINALITY x(seg,ord)
    WHERE jsonb_typeof(payload.value->'line_text')='string'
        AND x.seg ~* '(\mkill/orb\M|\mflying orb\M)'
        AND x.seg ~* '\mfrom\s+[0-9][0-9/.]*%?\s.*\mto\s+[0-9][0-9/.]*%'
), distinct_lines AS (
    SELECT DISTINCT ON (patch_id,btrim(seg)) * FROM lines ORDER BY patch_id,btrim(seg),id,ord
)
SELECT jsonb_build_object(
    'patch',label,'released_at',released_at,'line_text',btrim(seg),
    'citation',receipt_id::text,'source_revision',source_revision
)
FROM distinct_lines
ORDER BY released_at,patch_id,id,ord
LIMIT 32
