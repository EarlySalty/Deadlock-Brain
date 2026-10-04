BEGIN;
ALTER TABLE brain.entity_profile_facts_v1
    ADD COLUMN binding_identity_json jsonb;
COMMIT;
