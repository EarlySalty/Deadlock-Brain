BEGIN;
ALTER TABLE brain.entity_profile_entities_v1
    ADD COLUMN lookup_names text[] NOT NULL DEFAULT ARRAY[]::text[];
WITH bindings AS MATERIALIZED (
    SELECT DISTINCT entity_key,
        binding_identity_json->>'name' AS name,
        binding_identity_json->'aliases' AS aliases
    FROM brain.entity_profile_facts_v1
), names AS (
    SELECT entity_key, name FROM bindings
    UNION
    SELECT entity_key, alias.value
    FROM bindings
    CROSS JOIN LATERAL jsonb_array_elements_text(aliases) alias
)
UPDATE brain.entity_profile_entities_v1 e
SET lookup_names = ARRAY(
    SELECT DISTINCT name FROM names n
    WHERE n.entity_key = e.entity_key AND name IS NOT NULL AND name <> ''
    ORDER BY name
);
COMMIT;
