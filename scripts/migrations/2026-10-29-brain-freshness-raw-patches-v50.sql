BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Aktualitätsmigration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE VIEW ops.patch_listing AS
WITH object AS (
    SELECT o.id,o.path,o.fetched_at,convert_from(o.bytes,'UTF8')::jsonb AS body,
        r.applied_run IS NOT NULL AS applied,coalesce(r.applied_run,o.ingest_run_id) AS observed_run
    FROM raw.object o
    LEFT JOIN LATERAL (
        SELECT max(x.ingest_run_id) AS applied_run FROM canon.receipt x WHERE x.raw_object_id=o.id
    ) r ON true
    WHERE o.source='deadlock_patchnotes_db' AND o.path ~ '^[1-9][0-9]{0,17}$'
), latest AS (
    SELECT DISTINCT ON (path) path,body,
        CASE WHEN pg_input_is_valid(body->>'posted_at','timestamptz')
            THEN (body->>'posted_at')::timestamptz END AS published_at
    FROM object
    WHERE jsonb_typeof(body)='object'
    ORDER BY path,applied DESC,observed_run DESC,id DESC
), first AS (
    SELECT path,min(fetched_at) AS fetched_at FROM object GROUP BY path
)
SELECT l.path::bigint AS patch_id,btrim(l.body->>'title') AS label,l.body->>'url' AS url,
    l.published_at,f.fetched_at AS detected_at
FROM latest l
JOIN first f ON f.path=l.path
WHERE nullif(btrim(l.body->>'title'),'') IS NOT NULL
    AND isfinite(l.published_at);

CREATE OR REPLACE VIEW ops.patch_freshness AS
WITH listing AS (
    SELECT p.patch_id,p.label,p.url,p.published_at,
        least(
            p.detected_at,
            (SELECT l.detected_at FROM ops.patch_detection_legacy l WHERE l.published_at=p.published_at)
        ) AS detected_at
    FROM ops.patch_listing p
), patch AS (
    SELECT min(patch_id) AS patch_id,label,published_at,
        (array_agg(url ORDER BY patch_id))[1] AS url,min(detected_at) AS brain_detected_at
    FROM listing
    GROUP BY label,published_at
), upstream AS (
    SELECT p.*,u.revision AS upstream_revision,u.client_build,u.committed_at AS upstream_committed_at
    FROM patch p
    LEFT JOIN LATERAL (
        SELECT c.revision,c.client_build,c.committed_at FROM ops.upstream_commit c
        WHERE c.source='deadlock-data' AND c.client_build IS NOT NULL
            AND c.committed_at>=p.published_at
        ORDER BY c.committed_at,c.revision
        LIMIT 1
    ) u ON true
), live AS (
    SELECT u.*,i.run_id AS ingest_run_id,i.finished AS game_data_live_at,i.inferred AS ingest_inferred
    FROM upstream u
    LEFT JOIN LATERAL (
        SELECT r.id AS run_id,r.finished,v.inferred FROM ops.ingest_run_revision v
        JOIN ops.ingest_run r ON r.id=v.run_id AND r.status='succeeded' AND r.rejected=0
            AND r.accepted+r.ignored>0
        JOIN ops.upstream_commit c ON c.source=v.source AND c.revision=v.revision
        WHERE v.source='deadlock-data' AND c.committed_at>=u.upstream_committed_at
        ORDER BY r.finished,r.id
        LIMIT 1
    ) i ON u.upstream_committed_at IS NOT NULL
)
SELECT patch_id,label,url,published_at,brain_detected_at,
    upstream_revision,client_build,upstream_committed_at,ingest_run_id,ingest_inferred,
    game_data_live_at,
    brain_detected_at-published_at AS detect_lag,
    upstream_committed_at-published_at AS upstream_lag,
    game_data_live_at-upstream_committed_at AS ingest_lag,
    game_data_live_at-published_at AS total_lag
FROM live;

CREATE OR REPLACE VIEW ops.game_data_freshness AS
SELECT c.revision,c.client_build,c.committed_at,c.subject,p.patch_id,p.published_at,
    p.patch_id IS NULL AS silent,i.run_id AS ingest_run_id,i.inferred AS ingest_inferred,
    i.finished AS game_data_live_at,i.finished-c.committed_at AS ingest_lag
FROM ops.upstream_commit c
LEFT JOIN LATERAL (
    SELECT min(pl.patch_id) AS patch_id,pl.published_at FROM ops.patch_listing pl
    WHERE pl.published_at<=c.committed_at AND pl.published_at>c.committed_at-interval '12 hours'
    GROUP BY pl.published_at
    ORDER BY pl.published_at DESC
    LIMIT 1
) p ON true
LEFT JOIN LATERAL (
    SELECT r.id AS run_id,r.finished,v.inferred FROM ops.ingest_run_revision v
    JOIN ops.ingest_run r ON r.id=v.run_id AND r.status='succeeded' AND r.rejected=0
        AND r.accepted+r.ignored>0
    JOIN ops.upstream_commit n ON n.source=v.source AND n.revision=v.revision
    WHERE v.source=c.source AND n.committed_at>=c.committed_at
    ORDER BY r.finished,r.id
    LIMIT 1
) i ON true
WHERE c.source='deadlock-data' AND c.client_build IS NOT NULL;

REVOKE ALL ON ops.patch_listing FROM PUBLIC;
GRANT SELECT ON ops.patch_listing TO brain_ingest;

COMMIT;
