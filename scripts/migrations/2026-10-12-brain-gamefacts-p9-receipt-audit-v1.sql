BEGIN;

CREATE TABLE ops.receipt_audit (
    job_run_id bigint NOT NULL REFERENCES ops.job_run(id) ON DELETE RESTRICT,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    outcome text NOT NULL CHECK (outcome IN ('verified', 'mismatch', 'unsupported', 'unreadable')),
    observed_value_sha256 bytea CHECK (octet_length(observed_value_sha256) = 32),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (job_run_id, receipt_id),
    CHECK (outcome <> 'verified' OR observed_value_sha256 IS NOT NULL)
);
CREATE INDEX receipt_audit_receipt_idx ON ops.receipt_audit(receipt_id, recorded_at DESC);

CREATE TABLE canon.receipt_quarantine (
    receipt_id bigint PRIMARY KEY REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    job_run_id bigint NOT NULL,
    quarantined_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (job_run_id, receipt_id) REFERENCES ops.receipt_audit(job_run_id, receipt_id) ON DELETE RESTRICT
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
    INSERT INTO canon.receipt_quarantine(receipt_id, job_run_id)
        SELECT receipt_id, job_run_id FROM audit_results WHERE outcome = 'mismatch'
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

REVOKE ALL ON ops.receipt_audit, canon.receipt_quarantine FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON FUNCTION ops.serialize_receipt_audit_write(), ops.quarantine_audited_receipts(),
    serve.receipt_citation_usable(text), serve.audited_fact_section(jsonb) FROM PUBLIC, brain_tool, brain_ingest, brain_service;
GRANT SELECT ON ops.receipt_audit, canon.receipt_quarantine TO brain_ingest;
GRANT INSERT (job_run_id, receipt_id, outcome, observed_value_sha256) ON ops.receipt_audit TO brain_ingest;
GRANT SELECT ON canon.receipt_quarantine, serve.audited_entity, serve.audited_hero_profile TO brain_tool;
GRANT EXECUTE ON FUNCTION serve.receipt_citation_usable(text), serve.audited_fact_section(jsonb) TO brain_tool, brain_ingest;

COMMIT;
