BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Patchzeilen-Index benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE INDEX IF NOT EXISTS fact_patch_line_trgm ON canon.fact
    USING gin (lower(value_text::text) gin_trgm_ops)
    WHERE stat_key ~ '^(patch_change_r[0-9a-f]{64}_[0-9]+|patch_line_r[0-9a-f]{64}_[0-9]+_[0-9]+|patch_note_[0-9]+_[0-9]+)$';
CREATE VIEW serve.patch_line_text AS
SELECT f.id,f.entity_id,f.stat_key,lower(f.value_text::text) AS text_lower
FROM canon.fact f
WHERE f.stat_key ~ '^(patch_change_r[0-9a-f]{64}_[0-9]+|patch_line_r[0-9a-f]{64}_[0-9]+_[0-9]+|patch_note_[0-9]+_[0-9]+)$'
    AND EXISTS (SELECT 1 FROM serve.published_receipt p WHERE p.id=f.receipt_id);
REVOKE ALL ON serve.patch_line_text FROM PUBLIC;
GRANT SELECT ON serve.patch_line_text TO brain_tool;

COMMIT;
