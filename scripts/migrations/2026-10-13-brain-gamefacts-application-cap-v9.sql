BEGIN;
ALTER TABLE canon.receipt ADD COLUMN application_xid xid8 NOT NULL DEFAULT pg_catalog.pg_current_xact_id();
CREATE INDEX receipt_object_application ON canon.receipt(raw_object_id,ingest_run_id,extractor,application_xid);
CREATE OR REPLACE FUNCTION canon.check_object_rows() RETURNS trigger LANGUAGE plpgsql SET search_path=pg_catalog AS $rows$
BEGIN
    IF (SELECT count(*) FROM (SELECT 1 FROM canon.receipt WHERE raw_object_id=NEW.raw_object_id AND ingest_run_id=NEW.ingest_run_id AND extractor=NEW.extractor AND application_xid=NEW.application_xid LIMIT {{ROWS}}+1) application_rows)>{{ROWS}} THEN
        RAISE EXCEPTION 'Objekt enthält zu viele kanonische Zeilen' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$rows$;
REVOKE ALL ON FUNCTION canon.check_object_rows() FROM PUBLIC;
COMMIT;
