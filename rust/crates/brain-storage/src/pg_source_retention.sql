BEGIN;

CREATE TABLE brain.source_retention_supersessions_v1 (
    source_id text NOT NULL CHECK (source_id LIKE 'google-sheet/_%' OR source_id LIKE 'youtube-core/_%'),
    logical_id text NOT NULL,
    revision bigint NOT NULL CHECK (revision > 0),
    superseded_at timestamptz NOT NULL,
    PRIMARY KEY (source_id, logical_id, revision)
);
ALTER TABLE brain.source_retention_supersessions_v1 OWNER TO brain_migrate;
REVOKE ALL ON brain.source_retention_supersessions_v1 FROM PUBLIC, brain_ingest, brain_service, brain_readonly;

CREATE FUNCTION brain.purge_owned_source_history_v1(p_source text, p_generation bigint)
RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog
AS $$
DECLARE
    cutoff timestamptz := ((transaction_timestamp() AT TIME ZONE 'UTC') - interval '12 months') AT TIME ZONE 'UTC';
    candidate record;
    stored_generation bigint;
    candidates bigint := 0;
    deleted bigint := 0;
    pinned bigint := 0;
    replay_blocked bigint := 0;
    missing_candidates bigint := 0;
    blocking_releases jsonb := '[]'::jsonb;
    refs jsonb;
BEGIN
    IF p_source IS NULL OR length(p_source) > 512 OR p_source ~ '[[:cntrl:]]'
       OR NOT (p_source LIKE 'google-sheet/_%' OR p_source LIKE 'youtube-core/_%')
       OR btrim(substr(p_source, strpos(p_source, '/') + 1)) = ''
       OR p_generation IS NULL OR p_generation < 1 THEN
        RAISE EXCEPTION 'invalid owned source retention request';
    END IF;
    PERFORM pg_advisory_xact_lock(hashtext('core-source:' || p_source)::bigint);
    LOCK TABLE brain.source_record_revisions, brain.source_record_heads,
        brain.source_checkpoints_v1, brain.source_jobs_v1 IN SHARE ROW EXCLUSIVE MODE;
    LOCK TABLE brain.corpus_releases_v1 IN ACCESS EXCLUSIVE MODE;
    SELECT generation INTO stored_generation FROM brain.source_checkpoints_v1 WHERE source_id = p_source;
    IF stored_generation IS DISTINCT FROM p_generation THEN
        RAISE EXCEPTION 'retention checkpoint generation changed';
    END IF;
    IF EXISTS (SELECT FROM brain.source_jobs_v1 WHERE source_id = p_source AND state = 'running' AND lease_until > clock_timestamp()) THEN
        RAISE EXCEPTION 'retention source writer is still running';
    END IF;
    IF EXISTS (
        SELECT FROM brain.source_record_revisions r
        LEFT JOIN brain.source_record_heads h USING (source_id, logical_id)
        WHERE r.source_id = p_source AND (
            h.source_id IS NULL OR r.revision > h.revision
            OR r.record_json->>'source_id' IS DISTINCT FROM r.source_id
            OR r.record_json->>'logical_id' IS DISTINCT FROM r.logical_id
            OR r.record_json->>'revision' IS DISTINCT FROM r.revision::text
            OR r.record_json->>'content_hash' IS DISTINCT FROM r.content_hash
            OR r.record_json->>'tombstone' IS DISTINCT FROM r.tombstone::text
        )
    ) THEN
        RAISE EXCEPTION 'retention revision layout is inconsistent';
    END IF;
    IF EXISTS (
        SELECT FROM brain.source_record_heads h
        LEFT JOIN brain.source_record_revisions r USING (source_id, logical_id, revision)
        WHERE h.source_id = p_source AND (
            r.source_id IS NULL OR h.record_json IS DISTINCT FROM r.record_json
            OR h.content_hash IS DISTINCT FROM r.content_hash OR h.tombstone IS DISTINCT FROM r.tombstone
            OR h.record_json->>'source_id' IS DISTINCT FROM h.source_id
            OR h.record_json->>'logical_id' IS DISTINCT FROM h.logical_id
            OR h.record_json->>'revision' IS DISTINCT FROM h.revision::text
            OR h.record_json->>'content_hash' IS DISTINCT FROM h.content_hash
            OR h.record_json->>'tombstone' IS DISTINCT FROM h.tombstone::text
            OR (h.tombstone AND (h.record_json->>'content' IS DISTINCT FROM ''
                OR h.record_json->'metadata' IS DISTINCT FROM CASE split_part(p_source, '/', 1)
                    WHEN 'google-sheet' THEN jsonb_build_object('connector', 'google-sheet')
                    WHEN 'youtube-core' THEN jsonb_build_object('connector', 'youtube-core', 'removal_basis', 'not_retrievable_via_youtube_data_api_v3')
                END))
        )
    ) THEN
        RAISE EXCEPTION 'retention current record is inconsistent or tombstone retains data';
    END IF;
    IF EXISTS (
        SELECT FROM brain.source_checkpoints_v1 p
        WHERE p.source_id = p_source AND (
            jsonb_typeof(p.checkpoint_json->'state') IS DISTINCT FROM 'object'
            OR jsonb_typeof(p.batch_json->'records') IS DISTINCT FROM 'array'
            OR jsonb_typeof(p.checkpoint_json->'state'->'documents') IS DISTINCT FROM 'object'
            OR p.checkpoint_json->>'source_id' IS DISTINCT FROM p_source
            OR p.checkpoint_json->>'generation' IS DISTINCT FROM p_generation::text
            OR p.checkpoint_json->'state'->>'configuration' IS DISTINCT FROM p.configuration
            OR (p.checkpoint_json->'state') - 'configuration' - 'documents' <> '{}'::jsonb
        )
    ) OR EXISTS (
        SELECT FROM brain.source_checkpoints_v1 p
        CROSS JOIN LATERAL jsonb_each(p.checkpoint_json->'state'->'documents') d
        LEFT JOIN brain.source_record_heads h ON h.source_id = p_source AND h.logical_id = d.key
        WHERE p.source_id = p_source AND (
            h.source_id IS NULL OR d.value->>'revision' IS DISTINCT FROM h.revision::text
            OR d.value->>'content_hash' IS DISTINCT FROM h.content_hash
            OR d.value->>'tombstone' IS DISTINCT FROM h.tombstone::text
            OR d.value - 'revision' - 'content_hash' - 'semantic_hash' - 'tombstone' <> '{}'::jsonb
        )
    ) OR EXISTS (
        SELECT FROM brain.source_record_heads h
        JOIN brain.source_checkpoints_v1 p USING (source_id)
        WHERE h.source_id = p_source AND NOT (p.checkpoint_json->'state'->'documents' ? h.logical_id)
    ) THEN
        RAISE EXCEPTION 'retention checkpoint layout or materialized state is not supported';
    END IF;
    IF EXISTS (
        SELECT FROM brain.source_record_revisions r
        WHERE CASE WHEN r.record_json->'metadata' ? 'brain.domain.dependencies' THEN
            jsonb_path_exists((r.record_json->'metadata'->>'brain.domain.dependencies')::jsonb,
                '$.**.source_id ? (@ == $source)', jsonb_build_object('source', p_source))
            ELSE false END
        OR CASE WHEN r.record_json->'metadata' ? 'domain_contract' THEN
            jsonb_path_exists((r.record_json->>'content')::jsonb,
                '$.**.source_id ? (@ == $source)', jsonb_build_object('source', p_source))
            ELSE false END
        OR CASE WHEN r.source_id <> p_source AND r.record_json->'metadata' ? 'brain.origin' THEN
            EXISTS (
                SELECT FROM jsonb_array_elements_text(
                    (r.record_json->'metadata'->>'brain.origin')::jsonb->'data'->'origin_artifacts'
                ) a WHERE strpos(a, p_source) > 0
            ) ELSE false END
    ) THEN
        RAISE EXCEPTION 'retention materialized dependencies require a replacement and retirement contract';
    END IF;
    INSERT INTO brain.source_retention_supersessions_v1 (source_id, logical_id, revision, superseded_at)
    SELECT r.source_id, r.logical_id, r.revision, min(n.created_at)
    FROM brain.source_record_revisions r
    JOIN brain.source_record_heads h USING (source_id, logical_id)
    JOIN brain.source_record_revisions n ON n.source_id = r.source_id AND n.logical_id = r.logical_id
        AND n.revision > r.revision AND n.created_at >= r.created_at
    WHERE r.source_id = p_source AND r.revision < h.revision
    GROUP BY r.source_id, r.logical_id, r.revision
    ON CONFLICT (source_id, logical_id, revision) DO NOTHING;
    IF EXISTS (
        SELECT FROM brain.source_record_revisions r
        JOIN brain.source_record_heads h USING (source_id, logical_id)
        LEFT JOIN brain.source_retention_supersessions_v1 s ON s.source_id = r.source_id
            AND s.logical_id = r.logical_id AND s.revision = r.revision
        WHERE r.source_id = p_source AND r.revision < h.revision AND NOT h.tombstone
            AND s.source_id IS NULL
    ) THEN
        RAISE EXCEPTION 'retention supersession time cannot be established';
    END IF;
    FOR candidate IN
        SELECT r.source_id, r.logical_id, r.revision, h.tombstone AS source_missing
        FROM brain.source_record_revisions r
        JOIN brain.source_record_heads h USING (source_id, logical_id)
        LEFT JOIN brain.source_retention_supersessions_v1 s ON s.source_id = r.source_id
            AND s.logical_id = r.logical_id AND s.revision = r.revision
        WHERE r.source_id = p_source AND r.revision < h.revision
          AND (h.tombstone OR s.superseded_at < cutoff)
        ORDER BY r.logical_id, r.revision
    LOOP
        candidates := candidates + 1;
        IF candidate.source_missing THEN
            missing_candidates := missing_candidates + 1;
        END IF;
        SELECT coalesce(jsonb_agg(c.release_id ORDER BY c.release_id), '[]'::jsonb)
        INTO refs FROM brain.corpus_releases_v1 c
        WHERE EXISTS (
            SELECT FROM jsonb_each(c.release_json->'source_revisions') s
            CROSS JOIN LATERAL jsonb_each_text(s.value) d
            WHERE s.key = p_source AND d.key = candidate.logical_id AND d.value::bigint = candidate.revision
        );
        IF jsonb_array_length(refs) > 0 THEN
            pinned := pinned + 1;
            blocking_releases := blocking_releases || refs;
            CONTINUE;
        END IF;
        IF EXISTS (
            SELECT FROM brain.source_checkpoints_v1 p
            CROSS JOIN LATERAL jsonb_array_elements(p.batch_json->'records') b
            WHERE b->>'source_id' = p_source AND b->>'logical_id' = candidate.logical_id
              AND b->>'revision' = candidate.revision::text
        ) THEN
            replay_blocked := replay_blocked + 1;
            CONTINUE;
        END IF;
        DELETE FROM brain.source_record_revisions
        WHERE source_id = p_source AND logical_id = candidate.logical_id AND revision = candidate.revision;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'retention candidate changed';
        END IF;
        DELETE FROM brain.source_retention_supersessions_v1
        WHERE source_id = p_source AND logical_id = candidate.logical_id AND revision = candidate.revision;
        deleted := deleted + 1;
    END LOOP;
    RETURN jsonb_build_object(
        'source_id', p_source, 'generation', p_generation,
        'candidate_revisions', candidates, 'deleted_revisions', deleted,
        'release_blocked_revisions', pinned, 'checkpoint_blocked_revisions', replay_blocked,
        'missing_source_revisions', missing_candidates,
        'blocking_release_ids', (SELECT coalesce(jsonb_agg(id ORDER BY id), '[]'::jsonb) FROM (SELECT DISTINCT value AS id FROM jsonb_array_elements(blocking_releases)) ids)
    );
END;
$$;

ALTER FUNCTION brain.purge_owned_source_history_v1(text, bigint) OWNER TO brain_migrate;
REVOKE ALL ON FUNCTION brain.purge_owned_source_history_v1(text, bigint) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION brain.purge_owned_source_history_v1(text, bigint) TO brain_ingest;

COMMIT;
