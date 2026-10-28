BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Aktualitätsmigration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE TABLE ops.upstream_commit (
    source text NOT NULL CHECK (source='deadlock-data'),
    revision text NOT NULL CHECK (revision ~ '^([0-9a-f]{40}|[0-9a-f]{64})$'),
    committed_at timestamptz NOT NULL,
    subject text NOT NULL CHECK (length(subject)<=1000),
    client_build integer CHECK (client_build>0),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (source,revision)
);

CREATE INDEX upstream_commit_time ON ops.upstream_commit (source,committed_at);

CREATE TABLE ops.ingest_run_revision (
    run_id bigint PRIMARY KEY REFERENCES ops.ingest_run(id),
    source text NOT NULL,
    revision text NOT NULL,
    inferred boolean NOT NULL DEFAULT false,
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (source,revision) REFERENCES ops.upstream_commit(source,revision)
);

CREATE TABLE ops.patch_detection_legacy (
    published_at timestamptz PRIMARY KEY,
    detected_at timestamptz NOT NULL
);

DO $legacy$
BEGIN
    IF to_regnamespace('brain') IS NULL THEN
        RETURN;
    END IF;
    IF NOT has_schema_privilege('brain','USAGE') THEN
        RETURN;
    END IF;
    IF to_regclass('brain.patch_events') IS NOT NULL
        AND has_table_privilege('brain.patch_events','SELECT') THEN
        EXECUTE $copy$
            INSERT INTO ops.patch_detection_legacy (published_at,detected_at)
            SELECT posted_at,min(created_at) FROM brain.patch_events
            WHERE posted_at>='2026-08-01' AND created_at IS NOT NULL
            GROUP BY posted_at
        $copy$;
    END IF;
END
$legacy$;

CREATE VIEW ops.patch_freshness AS
WITH listing AS (
    SELECT c.patch_id,btrim(c.label) AS label,c.official_url AS url,c.released_at AS published_at,
        least(
            (SELECT min(r.fetched_at) FROM raw.object r
                WHERE r.source=o.source AND r.path=o.path),
            (SELECT l.detected_at FROM ops.patch_detection_legacy l WHERE l.published_at=c.released_at)
        ) AS detected_at
    FROM serve.p4_patch_catalog c
    JOIN raw.object o ON o.id=c.raw_object_id AND o.source='deadlock_patchnotes_db'
    WHERE c.released_at IS NOT NULL
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

CREATE VIEW ops.game_data_freshness AS
SELECT c.revision,c.client_build,c.committed_at,c.subject,p.patch_id,p.published_at,
    p.patch_id IS NULL AS silent,i.run_id AS ingest_run_id,i.inferred AS ingest_inferred,
    i.finished AS game_data_live_at,i.finished-c.committed_at AS ingest_lag
FROM ops.upstream_commit c
LEFT JOIN LATERAL (
    SELECT min(pc.patch_id) AS patch_id,pc.released_at AS published_at FROM serve.p4_patch_catalog pc
    WHERE pc.released_at<=c.committed_at AND pc.released_at>c.committed_at-interval '12 hours'
    GROUP BY pc.released_at
    ORDER BY pc.released_at DESC
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

REVOKE ALL ON ops.upstream_commit,ops.ingest_run_revision,ops.patch_detection_legacy,
    ops.patch_freshness,ops.game_data_freshness FROM PUBLIC;
GRANT SELECT,INSERT ON ops.upstream_commit,ops.ingest_run_revision TO brain_ingest;
GRANT SELECT ON ops.patch_detection_legacy,ops.patch_freshness,ops.game_data_freshness TO brain_ingest;

COMMIT;
