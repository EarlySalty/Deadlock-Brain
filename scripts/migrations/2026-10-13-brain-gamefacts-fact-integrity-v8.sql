BEGIN;
CREATE FUNCTION canon.guard_fact_integrity() RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $fact$
DECLARE
    receipt_run bigint;
    run_status text;
BEGIN
    IF TG_OP='UPDATE' THEN
        SELECT ingest_run_id INTO receipt_run FROM canon.receipt WHERE id=OLD.receipt_id;
        IF receipt_run IS DISTINCT FROM OLD.valid_from_run THEN
            RAISE EXCEPTION 'Spielwert mit widersprüchlichem Lauf und Beleg bleibt unverändert' USING ERRCODE='23514';
        END IF;
        SELECT status INTO run_status FROM ops.ingest_run WHERE id=OLD.valid_from_run FOR UPDATE;
        IF run_status IS DISTINCT FROM 'failed' THEN
            RAISE EXCEPTION 'Veröffentlichte Spielwerte bleiben unverändert' USING ERRCODE='23514';
        END IF;
    END IF;
    SELECT ingest_run_id INTO receipt_run FROM canon.receipt WHERE id=NEW.receipt_id;
    IF receipt_run IS DISTINCT FROM NEW.valid_from_run THEN
        RAISE EXCEPTION 'Spielwert benötigt den Beleg seines Laufs' USING ERRCODE='23514';
    END IF;
    SELECT status INTO run_status FROM ops.ingest_run WHERE id=NEW.valid_from_run FOR UPDATE;
    IF run_status IS DISTINCT FROM 'failed' THEN
        RAISE EXCEPTION 'Spielwert benötigt einen unveröffentlichten Lauf' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$fact$;
REVOKE ALL ON FUNCTION canon.guard_fact_integrity() FROM PUBLIC;
CREATE TRIGGER guard_fact_integrity BEFORE INSERT OR UPDATE ON canon.fact
FOR EACH ROW EXECUTE FUNCTION canon.guard_fact_integrity();
COMMIT;
