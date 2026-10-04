BEGIN;
CREATE TABLE brain.entity_profile_entities_v1 (
    entity_key text PRIMARY KEY,
    identity_json jsonb NOT NULL
);
CREATE TABLE brain.entity_profile_facts_v1 (
    entity_key text NOT NULL REFERENCES brain.entity_profile_entities_v1(entity_key),
    source_id text NOT NULL,
    logical_id text NOT NULL,
    revision bigint NOT NULL,
    fact_id text NOT NULL,
    fact_json text NOT NULL,
    PRIMARY KEY(entity_key,source_id,logical_id,revision,fact_id),
    FOREIGN KEY(source_id,logical_id,revision)
        REFERENCES brain.source_record_revisions(source_id,logical_id,revision)
);
COMMIT;
