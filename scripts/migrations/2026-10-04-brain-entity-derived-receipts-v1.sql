BEGIN;
CREATE TABLE brain.entity_derived_receipts_v1 (
    derived_source_id text NOT NULL,
    derived_logical_id text NOT NULL,
    derived_revision bigint NOT NULL,
    receipt_json text NOT NULL,
    PRIMARY KEY(derived_source_id,derived_logical_id,derived_revision),
    FOREIGN KEY(derived_source_id,derived_logical_id,derived_revision)
        REFERENCES brain.source_record_revisions(source_id,logical_id,revision)
);
DO $body$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_ingest') THEN
        GRANT SELECT,INSERT ON brain.entity_derived_receipts_v1 TO brain_ingest;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_service') THEN
        GRANT SELECT ON brain.entity_derived_receipts_v1 TO brain_service;
    END IF;
    IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname='brain_readonly') THEN
        GRANT SELECT ON brain.entity_derived_receipts_v1 TO brain_readonly;
    END IF;
END
$body$;
COMMIT;
