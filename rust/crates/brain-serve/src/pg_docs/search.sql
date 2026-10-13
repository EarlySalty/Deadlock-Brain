WITH terms AS (
    SELECT websearch_to_tsquery('pg_catalog.simple',$2) AS simple_query,
        websearch_to_tsquery('pg_catalog.german',$2) AS german_query
)
SELECT d.page_id,d.raw_object_id,d.receipt_id,d.valid_from_run,d.meta_key,d.scope,
    d.path,d.title,d.heading_path,left(d.body,1501) AS excerpt,
    d.line_start,d.line_end,d.source_revision
FROM serve.doc_section d CROSS JOIN terms
WHERE d.scope=$1 AND (d.tsv @@ terms.simple_query OR d.tsv_german @@ terms.german_query)
    AND ($3::bigint IS NULL OR EXISTS (
        SELECT 1 FROM serve.current_entity e JOIN serve.current_alias a ON a.entity_id=e.id
        WHERE e.id=$3 AND e.kind<>'doc_page' AND length(trim(a.alias_norm::text))>0
            AND serve.doc_tsv(d.title||' '||array_to_string(d.heading_path,' ')||' '||d.body)
                @@ plainto_tsquery('pg_catalog.simple',a.alias_norm::text)
    ))
ORDER BY greatest(ts_rank_cd(d.tsv,terms.simple_query),ts_rank_cd(d.tsv_german,terms.german_query)) DESC,
    d.page_id,d.ordinal,d.receipt_id
LIMIT $4
