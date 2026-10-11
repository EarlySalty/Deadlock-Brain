BEGIN;
CREATE OR REPLACE FUNCTION brain.find_published_hero_compare_v2(
    requested_release text, first_hero bigint, second_hero bigint, client_version bigint)
RETURNS text LANGUAGE sql STABLE SECURITY DEFINER
SET search_path = pg_catalog, brain
AS $function$
    SELECT a.artifact_id
    FROM brain.compare_artifacts_v1 a
    JOIN brain.corpus_releases_v1 c ON c.release_id = requested_release
    WHERE a.publication_receipt IS NOT NULL
      AND a.body_json->>'mechanism_version' IN ('dbrain-reasoner/hero-curve-health-v1', 'dbrain-reasoner/hero-curve-health-v2')
      AND a.body_json->'calculation'->>'metric' = 'health'
      AND a.body_json->'calculation'->>'client_version' = client_version::text
      AND ARRAY[a.body_json->'calculation'->'left'->>'hero_id', a.body_json->'calculation'->'right'->>'hero_id']
          @> ARRAY[first_hero::text, second_hero::text]
      AND first_hero <> second_hero
      AND jsonb_typeof(a.body_json->'dependencies') = 'array'
      AND jsonb_array_length(a.body_json->'dependencies') > 0
      AND NOT EXISTS (
          SELECT 1
          FROM jsonb_array_elements(a.body_json->'dependencies') d(dependency)
          WHERE NOT EXISTS (
              SELECT 1
              FROM brain.source_record_revisions r
              WHERE r.source_id = d.dependency->'document'->>'source_id'
                AND r.logical_id = d.dependency->'document'->>'logical_id'
                AND r.revision::text = c.release_json->'source_revisions'
                    ->(d.dependency->'document'->>'source_id')
                    ->>(d.dependency->'document'->>'logical_id')
                AND r.content_hash = d.dependency->'document'->>'content_hash'
                AND NOT r.tombstone))
      AND NOT (brain.read_compare_artifact_v1(a.artifact_id) ? 'blocked')
    ORDER BY a.body_json->'release'->>'release_id' = requested_release DESC, a.artifact_id
    LIMIT 1
$function$;
REVOKE ALL ON FUNCTION brain.find_published_hero_compare_v2(text, bigint, bigint, bigint) FROM PUBLIC;
DO $grant$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'brain_readonly') THEN
        GRANT EXECUTE ON FUNCTION brain.find_published_hero_compare_v2(text, bigint, bigint, bigint) TO brain_readonly;
    END IF;
END
$grant$;
COMMIT;
