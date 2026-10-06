BEGIN;

-- Die Projektionen entstehen beim Schreiben. Archivkörper und Releasepins bleiben unverändert.
CREATE OR REPLACE FUNCTION brain.source_read_header_v1(payload jsonb)
RETURNS jsonb LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $function$
    SELECT jsonb_build_object(
        'head', jsonb_build_object(
            'source_id', p.source_id, 'logical_id', p.logical_id, 'revision', p.revision,
            'visibility', p.visibility, 'allowed_scopes', p.allowed_scopes, 'tombstone', p.tombstone,
            'metadata', jsonb_strip_nulls(jsonb_build_object(
                'brain.origin', p.metadata->'brain.origin',
                'egress', p.metadata->'egress', 'patch', p.metadata->'patch'))),
        'content_hash', p.content_hash)
    FROM jsonb_to_record(payload) AS p(
        source_id jsonb, logical_id jsonb, revision jsonb, visibility jsonb,
        allowed_scopes jsonb, tombstone jsonb, content_hash jsonb, metadata jsonb)
$function$;

CREATE OR REPLACE FUNCTION brain.source_original_header_v1(payload jsonb)
RETURNS jsonb LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $function$
    SELECT jsonb_build_object(
        'contract_version', d.contract_version, 'source_id', d.source_id,
        'document_id', d.document_id, 'source_kind', d.source_kind, 'revision', d.revision,
        'observed_at', d.observed_at, 'license', d.license,
        'content_sha256', d.content_sha256, 'metadata', d.metadata)
    FROM jsonb_to_record((payload->'metadata'->>'wiki-spielwissen.document')::jsonb) AS d(
        contract_version jsonb, source_id jsonb, document_id jsonb, source_kind jsonb,
        revision jsonb, observed_at jsonb, license jsonb, content_sha256 jsonb, metadata jsonb)
$function$;

ALTER TABLE brain.source_record_revisions
    ADD COLUMN IF NOT EXISTS read_header_json jsonb
        GENERATED ALWAYS AS (brain.source_read_header_v1(record_json)) STORED,
    ADD COLUMN IF NOT EXISTS original_header_json jsonb
        GENERATED ALWAYS AS (brain.source_original_header_v1(record_json)) STORED;
ALTER TABLE brain.source_record_heads
    ADD COLUMN IF NOT EXISTS read_header_json jsonb
        GENERATED ALWAYS AS (brain.source_read_header_v1(record_json)) STORED;

COMMIT;
