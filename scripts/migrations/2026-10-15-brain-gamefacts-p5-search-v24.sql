BEGIN;

CREATE TABLE canon.doc_search (
    entity_id bigint NOT NULL REFERENCES canon.entity(id),
    valid_from_run bigint NOT NULL REFERENCES ops.ingest_run(id),
    meta_key text NOT NULL,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    raw_object_id bigint NOT NULL REFERENCES raw.object(id),
    title text NOT NULL,
    body text NOT NULL CHECK(octet_length(body) BETWEEN 1 AND 262144),
    search_text text NOT NULL,
    tsv tsvector GENERATED ALWAYS AS (to_tsvector('pg_catalog.simple'::regconfig,search_text)) STORED,
    tsv_german tsvector GENERATED ALWAYS AS (to_tsvector('pg_catalog.german'::regconfig,search_text)) STORED,
    PRIMARY KEY(entity_id,valid_from_run,meta_key)
);
CREATE INDEX doc_search_simple_gin ON canon.doc_search USING gin(tsv);
CREATE INDEX doc_search_german_gin ON canon.doc_search USING gin(tsv_german);
REVOKE ALL ON canon.doc_search FROM PUBLIC,brain_tool,brain_service,brain_ingest;

CREATE FUNCTION canon.refresh_doc_search(p_entity bigint,p_run bigint,p_key text) RETURNS void
LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $refresh$
BEGIN
    IF NOT EXISTS(SELECT 1 FROM canon.entity WHERE id=p_entity AND kind IN('doc_page','doc')) THEN
        RETURN;
    END IF;
    DELETE FROM canon.doc_search WHERE entity_id=p_entity AND valid_from_run=p_run
        AND (p_key IS NULL OR meta_key=p_key);
    IF p_key IS NULL AND (SELECT count(*) FROM (
        SELECT 1 FROM canon.fact WHERE entity_id=p_entity AND valid_from_run=p_run LIMIT 4097
    ) bounded)>4096 THEN
        RETURN;
    END IF;
    INSERT INTO canon.doc_search(entity_id,valid_from_run,meta_key,receipt_id,raw_object_id,title,body,search_text)
    SELECT m.entity_id,m.valid_from_run,m.stat_key,m.receipt_id,r.raw_object_id,t.value_text,chunks.body,
        t.value_text||' '||array_to_string(ARRAY(SELECT jsonb_array_elements_text(meta.value->'heading_path')),' ')||' '||chunks.body
    FROM canon.fact m
    JOIN canon.receipt r ON r.id=m.receipt_id AND r.ingest_run_id=m.valid_from_run
    JOIN raw.object o ON o.id=r.raw_object_id AND o.source='deadlock-docs'
    JOIN canon.fact t ON t.entity_id=m.entity_id AND t.valid_from_run=m.valid_from_run AND t.stat_key='doc_title'
    JOIN canon.receipt tr ON tr.id=t.receipt_id AND tr.raw_object_id=r.raw_object_id
    CROSS JOIN LATERAL (SELECT serve.doc_meta(m.value_text::text) AS value) meta
    CROSS JOIN LATERAL (
        SELECT string_agg(f.value_text::text,'' ORDER BY expected.chunk) AS body
        FROM generate_series(0,(meta.value->>'chunks')::integer-1) expected(chunk)
        JOIN canon.fact f ON f.entity_id=m.entity_id AND f.valid_from_run=m.valid_from_run
            AND f.stat_key::text=left(m.stat_key::text,length(m.stat_key::text)-5)||'_body_'||expected.chunk
        JOIN canon.receipt br ON br.id=f.receipt_id AND br.raw_object_id=r.raw_object_id
            AND br.locator_kind=r.locator_kind AND br.locator=r.locator AND br.value_sha256=r.value_sha256
        WHERE f.value_text IS NOT NULL
        HAVING count(*)=(meta.value->>'chunks')::integer AND sum(octet_length(f.value_text)) BETWEEN 1 AND 262144
    ) chunks
    WHERE m.entity_id=p_entity AND m.valid_from_run=p_run
        AND m.stat_key::text ~ '^doc_(intro|section_.+)_meta$'
        AND (p_key IS NULL OR m.stat_key::text=p_key)
        AND meta.value IS NOT NULL AND chunks.body IS NOT NULL AND t.value_text IS NOT NULL;
END
$refresh$;
REVOKE ALL ON FUNCTION canon.refresh_doc_search(bigint,bigint,text) FROM PUBLIC,brain_tool,brain_service,brain_ingest;

CREATE FUNCTION canon.write_doc_search() RETURNS trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $writer$
DECLARE section_key text;
BEGIN
    IF TG_OP='UPDATE' THEN
        PERFORM canon.refresh_doc_search(OLD.entity_id,OLD.valid_from_run,NULL);
    END IF;
    IF NOT EXISTS(SELECT 1 FROM canon.entity WHERE id=NEW.entity_id AND kind IN('doc_page','doc')) THEN
        RETURN NEW;
    END IF;
    IF NEW.stat_key='doc_title' THEN
        section_key:=NULL;
    ELSIF NEW.stat_key::text ~ '^doc_(intro|section_.+)_meta$' THEN
        section_key:=NEW.stat_key::text;
    ELSIF NEW.stat_key::text ~ '^doc_(intro|section_.+)_body_[0-9]+$' THEN
        section_key:=regexp_replace(NEW.stat_key::text,'_body_[0-9]+$','_meta');
    ELSE
        RETURN NEW;
    END IF;
    PERFORM canon.refresh_doc_search(NEW.entity_id,NEW.valid_from_run,section_key);
    RETURN NEW;
END
$writer$;
REVOKE ALL ON FUNCTION canon.write_doc_search() FROM PUBLIC,brain_tool,brain_service,brain_ingest;
CREATE TRIGGER write_doc_search AFTER INSERT OR UPDATE ON canon.fact
FOR EACH ROW EXECUTE FUNCTION canon.write_doc_search();

DO $backfill$
DECLARE doc record;
BEGIN
    FOR doc IN SELECT DISTINCT f.entity_id,f.valid_from_run FROM canon.fact f
        JOIN canon.entity e ON e.id=f.entity_id AND e.kind IN('doc_page','doc')
        WHERE f.stat_key='doc_title' LOOP
        PERFORM canon.refresh_doc_search(doc.entity_id,doc.valid_from_run,NULL);
    END LOOP;
END
$backfill$;

CREATE OR REPLACE VIEW serve.doc_section AS
SELECT m.page_id,m.raw_object_id,m.path,m.title,m.lang,m.scope,m.source_revision,m.receipt_id,m.valid_from_run,
    m.meta_key,(m.meta->>'ordinal')::integer AS ordinal,
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

COMMIT;
