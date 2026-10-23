WITH terms AS (
    SELECT websearch_to_tsquery('pg_catalog.simple',$2) AS simple_query,
        websearch_to_tsquery('pg_catalog.german',$2) AS german_query,
        CASE WHEN $2 !~ '"' AND $2 !~* '(^|[[:space:]])(-|or([[:space:]]|$))' THEN ARRAY(
            SELECT DISTINCT CASE WHEN w.command<>''
                    THEN '('||w.lexemes||' | ''/'||w.command||''')'
                    ELSE '('||w.lexemes||')' END
            FROM (SELECT regexp_replace(lower(word),'^/|[^[:alnum:]_-]','','g') AS command,
                    (SELECT string_agg(''''||replace(replace(l.lexeme,'\','\\'),'''','''''')||'''',' & ')
                        FROM (SELECT lexeme,bool_or(lexeme !~ '-') OVER () AS has_part
                            FROM unnest(tsvector_to_array(to_tsvector('pg_catalog.german',word))) lexeme) l
                        WHERE l.lexeme !~ '-' OR NOT l.has_part) AS lexemes
                FROM regexp_split_to_table(btrim($2),'[[:space:]]+') word) w
            WHERE w.lexemes IS NOT NULL
        ) END AS operands
), queries AS (
    SELECT t.simple_query,t.german_query,t.operands,
        CASE WHEN cardinality(t.operands)>0 THEN array_to_string(t.operands,' | ')::tsquery END AS any_query
    FROM terms t
), hits AS (
    SELECT h.page_id,h.valid_from_run,h.meta_key,h.receipt_id,cardinality(q.operands) AS terms,
        (h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query) AS strict,
        CASE WHEN h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query THEN 0
            ELSE (SELECT count(*) FROM unnest(q.operands) operand
                WHERE h.tsv_german @@ operand::tsquery OR h.tsv @@ operand::tsquery)
        END AS coverage,
        CASE WHEN h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query
            THEN greatest(ts_rank_cd(h.tsv,q.simple_query),ts_rank_cd(h.tsv_german,q.german_query))
            ELSE ts_rank_cd(h.tsv_german,q.any_query)
        END AS score
    FROM serve.doc_search_hit h CROSS JOIN queries q
    WHERE h.scope=$1
        AND (h.tsv @@ q.simple_query OR h.tsv_german @@ q.german_query
            OR h.tsv_german @@ q.any_query OR h.tsv @@ q.any_query)
        AND ($3::bigint IS NULL OR EXISTS (
            SELECT 1 FROM serve.current_entity e JOIN serve.current_alias a ON a.entity_id=e.id
            WHERE e.id=$3 AND e.kind<>'doc_page' AND length(trim(a.alias_norm::text))>0
                AND h.tsv @@ plainto_tsquery('pg_catalog.simple',a.alias_norm::text)
        ))
), ranked AS (
    SELECT s.*,row_number() OVER (
        ORDER BY s.strict DESC,s.coverage DESC,s.score DESC,s.page_id,s.meta_key,s.receipt_id
    ) AS rank
    FROM (SELECT h.*,bool_or(h.strict) OVER() AS any_strict FROM hits h) s
    WHERE s.strict OR (NOT s.any_strict AND s.coverage>=least(2,s.terms))
), found AS (
    SELECT c.rank,d.page_id,d.raw_object_id,d.receipt_id,d.valid_from_run,d.meta_key,d.scope,
        d.path,d.title,d.heading_path,left(d.body,1501) AS excerpt,
        d.line_start,d.line_end,d.source_revision,p.ingest_run_id,p.extractor,p.locator_kind,p.locator,
        encode(p.value_sha256,'hex') AS value_sha256
    FROM (SELECT * FROM ranked ORDER BY rank) c
    CROSS JOIN LATERAL (
        SELECT * FROM serve.doc_section d
        WHERE d.page_id=c.page_id AND d.valid_from_run=c.valid_from_run
            AND d.meta_key=c.meta_key AND d.receipt_id=c.receipt_id AND d.scope=$1
    ) d
    JOIN serve.published_receipt p ON p.id=d.receipt_id
    ORDER BY c.rank
    LIMIT $4+1
)
SELECT f.page_id,f.raw_object_id,f.receipt_id,f.valid_from_run,f.meta_key,f.scope,
    f.path,f.title,f.heading_path,f.excerpt,
    f.line_start,f.line_end,f.source_revision,f.ingest_run_id,f.extractor,f.locator_kind,f.locator,
    f.value_sha256,count(*) OVER() AS bounded_matches
FROM found f
ORDER BY f.rank
LIMIT $4
