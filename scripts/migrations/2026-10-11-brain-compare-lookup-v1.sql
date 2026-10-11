BEGIN;
CREATE OR REPLACE FUNCTION brain.find_published_hero_compare_v1(
    requested_release text, first_hero bigint, second_hero bigint, client_version bigint)
RETURNS text LANGUAGE sql STABLE SECURITY DEFINER
SET search_path = pg_catalog, brain
AS $function$
    SELECT a.artifact_id
    FROM brain.compare_artifacts_v1 a
    WHERE a.publication_receipt IS NOT NULL
      AND a.body_json->>'mechanism_version' = 'dbrain-reasoner/hero-curve-health-v1'
      AND a.body_json->'release'->>'release_id' = requested_release
      AND a.body_json->'calculation'->>'metric' = 'health'
      AND a.body_json->'calculation'->>'client_version' = client_version::text
      AND ARRAY[a.body_json->'calculation'->'left'->>'hero_id', a.body_json->'calculation'->'right'->>'hero_id']
          @> ARRAY[first_hero::text, second_hero::text]
      AND first_hero <> second_hero
      AND NOT (brain.read_compare_artifact_v1(a.artifact_id) ? 'blocked')
    ORDER BY a.artifact_id
    LIMIT 1
$function$;
REVOKE ALL ON FUNCTION brain.find_published_hero_compare_v1(text, bigint, bigint, bigint) FROM PUBLIC;
DO $grant$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'brain_readonly') THEN
        GRANT EXECUTE ON FUNCTION brain.find_published_hero_compare_v1(text, bigint, bigint, bigint) TO brain_readonly;
    END IF;
END
$grant$;
COMMIT;
