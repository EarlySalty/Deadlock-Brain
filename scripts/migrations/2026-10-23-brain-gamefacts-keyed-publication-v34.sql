BEGIN;

CREATE OR REPLACE VIEW serve.doc_page AS
SELECT e.id AS page_id,e.raw_object_id,e.receipt_id AS page_receipt_id,e.source_revision,
    p.value_text::text AS path,t.value_text::text AS title,l.value_text::text AS lang,
    n.value_num::integer AS section_count,
    CASE WHEN p.value_text LIKE 'public/%' THEN 'public' ELSE 'internal' END AS scope
FROM serve.current_entity e
JOIN serve.current_fact p ON p.entity_id=e.id AND p.stat_key='doc_path' AND p.raw_object_id=e.raw_object_id
JOIN serve.current_fact t ON t.entity_id=e.id AND t.stat_key='doc_title' AND t.raw_object_id=e.raw_object_id
JOIN serve.current_fact l ON l.entity_id=e.id AND l.stat_key='doc_lang' AND l.raw_object_id=e.raw_object_id
JOIN serve.current_fact n ON n.entity_id=e.id AND n.stat_key='doc_section_count' AND n.raw_object_id=e.raw_object_id
CROSS JOIN LATERAL (
    SELECT receipt.id FROM serve.published_receipt receipt
    WHERE receipt.id=e.receipt_id AND receipt.source='deadlock-docs'
        AND receipt.raw_object_id=e.raw_object_id AND receipt.path=p.value_text
    OFFSET 0
) receipt
WHERE e.kind='doc_page' AND e.source='deadlock-docs' AND e.source_revision ~ '^[0-9a-f]{40}$'
    AND p.value_text::text ~ '^(public|internal)/.+[.]html$'
    AND p.value_text::text !~ '(^|/)[.]{1,2}(/|$)|//|\\|[[:cntrl:]]'
    AND octet_length(p.value_text) BETWEEN 1 AND 2048
    AND octet_length(t.value_text) BETWEEN 1 AND 512 AND t.value_text::text !~ '[[:cntrl:]]'
    AND octet_length(l.value_text) BETWEEN 1 AND 16384
    AND btrim(l.value_text::text,U&'\0020\00A0\1680\2000\2001\2002\2003\2004\2005\2006\2007\2008\2009\200A\2028\2029\202F\205F\3000')<>''
    AND l.value_text::text !~ U&'[\0001-\001F\007F-\009F]'
    AND n.value_num BETWEEN 1 AND 4096 AND n.value_num=trunc(n.value_num);

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
        CROSS JOIN LATERAL (
            SELECT r.locator,r.locator_kind,r.value_sha256 FROM serve.published_receipt r
            WHERE r.id=m.receipt_id AND r.raw_object_id=p.raw_object_id AND r.locator_kind='byte_ranges'
            OFFSET 0
        ) r
        WHERE p.page_id=s.entity_id AND m.stat_key::text ~ '^doc_(intro|section_.+)_meta$'
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
    CROSS JOIN LATERAL (
        SELECT r.id FROM serve.published_receipt r
        WHERE r.id=f.receipt_id AND r.raw_object_id=m.raw_object_id
            AND r.locator_kind=m.locator_kind AND r.locator=m.locator AND r.value_sha256=m.value_sha256
        OFFSET 0
    ) r
    WHERE f.value_text IS NOT NULL)=(m.meta->>'chunks')::integer;

CREATE OR REPLACE VIEW serve.doc_search_hit AS
SELECT s.entity_id AS page_id,s.valid_from_run,s.meta_key,s.receipt_id,s.tsv,s.tsv_german,
    (SELECT CASE WHEN o.path LIKE 'public/%' THEN 'public' ELSE 'internal' END
        FROM raw.object o WHERE o.id=s.raw_object_id AND o.source='deadlock-docs') AS scope
FROM canon.doc_search s
WHERE EXISTS (SELECT 1 FROM serve.published_receipt p WHERE p.id=s.receipt_id AND p.raw_object_id=s.raw_object_id OFFSET 0)
AND NOT EXISTS (
    SELECT 1 FROM canon.doc_search newer
    JOIN serve.published_receipt np ON np.id=newer.receipt_id AND np.raw_object_id=newer.raw_object_id
    WHERE newer.entity_id=s.entity_id AND newer.valid_from_run>s.valid_from_run
    OFFSET 0
);

COMMIT;
