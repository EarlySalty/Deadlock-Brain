WITH hits AS MATERIALIZED (
    SELECT t.id,t.entity_id,t.stat_key FROM serve.patch_line_text t
    WHERE t.text_lower ~* $1
), facts AS MATERIALIZED (
    SELECT c.patch_id,c.label,c.released_at,c.extractor,c.raw_object_id,
        c.ingest_run_id,cf.id,cf.receipt_id,cf.value_text::text AS body
    FROM hits h
    CROSS JOIN LATERAL (
        SELECT * FROM serve.current_fact cf
        WHERE cf.entity_id=h.entity_id AND cf.stat_key=h.stat_key OFFSET 0
    ) cf
    CROSS JOIN LATERAL (
        SELECT * FROM serve.p4_patch_catalog c WHERE c.patch_id=cf.entity_id OFFSET 0
    ) c
    WHERE cf.id=h.id AND cf.raw_object_id=c.raw_object_id AND cf.valid_from_run=c.ingest_run_id
), lines AS (
    SELECT f.*,r.source_revision,btrim(x.seg,E' \t\r\n*-​') AS seg,x.ord FROM facts f
    JOIN serve.published_receipt r ON r.id=f.receipt_id
        AND r.extractor=f.extractor AND r.raw_object_id=f.raw_object_id
        AND r.ingest_run_id=f.ingest_run_id
        AND r.locator_kind='json_pointer' AND r.locator='/raw_content'
    CROSS JOIN LATERAL (
        SELECT CASE WHEN pg_input_is_valid(f.body,'jsonb')
            THEN f.body::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
    ) payload
    CROSS JOIN LATERAL unnest(regexp_split_to_array(payload.value->>'line_text','(\s+-\s+|\s+\*\s+)'))
        WITH ORDINALITY x(seg,ord)
    WHERE jsonb_typeof(payload.value->'line_text')='string'
        AND x.seg ~* $1
        AND ($2='' OR x.seg !~* $2)
        AND coalesce(payload.value->>'subject','') ~* ('^$|'||$1)
        AND ($2='' OR coalesce(payload.value->>'subject','') !~* $2)
), distinct_lines AS (
    SELECT DISTINCT ON (lower(seg)) * FROM lines
    WHERE length(seg) BETWEEN 8 AND 400
    ORDER BY lower(seg),released_at,patch_id,id,ord
)
SELECT jsonb_build_object(
    'patch',label,'released_at',released_at,'line_text',seg,
    'citation',receipt_id::text,'source_revision',source_revision
)
FROM distinct_lines
ORDER BY released_at DESC,patch_id DESC,id,ord
LIMIT $3
