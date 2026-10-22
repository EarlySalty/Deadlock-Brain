BEGIN;

CREATE OR REPLACE VIEW serve.doc_section AS
SELECT s.entity_id AS page_id,s.raw_object_id,m.path,m.title,m.lang,m.scope,m.source_revision,s.receipt_id,s.valid_from_run,
    s.meta_key,(m.meta->>'ordinal')::integer AS ordinal,
    ARRAY(SELECT jsonb_array_elements_text(m.meta->'heading_path')) AS heading_path,
    (m.meta->>'line_start')::integer AS line_start,(m.meta->>'line_end')::integer AS line_end,
    s.body,s.tsv,s.tsv_german
FROM canon.doc_search s CROSS JOIN LATERAL (
    WITH metadata AS MATERIALIZED (
        SELECT p.*,m.stat_key::text AS meta_key,m.receipt_id,m.valid_from_run,
            serve.doc_meta(m.value_text::text) AS meta,r.locator,r.locator_kind,r.value_sha256
        FROM serve.doc_page p JOIN serve.current_fact m ON m.entity_id=p.page_id AND m.raw_object_id=p.raw_object_id
        JOIN serve.published_receipt r ON r.id=m.receipt_id AND r.raw_object_id=p.raw_object_id
        WHERE p.page_id=s.entity_id AND m.stat_key::text ~ '^doc_(intro|section_.+)_meta$'
            AND r.locator_kind='byte_ranges'
    )
    SELECT m.* FROM metadata m WHERE m.meta IS NOT NULL
        AND (m.meta->>'ordinal')::integer<m.section_count
        AND (SELECT count(*) FROM metadata)=m.section_count
        AND NOT EXISTS (SELECT 1 FROM metadata other
            WHERE other.meta IS NULL OR (other.meta->>'ordinal')::integer>=m.section_count)
        AND (SELECT count(DISTINCT other.meta->>'ordinal') FROM metadata other)=m.section_count
        AND s.valid_from_run=m.valid_from_run AND s.meta_key=m.meta_key AND s.receipt_id=m.receipt_id
        AND s.raw_object_id=m.raw_object_id AND s.title=m.title
    OFFSET 0
) m
WHERE (SELECT count(*) FROM generate_series(0,(m.meta->>'chunks')::integer-1) expected(chunk)
    JOIN serve.current_fact f ON f.entity_id=m.page_id AND f.raw_object_id=m.raw_object_id AND f.valid_from_run=m.valid_from_run
        AND f.stat_key::text=left(m.meta_key,length(m.meta_key)-5)||'_body_'||expected.chunk
    JOIN serve.published_receipt r ON r.id=f.receipt_id AND r.raw_object_id=m.raw_object_id
        AND r.locator_kind=m.locator_kind AND r.locator=m.locator AND r.value_sha256=m.value_sha256
    WHERE f.value_text IS NOT NULL)=(m.meta->>'chunks')::integer;

CREATE VIEW serve.doc_search_hit AS
SELECT s.entity_id AS page_id,s.valid_from_run,s.meta_key,s.receipt_id,s.tsv,s.tsv_german,
    CASE WHEN p.path LIKE 'public/%' THEN 'public' ELSE 'internal' END AS scope
FROM canon.doc_search s
JOIN serve.published_receipt p ON p.id=s.receipt_id AND p.raw_object_id=s.raw_object_id AND p.source='deadlock-docs'
WHERE s.valid_from_run=(SELECT max(l.valid_from_run) FROM canon.doc_search l WHERE l.entity_id=s.entity_id);
REVOKE ALL ON serve.doc_search_hit FROM PUBLIC;
GRANT SELECT ON serve.doc_search_hit TO brain_tool,brain_service;

COMMIT;
