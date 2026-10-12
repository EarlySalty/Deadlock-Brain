BEGIN;
CREATE INDEX IF NOT EXISTS entity_profile_facts_site_v1
ON brain.entity_profile_facts_v1 (source_id, logical_id, revision, entity_key, fact_id)
WHERE binding_identity_json IS NOT NULL
  AND fact_json::jsonb->'provenance'->>'source_kind'='game_file';
COMMIT;
