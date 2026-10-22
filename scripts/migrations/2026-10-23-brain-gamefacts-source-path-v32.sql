BEGIN;
CREATE INDEX object_source_path ON raw.object(source,path);
COMMIT;
