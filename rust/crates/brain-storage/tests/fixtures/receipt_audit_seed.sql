BEGIN;
INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','fixture','v1');
INSERT INTO raw.source(kind,locator) VALUES ('fixture','synthetic://p9-receipts');
INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version)
    VALUES (1,1,'fixture-a','fixture','v1');
INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
    VALUES (1,'fixture.json','fixture-a',sha256(convert_to('{"health":100,"damage":25}', 'UTF8')),
        'application/json',convert_to('{"health":100,"damage":25}', 'UTF8'));
INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status)
    VALUES (1,'fixture.json',1,'upsert','succeeded');
INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version)
    VALUES (1,1,'fixture-a','fixture','v1');
INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
    SELECT 1,1,1,'fixture',field,sha256(convert_to(value, 'UTF8')),'fixture','v1'
    FROM (VALUES ('hero','hero'),('ability','ability'),('health','100'),('damage','25'),('link','ability')) fields(field,value);
INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','audit_hero',1),('ability','audit_ability',2);
INSERT INTO canon.stat(key,section,value_type) VALUES ('health','stats','number'),('damage','ability','number');
INSERT INTO canon.fact(entity_id,stat_key,source_id,source_path,value_num,receipt_id,valid_from_revision_id)
    VALUES (1,'health',1,'fixture.json',100,3,1),(2,'damage',1,'fixture.json',25,4,1);
INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id,source_id,source_path,valid_from_revision_id)
    VALUES (1,'audit_hero','en',1,1,'fixture.json',1);
INSERT INTO canon.relation(from_entity_id,kind,to_entity_id,slot,receipt_id,source_id,source_path,valid_from_revision_id)
    VALUES (1,'has_ability',2,1,5,1,'fixture.json',1);
UPDATE raw.run SET status='succeeded' WHERE id=1;
INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
UPDATE raw.source SET last_complete_revision='fixture-a' WHERE id=1;
COMMIT;
