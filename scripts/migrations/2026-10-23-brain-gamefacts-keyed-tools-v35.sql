BEGIN;

CREATE OR REPLACE VIEW serve.timer AS
SELECT a.alias_norm AS key,e.id AS entity_id,f.stat_key,
    f.value_text::jsonb->'value' AS value,
    f.value_text::jsonb->>'unit' AS unit,
    f.value_text::jsonb->'condition' AS condition,
    f.receipt_id,f.source,f.source_revision,f.id
FROM serve.current_alias a
CROSS JOIN LATERAL (
    SELECT e.id,e.source,e.raw_object_id FROM serve.current_entity e
    WHERE e.id=a.entity_id AND e.raw_object_id=a.raw_object_id AND e.source=a.source
        AND e.kind='timer'
    OFFSET 0
) e
CROSS JOIN LATERAL (
    SELECT f.* FROM serve.current_fact f
    WHERE f.entity_id=e.id AND f.raw_object_id=e.raw_object_id AND f.source=e.source
        AND f.value_text IS NOT NULL
    OFFSET 0
) f
WHERE a.lang='und'
    AND a.alias_norm IN ('urn','soul_box','mid_boss','rejuvenator')
    AND (a.alias_norm <> 'soul_box' OR EXISTS (
        SELECT 1 FROM canon.entity k CROSS JOIN LATERAL (
            SELECT 1 FROM serve.current_entity snapshot
            WHERE snapshot.id=k.id AND snapshot.source=e.source
                AND snapshot.raw_object_id=e.raw_object_id
            OFFSET 0
        ) snapshot
        WHERE k.key='timer_soul_box__'
            ||replace(e.source,'-','_')||'_citadel_breakable_prop_drop_gold'
    ));

CREATE OR REPLACE VIEW serve.p4_patch_catalog AS
SELECT lf.entity_id AS patch_id, m.label,
    CASE WHEN pg_input_is_valid(m.released_at, 'timestamp with time zone')
        THEN m.released_at::timestamptz END AS released_at,
    e.receipt_id AS official_receipt_id, e.raw_object_id,
    head.ingest_run_id, head.extractor, m.official_url,
    m.label_receipt_id, m.released_at_receipt_id, m.url_receipt_id
FROM serve.current_fact lf
CROSS JOIN LATERAL (
    SELECT e.* FROM serve.current_entity e
    WHERE e.id=lf.entity_id AND e.kind='patch' AND e.key ~ '^patch_[1-9][0-9]*$'
    OFFSET 0
) e
CROSS JOIN LATERAL (
    SELECT head.* FROM serve.published_receipt head
    WHERE head.id = e.receipt_id AND head.source = 'deadlock_patchnotes_db'
        AND head.locator_kind = 'raw_bytes' AND head.locator = ''
        AND head.value_sha256 = head.content_sha256
    OFFSET 0
) head
JOIN LATERAL (
    SELECT f.entity_id, min(f.raw_object_id) AS raw_object_id,
        min(f.valid_from_run) AS valid_from_run, min(r.extractor) AS extractor,
        max(f.value_text::text) FILTER (WHERE f.stat_key = 'label') AS label,
        max(f.value_text::text) FILTER (WHERE f.stat_key = 'released_at') AS released_at,
        max(f.value_text::text) FILTER (WHERE f.stat_key = 'official_url') AS official_url,
        max(f.receipt_id) FILTER (WHERE f.stat_key = 'label') AS label_receipt_id,
        max(f.receipt_id) FILTER (WHERE f.stat_key = 'released_at') AS released_at_receipt_id,
        max(f.receipt_id) FILTER (WHERE f.stat_key = 'official_url') AS url_receipt_id
    FROM serve.current_fact f
    CROSS JOIN LATERAL (
        SELECT r.extractor FROM serve.published_receipt r
        WHERE r.id = f.receipt_id AND r.raw_object_id = f.raw_object_id
            AND r.ingest_run_id = f.valid_from_run AND r.locator_kind = 'json_pointer'
            AND r.source = 'deadlock_patchnotes_db'
            AND ((f.stat_key = 'label' AND r.locator = '/title')
                OR (f.stat_key = 'released_at' AND r.locator = '/posted_at')
                OR (f.stat_key = 'official_url' AND r.locator = '/url'))
        OFFSET 0
    ) r
    WHERE f.entity_id = e.id
        AND f.stat_key COLLATE "C" IN ('label', 'released_at', 'official_url')
    GROUP BY f.entity_id
    HAVING count(*) = 3 AND min(f.raw_object_id) = max(f.raw_object_id)
        AND min(f.valid_from_run) = max(f.valid_from_run)
        AND min(r.extractor) = max(r.extractor)
    OFFSET 0
) m ON m.raw_object_id = e.raw_object_id
    AND m.valid_from_run = head.ingest_run_id AND m.extractor = head.extractor
WHERE lf.stat_key COLLATE "C" = 'label'
    AND m.label IS NOT NULL AND btrim(m.label) <> ''
    AND m.official_url IS NOT NULL AND btrim(m.official_url) <> ''
    AND m.released_at ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
    AND pg_input_is_valid(m.released_at, 'timestamp with time zone');

CREATE OR REPLACE VIEW serve.p4_patch_change AS
WITH coherent_aliases AS NOT MATERIALIZED (
        SELECT e.id AS entity_id, a.alias_norm, a.receipt_id AS alias_receipt_id,
            e.receipt_id AS entity_receipt_id
        FROM serve.current_entity e
        CROSS JOIN LATERAL (
            SELECT a.alias_norm, a.receipt_id FROM serve.current_alias a
            WHERE a.entity_id = e.id AND a.raw_object_id = e.raw_object_id
            OFFSET 0
        ) a
        CROSS JOIN LATERAL (
            SELECT ar.ingest_run_id, ar.extractor FROM serve.published_receipt ar
            WHERE ar.id = a.receipt_id
            OFFSET 0
        ) ar
        CROSS JOIN LATERAL (
            SELECT 1 FROM serve.published_receipt er
            WHERE er.id = e.receipt_id
                AND er.ingest_run_id = ar.ingest_run_id AND er.extractor = ar.extractor
            OFFSET 0
        ) er
        WHERE e.kind = 'hero'
    ),
    bindings AS NOT MATERIALIZED (
        SELECT a.alias_norm, a.entity_id, min(a.alias_receipt_id) AS alias_receipt_id,
            a.entity_receipt_id
        FROM coherent_aliases a
        WHERE NOT EXISTS (
            SELECT 1 FROM serve.current_alias ca
            CROSS JOIN LATERAL (
                SELECT ce.receipt_id FROM serve.current_entity ce
                WHERE ce.id = ca.entity_id AND ce.raw_object_id = ca.raw_object_id
                    AND ce.kind = 'hero'
                OFFSET 0
            ) ce
            CROSS JOIN LATERAL (
                SELECT 1 FROM serve.published_receipt car
                JOIN serve.published_receipt cer ON cer.id = ce.receipt_id
                    AND cer.ingest_run_id = car.ingest_run_id AND cer.extractor = car.extractor
                WHERE car.id = ca.receipt_id
                OFFSET 0
            ) coherent
            WHERE ca.alias_norm = a.alias_norm AND ca.entity_id <> a.entity_id
            OFFSET 0
        )
        GROUP BY a.alias_norm, a.entity_id, a.entity_receipt_id
    ),
    resolved AS NOT MATERIALIZED (
        SELECT f.*, payload.value AS payload, binding.entity_id AS bound_entity_id,
            binding.alias_receipt_id, binding.entity_receipt_id
        FROM bindings binding
        JOIN LATERAL (
            SELECT * FROM serve.current_fact candidate
            WHERE candidate.stat_key COLLATE "C" >=
                    ('patch_change_r' || encode(sha256(convert_to(binding.alias_norm::text, 'UTF8')), 'hex') || '_') COLLATE "C"
                AND candidate.stat_key COLLATE "C" <
                    ('patch_change_r' || encode(sha256(convert_to(binding.alias_norm::text, 'UTF8')), 'hex') || '`') COLLATE "C"
                AND candidate.stat_key ~ '^patch_change_r[0-9a-f]{64}_[0-9]+$'
            OFFSET 0
        ) f ON true
        CROSS JOIN LATERAL (
            SELECT CASE WHEN pg_input_is_valid(f.value_text::text, 'jsonb')
                THEN f.value_text::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
        ) payload
        WHERE jsonb_typeof(payload.value) = 'object'
            AND jsonb_typeof(payload.value -> 'reference') = 'string'
            AND payload.value ->> 'reference' = binding.alias_norm
            AND jsonb_typeof(payload.value -> 'subject') = 'string'
            AND jsonb_typeof(payload.value -> 'line_text') = 'string'
            AND btrim(payload.value ->> 'line_text') <> ''
            AND jsonb_typeof(payload.value -> 'statement_index') = 'number'
            AND payload.value ->> 'statement_index' = substring(f.stat_key::text FROM '[0-9]+$')
    )
SELECT f.id AS change_id, p.patch_id, f.bound_entity_id AS entity_id,
    stat.stat_key, f.payload -> 'old' AS old_value, f.payload -> 'new' AS new_value,
    'statement'::text AS change_kind, f.payload ->> 'line_text' AS line_text,
    f.receipt_id, receipt.source_revision,
    f.alias_receipt_id AS binding_receipt_id, f.entity_receipt_id, stat.receipt_id AS stat_receipt_id,
    f.payload ->> 'subject' AS subject, f.payload -> 'statement_index' AS statement_index,
    f.payload -> 'context_index' AS context_index, f.payload ->> 'stat_label' AS stat_label
FROM resolved f
CROSS JOIN LATERAL (
    SELECT p.patch_id, p.ingest_run_id, p.extractor, p.raw_object_id FROM serve.p4_patch_catalog p
    WHERE p.patch_id = f.entity_id
        AND p.raw_object_id = f.raw_object_id AND p.ingest_run_id = f.valid_from_run
    OFFSET 0
) p
CROSS JOIN LATERAL (
    SELECT receipt.source_revision FROM serve.published_receipt receipt
    WHERE receipt.id = f.receipt_id AND receipt.extractor = p.extractor
        AND receipt.raw_object_id = p.raw_object_id AND receipt.ingest_run_id = p.ingest_run_id
        AND receipt.locator_kind = 'json_pointer' AND receipt.locator = '/raw_content'
    OFFSET 0
) receipt
LEFT JOIN LATERAL (
    SELECT hf.stat_key, hf.receipt_id
    FROM serve.current_fact hf
    CROSS JOIN LATERAL (
        SELECT he.receipt_id FROM serve.current_entity he
        WHERE he.id = hf.entity_id AND he.raw_object_id = hf.raw_object_id AND he.kind = 'hero'
        OFFSET 0
    ) he
    CROSS JOIN LATERAL (
        SELECT 1 FROM serve.published_receipt hr
        JOIN serve.published_receipt fr ON fr.id = hf.receipt_id AND fr.extractor = hr.extractor
            AND fr.raw_object_id = hr.raw_object_id AND fr.ingest_run_id = hr.ingest_run_id
        WHERE hr.id = he.receipt_id AND hr.ingest_run_id = hf.valid_from_run
        OFFSET 0
    ) published
    WHERE hf.entity_id = f.bound_entity_id AND hf.stat_key = f.payload ->> 'stat_label'
    OFFSET 0
) stat ON true;

COMMIT;
