BEGIN;

CREATE FUNCTION serve.site_doc_page(requested_path text)
RETURNS TABLE(title text,body text,source_revision text,receipt_id bigint,ordinal integer)
LANGUAGE plpgsql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $page$
DECLARE page bigint;
BEGIN
    FOR page IN SELECT DISTINCT f.entity_id FROM canon.fact f
        WHERE f.stat_key COLLATE "C"='doc_path' AND f.value_text::text=requested_path LOOP
        RETURN QUERY
        WITH facts AS MATERIALIZED (
            SELECT f.stat_key::text AS stat_key,f.raw_object_id,f.valid_from_run,f.receipt_id,
                f.value_text::text AS value_text,r.locator,r.locator_kind,r.value_sha256
            FROM serve.current_fact f
            CROSS JOIN LATERAL (
                SELECT r.locator,r.locator_kind,r.value_sha256 FROM serve.published_receipt r
                WHERE r.id=f.receipt_id AND r.raw_object_id=f.raw_object_id
                OFFSET 0
            ) r
            WHERE f.entity_id=page AND f.stat_key::text ~ '^doc_(intro|section_.+)_(meta|body_[0-9]+)$'
        ),
        metadata AS MATERIALIZED (
            SELECT p.page_id,p.raw_object_id,p.path,p.title,p.scope,p.source_revision,p.section_count,
                m.stat_key AS meta_key,m.receipt_id AS meta_receipt_id,m.valid_from_run AS meta_run,
                serve.doc_meta(m.value_text) AS meta,m.locator,m.locator_kind,m.value_sha256
            FROM serve.doc_page p JOIN facts m ON m.raw_object_id=p.raw_object_id
            WHERE p.page_id=page AND m.stat_key ~ '^doc_(intro|section_.+)_meta$'
                AND m.locator_kind='byte_ranges'
        ),
        complete AS MATERIALIZED (
            SELECT m.* FROM metadata m WHERE m.meta IS NOT NULL
                AND (m.meta->>'ordinal')::integer<m.section_count
                AND (SELECT count(*) FROM metadata)=m.section_count
                AND NOT EXISTS (SELECT 1 FROM metadata other
                    WHERE other.meta IS NULL OR (other.meta->>'ordinal')::integer>=m.section_count)
                AND (SELECT count(DISTINCT other.meta->>'ordinal') FROM metadata other)=m.section_count
        )
        SELECT s.title::text,s.body::text,m.source_revision::text,s.receipt_id,(m.meta->>'ordinal')::integer
        FROM complete m
        JOIN canon.doc_search s ON s.entity_id=page AND s.valid_from_run=m.meta_run
            AND s.meta_key=m.meta_key AND s.receipt_id=m.meta_receipt_id
            AND s.raw_object_id=m.raw_object_id AND s.title=m.title
        WHERE m.scope='public' AND m.path=requested_path
            AND (SELECT count(*) FROM generate_series(0,(m.meta->>'chunks')::integer-1) expected(chunk)
                JOIN facts f ON f.raw_object_id=m.raw_object_id AND f.valid_from_run=m.meta_run
                    AND f.stat_key=left(m.meta_key,length(m.meta_key)-5)||'_body_'||expected.chunk
                    AND f.locator_kind=m.locator_kind AND f.locator=m.locator AND f.value_sha256=m.value_sha256
                WHERE f.value_text IS NOT NULL)=(m.meta->>'chunks')::integer;
    END LOOP;
END
$page$;
REVOKE ALL ON FUNCTION serve.site_doc_page(text) FROM PUBLIC;
DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.site_doc_page(text) TO brain_site;
    END IF;
END
$site$;

COMMIT;
