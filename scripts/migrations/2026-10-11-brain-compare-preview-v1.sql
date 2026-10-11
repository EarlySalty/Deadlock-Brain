BEGIN;
CREATE FUNCTION brain.read_compare_artifact_v2(requested_id text)
RETURNS jsonb LANGUAGE sql STABLE SECURITY DEFINER
SET search_path = pg_catalog, brain
AS $function$
    SELECT CASE WHEN jsonb_array_length(COALESCE(e.records, '[]'::jsonb)) = jsonb_array_length(a.body_json->'dependencies')
        AND a.body_json = a.body_text::jsonb
        AND (a.publication_receipt IS NULL OR a.publication_receipt->>'release_jsonb_sha256' = encode(sha256(convert_to(c.release_json::text, 'UTF8')), 'hex'))
        THEN jsonb_build_object(
        'body_text', a.body_text,
        'receipt', a.publication_receipt,
        'published', a.publication_receipt IS NOT NULL,
        'release', jsonb_build_object(
            'release_id', c.release_json->'release_id',
            'knowledge_version', c.release_json->'knowledge_version',
            'patch', c.release_json->'patch',
            'manifest_sha256', a.body_json->'release'->'manifest_sha256'),
        'records', COALESCE(e.records, '[]'::jsonb),
        'heads', COALESCE(e.heads, '[]'::jsonb))
        ELSE '{"blocked":true}'::jsonb END
    FROM brain.compare_artifacts_v1 a
    JOIN brain.corpus_releases_v1 c ON c.release_id = a.body_json->'release'->>'release_id'
    CROSS JOIN LATERAL (
        SELECT jsonb_agg(r.record_json ORDER BY d.ordinality) AS records,
               jsonb_agg(h.read_header_json ORDER BY d.ordinality) AS heads
        FROM jsonb_array_elements(a.body_json->'dependencies') WITH ORDINALITY d(document, ordinality)
        JOIN brain.source_record_revisions r
          ON r.source_id = d.document->'document'->>'source_id'
         AND r.logical_id = d.document->'document'->>'logical_id'
         AND r.revision = (d.document->'document'->>'revision')::bigint
         AND r.content_hash = d.document->'document'->>'content_hash'
        JOIN brain.source_record_heads h ON h.source_id = r.source_id AND h.logical_id = r.logical_id
        WHERE r.record_json->>'visibility' = 'public' AND h.record_json->>'visibility' = 'public'
          AND r.record_json->'allowed_scopes' = '[]'::jsonb AND h.record_json->'allowed_scopes' = '[]'::jsonb
          AND h.content_hash = h.read_header_json->>'content_hash'
          AND h.revision = (h.read_header_json->'head'->>'revision')::bigint
          AND c.release_json->'source_revisions'->r.source_id->r.logical_id = to_jsonb(r.revision)
          AND NOT r.tombstone AND NOT h.tombstone
    ) e
    WHERE a.artifact_id = requested_id
$function$;
REVOKE ALL ON FUNCTION brain.read_compare_artifact_v2(text) FROM PUBLIC;
DO $grant$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'brain_site') THEN
        GRANT EXECUTE ON FUNCTION brain.read_compare_artifact_v2(text) TO brain_site;
    END IF;
END
$grant$;
COMMIT;
