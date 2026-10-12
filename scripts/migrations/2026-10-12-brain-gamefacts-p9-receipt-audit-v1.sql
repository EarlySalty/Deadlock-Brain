BEGIN;

CREATE TABLE ops.receipt_audit_run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    started_at timestamptz NOT NULL,
    finished_at timestamptz NOT NULL,
    status text NOT NULL CHECK (status IN ('succeeded', 'failed')),
    sampled smallint NOT NULL CHECK (sampled BETWEEN 0 AND 200),
    verified_receipts smallint NOT NULL CHECK (verified_receipts BETWEEN 0 AND 200),
    rejected_receipts smallint NOT NULL CHECK (rejected_receipts BETWEEN 0 AND 200),
    rejected_objects smallint NOT NULL CHECK (rejected_objects BETWEEN 0 AND 200),
    error_code text CHECK (octet_length(error_code) BETWEEN 1 AND 128),
    CHECK (finished_at >= started_at),
    CHECK (verified_receipts + rejected_receipts <= sampled),
    CHECK (status <> 'succeeded' OR verified_receipts + rejected_receipts = sampled),
    CHECK (rejected_objects <= rejected_receipts)
);

CREATE TABLE ops.receipt_audit_rejection (
    audit_run_id bigint NOT NULL REFERENCES ops.receipt_audit_run(id) ON DELETE RESTRICT,
    raw_object_key text NOT NULL CHECK (octet_length(raw_object_key) BETWEEN 1 AND 2048),
    code text NOT NULL CHECK (octet_length(code) BETWEEN 1 AND 128),
    detail text NOT NULL CHECK (octet_length(detail) <= 512),
    affected_receipts smallint NOT NULL CHECK (affected_receipts BETWEEN 1 AND 200),
    PRIMARY KEY (audit_run_id, raw_object_key)
);

CREATE TABLE ops.receipt_audit (
    audit_run_id bigint NOT NULL REFERENCES ops.receipt_audit_run(id) ON DELETE RESTRICT,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    outcome text NOT NULL CHECK (outcome IN ('match', 'mismatch', 'locator_not_found', 'unsupported_locator_kind')),
    observed_value_sha256 bytea CHECK (octet_length(observed_value_sha256) = 32),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (audit_run_id, receipt_id),
    CHECK (outcome <> 'match' OR observed_value_sha256 IS NOT NULL)
);
CREATE INDEX receipt_audit_receipt_idx ON ops.receipt_audit(receipt_id, recorded_at DESC);

CREATE TABLE canon.receipt_quarantine (
    receipt_id bigint PRIMARY KEY REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    audit_run_id bigint NOT NULL,
    quarantined_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (audit_run_id, receipt_id) REFERENCES ops.receipt_audit(audit_run_id, receipt_id) ON DELETE RESTRICT
);

CREATE FUNCTION ops.serialize_receipt_audit_write() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $serialize$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Belegprüfungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    RETURN NULL;
END
$serialize$;
CREATE TRIGGER receipt_audit_write_lock BEFORE INSERT ON ops.receipt_audit
    FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_receipt_audit_write();

CREATE FUNCTION ops.quarantine_audited_receipts() RETURNS trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $quarantine$
BEGIN
    INSERT INTO canon.receipt_quarantine(receipt_id, audit_run_id)
        SELECT receipt_id, audit_run_id FROM audit_results WHERE outcome IN ('mismatch', 'locator_not_found')
        ON CONFLICT (receipt_id) DO NOTHING;
    IF FOUND THEN
        PERFORM serve.refresh_hero_profile();
    END IF;
    RETURN NULL;
END
$quarantine$;
CREATE TRIGGER receipt_audit_quarantine AFTER INSERT ON ops.receipt_audit
    REFERENCING NEW TABLE AS audit_results FOR EACH STATEMENT
    EXECUTE FUNCTION ops.quarantine_audited_receipts();

CREATE OR REPLACE VIEW serve.current_fact AS
SELECT f.*, s.section, s.unit, s.value_type, s.display_order, v.source_revision
FROM canon.fact f JOIN canon.stat s ON s.key = f.stat_key
JOIN canon.source_head h ON h.source_id = f.source_id
JOIN canon.revision v ON v.id = f.valid_from_revision_id
WHERE f.valid_from_revision_id <= h.published_revision_id
    AND (f.valid_to_revision_id IS NULL OR f.valid_to_revision_id > h.published_revision_id)
    AND NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = f.receipt_id);

CREATE OR REPLACE VIEW serve.current_alias AS
SELECT a.*, v.source_revision FROM canon.alias a JOIN canon.source_head h ON h.source_id = a.source_id
JOIN canon.revision v ON v.id = a.valid_from_revision_id
WHERE a.valid_from_revision_id <= h.published_revision_id
    AND (a.valid_to_revision_id IS NULL OR a.valid_to_revision_id > h.published_revision_id)
    AND NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = a.receipt_id);

CREATE OR REPLACE VIEW serve.current_relation AS
SELECT r.*, v.source_revision FROM canon.relation r JOIN canon.source_head h ON h.source_id = r.source_id
JOIN canon.revision v ON v.id = r.valid_from_revision_id
WHERE r.valid_from_revision_id <= h.published_revision_id
    AND (r.valid_to_revision_id IS NULL OR r.valid_to_revision_id > h.published_revision_id)
    AND NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = r.receipt_id);

ALTER TABLE canon.receipt ENABLE ROW LEVEL SECURITY;
CREATE POLICY receipt_tool_usable ON canon.receipt FOR SELECT TO brain_tool
    USING (NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = id));
CREATE POLICY receipt_ingest_read ON canon.receipt FOR SELECT TO brain_ingest USING (true);
CREATE POLICY receipt_ingest_insert ON canon.receipt FOR INSERT TO brain_ingest WITH CHECK (true);

CREATE VIEW serve.audited_entity AS
SELECT e.* FROM canon.entity e
WHERE NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = e.identity_receipt_id);

CREATE FUNCTION serve.receipt_citation_usable(citation text) RETURNS boolean
LANGUAGE sql STABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp AS $citation$
    SELECT EXISTS (SELECT 1 FROM canon.receipt r WHERE r.id = CASE
        WHEN citation ~ '^[1-9][0-9]{0,18}$' THEN CASE
            WHEN citation::numeric <= 9223372036854775807 THEN citation::bigint END END
        AND NOT EXISTS (SELECT 1 FROM canon.receipt_quarantine q WHERE q.receipt_id = r.id))
$citation$;

CREATE FUNCTION serve.audited_fact_section(section jsonb) RETURNS jsonb
LANGUAGE sql STABLE SET search_path = pg_catalog, pg_temp AS $section$
    SELECT jsonb_set(section, '{facts}', coalesce((
        SELECT jsonb_agg(fact ORDER BY ordinal)
        FROM jsonb_array_elements(section->'facts') WITH ORDINALITY AS facts(fact, ordinal)
        WHERE serve.receipt_citation_usable(fact->>'citation')
    ), '[]'::jsonb))
$section$;

CREATE VIEW serve.audited_hero_profile AS
SELECT h.entity_id, h.key,
    serve.audited_fact_section(h.identity) AS identity,
    serve.audited_fact_section(h.stats) AS stats,
    serve.audited_fact_section(h.weapon) AS weapon,
    jsonb_set(h.abilities, '{links}', coalesce((
        SELECT jsonb_agg(serve.audited_fact_section(link) ORDER BY ordinal)
        FROM jsonb_array_elements(h.abilities->'links') WITH ORDINALITY AS links(link, ordinal)
        WHERE serve.receipt_citation_usable(link->>'citation')
            AND serve.receipt_citation_usable(link->'identity'->>'citation')
    ), '[]'::jsonb)) AS abilities
FROM serve.hero_profile h
WHERE serve.receipt_citation_usable(h.identity->>'citation');

REVOKE ALL ON ops.receipt_audit_run, ops.receipt_audit_rejection, ops.receipt_audit, canon.receipt_quarantine FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON FUNCTION ops.serialize_receipt_audit_write(), ops.quarantine_audited_receipts(),
    serve.receipt_citation_usable(text), serve.audited_fact_section(jsonb) FROM PUBLIC, brain_tool, brain_ingest, brain_service;
GRANT SELECT ON ops.receipt_audit_run, ops.receipt_audit_rejection, ops.receipt_audit, canon.receipt_quarantine TO brain_ingest;
GRANT INSERT (started_at, finished_at, status, sampled, verified_receipts, rejected_receipts, rejected_objects, error_code) ON ops.receipt_audit_run TO brain_ingest;
GRANT USAGE, SELECT ON SEQUENCE ops.receipt_audit_run_id_seq TO brain_ingest;
GRANT INSERT (audit_run_id, raw_object_key, code, detail, affected_receipts) ON ops.receipt_audit_rejection TO brain_ingest;
GRANT INSERT (audit_run_id, receipt_id, outcome, observed_value_sha256) ON ops.receipt_audit TO brain_ingest;
GRANT SELECT ON canon.receipt_quarantine, serve.audited_entity, serve.audited_hero_profile TO brain_tool;
GRANT EXECUTE ON FUNCTION serve.receipt_citation_usable(text), serve.audited_fact_section(jsonb) TO brain_tool, brain_ingest;

COMMIT;
