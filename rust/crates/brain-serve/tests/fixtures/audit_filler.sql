DO $seed$
DECLARE run bigint; audit bigint;
BEGIN
    INSERT INTO ops.ingest_run(source,status) VALUES('assets','failed') RETURNING id INTO run;
    INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id)
        SELECT 'assets',repeat('a',40),'filler/'||n||'.json',sha256(('filler:'||n)::bytea),sha256(n::text::bytea),'{}'::bytea,run
        FROM generate_series(1,6000) n;
    INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256)
        SELECT o.id,run,'assets','bytes','0:2',sha256(o.id::text::bytea) FROM raw.object o WHERE o.ingest_run_id=run;
    INSERT INTO canon.entity(kind,key,receipt_id)
        SELECT 'item','filler_'||r.raw_object_id,r.id FROM canon.receipt r WHERE r.ingest_run_id=run;
    INSERT INTO canon.entity_identity(entity_id,receipt_id) SELECT e.id,e.receipt_id FROM canon.entity e
        JOIN canon.receipt r ON r.id=e.receipt_id WHERE r.ingest_run_id=run;
    INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) SELECT e.id,'cost',1,run,e.receipt_id
        FROM canon.entity e JOIN canon.receipt r ON r.id=e.receipt_id WHERE r.ingest_run_id=run;
    UPDATE ops.ingest_run SET status='succeeded',finished=clock_timestamp() WHERE id=run;
    FOR round IN 1..25 LOOP
        INSERT INTO ops.receipt_audit_run(sampled,verified_receipts,rejected_receipts,rejected_objects)
            VALUES(200,160,40,40) RETURNING id INTO audit;
        INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256)
            SELECT audit,s.id,CASE WHEN s.n<=40 THEN 'mismatch' ELSE 'match' END,sha256(''::bytea)
            FROM (SELECT r.id,row_number() OVER (ORDER BY r.id) AS n FROM canon.receipt r
                WHERE r.ingest_run_id=run AND r.id%25=round-1 ORDER BY r.id LIMIT 200) s;
        INSERT INTO ops.receipt_audit_rejection(run_id,raw_object_id,code,detail)
            SELECT audit,o.id,'hash_mismatch','seeded' FROM raw.object o
            WHERE o.ingest_run_id=run AND o.id%25=(round+12)%25 ORDER BY o.id DESC LIMIT 40;
    END LOOP;
END
$seed$
