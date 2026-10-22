WITH terms AS (
    SELECT websearch_to_tsquery('pg_catalog.simple',$2) AS simple_query,
        websearch_to_tsquery('pg_catalog.german',$2) AS german_query,
        CASE WHEN websearch_to_tsquery('pg_catalog.german',$2)::text !~ '[!|]'
            THEN ARRAY(SELECT DISTINCT CASE WHEN operand ~ '^''[^''/ ]+''( <-> ''[^''/ ]+'')*$'
                    THEN '('||operand||' | ''/'||replace(replace(operand,''' <-> ''','_'),'''','')||''')'
                    ELSE operand END
                FROM unnest(string_to_array(websearch_to_tsquery('pg_catalog.german',$2)::text,' & ')) operand)
        END AS operands
), queries AS (
    SELECT t.simple_query,t.german_query,t.operands,
        CASE WHEN cardinality(t.operands)>1 THEN array_to_string(t.operands,' | ')::tsquery END AS any_query
    FROM terms t
), hits AS (
    SELECT h.page_id,h.valid_from_run,h.meta_key,h.receipt_id,
        (h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query) AS strict,
        CASE WHEN h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query THEN 0
            ELSE (SELECT count(*) FROM unnest(q.operands) operand WHERE h.tsv_german @@ operand::tsquery)
        END AS coverage,
        CASE WHEN h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query
            THEN greatest(ts_rank_cd(h.tsv,q.simple_query),ts_rank_cd(h.tsv_german,q.german_query))
            ELSE ts_rank_cd(h.tsv_german,q.any_query)
        END AS score
    FROM serve.doc_search_hit h CROSS JOIN queries q
    WHERE h.scope=$1
        AND (h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query OR h.tsv_german @@ q.any_query)
), tiers AS (
    SELECT h.*,bool_or(h.strict) OVER() AS any_strict FROM hits h
), candidates AS MATERIALIZED (
    SELECT t.*,count(*) OVER() AS matching_sections FROM tiers t
    WHERE t.strict OR (NOT t.any_strict AND t.coverage>=2)
    ORDER BY t.strict DESC,t.coverage DESC,t.score DESC,t.page_id,t.meta_key,t.receipt_id
    LIMIT 12
)
SELECT d.page_id,d.raw_object_id,d.receipt_id,d.valid_from_run,d.meta_key,d.scope,
    d.path,d.title,d.heading_path,left(d.body,1501) AS excerpt,
    d.line_start,d.line_end,d.source_revision,p.ingest_run_id,p.extractor,p.locator_kind,p.locator,
    encode(p.value_sha256,'hex') AS value_sha256,c.matching_sections
FROM candidates c
JOIN serve.doc_section d ON d.page_id=c.page_id AND d.valid_from_run=c.valid_from_run
    AND d.meta_key=c.meta_key AND d.receipt_id=c.receipt_id
JOIN serve.published_receipt p ON p.id=d.receipt_id
WHERE d.scope=$1
    AND ($3::bigint IS NULL OR EXISTS (
        SELECT 1 FROM serve.current_entity e JOIN serve.current_alias a ON a.entity_id=e.id
        WHERE e.id=$3 AND e.kind<>'doc_page' AND length(trim(a.alias_norm::text))>0
            AND serve.doc_tsv(d.title||' '||array_to_string(d.heading_path,' ')||' '||d.body)
                @@ plainto_tsquery('pg_catalog.simple',a.alias_norm::text)
    ))
ORDER BY c.strict DESC,c.coverage DESC,c.score DESC,d.page_id,d.ordinal,d.receipt_id
LIMIT $4
