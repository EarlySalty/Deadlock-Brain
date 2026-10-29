WITH ability_names AS MATERIALIZED (
    SELECT DISTINCT '(^|[^[:alnum:]_])'||regexp_replace(lower(btrim(a.alias_norm::text)),
        '([.^$*+?()\[\]{}|\\-])','\\\1','g')||'($|[^[:alnum:]_])' AS pattern
    FROM serve.current_alias a
    WHERE a.entity_id=$1 AND a.lang='en' AND length(btrim(a.alias_norm::text))>=3
    ORDER BY 1 LIMIT 8
), hero_names AS MATERIALIZED (
    SELECT DISTINCT lower(btrim(a.alias_norm::text)) AS name FROM serve.current_relation r
    CROSS JOIN LATERAL (
        SELECT a.alias_norm FROM serve.current_alias a
        WHERE a.entity_id=r.from_entity AND a.lang='en' OFFSET 0
    ) a
    WHERE r.to_entity=$1 AND r.kind='has_ability'
    ORDER BY 1 LIMIT 8
), candidates AS MATERIALIZED (
    SELECT h.name AS hero,kind.prefix,f.entity_id,f.raw_object_id,f.valid_from_run,f.receipt_id,
        f.stat_key::text AS stat_key,f.position,CASE WHEN pg_input_is_valid(f.value_text::text,'jsonb')
            THEN f.value_text::text::jsonb END AS payload
    FROM hero_names h
    CROSS JOIN (VALUES ('patch_change_r'),('patch_line_r')) AS kind(prefix)
    CROSS JOIN LATERAL (
        SELECT f.*,row_number() OVER (ORDER BY f.entity_id DESC,f.stat_key COLLATE "C",f.receipt_id) AS position FROM (
            SELECT f.* FROM serve.current_fact f
            WHERE f.stat_key COLLATE "C" >=
                    (kind.prefix||encode(sha256(convert_to(h.name,'UTF8')),'hex')||'_') COLLATE "C"
                AND f.stat_key COLLATE "C" <
                    (kind.prefix||encode(sha256(convert_to(h.name,'UTF8')),'hex')||'`') COLLATE "C"
                AND f.value_text IS NOT NULL
            ORDER BY f.entity_id DESC,f.stat_key COLLATE "C",f.receipt_id
            LIMIT $3+1
        ) f
    ) f
), lines AS (
    SELECT c.entity_id AS patch_id,p.label,p.released_at,c.receipt_id,receipt.source_revision,
        btrim(c.payload->>'line_text',E' \t\r\n*-​') AS line,c.stat_key
    FROM candidates c
    CROSS JOIN LATERAL (
        SELECT p.label,p.released_at,p.extractor,p.ingest_run_id FROM serve.p4_patch_catalog p
        WHERE p.patch_id=c.entity_id AND p.raw_object_id=c.raw_object_id
            AND p.ingest_run_id=c.valid_from_run
        OFFSET 0
    ) p
    CROSS JOIN LATERAL (
        SELECT r.source_revision FROM serve.published_receipt r
        WHERE r.id=c.receipt_id AND r.extractor=p.extractor
            AND r.raw_object_id=c.raw_object_id AND r.ingest_run_id=p.ingest_run_id
            AND r.locator_kind='json_pointer' AND r.locator='/raw_content'
        OFFSET 0
    ) receipt
    WHERE c.position<=$3
        AND jsonb_typeof(c.payload)='object'
        AND jsonb_typeof(c.payload->'line_text')='string'
        AND lower(btrim(c.payload->>'reference'))=c.hero
        AND EXISTS(SELECT 1 FROM ability_names n WHERE lower(c.payload->>'line_text') ~ n.pattern)
), distinct_lines AS (
    SELECT DISTINCT ON (lower(line)) * FROM lines
    WHERE length(line)>=8
    ORDER BY lower(line),released_at DESC,patch_id DESC,stat_key
), ranked AS (
    SELECT d.*,row_number() OVER (ORDER BY released_at DESC,patch_id DESC,stat_key) AS position,
        count(*) OVER () AS total
    FROM distinct_lines d
)
SELECT jsonb_build_object(
    'lines',COALESCE((
        SELECT jsonb_agg(jsonb_build_object(
            'patch',label,'released_at',released_at,'line_text',line,
            'citation',receipt_id::text,'source_revision',source_revision
        ) ORDER BY position)
        FROM ranked WHERE position<=$2
    ),'[]'::jsonb),
    'total',COALESCE((SELECT max(total) FROM ranked),0),
    'lower_bound',EXISTS(SELECT 1 FROM candidates WHERE position>$3)
)
