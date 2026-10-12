BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate'
        OR to_regclass('canon.patch') IS NULL
        OR to_regclass('canon.fact') IS NULL
        OR to_regprocedure('canon.guard_assertion()') IS NULL
        OR NOT EXISTS (SELECT 1 FROM ops.gamefacts_schema_version WHERE version >= 1) THEN
        RAISE EXCEPTION 'Patchmigration benötigt das vorhandene Spielwertschema und dessen Eigentümer';
    END IF;
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Patchmigration benötigt READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
END
$owner$;

CREATE TABLE canon.patch_origin (
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    external_id text NOT NULL CHECK (octet_length(external_id) BETWEEN 1 AND 2048),
    patch_id bigint NOT NULL UNIQUE REFERENCES canon.patch(id) ON DELETE RESTRICT,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    PRIMARY KEY (source_id, external_id)
);
CREATE INDEX patch_origin_receipt_idx ON canon.patch_origin(receipt_id);
CREATE TABLE canon.source_revision_patch (
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    source_revision text NOT NULL CHECK (length(source_revision) BETWEEN 1 AND 256),
    patch_id bigint NOT NULL REFERENCES canon.patch_origin(patch_id) ON DELETE RESTRICT,
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    patch_receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    PRIMARY KEY (source_id, source_revision)
);
CREATE INDEX source_revision_patch_patch_idx ON canon.source_revision_patch(patch_id);
CREATE INDEX source_revision_patch_receipt_idx ON canon.source_revision_patch(receipt_id);
CREATE INDEX source_revision_patch_official_receipt_idx ON canon.source_revision_patch(patch_receipt_id);
CREATE FUNCTION canon.patch_change_key(entity_id bigint, stat_key text, old_value jsonb, new_value jsonb,
    change_kind text, line_text text) RETURNS text
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp AS $key$
    SELECT encode(sha256(convert_to(jsonb_build_array(entity_id, stat_key, old_value, new_value, change_kind, line_text)::text, 'UTF8')), 'hex')
$key$;
CREATE TABLE canon.patch_change (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    patch_id bigint NOT NULL REFERENCES canon.patch_origin(patch_id) ON DELETE RESTRICT,
    entity_id bigint NOT NULL REFERENCES canon.entity(id) ON DELETE RESTRICT,
    stat_key text REFERENCES canon.stat(key) ON DELETE RESTRICT,
    change_key text GENERATED ALWAYS AS (canon.patch_change_key(entity_id, stat_key, "old", "new", change_kind, line_text)) STORED,
    "old" jsonb CHECK ("old" IS NULL OR canon.is_atomic_json("old")),
    "new" jsonb CHECK ("new" IS NULL OR canon.is_atomic_json("new")),
    change_kind text NOT NULL CHECK (octet_length(change_kind) BETWEEN 1 AND 64),
    line_text text NOT NULL CHECK (octet_length(line_text) BETWEEN 1 AND 512),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    UNIQUE (patch_id, entity_id, receipt_id, change_key)
);
CREATE INDEX patch_change_entity_patch_idx ON canon.patch_change(entity_id, patch_id DESC, id DESC);
CREATE INDEX patch_change_stat_idx ON canon.patch_change(stat_key) WHERE stat_key IS NOT NULL;
CREATE INDEX patch_change_receipt_idx ON canon.patch_change(receipt_id);
CREATE INDEX patch_change_patch_idx ON canon.patch_change(patch_id);
CREATE INDEX patch_released_p4_idx ON canon.patch(released_at DESC, id DESC);

CREATE FUNCTION ops.serialize_gamefacts_p4_write() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $serialize$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Patchänderungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    RETURN NULL;
END
$serialize$;
CREATE TRIGGER patch_p4_write_lock BEFORE INSERT OR UPDATE OR DELETE ON canon.patch FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_p4_write();
CREATE TRIGGER patch_origin_p4_write_lock BEFORE INSERT OR UPDATE OR DELETE ON canon.patch_origin FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_p4_write();
CREATE TRIGGER source_revision_patch_p4_write_lock BEFORE INSERT OR UPDATE OR DELETE ON canon.source_revision_patch FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_p4_write();
CREATE TRIGGER patch_change_p4_write_lock BEFORE INSERT OR UPDATE OR DELETE ON canon.patch_change FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_p4_write();

CREATE FUNCTION canon.patch_p4_anchor(p_receipt_id bigint)
RETURNS TABLE (source_id bigint, external_id text, raw_object_id bigint, revision_id bigint, label text, released_at timestamptz)
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $anchor$
DECLARE payload json; proof canon.receipt; object raw.object; field_count integer; distinct_count integer;
BEGIN
    SELECT * INTO STRICT proof FROM canon.receipt c WHERE c.id = p_receipt_id;
    SELECT * INTO STRICT object FROM raw.object WHERE id = proof.raw_object_id;
    IF NOT EXISTS (SELECT 1 FROM raw.source s WHERE s.id = proof.source_id
        AND s.kind = 'patchnotes' AND s.locator = 'postgres://patchnotes-feed/public.patchnotes')
        OR object.media_type <> 'application/json' OR proof.locator_kind <> 'json_pointer' OR proof.locator <> '/title' THEN
        RAISE EXCEPTION 'Patchherkunft benötigt einen Titelbeleg aus dem offiziellen Patchfeed';
    END IF;
    payload := convert_from(object.body, 'UTF8')::json;
    IF json_typeof(payload) IS DISTINCT FROM 'object' THEN
        RAISE EXCEPTION 'Patchrohobjekt benötigt die unveränderten fünf offiziellen Felder';
    END IF;
    SELECT count(*), count(DISTINCT key) INTO field_count, distinct_count FROM json_each(payload);
    IF field_count <> 5 OR distinct_count <> 5
        OR NOT payload::jsonb ?& ARRAY['id', 'title', 'url', 'posted_at', 'raw_content']
        OR json_typeof(payload->'id') IS DISTINCT FROM 'number'
        OR (payload->>'id') IS DISTINCT FROM object.external_id
        OR json_typeof(payload->'title') IS DISTINCT FROM 'string'
        OR json_typeof(payload->'url') IS DISTINCT FROM 'string'
        OR json_typeof(payload->'posted_at') IS DISTINCT FROM 'string'
        OR json_typeof(payload->'raw_content') NOT IN ('string', 'null')
        OR length(payload->>'title') NOT BETWEEN 1 AND 256
        OR coalesce(payload->>'url', '') = ''
        OR (payload->>'posted_at') !~ '(Z|[+-][0-9]{2}:[0-9]{2})$'
        OR NOT pg_input_is_valid(payload->>'posted_at', 'timestamp with time zone')
        OR proof.value_sha256 <> sha256(convert_to((payload->'title')::text, 'UTF8')) THEN
        RAISE EXCEPTION 'Patchrohobjekt und Titelbeleg stimmen nicht mit dem offiziellen Post überein';
    END IF;
    RETURN QUERY SELECT proof.source_id, object.external_id, object.id, proof.revision_id,
        payload->>'title', (payload->>'posted_at')::timestamptz;
END
$anchor$;

CREATE FUNCTION canon.guard_patch_p4_origin() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $guard$
DECLARE anchor record; previous_revision bigint;
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'Offizielle Patchherkünfte dürfen nicht gelöscht werden';
    END IF;
    SELECT * INTO STRICT anchor FROM canon.patch_p4_anchor(NEW.receipt_id);
    IF anchor.source_id <> NEW.source_id OR anchor.external_id <> NEW.external_id THEN
        RAISE EXCEPTION 'Patchherkunft und Beleg müssen denselben offiziellen Post nachweisen';
    END IF;
    IF TG_OP = 'UPDATE' THEN
        SELECT revision_id INTO previous_revision FROM canon.receipt WHERE id = OLD.receipt_id;
        IF ROW(NEW.source_id, NEW.external_id, NEW.patch_id) IS DISTINCT FROM ROW(OLD.source_id, OLD.external_id, OLD.patch_id)
            OR anchor.revision_id <= previous_revision
            OR NOT EXISTS (SELECT 1 FROM canon.source_head h WHERE h.source_id = NEW.source_id
                AND h.published_revision_id = anchor.revision_id) THEN
            RAISE EXCEPTION 'Patchkorrekturen benötigen einen neueren veröffentlichten Beleg desselben Posts';
        END IF;
    ELSIF NOT EXISTS (SELECT 1 FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
        WHERE v.id = anchor.revision_id AND r.status <> 'succeeded') THEN
        RAISE EXCEPTION 'Neue Patchherkünfte benötigen einen offenen Quellenlauf';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER patch_origin_p4_guard BEFORE INSERT OR UPDATE OR DELETE ON canon.patch_origin FOR EACH ROW EXECUTE FUNCTION canon.guard_patch_p4_origin();

CREATE FUNCTION canon.check_patch_p4_origin() RETURNS trigger
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $guard$
DECLARE patch bigint;
BEGIN
    IF TG_TABLE_NAME = 'patch' THEN patch := NEW.id; ELSE patch := NEW.patch_id; END IF;
    IF NOT EXISTS (SELECT 1 FROM canon.patch p JOIN canon.patch_origin origin ON origin.patch_id = p.id
        CROSS JOIN LATERAL canon.patch_p4_anchor(origin.receipt_id) anchor
        WHERE p.id = patch AND p.label = anchor.label AND p.released_at = anchor.released_at
            AND p.raw_object_id = anchor.raw_object_id AND origin.source_id = anchor.source_id
            AND origin.external_id = anchor.external_id) THEN
        RAISE EXCEPTION 'Patchtitel, Datum und Rohanker benötigen denselben offiziellen Herkunftsbeleg';
    END IF;
    RETURN NULL;
END
$guard$;
CREATE CONSTRAINT TRIGGER patch_p4_origin_required AFTER INSERT OR UPDATE ON canon.patch DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION canon.check_patch_p4_origin();
CREATE CONSTRAINT TRIGGER patch_origin_p4_anchor_required AFTER INSERT OR UPDATE ON canon.patch_origin DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION canon.check_patch_p4_origin();

CREATE FUNCTION canon.stage_patch_origin(p_receipt_id bigint) RETURNS bigint
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $stage$
DECLARE anchor record; patch bigint;
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Patchänderungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    SELECT * INTO STRICT anchor FROM canon.patch_p4_anchor(p_receipt_id);
    IF NOT EXISTS (SELECT 1 FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
        WHERE v.id = anchor.revision_id AND r.status <> 'succeeded') THEN
        RAISE EXCEPTION 'Patchvorbereitung benötigt einen offenen Quellenlauf';
    END IF;
    SELECT patch_id INTO patch FROM canon.patch_origin WHERE source_id = anchor.source_id AND external_id = anchor.external_id;
    IF patch IS NULL THEN
        INSERT INTO canon.patch(label, released_at, raw_object_id) VALUES (anchor.label, anchor.released_at, anchor.raw_object_id) RETURNING id INTO patch;
        INSERT INTO canon.patch_origin(source_id, external_id, patch_id, receipt_id) VALUES (anchor.source_id, anchor.external_id, patch, p_receipt_id);
    END IF;
    RETURN patch;
END
$stage$;
CREATE FUNCTION canon.publish_patch_origin(p_receipt_id bigint) RETURNS bigint
LANGUAGE plpgsql SECURITY DEFINER SET search_path = pg_catalog, pg_temp AS $publish$
DECLARE anchor record; origin canon.patch_origin;
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Patchveröffentlichungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    SELECT * INTO STRICT anchor FROM canon.patch_p4_anchor(p_receipt_id);
    SELECT * INTO STRICT origin FROM canon.patch_origin WHERE source_id = anchor.source_id AND external_id = anchor.external_id;
    IF NOT EXISTS (SELECT 1 FROM canon.source_head h JOIN canon.revision v ON v.id = h.published_revision_id
        JOIN raw.run r ON r.id = v.raw_run_id WHERE h.source_id = anchor.source_id
            AND h.published_revision_id = anchor.revision_id AND r.status = 'succeeded') THEN
        RAISE EXCEPTION 'Patchveröffentlichung benötigt den atomar veröffentlichten Quellenkopf';
    END IF;
    IF origin.receipt_id <> p_receipt_id THEN
        UPDATE canon.patch_origin SET receipt_id = p_receipt_id WHERE patch_id = origin.patch_id;
        UPDATE canon.patch SET label = anchor.label, released_at = anchor.released_at, raw_object_id = anchor.raw_object_id WHERE id = origin.patch_id;
    END IF;
    RETURN origin.patch_id;
END
$publish$;

CREATE FUNCTION canon.guard_source_revision_p4_patch() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $guard$
DECLARE payload json; official_payload json; origin canon.patch_origin; proof canon.receipt;
BEGIN
    IF TG_OP <> 'INSERT' THEN
        RAISE EXCEPTION 'Belegte Patchzuordnungen sind unveränderlich';
    END IF;
    SELECT * INTO STRICT proof FROM canon.receipt WHERE id = NEW.receipt_id;
    SELECT * INTO STRICT origin FROM canon.patch_origin WHERE patch_id = NEW.patch_id;
    IF proof.source_id <> NEW.source_id OR NOT EXISTS (SELECT 1 FROM canon.revision v
        WHERE v.id = proof.revision_id AND v.source_id = NEW.source_id AND v.source_revision = NEW.source_revision) THEN
        RAISE EXCEPTION 'Patchzuordnung benötigt einen Beleg derselben Quellenrevision';
    END IF;
    SELECT convert_from(o.body, 'UTF8')::json INTO payload FROM raw.object o WHERE o.id = proof.raw_object_id AND o.media_type = 'application/json';
    SELECT convert_from(o.body, 'UTF8')::json INTO official_payload FROM canon.receipt c JOIN raw.object o ON o.id = c.raw_object_id WHERE c.id = origin.receipt_id;
    IF proof.locator_kind <> 'json_pointer' OR proof.locator <> '/patch_url'
        OR json_typeof(payload->'patch_url') IS DISTINCT FROM 'string'
        OR (payload->>'patch_url') IS DISTINCT FROM (official_payload->>'url')
        OR proof.value_sha256 <> sha256(convert_to((payload->'patch_url')::text, 'UTF8')) THEN
        RAISE EXCEPTION 'Patchzuordnung benötigt einen ausdrücklichen Rohbeleg mit der offiziellen Patch-URL';
    END IF;
    NEW.patch_receipt_id := origin.receipt_id;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER source_revision_patch_p4_guard BEFORE INSERT OR UPDATE OR DELETE ON canon.source_revision_patch FOR EACH ROW EXECUTE FUNCTION canon.guard_source_revision_p4_patch();

CREATE FUNCTION canon.guard_patch_p4_change() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $guard$
DECLARE payload json; proof canon.receipt;
BEGIN
    IF TG_OP <> 'INSERT' THEN
        RAISE EXCEPTION 'Patchänderungen sind unveränderlich';
    END IF;
    SELECT * INTO STRICT proof FROM canon.receipt WHERE id = NEW.receipt_id;
    IF NOT EXISTS (SELECT 1 FROM canon.patch_origin origin JOIN raw.object o ON o.source_id = origin.source_id AND o.external_id = origin.external_id
        JOIN canon.revision v ON v.id = proof.revision_id JOIN raw.run r ON r.id = v.raw_run_id
        JOIN canon.receipt head ON head.id = origin.receipt_id
        WHERE origin.patch_id = NEW.patch_id AND o.id = proof.raw_object_id AND proof.source_id = origin.source_id
            AND proof.revision_id >= head.revision_id AND r.status <> 'succeeded') THEN
        RAISE EXCEPTION 'Patchänderung benötigt einen offenen Beleg desselben offiziellen Posts';
    END IF;
    SELECT convert_from(body, 'UTF8')::json INTO payload FROM raw.object WHERE id = proof.raw_object_id;
    IF proof.locator_kind <> 'json_pointer' OR proof.locator <> '/raw_content'
        OR json_typeof(payload->'raw_content') IS DISTINCT FROM 'string'
        OR proof.value_sha256 <> sha256(convert_to((payload->'raw_content')::text, 'UTF8')) THEN
        RAISE EXCEPTION 'Patchänderung benötigt den unveränderten Rohtextbeleg';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER patch_change_p4_guard BEFORE INSERT OR UPDATE OR DELETE ON canon.patch_change FOR EACH ROW EXECUTE FUNCTION canon.guard_patch_p4_change();

CREATE FUNCTION canon.stamp_fact_p4_patch_endpoints() RETURNS trigger
LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp AS $stamp$
BEGIN
    IF TG_OP = 'INSERT' THEN
        SELECT m.patch_id INTO NEW.valid_from_patch FROM canon.revision v
        JOIN canon.source_revision_patch m ON m.source_id = v.source_id AND m.source_revision = v.source_revision
        WHERE v.id = NEW.valid_from_revision_id AND v.source_id = NEW.source_id;
    ELSE
        NEW.valid_from_patch := OLD.valid_from_patch;
    END IF;
    NEW.valid_to_patch := NULL;
    IF NEW.valid_to_revision_id IS NOT NULL THEN
        SELECT m.patch_id INTO NEW.valid_to_patch FROM canon.revision v
        JOIN canon.source_revision_patch m ON m.source_id = v.source_id AND m.source_revision = v.source_revision
        WHERE v.id = NEW.valid_to_revision_id AND v.source_id = NEW.source_id;
    END IF;
    RETURN NEW;
END
$stamp$;
CREATE TRIGGER fact_p4_patch_endpoints BEFORE INSERT OR UPDATE OF valid_to_revision_id ON canon.fact
    FOR EACH ROW EXECUTE FUNCTION canon.stamp_fact_p4_patch_endpoints();

CREATE VIEW serve.published_patch_origin AS
SELECT origin.* FROM canon.patch_origin origin
JOIN canon.receipt head ON head.id = origin.receipt_id
JOIN canon.source_head published ON published.source_id = origin.source_id AND published.published_revision_id >= head.revision_id
JOIN LATERAL (SELECT object_id, action FROM canon.revision v
    JOIN raw.run_object observed ON observed.run_id = v.raw_run_id AND observed.path = origin.external_id
    WHERE v.source_id = origin.source_id AND v.id <= published.published_revision_id
    ORDER BY v.id DESC LIMIT 1) latest ON latest.action = 'upsert' AND latest.object_id = head.raw_object_id;
CREATE VIEW serve.current_patch AS
SELECT p.* FROM canon.patch p JOIN serve.published_patch_origin origin ON origin.patch_id = p.id
ORDER BY p.released_at DESC, p.id DESC LIMIT 1;
CREATE VIEW serve.published_patch_change AS
SELECT change.* FROM canon.patch_change change
WHERE (SELECT true FROM serve.published_patch_origin origin
    JOIN canon.receipt head ON head.id = origin.receipt_id
    JOIN canon.receipt proof ON proof.id = change.receipt_id AND proof.source_id = origin.source_id
        AND proof.revision_id = head.revision_id AND proof.raw_object_id = head.raw_object_id
    WHERE origin.patch_id = change.patch_id);
CREATE VIEW serve.published_source_revision_patch AS
SELECT m.*, c.revision_id AS evidence_revision_id FROM canon.source_revision_patch m
JOIN canon.receipt c ON c.id = m.receipt_id
JOIN canon.source_head h ON h.source_id = m.source_id AND h.published_revision_id >= c.revision_id
JOIN serve.published_patch_origin origin ON origin.patch_id = m.patch_id
JOIN canon.receipt official ON official.id = origin.receipt_id
JOIN canon.receipt mapping_official ON mapping_official.id = m.patch_receipt_id
    AND mapping_official.source_id = origin.source_id
JOIN canon.source_head official_head ON official_head.source_id = origin.source_id
    AND official_head.published_revision_id >= official.revision_id
    AND official_head.published_revision_id >= mapping_official.revision_id;
CREATE VIEW serve.fact_patch_evidence AS
SELECT f.*, opening.patch_id AS proven_from_patch, closing.patch_id AS proven_to_patch,
    opening.receipt_id AS from_patch_receipt_id, closing.receipt_id AS to_patch_receipt_id,
    v.source_revision, s.section, s.unit, s.value_type, s.display_order
FROM canon.fact f JOIN canon.revision v ON v.id = f.valid_from_revision_id
JOIN canon.source_head h ON h.source_id = f.source_id AND h.published_revision_id >= f.valid_from_revision_id
JOIN canon.stat s ON s.key = f.stat_key
JOIN serve.published_source_revision_patch opening ON opening.source_id = f.source_id AND opening.source_revision = v.source_revision
LEFT JOIN canon.revision ending ON ending.id = f.valid_to_revision_id
LEFT JOIN serve.published_source_revision_patch closing ON closing.source_id = f.source_id AND closing.source_revision = ending.source_revision
WHERE f.valid_to_revision_id IS NULL OR (closing.patch_id IS NOT NULL AND h.published_revision_id >= f.valid_to_revision_id);
CREATE VIEW serve.fact_patch_snapshot AS
SELECT f.*, snapshot.patch_id AS snapshot_patch_id, snapshot.receipt_id AS snapshot_receipt_id,
    snapshot.patch_receipt_id AS snapshot_official_receipt_id, snapshot.evidence_revision_id AS snapshot_revision_id
FROM serve.fact_patch_evidence f JOIN (
    SELECT DISTINCT ON (source_id, patch_id) * FROM serve.published_source_revision_patch
    ORDER BY source_id, patch_id, evidence_revision_id DESC
) snapshot ON snapshot.source_id = f.source_id
WHERE f.valid_from_revision_id <= snapshot.evidence_revision_id
    AND (f.valid_to_revision_id IS NULL OR f.valid_to_revision_id > snapshot.evidence_revision_id);

REVOKE ALL ON TABLE canon.patch_origin, canon.source_revision_patch, canon.patch_change,
    serve.current_patch, serve.published_patch_origin, serve.published_patch_change, serve.published_source_revision_patch, serve.fact_patch_evidence,
    serve.fact_patch_snapshot FROM PUBLIC, brain_ingest, brain_service, brain_tool;
REVOKE ALL ON SEQUENCE canon.patch_change_id_seq FROM PUBLIC, brain_ingest, brain_service, brain_tool;
REVOKE ALL ON FUNCTION ops.serialize_gamefacts_p4_write(), canon.patch_p4_anchor(bigint), canon.guard_patch_p4_origin(),
    canon.check_patch_p4_origin(), canon.stage_patch_origin(bigint), canon.publish_patch_origin(bigint),
    canon.guard_source_revision_p4_patch(), canon.guard_patch_p4_change(), canon.stamp_fact_p4_patch_endpoints(),
    canon.patch_change_key(bigint, text, jsonb, jsonb, text, text) FROM PUBLIC, brain_ingest, brain_service, brain_tool;
GRANT SELECT ON TABLE canon.patch_origin, canon.source_revision_patch, canon.patch_change TO brain_ingest;
GRANT SELECT ON TABLE serve.current_patch, serve.published_patch_origin, serve.published_patch_change,
    serve.published_source_revision_patch, serve.fact_patch_evidence, serve.fact_patch_snapshot TO brain_ingest, brain_tool;
GRANT INSERT (source_id, source_revision, patch_id, receipt_id) ON canon.source_revision_patch TO brain_ingest;
GRANT INSERT (patch_id, entity_id, stat_key, "old", "new", change_kind, line_text, receipt_id) ON canon.patch_change TO brain_ingest;
GRANT USAGE ON SEQUENCE canon.patch_change_id_seq TO brain_ingest;
GRANT EXECUTE ON FUNCTION canon.stage_patch_origin(bigint), canon.publish_patch_origin(bigint),
    canon.patch_change_key(bigint, text, jsonb, jsonb, text, text) TO brain_ingest;

COMMIT;
