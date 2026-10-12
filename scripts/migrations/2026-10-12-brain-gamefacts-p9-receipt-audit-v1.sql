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
    quarantined_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY(run_id,receipt_id) REFERENCES ops.receipt_audit(run_id,receipt_id)
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
    INSERT INTO canon.receipt_quarantine(receipt_id,run_id)
        SELECT receipt_id,run_id FROM audit_results WHERE outcome IN('mismatch','locator_not_found')
        ON CONFLICT(receipt_id) DO NOTHING;
    RETURN NULL;
END
$quarantine$;
CREATE TRIGGER receipt_audit_quarantine AFTER INSERT ON ops.receipt_audit
    REFERENCING NEW TABLE AS audit_results FOR EACH STATEMENT
    EXECUTE FUNCTION ops.quarantine_audited_receipts();

CREATE OR REPLACE VIEW serve.published_receipt AS
SELECT r.*,o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256
FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
JOIN raw.object o ON o.id=r.raw_object_id
WHERE NOT EXISTS(SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id=r.id);

REVOKE ALL ON ops.receipt_audit_run,ops.receipt_audit_rejection,ops.receipt_audit,canon.receipt_quarantine
    FROM PUBLIC,brain_tool,brain_ingest,brain_service;
REVOKE ALL ON FUNCTION ops.serialize_receipt_audit_write(),ops.quarantine_audited_receipts()
    FROM PUBLIC,brain_tool,brain_ingest,brain_service;
GRANT USAGE ON SEQUENCE ops.receipt_audit_run_id_seq TO brain_ingest;
GRANT SELECT ON ops.receipt_audit_run,ops.receipt_audit_rejection,ops.receipt_audit,canon.receipt_quarantine,serve.published_receipt TO brain_ingest;
GRANT USAGE ON SCHEMA serve TO brain_ingest;
GRANT INSERT(sampled,verified_receipts,rejected_receipts,rejected_objects),
    UPDATE(finished,sampled,verified_receipts,rejected_receipts,rejected_objects) ON ops.receipt_audit_run TO brain_ingest;
GRANT INSERT(run_id,raw_object_id,code,detail) ON ops.receipt_audit_rejection TO brain_ingest;
GRANT INSERT(run_id,receipt_id,outcome,observed_value_sha256) ON ops.receipt_audit TO brain_ingest;

COMMIT;
