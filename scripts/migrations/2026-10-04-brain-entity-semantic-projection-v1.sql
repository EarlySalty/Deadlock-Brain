BEGIN;
CREATE TABLE brain.entity_semantic_projections_v1 (
    entity_key text NOT NULL,
    source_id text NOT NULL,
    logical_id text NOT NULL,
    revision bigint NOT NULL,
    fact_id text NOT NULL,
    relative_pointer text NOT NULL,
    semantic_predicate text NOT NULL,
    semantic_qualifiers_json text NOT NULL,
    semantic_unit text,
    PRIMARY KEY(entity_key,source_id,logical_id,revision,fact_id),
    FOREIGN KEY(entity_key,source_id,logical_id,revision,fact_id)
        REFERENCES brain.entity_profile_facts_v1(entity_key,source_id,logical_id,revision,fact_id)
);
CREATE TABLE brain.entity_patch_intervals_v1 (
    entity_key text NOT NULL,
    source_id text NOT NULL,
    logical_id text NOT NULL,
    revision bigint NOT NULL,
    fact_id text NOT NULL,
    current_patch_fact_id text NOT NULL,
    interval_json text NOT NULL,
    PRIMARY KEY(entity_key,source_id,logical_id,revision,fact_id),
    FOREIGN KEY(entity_key,source_id,logical_id,revision,fact_id)
        REFERENCES brain.entity_semantic_projections_v1(entity_key,source_id,logical_id,revision,fact_id)
);
DO $body$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_ingest') THEN
        GRANT SELECT,INSERT ON brain.entity_semantic_projections_v1 TO brain_ingest;
        GRANT SELECT,INSERT,UPDATE ON brain.entity_patch_intervals_v1 TO brain_ingest;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_service') THEN
        GRANT SELECT ON brain.entity_semantic_projections_v1,brain.entity_patch_intervals_v1 TO brain_service;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_readonly') THEN
        GRANT SELECT ON brain.entity_semantic_projections_v1,brain.entity_patch_intervals_v1 TO brain_readonly;
    END IF;
END
$body$;
COMMIT;
