BEGIN;
CREATE FUNCTION canon.write_run_fact(
    p_entity bigint, p_key text, p_num double precision, p_text text,
    p_bool boolean, p_run bigint, p_receipt bigint
) RETURNS void LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog AS $fact$
DECLARE
    run_status text;
BEGIN
    SELECT status INTO run_status FROM ops.ingest_run WHERE id=p_run FOR UPDATE;
    IF run_status IS DISTINCT FROM 'failed' OR NOT EXISTS (
        SELECT 1 FROM canon.receipt WHERE id=p_receipt AND ingest_run_id=p_run
    ) THEN
        RAISE EXCEPTION 'Spielwert benötigt einen unveröffentlichten Lauf und dessen Beleg' USING ERRCODE='23514';
    END IF;
    INSERT INTO canon.fact(entity_id,stat_key,value_num,value_text,value_bool,valid_from_run,receipt_id)
    VALUES(p_entity,p_key,p_num,p_text,p_bool,p_run,p_receipt)
    ON CONFLICT(entity_id,stat_key,valid_from_run) DO UPDATE
    SET value_num=EXCLUDED.value_num,value_text=EXCLUDED.value_text,
        value_bool=EXCLUDED.value_bool,receipt_id=EXCLUDED.receipt_id;
END;
$fact$;
REVOKE ALL ON FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) FROM PUBLIC;
GRANT EXECUTE ON FUNCTION canon.write_run_fact(bigint,text,double precision,text,boolean,bigint,bigint) TO brain_ingest;
CREATE FUNCTION ops.keep_published_run() RETURNS trigger LANGUAGE plpgsql SET search_path=pg_catalog AS $run$
BEGIN
    IF OLD.status='succeeded' AND NEW.status IS DISTINCT FROM 'succeeded' THEN
        RAISE EXCEPTION 'Veröffentlichte Läufe bleiben unverändert sichtbar' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$run$;
REVOKE ALL ON FUNCTION ops.keep_published_run() FROM PUBLIC;
CREATE TRIGGER keep_published_run BEFORE UPDATE OF status ON ops.ingest_run
FOR EACH ROW EXECUTE FUNCTION ops.keep_published_run();
COMMIT;
