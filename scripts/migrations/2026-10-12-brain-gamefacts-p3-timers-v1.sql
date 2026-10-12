BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Timer-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

CREATE INDEX timer_key_idx ON canon.alias(alias_norm,entity_id)
WHERE lang='und' AND alias_norm IN ('urn','soul_box','mid_boss','rejuvenator');

CREATE VIEW serve.timer AS
SELECT a.alias_norm AS key,e.id AS entity_id,f.stat_key,
    f.value_text::jsonb->'value' AS value,
    f.value_text::jsonb->>'unit' AS unit,
    f.value_text::jsonb->'condition' AS condition,
    f.receipt_id,f.source,f.source_revision,f.id
FROM serve.current_alias a
JOIN serve.current_entity e ON e.id=a.entity_id
    AND e.raw_object_id=a.raw_object_id AND e.source=a.source
JOIN serve.current_fact f ON f.entity_id=e.id
    AND f.raw_object_id=e.raw_object_id AND f.source=e.source
WHERE e.kind='timer' AND a.lang='und'
    AND a.alias_norm IN ('urn','soul_box','mid_boss','rejuvenator')
    AND f.value_text IS NOT NULL;

REVOKE ALL ON serve.timer FROM PUBLIC;
GRANT SELECT ON serve.timer TO brain_tool;

COMMIT;
