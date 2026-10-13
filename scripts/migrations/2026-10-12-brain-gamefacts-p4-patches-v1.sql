BEGIN;

ALTER TABLE canon.fact
    ADD COLUMN valid_from_patch bigint REFERENCES canon.entity(id),
    ADD COLUMN valid_to_patch bigint REFERENCES canon.entity(id);

CREATE INDEX fact_patch_reference ON canon.fact(stat_key COLLATE "C", entity_id);
CREATE INDEX fact_patch_validity ON canon.fact(entity_id, valid_from_patch)
    WHERE valid_from_patch IS NOT NULL;
CREATE INDEX fact_patch_end ON canon.fact(valid_to_patch)
    WHERE valid_to_patch IS NOT NULL;

CREATE FUNCTION canon.check_fact_patch_identity() RETURNS trigger LANGUAGE plpgsql
    SECURITY DEFINER SET search_path = pg_catalog AS $patch$
DECLARE
    patch record;
BEGIN
    IF NEW.valid_from_patch IS NULL AND NEW.valid_to_patch IS NULL THEN
        RETURN NEW;
    END IF;
    FOR patch IN SELECT kind, key FROM canon.entity
        WHERE id IN (NEW.valid_from_patch, NEW.valid_to_patch) ORDER BY id FOR SHARE
    LOOP
        IF patch.kind <> 'patch' OR patch.key !~ '^patch_[1-9][0-9]*$' THEN
            RAISE EXCEPTION 'Gültigkeitsgrenze benötigt eine Patch-Identität' USING ERRCODE='23514';
        END IF;
    END LOOP;
    RETURN NEW;
END;
$patch$;
REVOKE ALL ON FUNCTION canon.check_fact_patch_identity() FROM PUBLIC;
CREATE TRIGGER fact_patch_identity BEFORE INSERT OR UPDATE OF valid_from_patch, valid_to_patch
    ON canon.fact FOR EACH ROW EXECUTE FUNCTION canon.check_fact_patch_identity();

CREATE FUNCTION canon.preserve_fact_patch_identity() RETURNS trigger LANGUAGE plpgsql AS $patch$
BEGIN
    IF (NEW.kind IS DISTINCT FROM OLD.kind OR NEW.key IS DISTINCT FROM OLD.key)
        AND EXISTS (SELECT 1 FROM canon.fact
            WHERE valid_from_patch = NEW.id OR valid_to_patch = NEW.id) THEN
        RAISE EXCEPTION 'Referenzierte Patch-Identität muss erhalten bleiben' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$patch$;
REVOKE ALL ON FUNCTION canon.preserve_fact_patch_identity() FROM PUBLIC;
CREATE TRIGGER preserve_fact_patch_identity BEFORE UPDATE OF kind, key
    ON canon.entity FOR EACH ROW EXECUTE FUNCTION canon.preserve_fact_patch_identity();

CREATE VIEW serve.p4_patch_catalog AS
SELECT e.id AS patch_id, m.label,
    CASE WHEN pg_input_is_valid(m.released_at, 'timestamp with time zone')
        THEN m.released_at::timestamptz END AS released_at,
    e.receipt_id AS official_receipt_id, e.raw_object_id,
    head.ingest_run_id, head.extractor, m.official_url,
    m.label_receipt_id, m.released_at_receipt_id, m.url_receipt_id
FROM serve.current_entity e
JOIN serve.published_receipt head ON head.id = e.receipt_id
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
    JOIN serve.published_receipt r ON r.id = f.receipt_id AND r.raw_object_id = f.raw_object_id
        AND r.ingest_run_id = f.valid_from_run AND r.locator_kind = 'json_pointer'
    WHERE f.entity_id = e.id AND r.source = 'deadlock_patchnotes_db'
        AND f.stat_key COLLATE "C" IN ('label', 'released_at', 'official_url')
        AND ((f.stat_key = 'label' AND r.locator = '/title')
            OR (f.stat_key = 'released_at' AND r.locator = '/posted_at')
            OR (f.stat_key = 'official_url' AND r.locator = '/url'))
    GROUP BY f.entity_id
    HAVING count(*) = 3 AND min(f.raw_object_id) = max(f.raw_object_id)
        AND min(f.valid_from_run) = max(f.valid_from_run)
        AND min(r.extractor) = max(r.extractor)
    OFFSET 0
) m ON m.raw_object_id = e.raw_object_id
    AND m.valid_from_run = head.ingest_run_id AND m.extractor = head.extractor
WHERE e.kind = 'patch' AND e.key ~ '^patch_[1-9][0-9]*$'
    AND head.source = 'deadlock_patchnotes_db'
    AND head.locator_kind = 'raw_bytes' AND head.locator = ''
    AND head.value_sha256 = head.content_sha256
    AND m.label IS NOT NULL AND btrim(m.label) <> ''
    AND m.official_url IS NOT NULL AND btrim(m.official_url) <> ''
    AND m.released_at ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
    AND pg_input_is_valid(m.released_at, 'timestamp with time zone');

CREATE VIEW serve.p4_patch_change AS
WITH published AS NOT MATERIALIZED (SELECT * FROM serve.published_receipt),
    heroes AS NOT MATERIALIZED (SELECT * FROM serve.current_entity WHERE kind = 'hero'),
    aliases AS NOT MATERIALIZED (SELECT * FROM serve.current_alias),
    facts AS NOT MATERIALIZED (SELECT * FROM serve.current_fact),
    catalog AS NOT MATERIALIZED (SELECT * FROM serve.p4_patch_catalog),
    coherent_aliases AS MATERIALIZED (
        SELECT a.alias_norm, e.id AS entity_id, a.receipt_id AS alias_receipt_id,
            e.receipt_id AS entity_receipt_id
        FROM aliases a
        JOIN heroes e ON e.id = a.entity_id AND e.raw_object_id = a.raw_object_id
        JOIN published ar ON ar.id = a.receipt_id
        JOIN published er ON er.id = e.receipt_id
            AND er.ingest_run_id = ar.ingest_run_id AND er.extractor = ar.extractor
    ),
    bindings AS NOT MATERIALIZED (
        SELECT a.alias_norm, a.entity_id, min(a.alias_receipt_id) AS alias_receipt_id,
            a.entity_receipt_id
        FROM coherent_aliases a
        WHERE NOT EXISTS (
            SELECT 1 FROM coherent_aliases conflict
            WHERE conflict.alias_norm = a.alias_norm AND conflict.entity_id <> a.entity_id
        )
        GROUP BY a.alias_norm, a.entity_id, a.entity_receipt_id
    ),
    hero_stats AS NOT MATERIALIZED (
        SELECT hf.entity_id, hf.stat_key, hf.receipt_id
        FROM facts hf
        JOIN heroes he ON he.id = hf.entity_id AND he.raw_object_id = hf.raw_object_id
        JOIN published hr ON hr.id = he.receipt_id AND hr.ingest_run_id = hf.valid_from_run
        JOIN published fr ON fr.id = hf.receipt_id AND fr.extractor = hr.extractor
            AND fr.raw_object_id = hr.raw_object_id AND fr.ingest_run_id = hr.ingest_run_id
    ),
    resolved AS NOT MATERIALIZED (
        SELECT f.*, payload.value AS payload, binding.entity_id AS bound_entity_id,
            binding.alias_receipt_id, binding.entity_receipt_id
        FROM bindings binding
        JOIN LATERAL (
            SELECT * FROM facts candidate
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
JOIN catalog p ON p.patch_id = f.entity_id
    AND f.raw_object_id = p.raw_object_id AND f.valid_from_run = p.ingest_run_id
JOIN published receipt ON receipt.id = f.receipt_id AND receipt.extractor = p.extractor
    AND receipt.raw_object_id = p.raw_object_id AND receipt.ingest_run_id = p.ingest_run_id
    AND receipt.locator_kind = 'json_pointer' AND receipt.locator = '/raw_content'
LEFT JOIN hero_stats stat ON stat.entity_id = f.bound_entity_id
    AND stat.stat_key = f.payload ->> 'stat_label';

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
