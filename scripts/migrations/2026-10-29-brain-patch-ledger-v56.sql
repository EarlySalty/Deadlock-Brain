BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Patch-Ledger benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE OR REPLACE VIEW ops.patch_listing AS
WITH object AS (
    SELECT o.id,o.path,o.fetched_at,ops.patch_body(o.bytes) AS body,
        r.applied_run IS NOT NULL AS applied,coalesce(r.applied_run,o.ingest_run_id) AS observed_run
    FROM raw.object o
    LEFT JOIN LATERAL (
        SELECT max(x.ingest_run_id) AS applied_run FROM canon.receipt x WHERE x.raw_object_id=o.id
    ) r ON true
    WHERE o.source='deadlock_patchnotes_db' AND ops.is_patch_id(o.path)
), observed AS (
    SELECT DISTINCT ON (p.path) p.path,p.raw_object_id FROM ops.patch_observation p
    ORDER BY p.path,p.run_id DESC
), latest AS (
    SELECT DISTINCT ON (path) path,body,
        CASE WHEN pg_input_is_valid(body->>'posted_at','timestamptz')
            THEN (body->>'posted_at')::timestamptz END AS published_at
    FROM object
    WHERE jsonb_typeof(body)='object'
    ORDER BY path,
        EXISTS(SELECT 1 FROM observed v WHERE v.path=object.path AND v.raw_object_id=object.id) DESC,
        applied DESC,observed_run DESC,id DESC
), first AS (
    SELECT path,min(fetched_at) AS fetched_at FROM object GROUP BY path
)
SELECT l.path::bigint AS patch_id,btrim(l.body->>'title') AS label,l.body->>'url' AS url,
    l.published_at,f.fetched_at AS detected_at,coalesce(l.body->>'raw_content','') AS raw_content
FROM latest l
JOIN first f ON f.path=l.path
WHERE nullif(btrim(l.body->>'title'),'') IS NOT NULL
    AND isfinite(l.published_at);

CREATE TABLE ops.patch_ledger (
    ledger_id bigint PRIMARY KEY CHECK (ledger_id>0),
    dated_post_id bigint NOT NULL CHECK (dated_post_id>0),
    released_at timestamptz NOT NULL CHECK (isfinite(released_at)),
    kind text NOT NULL CHECK (kind IN ('major','gameplay','hotfix')),
    label text NOT NULL CHECK (btrim(label)<>''),
    source_url text NOT NULL CHECK (source_url ~ '^https://'),
    run_id bigint NOT NULL REFERENCES ops.ingest_run(id),
    CONSTRAINT patch_ledger_label_key UNIQUE (label) DEFERRABLE INITIALLY DEFERRED
);
CREATE INDEX patch_ledger_released ON ops.patch_ledger (released_at,ledger_id);

CREATE TABLE ops.patch_ledger_post (
    post_id bigint PRIMARY KEY CHECK (post_id>0),
    ledger_id bigint REFERENCES ops.patch_ledger(ledger_id) DEFERRABLE INITIALLY DEFERRED,
    disposition text NOT NULL CHECK (disposition IN ('primary','duplicate','excluded')),
    reason text NOT NULL CHECK (btrim(reason)<>''),
    run_id bigint NOT NULL REFERENCES ops.ingest_run(id),
    CHECK ((disposition='excluded')=(ledger_id IS NULL)),
    CHECK (disposition<>'primary' OR ledger_id=post_id)
);
CREATE INDEX patch_ledger_post_ledger ON ops.patch_ledger_post (ledger_id);

CREATE VIEW serve.patch_ledger AS
WITH catalog AS MATERIALIZED (
    SELECT c.patch_id,c.released_at,c.released_at_receipt_id,c.official_url
    FROM serve.p4_patch_catalog c
)
SELECT l.ledger_id,l.kind,l.label,l.released_at,l.source_url,p.patch_id,
    d.released_at_receipt_id,r.source_revision,
    (SELECT count(*) FROM ops.patch_ledger_post m WHERE m.ledger_id=l.ledger_id) AS posts
FROM ops.patch_ledger l
JOIN canon.entity pe ON pe.key='patch_'||l.ledger_id
JOIN catalog p ON p.patch_id=pe.id
JOIN canon.entity de ON de.key='patch_'||l.dated_post_id
JOIN catalog d ON d.patch_id=de.id AND d.released_at=l.released_at
CROSS JOIN LATERAL (
    SELECT r.source_revision FROM serve.published_receipt r
    WHERE r.id=d.released_at_receipt_id
    OFFSET 0
) r;

CREATE VIEW serve.patch_ledger_member AS
SELECT e.id AS patch_id,m.post_id,m.ledger_id,m.disposition,l.released_at AS ledger_released_at
FROM ops.patch_ledger_post m
JOIN canon.entity e ON e.key='patch_'||m.post_id
LEFT JOIN ops.patch_ledger l ON l.ledger_id=m.ledger_id;

REVOKE ALL ON ops.patch_ledger,ops.patch_ledger_post,serve.patch_ledger,serve.patch_ledger_member FROM PUBLIC;
GRANT SELECT ON ops.patch_listing TO brain_ingest;
GRANT SELECT,INSERT,UPDATE,DELETE ON ops.patch_ledger,ops.patch_ledger_post TO brain_ingest;
GRANT SELECT ON serve.patch_ledger,serve.patch_ledger_member TO brain_tool;

COMMIT;
