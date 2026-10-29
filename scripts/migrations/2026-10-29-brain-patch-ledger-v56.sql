BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Patch-Ledger benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE VIEW ops.patch_ledger_source AS
SELECT o.path::bigint AS post_id,
    coalesce(nullif(btrim(b.body->>'title'),''),btrim(c.label)) AS title,
    c.label,c.official_url AS url,c.released_at AS published_at,
    coalesce(b.body->>'raw_content','') AS raw_content
FROM serve.p4_patch_catalog c
JOIN raw.object o ON o.id=c.raw_object_id AND o.source='deadlock_patchnotes_db' AND ops.is_patch_id(o.path)
JOIN canon.entity e ON e.id=c.patch_id AND e.key='patch_'||o.path
CROSS JOIN LATERAL (SELECT ops.patch_body(o.bytes) AS body) b
WHERE nullif(btrim(c.label),'') IS NOT NULL
    AND isfinite(c.released_at)
    AND c.official_url ~ '^https://';

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
JOIN catalog p ON p.patch_id=pe.id AND p.official_url=l.source_url
JOIN canon.entity de ON de.key='patch_'||l.dated_post_id
JOIN catalog d ON d.patch_id=de.id AND d.released_at=l.released_at
CROSS JOIN LATERAL (
    SELECT r.source_revision FROM serve.published_receipt r
    WHERE r.id=d.released_at_receipt_id
    OFFSET 0
) r;

CREATE VIEW serve.patch_ledger_member AS
SELECT e.id AS patch_id,m.post_id,m.ledger_id,m.disposition,l.released_at AS ledger_released_at,
    l.label AS ledger_label
FROM ops.patch_ledger_post m
JOIN canon.entity e ON e.key='patch_'||m.post_id
LEFT JOIN ops.patch_ledger l ON l.ledger_id=m.ledger_id;

CREATE OR REPLACE FUNCTION serve.site_generation()
RETURNS text
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $generation$
BEGIN
    RETURN md5(concat_ws('|',
        (SELECT string_agg(r.id::text,',' ORDER BY r.id) FROM ops.ingest_run r
            WHERE r.status='succeeded'
                AND EXISTS(SELECT 1 FROM canon.receipt c WHERE c.ingest_run_id=r.id)),
        (SELECT count(*)::text||':'||COALESCE(sum(q.receipt_id),0)::text||':'
                ||count(*) FILTER (WHERE q.cause='audited')::text
            FROM canon.receipt_quarantine q),
        (SELECT count(*)::text||':'||COALESCE(sum(a.raw_object_id),0)::text||':'
                ||COALESCE(sum(a.run_id),0)::text
            FROM ops.receipt_audit_rejection a),
        (SELECT md5(string_agg(e.id::text||':'||e.kind::text||':'||e.key::text,',' ORDER BY e.id))
            FROM canon.entity e),
        (SELECT 'ledger:'||md5(COALESCE(string_agg(concat_ws(':',l.ledger_id,l.dated_post_id,
                extract(epoch FROM l.released_at),l.kind,l.label,l.source_url),',' ORDER BY l.ledger_id),''))
            FROM ops.patch_ledger l),
        (SELECT 'posts:'||md5(COALESCE(string_agg(concat_ws(':',m.post_id,m.ledger_id,m.disposition),
                ',' ORDER BY m.post_id),''))
            FROM ops.patch_ledger_post m)
    ));
END
$generation$;

REVOKE ALL ON ops.patch_ledger_source,ops.patch_ledger,ops.patch_ledger_post,serve.patch_ledger,serve.patch_ledger_member FROM PUBLIC;
GRANT SELECT ON ops.patch_ledger_source TO brain_ingest;
GRANT SELECT,INSERT,UPDATE,DELETE ON ops.patch_ledger,ops.patch_ledger_post TO brain_ingest;
GRANT SELECT ON serve.patch_ledger,serve.patch_ledger_member TO brain_tool;

COMMIT;
