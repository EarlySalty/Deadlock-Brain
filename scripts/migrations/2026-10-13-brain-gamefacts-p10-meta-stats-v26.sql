BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Metastatistik-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

CREATE VIEW serve.meta_stat_set AS
SELECT e.id AS entity_id,e.key AS stat_set
FROM canon.entity e
WHERE e.kind='meta_stats' AND e.key IN ('meta_hero_stats','meta_item_stats');

CREATE VIEW serve.meta_stat AS
SELECT f.entity_id,f.stat_key,f.valid_from_run AS run_id,run.started AS snapshot_at,
    f.value_text::jsonb AS value,f.receipt_id,p.source,p.source_revision
FROM canon.fact f
JOIN canon.entity e ON e.id=f.entity_id AND e.kind='meta_stats'
    AND e.key IN ('meta_hero_stats','meta_item_stats')
JOIN serve.published_receipt p ON p.id=f.receipt_id AND p.ingest_run_id=f.valid_from_run
JOIN ops.ingest_run run ON run.id=f.valid_from_run
WHERE p.source='deadlock-api-meta' AND f.value_text IS NOT NULL;

REVOKE ALL ON serve.meta_stat_set,serve.meta_stat FROM PUBLIC;
GRANT SELECT ON serve.meta_stat_set,serve.meta_stat TO brain_tool;

COMMIT;
