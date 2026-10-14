BEGIN;
CREATE FUNCTION canon.guard_receipt_insert() RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $receipt$
DECLARE
    run_status text;
BEGIN
    SELECT status INTO run_status FROM ops.ingest_run WHERE id=NEW.ingest_run_id FOR UPDATE;
    IF run_status IS DISTINCT FROM 'failed' THEN
        RAISE EXCEPTION 'Beleg benötigt einen unveröffentlichten Lauf' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$receipt$;
CREATE FUNCTION canon.guard_reference_insert() RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $reference$
DECLARE
    receipt_run bigint;
    run_status text;
BEGIN
    SELECT ingest_run_id INTO receipt_run FROM canon.receipt WHERE id=NEW.receipt_id;
    SELECT status INTO run_status FROM ops.ingest_run WHERE id=receipt_run FOR UPDATE;
    IF run_status IS DISTINCT FROM 'failed' THEN
        RAISE EXCEPTION 'Kanonische Zeile benötigt einen unveröffentlichten Beleg' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$reference$;
REVOKE ALL ON FUNCTION canon.guard_receipt_insert(),canon.guard_reference_insert() FROM PUBLIC,brain_ingest,brain_tool,brain_service,brain_readonly;
CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.receipt FOR EACH ROW EXECUTE FUNCTION canon.guard_receipt_insert();
CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.entity FOR EACH ROW EXECUTE FUNCTION canon.guard_reference_insert();
CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.entity_identity FOR EACH ROW EXECUTE FUNCTION canon.guard_reference_insert();
CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.alias FOR EACH ROW EXECUTE FUNCTION canon.guard_reference_insert();
CREATE TRIGGER guard_published_insert BEFORE INSERT ON canon.relation FOR EACH ROW EXECUTE FUNCTION canon.guard_reference_insert();
COMMIT;
