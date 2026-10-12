BEGIN;
CREATE SCHEMA raw AUTHORIZATION brain_migrate;
CREATE SCHEMA canon AUTHORIZATION brain_migrate;
CREATE SCHEMA serve AUTHORIZATION brain_migrate;
CREATE SCHEMA ops AUTHORIZATION brain_migrate;
REVOKE ALL ON SCHEMA raw,canon,serve,ops FROM PUBLIC;
CREATE DOMAIN canon.key AS text CHECK (octet_length(VALUE)<={{KEY_BYTES}} AND VALUE COLLATE "C" ~ '{{KEY_PATTERN}}');
CREATE DOMAIN canon.short_text AS text CHECK (octet_length(VALUE)<={{ALIAS_BYTES}});
CREATE DOMAIN canon.value_text AS text CHECK (octet_length(VALUE)<={{TEXT_BYTES}});
CREATE DOMAIN canon.locator AS text CHECK (octet_length(VALUE)<={{LOCATOR_BYTES}});
CREATE DOMAIN canon.sha256 AS bytea CHECK (octet_length(VALUE)=32);
CREATE TABLE ops.gamefacts_schema_version(version integer PRIMARY KEY CHECK(version>0),checksum bytea NOT NULL CHECK(octet_length(checksum)=32));
CREATE TABLE ops.ingest_run(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source text NOT NULL,
    started timestamptz NOT NULL DEFAULT clock_timestamp(),
    finished timestamptz NOT NULL DEFAULT clock_timestamp(),
    status text NOT NULL DEFAULT 'failed' CHECK(status IN ('succeeded','failed')),
    accepted bigint NOT NULL DEFAULT 0 CHECK(accepted>=0),
    rejected bigint NOT NULL DEFAULT 0 CHECK(rejected>=0),
    ignored bigint NOT NULL DEFAULT 0 CHECK(ignored>=0)
);
CREATE TABLE raw.object(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source text NOT NULL,
    revision text NOT NULL,
    path text NOT NULL,
    source_path_sha256 canon.sha256 NOT NULL,
    content_sha256 canon.sha256 NOT NULL,
    bytes bytea NOT NULL,
    ingest_run_id bigint NOT NULL REFERENCES ops.ingest_run(id),
    fetched_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE(source_path_sha256,content_sha256)
);
CREATE TABLE ops.ingest_rejection(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    run_id bigint NOT NULL REFERENCES ops.ingest_run(id),
    raw_object_id bigint NOT NULL REFERENCES raw.object(id),
    extractor text NOT NULL,
    code text NOT NULL,
    detail text NOT NULL CHECK(octet_length(detail)<={{DETAIL_BYTES}})
);
CREATE TABLE canon.receipt(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    raw_object_id bigint NOT NULL REFERENCES raw.object(id),
    ingest_run_id bigint NOT NULL REFERENCES ops.ingest_run(id),
    extractor text NOT NULL,
    locator_kind canon.key NOT NULL,
    locator canon.locator NOT NULL,
    value_sha256 canon.sha256 NOT NULL
);
CREATE INDEX receipt_object_run ON canon.receipt(raw_object_id,ingest_run_id,extractor);
CREATE FUNCTION canon.check_object_rows() RETURNS trigger LANGUAGE plpgsql AS $rows$
BEGIN
    IF (SELECT count(*) FROM canon.receipt WHERE raw_object_id=NEW.raw_object_id AND ingest_run_id=NEW.ingest_run_id AND extractor=NEW.extractor)>{{ROWS}} THEN
        RAISE EXCEPTION 'Objekt enthält zu viele kanonische Zeilen' USING ERRCODE='23514';
    END IF;
    RETURN NEW;
END;
$rows$;
REVOKE ALL ON FUNCTION canon.check_object_rows() FROM PUBLIC;
CREATE CONSTRAINT TRIGGER object_rows AFTER INSERT ON canon.receipt DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION canon.check_object_rows();
CREATE TABLE canon.entity(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    kind canon.key NOT NULL,
    key canon.key NOT NULL UNIQUE,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id)
);
CREATE TABLE canon.entity_identity(
    entity_id bigint NOT NULL REFERENCES canon.entity(id),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    PRIMARY KEY(entity_id,receipt_id)
);
CREATE TABLE canon.alias(
    entity_id bigint NOT NULL REFERENCES canon.entity(id),
    alias_norm canon.short_text NOT NULL,
    lang canon.key NOT NULL,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    PRIMARY KEY(entity_id,alias_norm,lang,receipt_id)
);
CREATE INDEX alias_lookup ON canon.alias(alias_norm);
CREATE TABLE canon.fact(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    entity_id bigint NOT NULL REFERENCES canon.entity(id),
    stat_key canon.key NOT NULL,
    value_num double precision CHECK(value_num>=-{{NUMBER_ABS}} AND value_num<={{NUMBER_ABS}}),
    value_text canon.value_text,
    value_bool boolean,
    valid_from_run bigint NOT NULL REFERENCES ops.ingest_run(id),
    valid_to_run bigint REFERENCES ops.ingest_run(id),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    CHECK(num_nonnulls(value_num,value_text,value_bool)=1),
    UNIQUE(entity_id,stat_key,valid_from_run)
);
CREATE INDEX fact_history ON canon.fact(entity_id,stat_key,valid_from_run DESC);
CREATE TABLE canon.relation(
    from_entity bigint NOT NULL REFERENCES canon.entity(id),
    kind canon.key NOT NULL,
    to_entity bigint NOT NULL REFERENCES canon.entity(id),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id),
    PRIMARY KEY(from_entity,kind,to_entity,receipt_id)
);
CREATE VIEW serve.published_receipt AS
SELECT r.*,o.source,o.revision,o.revision AS source_revision,o.path,o.content_sha256
FROM canon.receipt r JOIN ops.ingest_run run ON run.id=r.ingest_run_id AND run.status='succeeded'
JOIN raw.object o ON o.id=r.raw_object_id;
CREATE VIEW serve.current_entity AS
SELECT DISTINCT ON(e.id) e.id,e.kind,e.key,i.receipt_id,p.revision,p.source_revision,p.source,p.raw_object_id
FROM canon.entity e JOIN canon.entity_identity i ON i.entity_id=e.id
JOIN serve.published_receipt p ON p.id=i.receipt_id ORDER BY e.id,p.ingest_run_id DESC,p.id DESC;
CREATE VIEW serve.current_alias AS
SELECT DISTINCT ON(a.entity_id,a.alias_norm,a.lang) a.*,p.revision,p.source_revision,p.source,p.raw_object_id
FROM canon.alias a JOIN serve.published_receipt p ON p.id=a.receipt_id
ORDER BY a.entity_id,a.alias_norm,a.lang,p.ingest_run_id DESC,p.id DESC;
CREATE VIEW serve.current_fact AS
SELECT DISTINCT ON(f.entity_id,f.stat_key) f.*,p.revision,p.source_revision,p.source,p.raw_object_id
FROM canon.fact f JOIN serve.published_receipt p ON p.id=f.receipt_id
ORDER BY f.entity_id,f.stat_key,p.ingest_run_id DESC,p.id DESC;
CREATE VIEW serve.current_relation AS
SELECT DISTINCT ON(r.from_entity,r.kind,r.to_entity) r.*,p.revision,p.source_revision,p.source,p.raw_object_id
FROM canon.relation r JOIN serve.published_receipt p ON p.id=r.receipt_id
ORDER BY r.from_entity,r.kind,r.to_entity,p.ingest_run_id DESC,p.id DESC;
CREATE TABLE ops.answer_trace(
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    question_sha256 canon.sha256 NOT NULL,
    tool_calls jsonb NOT NULL,
    citation_ids jsonb NOT NULL,
    status text NOT NULL,
    tokens jsonb NOT NULL,
    latency_ms integer NOT NULL CHECK(latency_ms>=0),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp()
);
{{HERO_PROFILE}}
GRANT USAGE ON SCHEMA ops TO brain_service;
GRANT USAGE ON SEQUENCE ops.answer_trace_id_seq TO brain_service;
GRANT INSERT(question_sha256,tool_calls,citation_ids,status,tokens,latency_ms) ON ops.answer_trace TO brain_service;
GRANT USAGE ON SCHEMA raw,canon,ops TO brain_ingest;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA raw,canon,ops TO brain_ingest;
GRANT SELECT ON ALL TABLES IN SCHEMA raw,canon,ops TO brain_ingest;
GRANT INSERT(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) ON raw.object TO brain_ingest;
GRANT INSERT(source),UPDATE(finished,status,accepted,rejected,ignored) ON ops.ingest_run TO brain_ingest;
GRANT INSERT(run_id,raw_object_id,extractor,code,detail) ON ops.ingest_rejection TO brain_ingest;
GRANT INSERT(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) ON canon.receipt TO brain_ingest;
GRANT INSERT(kind,key,receipt_id) ON canon.entity TO brain_ingest;
GRANT INSERT(entity_id,receipt_id) ON canon.entity_identity TO brain_ingest;
GRANT INSERT(entity_id,alias_norm,lang,receipt_id) ON canon.alias TO brain_ingest;
GRANT INSERT(entity_id,stat_key,value_num,value_text,value_bool,valid_from_run,receipt_id) ON canon.fact TO brain_ingest;
GRANT INSERT(from_entity,kind,to_entity,receipt_id) ON canon.relation TO brain_ingest;
GRANT USAGE ON SCHEMA serve,canon TO brain_tool,brain_service;
GRANT SELECT ON ALL TABLES IN SCHEMA serve TO brain_tool,brain_service;
COMMIT;
