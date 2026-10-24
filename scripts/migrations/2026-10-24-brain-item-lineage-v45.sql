BEGIN;

CREATE VIEW serve.item_lineage_entry AS
SELECT substring(f.stat_key::text FROM '^entry_([0-9]+)$')::integer AS ordinal,
    entry.value AS entry,item.id AS item_id,successor.id AS successor_id,
    f.receipt_id,f.source_revision
FROM canon.entity lineage
CROSS JOIN LATERAL (
    SELECT e.receipt_id,e.raw_object_id,e.source FROM serve.current_entity e
    WHERE e.id=lineage.id AND e.source='item-lineage'
    OFFSET 0
) e
CROSS JOIN LATERAL (
    SELECT head.ingest_run_id,head.extractor FROM serve.published_receipt head
    WHERE head.id=e.receipt_id AND head.locator_kind='raw_bytes' AND head.locator=''
        AND head.value_sha256=head.content_sha256
    OFFSET 0
) head
CROSS JOIN LATERAL (
    SELECT f.stat_key,f.value_text,f.receipt_id,f.source_revision FROM serve.current_fact f
    WHERE f.entity_id=lineage.id AND f.raw_object_id=e.raw_object_id
        AND f.valid_from_run=head.ingest_run_id AND f.value_text IS NOT NULL
        AND f.stat_key::text ~ '^entry_[0-9]+$'
    OFFSET 0
) f
CROSS JOIN LATERAL (
    SELECT 1 FROM serve.published_receipt r
    WHERE r.id=f.receipt_id AND r.ingest_run_id=head.ingest_run_id
        AND r.extractor=head.extractor AND r.raw_object_id=e.raw_object_id
        AND r.locator_kind='json_pointer'
        AND r.locator='/entries/'||substring(f.stat_key::text FROM '^entry_([0-9]+)$')
    OFFSET 0
) receipt
CROSS JOIN LATERAL (
    SELECT CASE WHEN pg_input_is_valid(f.value_text::text,'jsonb')
        THEN f.value_text::text::jsonb END AS value
) entry
JOIN canon.entity item ON item.key=entry.value->>'item' AND item.kind='item'
LEFT JOIN canon.entity successor ON successor.key=entry.value->>'successor'
    AND successor.kind='item'
WHERE lineage.key='item_lineage' AND lineage.kind='lineage'
    AND jsonb_typeof(entry.value)='object'
    AND entry.value->>'event' IN('renamed','removed')
    AND (entry.value->>'successor' IS NULL OR successor.id IS NOT NULL);
REVOKE ALL ON serve.item_lineage_entry FROM PUBLIC;

CREATE FUNCTION serve.item_names(target bigint)
RETURNS jsonb
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $names$
    SELECT jsonb_build_object('entity_id',e.id,'key',e.key,
        'name_en',(SELECT btrim(f.value_text::text) FROM serve.current_fact f
            WHERE f.entity_id=e.id AND f.stat_key='name_en' AND f.source='assets' LIMIT 1),
        'name_de',(SELECT btrim(f.value_text::text) FROM serve.current_fact f
            WHERE f.entity_id=e.id AND f.stat_key='name_de' AND f.source='assets' LIMIT 1))
    FROM canon.entity e WHERE e.id=target AND e.kind='item'
$names$;
REVOKE ALL ON FUNCTION serve.item_names(bigint) FROM PUBLIC;

CREATE FUNCTION serve.item_patch_lines(target bigint,max_rows integer)
RETURNS TABLE(prefix text,patch_id bigint,patch_label text,released_at timestamptz,
    official_url text,line_text text,subject text,renamed_to text,line_index integer,
    piece_index integer,citation text,source_revision text)
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $lines$
    WITH names AS MATERIALIZED (
        SELECT DISTINCT a.alias_norm::text AS alias_norm FROM serve.current_alias a
        WHERE a.entity_id=target AND a.lang='en'
        LIMIT 32
    ), candidates AS MATERIALIZED (
        SELECT kind.prefix,f.entity_id,f.raw_object_id,f.valid_from_run,f.receipt_id,
            names.alias_norm,CASE WHEN pg_input_is_valid(f.value_text::text,'jsonb')
                THEN f.value_text::text::jsonb END AS payload,
            f.stat_key::text AS stat_key
        FROM names
        CROSS JOIN (VALUES ('patch_line_r'),('patch_rename_r')) AS kind(prefix)
        CROSS JOIN LATERAL (
            SELECT f.* FROM serve.current_fact f
            WHERE f.stat_key COLLATE "C" >=
                    (kind.prefix||encode(sha256(convert_to(names.alias_norm,'UTF8')),'hex')||'_') COLLATE "C"
                AND f.stat_key COLLATE "C" <
                    (kind.prefix||encode(sha256(convert_to(names.alias_norm,'UTF8')),'hex')||'`') COLLATE "C"
                AND f.value_text IS NOT NULL
            LIMIT 400
        ) f
    )
    SELECT DISTINCT ON (r.released_at,r.patch_id,r.line_index,r.piece_index)
        r.prefix,r.patch_id,r.patch_label,r.released_at,r.official_url,r.line_text,r.subject,
        r.renamed_to,r.line_index,r.piece_index,r.citation,r.source_revision
    FROM (
        SELECT c.prefix,p.patch_id,btrim(p.label) AS patch_label,p.released_at,
            btrim(p.official_url) AS official_url,btrim(c.payload->>'line_text') AS line_text,
            c.payload->>'subject' AS subject,c.payload->>'renamed_to' AS renamed_to,
            (c.payload->>'line_index')::integer AS line_index,
            (c.payload->>'piece_index')::integer AS piece_index,
            c.receipt_id::text AS citation,receipt.source_revision
        FROM candidates c
        CROSS JOIN LATERAL (
            SELECT p.patch_id,p.label,p.released_at,p.official_url,p.ingest_run_id,p.extractor
            FROM serve.p4_patch_catalog p
            WHERE p.patch_id=c.entity_id AND p.raw_object_id=c.raw_object_id
                AND p.ingest_run_id=c.valid_from_run
            OFFSET 0
        ) p
        CROSS JOIN LATERAL (
            SELECT receipt.source_revision FROM serve.published_receipt receipt
            WHERE receipt.id=c.receipt_id AND receipt.extractor=p.extractor
                AND receipt.raw_object_id=c.raw_object_id
                AND receipt.ingest_run_id=p.ingest_run_id
                AND receipt.locator_kind='json_pointer' AND receipt.locator='/raw_content'
            OFFSET 0
        ) receipt
        WHERE jsonb_typeof(c.payload)='object'
            AND jsonb_typeof(c.payload->'line_text')='string'
            AND NULLIF(btrim(c.payload->>'line_text'),'') IS NOT NULL
            AND jsonb_typeof(c.payload->'subject')='string'
            AND jsonb_typeof(c.payload->'line_index')='number'
            AND jsonb_typeof(c.payload->'piece_index')='number'
            AND c.stat_key ~ '_[0-9]+_[0-9]+$'
            AND c.stat_key=c.prefix||encode(sha256(convert_to(c.alias_norm,'UTF8')),'hex')
                ||'_'||(c.payload->>'line_index')||'_'||(c.payload->>'piece_index')
            AND CASE c.prefix
                WHEN 'patch_line_r' THEN c.payload->>'reference'=c.alias_norm
                ELSE jsonb_typeof(c.payload->'renamed_to')='string'
                    AND lower(btrim(c.payload->>'renamed_to'))=c.alias_norm
            END
    ) r
    ORDER BY r.released_at DESC NULLS LAST,r.patch_id,r.line_index,r.piece_index,r.prefix
    LIMIT max_rows
$lines$;
REVOKE ALL ON FUNCTION serve.item_patch_lines(bigint,integer) FROM PUBLIC;

CREATE FUNCTION serve.item_lineage(target bigint,history_limit integer)
RETURNS jsonb
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $lineage$
    WITH own AS MATERIALIZED (
        SELECT l.* FROM serve.item_lineage_entry l WHERE l.item_id=target
    ), predecessors AS MATERIALIZED (
        SELECT l.* FROM serve.item_lineage_entry l
        WHERE l.successor_id=target AND l.entry->>'event'='removed'
    ), source_entry AS (
        SELECT l.entry,l.receipt_id,l.source_revision,
            jsonb_build_object(
                'patch_label',l.entry->>'patch_label','patch_date',l.entry->>'patch_date',
                'patch_url',l.entry->>'patch_url','source_url',l.entry->>'source_url',
                'quote',l.entry->>'source_quote','citation',l.receipt_id::text,
                'source_revision',l.source_revision,'patch',NULL
            ) AS evidence,l.ordinal,l.item_id,l.successor_id
        FROM (SELECT * FROM own UNION ALL SELECT * FROM predecessors) l
    ), lines AS MATERIALIZED (
        SELECT * FROM serve.item_patch_lines(target,greatest(history_limit,0)+1+32)
    ), patch_renames AS (
        SELECT DISTINCT ON (lower(l.subject)) l.* FROM lines l
        WHERE l.prefix='patch_rename_r' AND l.subject IS NOT NULL
            AND NOT EXISTS(SELECT 1 FROM own o WHERE o.entry->>'event'='renamed'
                AND lower(o.entry->>'former_name')=lower(l.subject))
        ORDER BY lower(l.subject),l.released_at DESC NULLS LAST
    ), history AS (
        SELECT DISTINCT ON (l.patch_id,l.line_index,l.piece_index) l.* FROM lines l
        ORDER BY l.patch_id,l.line_index,l.piece_index,l.prefix
    ), history_rows AS (
        SELECT h.*,row_number() OVER (ORDER BY h.released_at DESC NULLS LAST,h.patch_id,
            h.line_index,h.piece_index) AS position,count(*) OVER () AS total
        FROM history h
    )
    SELECT jsonb_build_object(
        'item',serve.item_names(target),
        'status',CASE WHEN EXISTS(SELECT 1 FROM own WHERE entry->>'event'='removed')
            THEN 'removed' ELSE 'current' END,
        'removed',(SELECT s.evidence||jsonb_build_object('successor',
                CASE WHEN s.successor_id IS NOT NULL THEN serve.item_names(s.successor_id) END)
            FROM source_entry s WHERE s.item_id=target AND s.entry->>'event'='removed'
            ORDER BY s.ordinal LIMIT 1),
        'former_names',COALESCE((
            SELECT jsonb_agg(n.value ORDER BY n.patch_date DESC NULLS LAST,n.name)
            FROM (
                SELECT s.entry->>'former_name' AS name,s.entry->>'patch_date' AS patch_date,
                    s.evidence||jsonb_build_object('name',s.entry->>'former_name',
                        'lang',s.entry->>'lang','origin','lineage') AS value
                FROM source_entry s
                WHERE s.item_id=target AND s.entry->>'event'='renamed'
                UNION ALL
                SELECT r.subject,to_char(r.released_at AT TIME ZONE 'UTC','YYYY-MM-DD'),
                    jsonb_build_object('name',r.subject,'lang','en','origin','patch_note',
                        'patch_label',r.patch_label,
                        'patch_date',to_char(r.released_at AT TIME ZONE 'UTC','YYYY-MM-DD'),
                        'patch_url',r.official_url,'source_url',r.official_url,
                        'quote',r.line_text,'citation',r.citation,
                        'source_revision',r.source_revision,'patch',NULL)
                FROM patch_renames r
            ) n
        ),'[]'::jsonb),
        'predecessors',COALESCE((
            SELECT jsonb_agg(serve.item_names(s.item_id)||jsonb_build_object('removed',s.evidence)
                ORDER BY s.ordinal)
            FROM source_entry s WHERE s.successor_id=target AND s.item_id<>target
        ),'[]'::jsonb),
        'history',jsonb_build_object(
            'changes',COALESCE((
                SELECT jsonb_agg(jsonb_build_object(
                    'patch_label',h.patch_label,
                    'patch_date',to_char(h.released_at AT TIME ZONE 'UTC','YYYY-MM-DD'),
                    'patch_url',h.official_url,'subject',h.subject,'line_text',h.line_text,
                    'renamed_to',h.renamed_to,'citation',h.citation,
                    'source_revision',h.source_revision,'patch',NULL
                ) ORDER BY h.position)
                FROM history_rows h WHERE h.position<=greatest(history_limit,0)
            ),'[]'::jsonb),
            'truncated',COALESCE((SELECT max(h.total) FROM history_rows h),0)>greatest(history_limit,0),
            'omitted_count',greatest(COALESCE((SELECT max(h.total) FROM history_rows h),0)
                -greatest(history_limit,0),0)
        )
    )
    FROM canon.entity e WHERE e.id=target AND e.kind='item'
$lineage$;
REVOKE ALL ON FUNCTION serve.item_lineage(bigint,integer) FROM PUBLIC;

CREATE FUNCTION serve.item_rename_target(requested text)
RETURNS TABLE(entity_id bigint,alias_norm text,lang text,receipt_id bigint,source_revision text)
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $rename$
    SELECT DISTINCT ON (target.entity_id) target.entity_id,requested,'en'::text,
        line.receipt_id,line.source_revision
    FROM (
        SELECT f.receipt_id,f.raw_object_id,f.valid_from_run,f.entity_id AS patch_id,
            f.source_revision,f.value_text::text::jsonb AS payload
        FROM serve.current_fact f
        WHERE f.stat_key COLLATE "C" >=
                ('patch_line_r'||encode(sha256(convert_to(requested,'UTF8')),'hex')||'_') COLLATE "C"
            AND f.stat_key COLLATE "C" <
                ('patch_line_r'||encode(sha256(convert_to(requested,'UTF8')),'hex')||'`') COLLATE "C"
            AND f.value_text IS NOT NULL AND pg_input_is_valid(f.value_text::text,'jsonb')
        LIMIT 64
    ) line
    CROSS JOIN LATERAL (
        SELECT p.released_at FROM serve.p4_patch_catalog p
        WHERE p.patch_id=line.patch_id AND p.raw_object_id=line.raw_object_id
            AND p.ingest_run_id=line.valid_from_run
        OFFSET 0
    ) p
    CROSS JOIN LATERAL (
        SELECT a.entity_id FROM serve.current_alias a
        JOIN canon.entity e ON e.id=a.entity_id AND e.kind='item'
        WHERE a.alias_norm=lower(btrim(line.payload->>'renamed_to')) AND a.lang='en'
        LIMIT 4
    ) target
    WHERE jsonb_typeof(line.payload)='object'
        AND line.payload->>'reference'=requested
        AND jsonb_typeof(line.payload->'renamed_to')='string'
        AND requested=lower(btrim(requested))
    ORDER BY target.entity_id,p.released_at DESC NULLS LAST,line.receipt_id
    LIMIT 4
$rename$;
REVOKE ALL ON FUNCTION serve.item_rename_target(text) FROM PUBLIC;

CREATE FUNCTION serve.site_item_lineage(requested_key text)
RETURNS jsonb
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $site$
    SELECT serve.item_lineage(e.id,120) FROM canon.entity e
    WHERE e.key=requested_key AND e.kind='item'
$site$;
REVOKE ALL ON FUNCTION serve.site_item_lineage(text) FROM PUBLIC;

GRANT EXECUTE ON FUNCTION serve.item_lineage(bigint,integer),
    serve.item_rename_target(text) TO brain_tool,brain_service;

DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.site_item_lineage(text) TO brain_site;
    END IF;
END
$site$;

COMMIT;
