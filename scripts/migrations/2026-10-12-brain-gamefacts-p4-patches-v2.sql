BEGIN;

CREATE VIEW serve.p4_patch_catalog AS
SELECT e.id AS patch_id, label.value_text AS label,
    CASE WHEN released.value_text ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
        AND pg_input_is_valid(released.value_text::text, 'timestamp with time zone')
        THEN released.value_text::timestamptz END AS released_at,
    e.receipt_id AS official_receipt_id, e.raw_object_id,
    head.ingest_run_id, head.extractor, url.value_text AS official_url,
    label.receipt_id AS label_receipt_id, released.receipt_id AS released_at_receipt_id,
    url.receipt_id AS url_receipt_id
FROM serve.current_entity e
JOIN serve.published_receipt head ON head.id = e.receipt_id
JOIN serve.current_fact label ON label.entity_id = e.id AND label.stat_key = 'label'
    AND label.raw_object_id = e.raw_object_id AND label.valid_from_run = head.ingest_run_id
JOIN serve.published_receipt lr ON lr.id = label.receipt_id AND lr.extractor = head.extractor
    AND lr.raw_object_id = head.raw_object_id AND lr.ingest_run_id = head.ingest_run_id
    AND lr.locator_kind = 'json_pointer' AND lr.locator = '/title'
JOIN serve.current_fact released ON released.entity_id = e.id AND released.stat_key = 'released_at'
    AND released.raw_object_id = e.raw_object_id AND released.valid_from_run = head.ingest_run_id
JOIN serve.published_receipt rr ON rr.id = released.receipt_id AND rr.extractor = head.extractor
    AND rr.raw_object_id = head.raw_object_id AND rr.ingest_run_id = head.ingest_run_id
    AND rr.locator_kind = 'json_pointer' AND rr.locator = '/posted_at'
JOIN serve.current_fact url ON url.entity_id = e.id AND url.stat_key = 'official_url'
    AND url.raw_object_id = e.raw_object_id AND url.valid_from_run = head.ingest_run_id
JOIN serve.published_receipt ur ON ur.id = url.receipt_id AND ur.extractor = head.extractor
    AND ur.raw_object_id = head.raw_object_id AND ur.ingest_run_id = head.ingest_run_id
    AND ur.locator_kind = 'json_pointer' AND ur.locator = '/url'
WHERE e.kind = 'patch' AND e.key ~ '^patch_[1-9][0-9]*$'
    AND head.source = 'deadlock_patchnotes_db'
    AND head.locator_kind = 'raw_bytes' AND head.locator = ''
    AND head.value_sha256 = head.content_sha256
    AND label.value_text IS NOT NULL AND btrim(label.value_text::text) <> ''
    AND url.value_text IS NOT NULL AND btrim(url.value_text::text) <> ''
    AND released.value_text ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
    AND pg_input_is_valid(released.value_text::text, 'timestamp with time zone');

CREATE VIEW serve.p4_patch_change AS
SELECT f.id AS change_id, p.patch_id, binding.entity_id,
    stat.stat_key, payload.value -> 'old' AS old_value, payload.value -> 'new' AS new_value,
    'statement'::text AS change_kind, payload.value ->> 'line_text' AS line_text,
    f.receipt_id, receipt.source_revision,
    binding.alias_receipt_id AS binding_receipt_id, binding.entity_receipt_id, stat.receipt_id AS stat_receipt_id,
    payload.value ->> 'subject' AS subject, payload.value -> 'statement_index' AS statement_index,
    payload.value -> 'context_index' AS context_index, payload.value ->> 'stat_label' AS stat_label
FROM serve.p4_patch_catalog p
JOIN serve.current_fact f ON f.entity_id = p.patch_id AND f.stat_key ~ '^patch_change_[0-9]+$'
    AND f.raw_object_id = p.raw_object_id AND f.valid_from_run = p.ingest_run_id
JOIN serve.published_receipt receipt ON receipt.id = f.receipt_id AND receipt.extractor = p.extractor
    AND receipt.raw_object_id = p.raw_object_id AND receipt.ingest_run_id = p.ingest_run_id
    AND receipt.locator_kind = 'json_pointer' AND receipt.locator = '/raw_content'
CROSS JOIN LATERAL (
    SELECT CASE WHEN pg_input_is_valid(f.value_text::text, 'jsonb')
        THEN f.value_text::jsonb ELSE '{}'::jsonb END AS value
) payload
JOIN LATERAL (
    SELECT min(e.id) AS entity_id, min(a.receipt_id) AS alias_receipt_id,
        min(e.receipt_id) AS entity_receipt_id
    FROM serve.current_alias a
    JOIN serve.current_entity e ON e.id = a.entity_id AND e.kind = 'hero'
        AND e.raw_object_id = a.raw_object_id
    JOIN serve.published_receipt ar ON ar.id = a.receipt_id
    JOIN serve.published_receipt er ON er.id = e.receipt_id
        AND er.ingest_run_id = ar.ingest_run_id AND er.extractor = ar.extractor
    WHERE a.alias_norm = payload.value ->> 'reference'
    HAVING count(DISTINCT e.id) = 1
) binding ON true
LEFT JOIN LATERAL (
    SELECT hf.stat_key, hf.receipt_id
    FROM serve.current_fact hf
    JOIN serve.current_entity he ON he.id = hf.entity_id AND he.raw_object_id = hf.raw_object_id
    JOIN serve.published_receipt hr ON hr.id = he.receipt_id AND hr.ingest_run_id = hf.valid_from_run
    JOIN serve.published_receipt fr ON fr.id = hf.receipt_id AND fr.extractor = hr.extractor
        AND fr.raw_object_id = hr.raw_object_id AND fr.ingest_run_id = hr.ingest_run_id
    WHERE hf.entity_id = binding.entity_id AND hf.stat_key = payload.value ->> 'stat_label'
) stat ON true
WHERE jsonb_typeof(payload.value) = 'object'
    AND jsonb_typeof(payload.value -> 'reference') = 'string'
    AND jsonb_typeof(payload.value -> 'subject') = 'string'
    AND jsonb_typeof(payload.value -> 'line_text') = 'string'
    AND btrim(payload.value ->> 'line_text') <> ''
    AND jsonb_typeof(payload.value -> 'statement_index') = 'number'
    AND payload.value ->> 'statement_index' = substring(f.stat_key::text FROM 14);

CREATE VIEW serve.p4_patch_receipt AS
SELECT p.id AS receipt_id, p.source AS source_id, p.ingest_run_id AS evidence_revision,
    jsonb_build_object(
        'citation', p.id::text, 'raw_object_id', p.raw_object_id,
        'ingest_run_id', p.ingest_run_id, 'source', p.source,
        'source_revision', p.source_revision, 'path', p.path,
        'content_sha256', encode(p.content_sha256, 'hex'), 'extractor', p.extractor,
        'locator_kind', p.locator_kind, 'locator', p.locator,
        'value_sha256', encode(p.value_sha256, 'hex'), 'patch', NULL
    ) AS receipt
FROM serve.published_receipt p;

GRANT SELECT ON TABLE serve.p4_patch_catalog, serve.p4_patch_change,
    serve.p4_patch_receipt TO brain_tool;

COMMIT;
