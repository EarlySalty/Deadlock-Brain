BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Metastatistik-Fenster-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE OR REPLACE VIEW serve.meta_stat_set AS
SELECT e.id AS entity_id,e.key AS stat_set,
    CASE e.key WHEN 'meta_hero_stats' THEN 'hero-stats.json' ELSE 'item-stats.json' END AS path,
    CASE e.key WHEN 'meta_hero_stats' THEN 'hero-stats-14d.json' ELSE 'item-stats-14d.json' END
        AS fallback_path
FROM canon.entity e
WHERE e.kind='meta_stats' AND e.key IN ('meta_hero_stats','meta_item_stats');

CREATE OR REPLACE VIEW serve.meta_stat AS
SELECT f.entity_id,f.stat_key,f.valid_from_run AS run_id,
    CASE WHEN p.ingest_run_id<>o.ingest_run_id THEN date_trunc('hour',run.started,'UTC')
        WHEN p.source_revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$'
        THEN p.source_revision::timestamptz END AS snapshot_at,
    f.value_text::jsonb AS value,f.receipt_id,p.raw_object_id,p.source,p.source_revision,
    CASE p.path WHEN 'hero-stats-14d.json' THEN 14 WHEN 'item-stats-14d.json' THEN 14 ELSE 30 END
        AS window_days
FROM canon.fact f
JOIN canon.entity e ON e.id=f.entity_id AND e.kind='meta_stats'
    AND e.key IN ('meta_hero_stats','meta_item_stats')
JOIN serve.published_receipt p ON p.id=f.receipt_id AND p.ingest_run_id=f.valid_from_run
JOIN raw.object o ON o.id=p.raw_object_id
JOIN ops.ingest_run run ON run.id=p.ingest_run_id
WHERE p.source='deadlock-api-meta' AND f.value_text IS NOT NULL
    AND p.source_revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$';

COMMIT;
