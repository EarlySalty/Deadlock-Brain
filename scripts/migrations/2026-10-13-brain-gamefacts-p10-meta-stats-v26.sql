BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Metastatistik-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

CREATE INDEX meta_stat_capture_order ON raw.object(path,(revision COLLATE "C") DESC,id DESC)
WHERE source='deadlock-api-meta';

CREATE VIEW serve.meta_stat_set AS
SELECT e.id AS entity_id,e.key AS stat_set,
    CASE e.key WHEN 'meta_hero_stats' THEN 'hero-stats.json' ELSE 'item-stats.json' END AS path
FROM canon.entity e
WHERE e.kind='meta_stats' AND e.key IN ('meta_hero_stats','meta_item_stats');

CREATE VIEW serve.meta_stat_capture AS
SELECT o.path,o.id AS raw_object_id,o.revision COLLATE "C" AS capture_key,
    o.ingest_run_id AS run_id,
    CASE WHEN o.revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$'
        THEN o.revision::timestamptz END AS snapshot_at
FROM raw.object o
WHERE o.source='deadlock-api-meta'
    AND o.revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$'
    AND EXISTS (
        SELECT 1 FROM canon.receipt r
        JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
        WHERE r.raw_object_id=o.id AND r.ingest_run_id=o.ingest_run_id
    );

CREATE VIEW serve.meta_stat AS
SELECT f.entity_id,f.stat_key,f.valid_from_run AS run_id,
    CASE WHEN p.ingest_run_id<>o.ingest_run_id THEN date_trunc('hour',run.started,'UTC')
        WHEN p.source_revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$'
        THEN p.source_revision::timestamptz END AS snapshot_at,
    f.value_text::jsonb AS value,f.receipt_id,p.raw_object_id,p.source,p.source_revision
FROM canon.fact f
JOIN canon.entity e ON e.id=f.entity_id AND e.kind='meta_stats'
    AND e.key IN ('meta_hero_stats','meta_item_stats')
JOIN serve.published_receipt p ON p.id=f.receipt_id AND p.ingest_run_id=f.valid_from_run
JOIN raw.object o ON o.id=p.raw_object_id
JOIN ops.ingest_run run ON run.id=p.ingest_run_id
WHERE p.source='deadlock-api-meta' AND f.value_text IS NOT NULL
    AND p.source_revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$';

REVOKE ALL ON serve.meta_stat_set,serve.meta_stat_capture,serve.meta_stat FROM PUBLIC;
GRANT SELECT ON serve.meta_stat_set,serve.meta_stat_capture,serve.meta_stat TO brain_tool;

COMMIT;
