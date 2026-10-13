BEGIN;

CREATE FUNCTION serve.doc_meta(value text) RETURNS jsonb
LANGUAGE plpgsql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $meta$
DECLARE parsed jsonb; heading jsonb; field text;
BEGIN
    parsed := value::jsonb;
    IF jsonb_typeof(parsed) IS DISTINCT FROM 'object'
        OR jsonb_typeof(parsed->'heading_path') IS DISTINCT FROM 'array'
        OR jsonb_typeof(parsed->'locator') IS DISTINCT FROM 'string' THEN
        RETURN NULL;
    END IF;
    IF jsonb_array_length(parsed->'heading_path') > 16
        OR octet_length(parsed->>'locator') NOT BETWEEN 1 AND 4096 THEN
        RETURN NULL;
    END IF;
    FOREACH field IN ARRAY ARRAY['ordinal','line_start','line_end','chunks'] LOOP
        IF jsonb_typeof(parsed->field) IS DISTINCT FROM 'number' OR parsed->>field !~ '^[0-9]{1,9}$' THEN
            RETURN NULL;
        END IF;
    END LOOP;
    IF (parsed->>'ordinal')::integer > 4095 OR (parsed->>'chunks')::integer NOT BETWEEN 1 AND 4096
        OR (parsed->>'line_start')::integer < 1
        OR (parsed->>'line_end')::integer < (parsed->>'line_start')::integer THEN
        RETURN NULL;
    END IF;
    FOR heading IN SELECT jsonb_array_elements(parsed->'heading_path') LOOP
        IF jsonb_typeof(heading) IS DISTINCT FROM 'string' OR octet_length(heading #>> '{}') NOT BETWEEN 1 AND 256
            OR (heading #>> '{}') ~ '[[:cntrl:]]' THEN
            RETURN NULL;
        END IF;
    END LOOP;
    RETURN parsed;
EXCEPTION WHEN invalid_text_representation THEN
    RETURN NULL;
END
$meta$;

CREATE FUNCTION serve.doc_tsv(value text) RETURNS tsvector
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $fts$
    SELECT to_tsvector('pg_catalog.simple'::regconfig, value)
$fts$;

CREATE INDEX doc_body_fts ON canon.fact USING gin(serve.doc_tsv(value_text::text))
WHERE stat_key::text ~ '^doc_(intro|section_.+)_body_[0-9]+$';

CREATE VIEW serve.doc_page AS
SELECT e.id AS page_id,e.raw_object_id,e.receipt_id AS page_receipt_id,e.source_revision,
    p.value_text::text AS path,t.value_text::text AS title,l.value_text::text AS lang,
    n.value_num::integer AS section_count,
    CASE WHEN p.value_text LIKE 'public/%' THEN 'public' ELSE 'internal' END AS scope
FROM serve.current_entity e
JOIN serve.current_fact p ON p.entity_id=e.id AND p.stat_key='doc_path' AND p.raw_object_id=e.raw_object_id
JOIN serve.current_fact t ON t.entity_id=e.id AND t.stat_key='doc_title' AND t.raw_object_id=e.raw_object_id
JOIN serve.current_fact l ON l.entity_id=e.id AND l.stat_key='doc_lang' AND l.raw_object_id=e.raw_object_id
JOIN serve.current_fact n ON n.entity_id=e.id AND n.stat_key='doc_section_count' AND n.raw_object_id=e.raw_object_id
JOIN serve.published_receipt receipt ON receipt.id=e.receipt_id AND receipt.source='deadlock-docs'
    AND receipt.raw_object_id=e.raw_object_id AND receipt.path=p.value_text
WHERE e.kind='doc_page' AND e.source='deadlock-docs' AND e.source_revision ~ '^[0-9a-f]{40}$'
    AND p.value_text::text ~ '^(public|internal)/.+[.]html$'
    AND p.value_text::text !~ '(^|/)[.]{1,2}(/|$)|//|\\|[[:cntrl:]]'
    AND octet_length(p.value_text) BETWEEN 1 AND 2048
    AND octet_length(t.value_text) BETWEEN 1 AND 512 AND t.value_text::text !~ '[[:cntrl:]]'
    AND octet_length(l.value_text) BETWEEN 1 AND 16384
    AND btrim(l.value_text::text,U&'\0020\00A0\1680\2000\2001\2002\2003\2004\2005\2006\2007\2008\2009\200A\2028\2029\202F\205F\3000')<>''
    AND l.value_text::text !~ U&'[\0001-\001F\007F-\009F]'
    AND n.value_num BETWEEN 1 AND 4096 AND n.value_num=trunc(n.value_num);

CREATE VIEW serve.doc_section AS
WITH metadata AS MATERIALIZED (
    SELECT p.*,m.stat_key::text AS meta_key,m.receipt_id,m.valid_from_run,
        serve.doc_meta(m.value_text::text) AS meta,r.locator,r.locator_kind,r.value_sha256
    FROM serve.doc_page p JOIN serve.current_fact m ON m.entity_id=p.page_id AND m.raw_object_id=p.raw_object_id
    JOIN serve.published_receipt r ON r.id=m.receipt_id AND r.raw_object_id=p.raw_object_id
    WHERE m.stat_key::text ~ '^doc_(intro|section_.+)_meta$' AND r.locator_kind='byte_ranges'
), complete_metadata AS MATERIALIZED (
    SELECT m.* FROM metadata m WHERE m.meta IS NOT NULL
        AND (m.meta->>'ordinal')::integer<m.section_count
        AND (SELECT count(*) FROM metadata other WHERE other.page_id=m.page_id)=m.section_count
        AND NOT EXISTS (SELECT 1 FROM metadata other WHERE other.page_id=m.page_id
            AND (other.meta IS NULL OR (other.meta->>'ordinal')::integer>=m.section_count))
        AND (SELECT count(DISTINCT other.meta->>'ordinal') FROM metadata other WHERE other.page_id=m.page_id)=m.section_count
), bodies AS (
    SELECT m.*,chunks.body
    FROM complete_metadata m
    CROSS JOIN LATERAL (
        SELECT string_agg(f.value_text::text,'' ORDER BY expected.chunk) AS body
        FROM generate_series(0,(m.meta->>'chunks')::integer-1) expected(chunk)
        JOIN serve.current_fact f ON f.entity_id=m.page_id AND f.raw_object_id=m.raw_object_id
            AND f.stat_key::text=left(m.meta_key,length(m.meta_key)-5)||'_body_'||expected.chunk
        JOIN serve.published_receipt r ON r.id=f.receipt_id AND r.raw_object_id=m.raw_object_id
            AND r.locator_kind=m.locator_kind AND r.locator=m.locator AND r.value_sha256=m.value_sha256
        WHERE f.value_text IS NOT NULL
        HAVING count(*)=(m.meta->>'chunks')::integer AND sum(octet_length(f.value_text)) BETWEEN 1 AND 262144
    ) chunks
)
SELECT page_id,raw_object_id,path,title,lang,scope,source_revision,receipt_id,valid_from_run,
    meta_key,(meta->>'ordinal')::integer AS ordinal,
    ARRAY(SELECT jsonb_array_elements_text(meta->'heading_path')) AS heading_path,
    (meta->>'line_start')::integer AS line_start,(meta->>'line_end')::integer AS line_end,
    body,serve.doc_tsv(title||' '||array_to_string(ARRAY(SELECT jsonb_array_elements_text(meta->'heading_path')),' ')||' '||body) AS tsv,
    to_tsvector('pg_catalog.german'::regconfig,title||' '||array_to_string(ARRAY(SELECT jsonb_array_elements_text(meta->'heading_path')),' ')||' '||body) AS tsv_german
FROM bodies WHERE body IS NOT NULL;

REVOKE ALL ON FUNCTION serve.doc_meta(text),serve.doc_tsv(text) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION serve.doc_meta(text),serve.doc_tsv(text) TO brain_tool,brain_service,brain_ingest;
GRANT SELECT ON serve.doc_page,serve.doc_section TO brain_tool,brain_service;

COMMIT;
