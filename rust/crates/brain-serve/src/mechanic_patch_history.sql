WITH hits AS MATERIALIZED (
    SELECT t.id,t.entity_id,t.stat_key FROM serve.patch_line_text t
    WHERE t.text_lower ~* $1
), facts AS MATERIALIZED (
    SELECT c.patch_id,c.label,c.ledger_at AS released_at,c.extractor,c.raw_object_id,
        c.ingest_run_id,cf.id,cf.receipt_id,cf.value_text::text AS body
    FROM hits h
    CROSS JOIN LATERAL (
        SELECT * FROM serve.current_fact cf
        WHERE cf.entity_id=h.entity_id AND cf.stat_key=h.stat_key OFFSET 0
    ) cf
    CROSS JOIN LATERAL (
        SELECT c.*,coalesce(l.ledger_released_at,c.released_at) AS ledger_at
        FROM serve.p4_patch_catalog c
        LEFT JOIN serve.patch_ledger_member l ON l.patch_id=c.patch_id
        WHERE c.patch_id=cf.entity_id AND (l.disposition IS NULL OR l.disposition='primary')
        OFFSET 0
    ) c
    WHERE cf.id=h.id AND cf.raw_object_id=c.raw_object_id AND cf.valid_from_run=c.ingest_run_id
), lines AS (
    SELECT f.*,r.source_revision,
        btrim(payload.value->>'line_text',E' \t\r\n*-​') AS line,
        coalesce(payload.value->>'section','') AS section,
        coalesce(payload.value->>'subject','') AS subject
    FROM facts f
    JOIN serve.published_receipt r ON r.id=f.receipt_id
        AND r.extractor=f.extractor AND r.raw_object_id=f.raw_object_id
        AND r.ingest_run_id=f.ingest_run_id
        AND r.locator_kind='json_pointer' AND r.locator='/raw_content'
    CROSS JOIN LATERAL (
        SELECT CASE WHEN pg_input_is_valid(f.body,'jsonb')
            THEN f.body::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
    ) payload
    WHERE jsonb_typeof(payload.value->'line_text')='string'
), scoped AS (
    SELECT * FROM lines
    WHERE (line ~* $1 OR section ~* $1)
        AND ($2='' OR line !~* $2)
        AND ($4='' OR section !~* $4)
        AND subject ~* ('^$|'||$1)
        AND ($2='' OR subject !~* $2)
        AND length(line) BETWEEN 8 AND 600
), distinct_lines AS (
    SELECT DISTINCT ON (released_at,lower(line)) * FROM scoped
    ORDER BY released_at,lower(line),patch_id,id
), bucketed AS (
    SELECT d.*,b.bucket,b.keep FROM distinct_lines d
    LEFT JOIN LATERAL (
        SELECT u.bucket,u.keep FROM unnest($5::text[],$6::int[]) WITH ORDINALITY u(pattern,keep,bucket)
        WHERE d.line ~* u.pattern ORDER BY u.bucket LIMIT 1
    ) b ON true
), ranked AS (
    SELECT *,row_number() OVER (
        PARTITION BY bucket
        ORDER BY line ~* '^fix(ed|es)?\M',released_at DESC,line ~ '[0-9]' DESC,patch_id DESC,id
    ) AS bucket_rank,row_number() OVER (
        ORDER BY released_at DESC,line ~ '[0-9]' DESC,patch_id DESC,id
    ) AS recency,count(*) OVER () AS total FROM bucketed
)
SELECT jsonb_build_object(
    'patch',label,'released_at',released_at,'section',nullif(section,''),'line_text',line,
    'citation',receipt_id::text,'source_revision',source_revision,'total',total
)
FROM (
    SELECT * FROM ranked
    ORDER BY recency=1 DESC,CASE WHEN bucket_rank<=keep THEN bucket_rank END NULLS LAST,
        line ~ '[0-9]' DESC,released_at DESC,patch_id DESC,id
    LIMIT $3
) picked
ORDER BY released_at DESC,patch_id DESC,id
