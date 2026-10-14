BEGIN;

CREATE TABLE ops.receipt_audit_run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    started timestamptz NOT NULL DEFAULT clock_timestamp(),
    finished timestamptz NOT NULL DEFAULT clock_timestamp(),
    status text NOT NULL DEFAULT 'succeeded' CHECK(status='succeeded'),
    sampled integer NOT NULL DEFAULT 0 CHECK(sampled BETWEEN 0 AND 200),
    verified_receipts integer NOT NULL DEFAULT 0 CHECK(verified_receipts>=0),
    rejected_receipts integer NOT NULL DEFAULT 0 CHECK(rejected_receipts>=0),
    rejected_objects integer NOT NULL DEFAULT 0 CHECK(rejected_objects BETWEEN 0 AND 200),
    CHECK(sampled=verified_receipts+rejected_receipts)
);
CREATE TABLE ops.receipt_audit_rejection (
    run_id bigint NOT NULL REFERENCES ops.receipt_audit_run(id),
    raw_object_id bigint NOT NULL REFERENCES raw.object(id),
    code text NOT NULL CHECK(octet_length(code)<=128),
    detail text NOT NULL CHECK(octet_length(detail)<={{DETAIL_BYTES}}),
    PRIMARY KEY(run_id,raw_object_id)
);
CREATE INDEX receipt_audit_rejection_object_idx ON ops.receipt_audit_rejection(raw_object_id);
CREATE TABLE ops.receipt_audit (
    run_id bigint NOT NULL REFERENCES ops.receipt_audit_run(id),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    outcome text NOT NULL CHECK(outcome IN('match','mismatch','locator_not_found','unsupported_locator_kind','object_rejected')),
    observed_value_sha256 bytea CHECK(octet_length(observed_value_sha256)=32),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY(run_id,receipt_id),
    CHECK((outcome IN('match','mismatch'))=(observed_value_sha256 IS NOT NULL))
);
CREATE INDEX receipt_audit_receipt_idx ON ops.receipt_audit(receipt_id,run_id DESC);
CREATE TABLE canon.receipt_quarantine (
    receipt_id bigint PRIMARY KEY REFERENCES canon.receipt(id),
    run_id bigint NOT NULL,
    audited_receipt_id bigint NOT NULL,
    cause text NOT NULL CHECK(cause IN('audited','shadowed')),
    quarantined_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    CHECK((cause='audited')=(audited_receipt_id=receipt_id)),
    FOREIGN KEY(run_id,audited_receipt_id) REFERENCES ops.receipt_audit(run_id,receipt_id)
);

CREATE FUNCTION ops.serialize_receipt_audit_write() RETURNS trigger
LANGUAGE plpgsql SET search_path=pg_catalog,pg_temp AS $serialize$
BEGIN
    IF current_setting('transaction_isolation')<>'read committed' THEN
        RAISE EXCEPTION 'Belegprüfungen benötigen READ COMMITTED' USING ERRCODE='0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    RETURN NULL;
END
$serialize$;
CREATE TRIGGER receipt_audit_write_lock BEFORE INSERT ON ops.receipt_audit_run
    FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_receipt_audit_write();

CREATE FUNCTION ops.quarantine_audited_receipts() RETURNS trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $quarantine$
BEGIN
    IF EXISTS(SELECT 1 FROM audit_results a WHERE
        (SELECT count(*) FROM ops.receipt_audit r WHERE r.run_id=a.run_id)>200) THEN
        RAISE EXCEPTION 'Belegprüfung enthält zu viele Ergebnisse' USING ERRCODE='23514';
    END IF;
    INSERT INTO canon.receipt_quarantine AS q(receipt_id,run_id,audited_receipt_id,cause)
        SELECT DISTINCT ON(receipt_id) receipt_id,run_id,receipt_id,'audited' FROM audit_results
        WHERE outcome IN('mismatch','locator_not_found','object_rejected') ORDER BY receipt_id,run_id DESC
        ON CONFLICT(receipt_id) DO UPDATE SET run_id=EXCLUDED.run_id,
            audited_receipt_id=EXCLUDED.audited_receipt_id,cause='audited',quarantined_at=clock_timestamp()
        WHERE q.cause='shadowed';
    INSERT INTO canon.receipt_quarantine(receipt_id,run_id,audited_receipt_id,cause)
        SELECT DISTINCT ON(older.id) older.id,a.run_id,a.receipt_id,'shadowed'
        FROM audit_results a JOIN canon.receipt bad ON bad.id=a.receipt_id
        CROSS JOIN LATERAL(
            SELECT o.receipt_id FROM canon.fact c JOIN canon.fact o USING(entity_id,stat_key)
                WHERE c.receipt_id=a.receipt_id
            UNION ALL SELECT o.receipt_id FROM canon.alias c JOIN canon.alias o USING(entity_id,alias_norm,lang)
                WHERE c.receipt_id=a.receipt_id
            UNION ALL SELECT o.receipt_id FROM canon.relation c JOIN canon.relation o USING(from_entity,kind,to_entity)
                WHERE c.receipt_id=a.receipt_id
            UNION ALL SELECT o.receipt_id FROM canon.entity_identity c JOIN canon.entity_identity o USING(entity_id)
                WHERE c.receipt_id=a.receipt_id
        ) peer
        JOIN canon.receipt older ON older.id=peer.receipt_id AND (older.ingest_run_id,older.id)<(bad.ingest_run_id,bad.id)
        WHERE a.outcome IN('mismatch','locator_not_found','object_rejected')
        ORDER BY older.id,a.run_id DESC,a.receipt_id
        ON CONFLICT(receipt_id) DO NOTHING;
    RETURN NULL;
END
$quarantine$;
CREATE TRIGGER receipt_audit_quarantine AFTER INSERT ON ops.receipt_audit
    REFERENCING NEW TABLE AS audit_results FOR EACH STATEMENT
    EXECUTE FUNCTION ops.quarantine_audited_receipts();

CREATE OR REPLACE VIEW serve.published_receipt AS
WITH boundary AS MATERIALIZED(
    SELECT bad.id,bad.ingest_run_id FROM (
        SELECT receipt_id FROM canon.receipt_quarantine WHERE cause='audited'
        UNION
        SELECT affected.id FROM ops.receipt_audit_rejection rejected
            JOIN canon.receipt affected ON affected.raw_object_id=rejected.raw_object_id
    ) b JOIN canon.receipt bad ON bad.id=b.receipt_id
), shadowed AS MATERIALIZED(
    SELECT older.id FROM boundary bad
    JOIN canon.fact c ON c.receipt_id=bad.id
    JOIN canon.fact o USING(entity_id,stat_key)
    JOIN canon.receipt older ON older.id=o.receipt_id AND (older.ingest_run_id,older.id)<(bad.ingest_run_id,bad.id)
    UNION ALL
    SELECT older.id FROM boundary bad
    JOIN canon.alias c ON c.receipt_id=bad.id
    JOIN canon.alias o USING(entity_id,alias_norm,lang)
    JOIN canon.receipt older ON older.id=o.receipt_id AND (older.ingest_run_id,older.id)<(bad.ingest_run_id,bad.id)
    UNION ALL
    SELECT older.id FROM boundary bad
    JOIN canon.relation c ON c.receipt_id=bad.id
    JOIN canon.relation o USING(from_entity,kind,to_entity)
    JOIN canon.receipt older ON older.id=o.receipt_id AND (older.ingest_run_id,older.id)<(bad.ingest_run_id,bad.id)
    UNION ALL
    SELECT older.id FROM boundary bad
    JOIN canon.entity_identity c ON c.receipt_id=bad.id
    JOIN canon.entity_identity o USING(entity_id)
    JOIN canon.receipt older ON older.id=o.receipt_id AND (older.ingest_run_id,older.id)<(bad.ingest_run_id,bad.id)
)
SELECT r.id,r.raw_object_id,r.ingest_run_id,r.extractor,r.locator_kind,r.locator,r.value_sha256,
    o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256
FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
JOIN raw.object o ON o.id=r.raw_object_id
WHERE NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id)
AND NOT EXISTS(SELECT 1 FROM ops.receipt_audit_rejection rejected WHERE rejected.raw_object_id=r.raw_object_id)
AND NOT EXISTS(SELECT 1 FROM shadowed s WHERE s.id=r.id);

CREATE FUNCTION ops.sample_receipt_audit()
RETURNS TABLE(id bigint,raw_object_id bigint,locator_kind text,locator text,value_sha256 bytea)
LANGUAGE sql STABLE SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $sample$
    WITH serving AS(
        SELECT receipt_id FROM serve.current_fact
        UNION SELECT receipt_id FROM serve.current_alias
        UNION SELECT receipt_id FROM serve.current_relation
        UNION SELECT receipt_id FROM serve.current_entity
    )
    SELECT p.id,p.raw_object_id,p.locator_kind,p.locator,p.value_sha256
    FROM serving s JOIN serve.published_receipt p ON p.id=s.receipt_id
    LEFT JOIN LATERAL(
        SELECT a.run_id FROM ops.receipt_audit a WHERE a.receipt_id=s.receipt_id
        ORDER BY a.run_id DESC LIMIT 1
    ) last ON true
    ORDER BY last.run_id NULLS FIRST,p.id LIMIT 200
$sample$;

REVOKE ALL ON ops.receipt_audit_run,ops.receipt_audit_rejection,ops.receipt_audit,canon.receipt_quarantine
    FROM PUBLIC,brain_tool,brain_ingest,brain_service;
REVOKE ALL ON FUNCTION ops.serialize_receipt_audit_write(),ops.quarantine_audited_receipts(),ops.sample_receipt_audit()
    FROM PUBLIC,brain_tool,brain_ingest,brain_service;
GRANT EXECUTE ON FUNCTION ops.sample_receipt_audit() TO brain_ingest;
GRANT USAGE ON SEQUENCE ops.receipt_audit_run_id_seq TO brain_ingest;
GRANT SELECT ON ops.receipt_audit_run,ops.receipt_audit_rejection,ops.receipt_audit,canon.receipt_quarantine TO brain_ingest;
GRANT INSERT(sampled,verified_receipts,rejected_receipts,rejected_objects),
    UPDATE(finished,sampled,verified_receipts,rejected_receipts,rejected_objects) ON ops.receipt_audit_run TO brain_ingest;
GRANT INSERT(run_id,raw_object_id,code,detail) ON ops.receipt_audit_rejection TO brain_ingest;
GRANT INSERT(run_id,receipt_id,outcome,observed_value_sha256) ON ops.receipt_audit TO brain_ingest;

COMMIT;
