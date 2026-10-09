BEGIN;
CREATE FUNCTION brain.read_site_profile_bindings_v1(requested_release text, requested_key text)
RETURNS jsonb LANGUAGE sql STABLE SECURITY DEFINER
SET search_path = pg_catalog, brain
AS $function$
    WITH allowed AS MATERIALIZED (
        SELECT r.source_id, r.logical_id, r.revision, r.record_json, h.read_header_json
        FROM brain.corpus_releases_v1 c
        CROSS JOIN LATERAL jsonb_each(c.release_json->'source_revisions') source
        CROSS JOIN LATERAL jsonb_each_text(source.value) document
        JOIN brain.source_record_revisions r
          ON r.source_id=source.key AND r.logical_id=document.key AND r.revision=document.value::bigint
        JOIN brain.source_record_heads h ON h.source_id=r.source_id AND h.logical_id=r.logical_id
        WHERE c.release_id=requested_release
          AND r.record_json->>'visibility'='public' AND h.record_json->>'visibility'='public'
          AND r.record_json->'allowed_scopes'='[]'::jsonb AND h.record_json->'allowed_scopes'='[]'::jsonb
          AND (r.record_json->'metadata'->>'brain.origin')::jsonb->'data'->'policy'->'publication_allowed'='true'::jsonb
          AND (h.record_json->'metadata'->>'brain.origin')::jsonb->'data'->'policy'->'publication_allowed'='true'::jsonb
          AND h.content_hash=h.read_header_json->>'content_hash'
          AND h.revision=(h.read_header_json->'head'->>'revision')::bigint
          AND NOT r.tombstone AND NOT h.tombstone
    ), keys AS MATERIALIZED (
        (SELECT DISTINCT ON(f.entity_key) f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id
         FROM brain.entity_profile_facts_v1 f JOIN allowed a USING(source_id,logical_id,revision)
         WHERE requested_key IS NULL AND f.binding_identity_json IS NOT NULL
           AND f.fact_json::jsonb->'provenance'->>'source_kind'='game_file'
         ORDER BY f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id LIMIT 20001)
        UNION ALL
        (SELECT f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id
         FROM brain.entity_profile_facts_v1 f JOIN allowed a USING(source_id,logical_id,revision)
         WHERE requested_key IS NOT NULL AND f.entity_key=requested_key
           AND f.binding_identity_json IS NOT NULL
           AND f.fact_json::jsonb->'provenance'->>'source_kind'='game_file'
         ORDER BY f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id LIMIT 20001)
    ), bindings AS MATERIALIZED (
        SELECT f.source_id, f.logical_id, f.revision,
               f.fact_json::jsonb AS fact, f.binding_identity_json AS identity,
               CASE WHEN s.relative_pointer IS NULL THEN NULL ELSE jsonb_build_object(
                   'relative_pointer',s.relative_pointer,'predicate',s.semantic_predicate,
                   'qualifiers',s.semantic_qualifiers_json::jsonb,'unit',s.semantic_unit) END AS semantic
        FROM brain.entity_profile_facts_v1 f
        JOIN keys k USING(entity_key,source_id,logical_id,revision,fact_id)
        LEFT JOIN brain.entity_semantic_projections_v1 s
          ON s.entity_key=f.entity_key AND s.source_id=f.source_id AND s.logical_id=f.logical_id
         AND s.revision=f.revision AND s.fact_id=f.fact_id
        WHERE (requested_key IS NULL OR f.entity_key=requested_key)
          AND f.binding_identity_json IS NOT NULL
          AND f.fact_json::jsonb->'provenance'->>'source_kind'='game_file'
        ORDER BY f.entity_key,f.source_id,f.logical_id,f.revision,f.fact_id
        LIMIT 20001
    )
    SELECT CASE WHEN (SELECT count(*) FROM bindings)>20000
        OR NOT EXISTS(SELECT 1 FROM brain.corpus_releases_v1 WHERE release_id=requested_release)
        THEN NULL ELSE jsonb_build_object(
        'bindings',COALESCE((SELECT jsonb_agg(to_jsonb(b)) FROM bindings b),'[]'::jsonb),
        'documents',COALESCE((SELECT jsonb_agg(jsonb_build_object('record',a.record_json,'head',a.read_header_json))
            FROM allowed a WHERE EXISTS(SELECT 1 FROM bindings b
                WHERE b.source_id=a.source_id AND b.logical_id=a.logical_id AND b.revision=a.revision)),'[]'::jsonb)) END
$function$;
REVOKE ALL ON FUNCTION brain.read_site_profile_bindings_v1(text,text) FROM PUBLIC;
DO $grant$
BEGIN
    IF NOT EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        RAISE EXCEPTION 'Die isolierte Rolle brain_site fehlt';
    END IF;
    GRANT EXECUTE ON FUNCTION brain.read_site_profile_bindings_v1(text,text) TO brain_site;
END
$grant$;
COMMIT;
