BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Timer-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

CREATE INDEX timer_key_idx ON canon.entity(key) WHERE kind = 'timer';

CREATE VIEW serve.timer AS
SELECT e.key, f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json,
    f.unit, f.condition, f.receipt_id, f.source_revision, f.source_id, f.id
FROM canon.entity e
JOIN canon.receipt identity_proof ON identity_proof.id = e.identity_receipt_id
JOIN canon.source_head identity_head ON identity_head.source_id = identity_proof.source_id
    AND identity_head.published_revision_id >= identity_proof.revision_id
JOIN serve.current_fact f ON f.entity_id = e.id
WHERE e.kind = 'timer';

REVOKE ALL ON serve.timer FROM PUBLIC;
GRANT SELECT ON serve.timer TO brain_tool, brain_ingest;
GRANT EXECUTE ON FUNCTION serve.atomic_fact(bigint, text, numeric, text, jsonb, text, jsonb, bigint, text) TO brain_tool;

COMMIT;
