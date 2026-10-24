BEGIN;

CREATE INDEX receipt_ingest_run ON canon.receipt(ingest_run_id);

CREATE FUNCTION serve.site_generation()
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
            FROM canon.entity e)
    ));
END
$generation$;
REVOKE ALL ON FUNCTION serve.site_generation() FROM PUBLIC;

DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.site_generation() TO brain_site;
    END IF;
END
$site$;

COMMIT;
