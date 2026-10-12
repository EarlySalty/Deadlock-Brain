BEGIN;

DO $roles$
BEGIN
    IF current_user <> 'brain_migrate' OR NOT EXISTS (
        SELECT 1 FROM pg_roles WHERE rolname = 'brain_tool' AND NOT rolcanlogin
            AND NOT rolsuper AND NOT rolcreatedb AND NOT rolcreaterole
            AND NOT rolreplication AND NOT rolbypassrls
    ) THEN
        RAISE EXCEPTION 'Spielwertmigration benötigt den Migrationseigentümer und die vorbereitete Werkzeugrolle';
    END IF;
END
$roles$;

CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;
CREATE SCHEMA IF NOT EXISTS raw AUTHORIZATION brain_migrate;
CREATE SCHEMA IF NOT EXISTS canon AUTHORIZATION brain_migrate;
CREATE SCHEMA IF NOT EXISTS docs AUTHORIZATION brain_migrate;
CREATE SCHEMA IF NOT EXISTS serve AUTHORIZATION brain_migrate;
CREATE SCHEMA IF NOT EXISTS ops AUTHORIZATION brain_migrate;
REVOKE ALL ON SCHEMA raw, canon, docs, serve, ops FROM PUBLIC;

CREATE FUNCTION ops.is_numeric_counts(value jsonb) RETURNS boolean
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $counts$
    SELECT CASE WHEN jsonb_typeof(value) = 'object' AND octet_length(value::text) <= 2048 THEN
        (SELECT count(*) <= 24 AND coalesce(bool_and(k ~ '^[A-Za-z0-9_]{1,64}$'
            AND jsonb_typeof(v) = 'number' AND v::text ~ '^[0-9]{1,18}$'), true)
        FROM jsonb_each(value) e(k, v)) ELSE false END
$counts$;

CREATE TABLE ops.gamefacts_schema_version (
    version integer PRIMARY KEY CHECK (version > 0)
);
CREATE TABLE ops.job_run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    job text NOT NULL CHECK (length(job) BETWEEN 1 AND 128),
    extractor text NOT NULL CHECK (length(extractor) BETWEEN 1 AND 128),
    extractor_version text NOT NULL CHECK (length(extractor_version) BETWEEN 1 AND 128),
    started_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    finished_at timestamptz,
    status text NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'succeeded', 'partial', 'failed')),
    counts jsonb NOT NULL DEFAULT '{}' CHECK (ops.is_numeric_counts(counts)),
    error_code text CHECK (error_code ~ '^[A-Za-z0-9_.:-]{1,128}$'),
    CHECK (finished_at IS NULL OR finished_at >= started_at)
);
CREATE TABLE ops.job_lease (
    lease_key text PRIMARY KEY CHECK (length(lease_key) BETWEEN 1 AND 256),
    run_id bigint NOT NULL REFERENCES ops.job_run(id) ON DELETE RESTRICT,
    epoch bigint NOT NULL CHECK (epoch > 0),
    expires_at timestamptz NOT NULL
);
CREATE INDEX job_lease_run_idx ON ops.job_lease(run_id);
CREATE INDEX job_run_started_idx ON ops.job_run(job, started_at DESC);

CREATE TABLE raw.source (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    kind text NOT NULL CHECK (length(kind) BETWEEN 1 AND 64),
    locator text NOT NULL UNIQUE CHECK (octet_length(locator) BETWEEN 1 AND 2048),
    enabled boolean NOT NULL DEFAULT true,
    last_complete_revision text CHECK (length(last_complete_revision) BETWEEN 1 AND 256)
);
CREATE TABLE raw.object (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    external_id text NOT NULL CHECK (octet_length(external_id) BETWEEN 1 AND 2048),
    revision text NOT NULL CHECK (length(revision) BETWEEN 1 AND 256),
    fetched_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    content_sha256 bytea NOT NULL CHECK (octet_length(content_sha256) = 32),
    media_type text NOT NULL CHECK (length(media_type) BETWEEN 1 AND 128),
    body bytea NOT NULL,
    UNIQUE (source_id, external_id, content_sha256),
    UNIQUE (id, source_id)
);
CREATE TABLE raw.run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    job_run_id bigint NOT NULL REFERENCES ops.job_run(id) ON DELETE RESTRICT,
    base_revision text CHECK (length(base_revision) BETWEEN 1 AND 256),
    target_revision text NOT NULL CHECK (length(target_revision) BETWEEN 1 AND 256),
    extractor text NOT NULL CHECK (length(extractor) BETWEEN 1 AND 128),
    extractor_version text NOT NULL CHECK (length(extractor_version) BETWEEN 1 AND 128),
    status text NOT NULL DEFAULT 'running' CHECK (status IN ('running', 'partial', 'failed', 'succeeded')),
    last_completed_path text CHECK (length(last_completed_path) BETWEEN 1 AND 2048),
    UNIQUE (id, source_id, target_revision, extractor, extractor_version)
);
CREATE UNIQUE INDEX run_incomplete_source_idx ON raw.run(source_id) WHERE status <> 'succeeded';
CREATE INDEX run_source_idx ON raw.run(source_id, id);
CREATE INDEX run_job_idx ON raw.run(job_run_id);
CREATE TABLE raw.run_object (
    run_id bigint NOT NULL REFERENCES raw.run(id) ON DELETE RESTRICT,
    path text NOT NULL CHECK (length(path) BETWEEN 1 AND 2048),
    object_id bigint REFERENCES raw.object(id) ON DELETE RESTRICT,
    action text NOT NULL CHECK (action IN ('upsert', 'delete')),
    parse_status text NOT NULL DEFAULT 'pending' CHECK (parse_status IN ('pending', 'succeeded', 'skipped', 'failed')),
    PRIMARY KEY (run_id, path),
    CHECK (action = 'delete' OR object_id IS NOT NULL)
);
CREATE INDEX run_object_object_idx ON raw.run_object(object_id);

CREATE TABLE canon.revision (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    raw_run_id bigint NOT NULL UNIQUE,
    source_revision text NOT NULL CHECK (length(source_revision) BETWEEN 1 AND 256),
    extractor text NOT NULL,
    extractor_version text NOT NULL,
    observed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (id, source_id),
    UNIQUE (id, source_id, extractor, extractor_version),
    FOREIGN KEY (raw_run_id, source_id, source_revision, extractor, extractor_version)
        REFERENCES raw.run(id, source_id, target_revision, extractor, extractor_version) ON DELETE RESTRICT
);
CREATE INDEX revision_source_idx ON canon.revision(source_id, id);
CREATE TABLE canon.source_head (
    source_id bigint PRIMARY KEY REFERENCES raw.source(id) ON DELETE RESTRICT,
    published_revision_id bigint NOT NULL,
    FOREIGN KEY (published_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT
);
CREATE INDEX source_head_revision_idx ON canon.source_head(published_revision_id, source_id);
CREATE TABLE canon.patch (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    label text NOT NULL CHECK (length(label) BETWEEN 1 AND 256),
    released_at timestamptz NOT NULL,
    raw_object_id bigint NOT NULL REFERENCES raw.object(id) ON DELETE RESTRICT
);
CREATE INDEX patch_object_idx ON canon.patch(raw_object_id);
CREATE TABLE canon.receipt (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    raw_object_id bigint NOT NULL,
    source_id bigint NOT NULL,
    revision_id bigint NOT NULL,
    locator_kind text NOT NULL CHECK (length(locator_kind) BETWEEN 1 AND 64),
    locator text NOT NULL CHECK (octet_length(locator) BETWEEN 1 AND 2048),
    value_sha256 bytea NOT NULL CHECK (octet_length(value_sha256) = 32),
    extractor text NOT NULL,
    extractor_version text NOT NULL,
    UNIQUE (id, source_id, revision_id),
    FOREIGN KEY (raw_object_id, source_id) REFERENCES raw.object(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (revision_id, source_id, extractor, extractor_version)
        REFERENCES canon.revision(id, source_id, extractor, extractor_version) ON DELETE RESTRICT
);
CREATE INDEX receipt_object_idx ON canon.receipt(raw_object_id, source_id);
CREATE INDEX receipt_revision_idx ON canon.receipt(revision_id, source_id);
CREATE INDEX receipt_source_idx ON canon.receipt(source_id);
CREATE TABLE canon.entity (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    kind text NOT NULL CHECK (kind IN ('hero', 'ability', 'item', 'timer')),
    key text NOT NULL CHECK (length(key) BETWEEN 1 AND 128),
    status text NOT NULL DEFAULT 'unknown' CHECK (status = 'unknown'),
    identity_receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    UNIQUE (kind, key)
);
CREATE INDEX entity_receipt_idx ON canon.entity(identity_receipt_id);
CREATE TABLE canon.stat (
    key text PRIMARY KEY CHECK (length(key) BETWEEN 1 AND 128),
    section text NOT NULL CHECK (section IN ('identity', 'stats', 'weapon', 'ability')),
    unit text CHECK (length(unit) <= 64),
    value_type text NOT NULL CHECK (value_type IN ('number', 'text', 'json')),
    label_en text CHECK (length(label_en) <= 128),
    label_de text CHECK (length(label_de) <= 128),
    display_order integer NOT NULL DEFAULT 0
);

CREATE FUNCTION canon.is_atomic_json(value jsonb) RETURNS boolean
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $atomic$
    SELECT octet_length(value::text) <= 1024 AND CASE jsonb_typeof(value)
        WHEN 'object' THEN (SELECT count(*) <= 16 AND coalesce(bool_and(jsonb_typeof(v) NOT IN ('object', 'array')), true) FROM jsonb_each(value) e(k, v))
        WHEN 'array' THEN (SELECT count(*) <= 32 AND coalesce(bool_and(jsonb_typeof(v) NOT IN ('object', 'array')), true) FROM jsonb_array_elements(value) e(v))
        ELSE true END
$atomic$;

CREATE TABLE canon.fact (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    entity_id bigint NOT NULL REFERENCES canon.entity(id) ON DELETE RESTRICT,
    stat_key text NOT NULL REFERENCES canon.stat(key) ON DELETE RESTRICT,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    source_path text NOT NULL CHECK (octet_length(source_path) BETWEEN 1 AND 512),
    condition jsonb NOT NULL DEFAULT '{}' CHECK (jsonb_typeof(condition) = 'object' AND canon.is_atomic_json(condition)),
    value_num numeric CHECK (value_num::text NOT IN ('NaN', 'Infinity', '-Infinity') AND length(value_num::text) <= 64),
    value_text text CHECK (octet_length(value_text) <= 512),
    value_json jsonb CHECK (canon.is_atomic_json(value_json)),
    receipt_id bigint NOT NULL,
    valid_from_revision_id bigint NOT NULL,
    valid_to_revision_id bigint,
    valid_from_patch bigint REFERENCES canon.patch(id) ON DELETE RESTRICT,
    valid_to_patch bigint REFERENCES canon.patch(id) ON DELETE RESTRICT,
    CHECK (num_nonnulls(value_num, value_text, value_json) = 1),
    CHECK (valid_to_revision_id IS NULL OR valid_to_revision_id > valid_from_revision_id),
    FOREIGN KEY (receipt_id, source_id, valid_from_revision_id) REFERENCES canon.receipt(id, source_id, revision_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_from_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_to_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT
);
CREATE UNIQUE INDEX fact_active_idx ON canon.fact(source_id, source_path, entity_id, stat_key, condition) WHERE valid_to_revision_id IS NULL;
CREATE INDEX fact_entity_stat_idx ON canon.fact(entity_id, stat_key, source_id, valid_from_revision_id);
CREATE INDEX fact_source_history_idx ON canon.fact(source_id, source_path, valid_from_revision_id) INCLUDE (valid_to_revision_id, receipt_id);
CREATE INDEX fact_stat_idx ON canon.fact(stat_key);
CREATE INDEX fact_receipt_idx ON canon.fact(receipt_id, source_id, valid_from_revision_id);
CREATE INDEX fact_from_idx ON canon.fact(valid_from_revision_id, source_id);
CREATE INDEX fact_to_idx ON canon.fact(valid_to_revision_id, source_id);
CREATE INDEX fact_patch_from_idx ON canon.fact(valid_from_patch);
CREATE INDEX fact_patch_to_idx ON canon.fact(valid_to_patch);
CREATE TABLE canon.alias (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    entity_id bigint NOT NULL REFERENCES canon.entity(id) ON DELETE RESTRICT,
    alias_norm text NOT NULL CHECK (octet_length(alias_norm) BETWEEN 1 AND 256),
    lang text NOT NULL CHECK (length(lang) BETWEEN 1 AND 16),
    receipt_id bigint NOT NULL,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    source_path text NOT NULL CHECK (octet_length(source_path) BETWEEN 1 AND 512),
    valid_from_revision_id bigint NOT NULL,
    valid_to_revision_id bigint,
    CHECK (valid_to_revision_id IS NULL OR valid_to_revision_id > valid_from_revision_id),
    FOREIGN KEY (receipt_id, source_id, valid_from_revision_id) REFERENCES canon.receipt(id, source_id, revision_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_from_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_to_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT
);
CREATE UNIQUE INDEX alias_active_idx ON canon.alias(source_id, source_path, entity_id, alias_norm, lang) WHERE valid_to_revision_id IS NULL;
CREATE INDEX alias_exact_idx ON canon.alias(alias_norm, lang);
CREATE INDEX alias_trigram_idx ON canon.alias USING gin(alias_norm public.gin_trgm_ops);
CREATE INDEX alias_entity_idx ON canon.alias(entity_id);
CREATE INDEX alias_source_history_idx ON canon.alias(source_id, source_path, valid_from_revision_id) INCLUDE (valid_to_revision_id, receipt_id);
CREATE INDEX alias_receipt_idx ON canon.alias(receipt_id, source_id, valid_from_revision_id);
CREATE INDEX alias_from_idx ON canon.alias(valid_from_revision_id, source_id);
CREATE INDEX alias_to_idx ON canon.alias(valid_to_revision_id, source_id);
CREATE TABLE canon.relation (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    from_entity_id bigint NOT NULL REFERENCES canon.entity(id) ON DELETE RESTRICT,
    kind text NOT NULL CHECK (kind = 'has_ability'),
    to_entity_id bigint NOT NULL REFERENCES canon.entity(id) ON DELETE RESTRICT,
    slot smallint NOT NULL CHECK (slot BETWEEN 1 AND 4),
    receipt_id bigint NOT NULL,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    source_path text NOT NULL CHECK (octet_length(source_path) BETWEEN 1 AND 512),
    valid_from_revision_id bigint NOT NULL,
    valid_to_revision_id bigint,
    CHECK (valid_to_revision_id IS NULL OR valid_to_revision_id > valid_from_revision_id),
    FOREIGN KEY (receipt_id, source_id, valid_from_revision_id) REFERENCES canon.receipt(id, source_id, revision_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_from_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_to_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT
);
CREATE UNIQUE INDEX relation_active_slot_idx ON canon.relation(source_id, from_entity_id, kind, slot) WHERE valid_to_revision_id IS NULL;
CREATE INDEX relation_to_entity_idx ON canon.relation(to_entity_id);
CREATE INDEX relation_source_history_idx ON canon.relation(source_id, source_path, valid_from_revision_id) INCLUDE (valid_to_revision_id, receipt_id);
CREATE INDEX relation_from_entity_idx ON canon.relation(from_entity_id, kind, slot, source_id, id) INCLUDE (to_entity_id, valid_from_revision_id, valid_to_revision_id, receipt_id);
CREATE INDEX relation_receipt_idx ON canon.relation(receipt_id, source_id, valid_from_revision_id);
CREATE INDEX relation_from_idx ON canon.relation(valid_from_revision_id, source_id);
CREATE INDEX relation_to_idx ON canon.relation(valid_to_revision_id, source_id);

CREATE FUNCTION ops.serialize_gamefacts_write() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $serialize$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Spielwertänderungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    RETURN NULL;
END
$serialize$;
CREATE TRIGGER source_write_lock BEFORE INSERT OR UPDATE ON raw.source FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER object_write_lock BEFORE INSERT ON raw.object FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER run_write_lock BEFORE INSERT OR UPDATE ON raw.run FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER run_object_write_lock BEFORE INSERT OR UPDATE ON raw.run_object FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER revision_write_lock BEFORE INSERT ON canon.revision FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER head_write_lock BEFORE INSERT OR UPDATE ON canon.source_head FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER receipt_write_lock BEFORE INSERT ON canon.receipt FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER entity_write_lock BEFORE INSERT ON canon.entity FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER stat_write_lock BEFORE INSERT ON canon.stat FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER fact_write_lock BEFORE INSERT OR UPDATE ON canon.fact FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER alias_write_lock BEFORE INSERT OR UPDATE ON canon.alias FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER relation_write_lock BEFORE INSERT OR UPDATE ON canon.relation FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER job_run_write_lock BEFORE INSERT OR UPDATE ON ops.job_run FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();
CREATE TRIGGER lease_write_lock BEFORE INSERT OR UPDATE ON ops.job_lease FOR EACH STATEMENT EXECUTE FUNCTION ops.serialize_gamefacts_write();

CREATE FUNCTION canon.guard_revision() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
BEGIN
    PERFORM 1 FROM raw.source WHERE id = NEW.source_id FOR NO KEY UPDATE;
    IF NEW.id <= 0 OR EXISTS (SELECT 1 FROM canon.revision WHERE source_id = NEW.source_id AND id >= NEW.id) THEN
        RAISE EXCEPTION 'Quellenrevisionen müssen streng aufsteigend angelegt werden';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER revision_guard BEFORE INSERT ON canon.revision FOR EACH ROW EXECUTE FUNCTION canon.guard_revision();

CREATE FUNCTION raw.guard_run() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
DECLARE checkpoint text;
BEGIN
    IF TG_OP = 'INSERT' THEN
        SELECT last_complete_revision INTO checkpoint FROM raw.source WHERE id = NEW.source_id AND enabled FOR NO KEY UPDATE;
        IF NOT FOUND OR checkpoint IS DISTINCT FROM NEW.base_revision OR NEW.status <> 'running' THEN
            RAISE EXCEPTION 'Quellenlauf benötigt den veröffentlichten Ausgangsstand';
        END IF;
    ELSE
        IF OLD.status = 'succeeded' OR ROW(NEW.source_id, NEW.base_revision, NEW.target_revision, NEW.extractor, NEW.extractor_version)
            IS DISTINCT FROM ROW(OLD.source_id, OLD.base_revision, OLD.target_revision, OLD.extractor, OLD.extractor_version) THEN
            RAISE EXCEPTION 'Der Zielstand eines Quellenlaufs ist unveränderlich';
        END IF;
        IF NEW.status = 'succeeded' AND EXISTS (SELECT 1 FROM raw.run_object WHERE run_id = NEW.id AND parse_status NOT IN ('succeeded', 'skipped')) THEN
            RAISE EXCEPTION 'Ein unvollständiger Quellenlauf darf nicht veröffentlicht werden';
        END IF;
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER run_guard BEFORE INSERT OR UPDATE ON raw.run FOR EACH ROW EXECUTE FUNCTION raw.guard_run();
CREATE FUNCTION raw.guard_run_object() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
DECLARE source bigint; run_status text;
BEGIN
    SELECT source_id, status INTO source, run_status FROM raw.run WHERE id = NEW.run_id FOR UPDATE;
    IF run_status = 'succeeded' OR (NEW.object_id IS NOT NULL AND NOT EXISTS (
        SELECT 1 FROM raw.object WHERE id = NEW.object_id AND source_id = source AND external_id = NEW.path
    )) THEN
        RAISE EXCEPTION 'Rohobjekt gehört nicht zum offenen Quellenlauf';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER run_object_guard BEFORE INSERT OR UPDATE ON raw.run_object FOR EACH ROW EXECUTE FUNCTION raw.guard_run_object();

CREATE FUNCTION canon.guard_receipt() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
        JOIN raw.run_object ro ON ro.run_id = r.id AND ro.object_id = NEW.raw_object_id
        WHERE v.id = NEW.revision_id AND r.status <> 'succeeded' AND ro.action = 'upsert' FOR UPDATE OF r
    ) THEN
        RAISE EXCEPTION 'Belege benötigen ein Rohobjekt im offenen Quellenlauf';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER receipt_guard BEFORE INSERT ON canon.receipt FOR EACH ROW EXECUTE FUNCTION canon.guard_receipt();

CREATE FUNCTION canon.guard_assertion() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
DECLARE revision bigint; run_status text; field_type text;
BEGIN
    IF TG_OP = 'UPDATE' THEN
        IF OLD.valid_to_revision_id IS NOT NULL OR NEW.valid_to_revision_id IS NULL THEN
            RAISE EXCEPTION 'Eine Aussage darf nur einmal geschlossen werden';
        END IF;
        revision := NEW.valid_to_revision_id;
    ELSE
        IF NEW.valid_to_revision_id IS NOT NULL THEN
            RAISE EXCEPTION 'Neue Aussagen müssen offen beginnen';
        END IF;
        revision := NEW.valid_from_revision_id;
        IF NOT EXISTS (SELECT 1 FROM canon.receipt p JOIN raw.object o ON o.id = p.raw_object_id
            WHERE p.id = NEW.receipt_id AND o.external_id = NEW.source_path) THEN
            RAISE EXCEPTION 'Aussage und Beleg müssen denselben Quellenpfad besitzen';
        END IF;
    END IF;
    SELECT r.status INTO run_status FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
        WHERE v.id = revision AND v.source_id = NEW.source_id FOR UPDATE OF r;
    IF run_status IS NULL OR run_status = 'succeeded' OR NOT EXISTS (
        SELECT 1 FROM canon.revision v JOIN raw.run_object ro ON ro.run_id = v.raw_run_id
            WHERE v.id = revision AND ro.path = NEW.source_path
    ) THEN
        RAISE EXCEPTION 'Aussagen benötigen einen geplanten Pfad im offenen Quellenlauf';
    END IF;
    IF TG_TABLE_NAME = 'fact' THEN
        SELECT value_type INTO field_type FROM canon.stat WHERE key = NEW.stat_key;
        IF (field_type = 'number' AND NEW.value_num IS NULL) OR (field_type = 'text' AND NEW.value_text IS NULL)
            OR (field_type = 'json' AND NEW.value_json IS NULL) THEN
            RAISE EXCEPTION 'Der Wert passt nicht zum Feldtyp';
        END IF;
    ELSIF TG_TABLE_NAME = 'relation' THEN
        IF NOT EXISTS (
            SELECT 1 FROM canon.entity h, canon.entity a WHERE h.id = NEW.from_entity_id AND h.kind = 'hero'
                AND a.id = NEW.to_entity_id AND a.kind = 'ability'
        ) THEN
            RAISE EXCEPTION 'Fähigkeitsverknüpfungen benötigen Held und Fähigkeit';
        END IF;
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER fact_guard BEFORE INSERT OR UPDATE ON canon.fact FOR EACH ROW EXECUTE FUNCTION canon.guard_assertion();
CREATE TRIGGER alias_guard BEFORE INSERT OR UPDATE ON canon.alias FOR EACH ROW EXECUTE FUNCTION canon.guard_assertion();
CREATE TRIGGER relation_guard BEFORE INSERT OR UPDATE ON canon.relation FOR EACH ROW EXECUTE FUNCTION canon.guard_assertion();

CREATE FUNCTION canon.guard_publication() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $guard$
DECLARE run_status text; base text; checkpoint text;
BEGIN
    SELECT last_complete_revision INTO checkpoint FROM raw.source WHERE id = NEW.source_id FOR NO KEY UPDATE;
    SELECT r.status, r.base_revision INTO run_status, base FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
        WHERE v.id = NEW.published_revision_id AND v.source_id = NEW.source_id FOR UPDATE OF r;
    IF run_status IS DISTINCT FROM 'succeeded' OR (TG_OP = 'UPDATE' AND NEW.published_revision_id <= OLD.published_revision_id)
        OR NOT EXISTS (SELECT 1 FROM canon.revision v WHERE v.id = NEW.published_revision_id
            AND (checkpoint IS NOT DISTINCT FROM base OR checkpoint = v.source_revision)) THEN
        RAISE EXCEPTION 'Nur ein abgeschlossener Folgestand darf veröffentlicht werden';
    END IF;
    RETURN NEW;
END
$guard$;
CREATE TRIGGER source_head_guard BEFORE INSERT OR UPDATE ON canon.source_head FOR EACH ROW EXECUTE FUNCTION canon.guard_publication();
CREATE FUNCTION canon.check_checkpoint() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $checkpoint$
DECLARE source bigint;
BEGIN
    IF TG_TABLE_SCHEMA = 'raw' THEN
        source := NEW.id;
    ELSE
        source := NEW.source_id;
    END IF;
    IF EXISTS (
        SELECT 1 FROM raw.source s LEFT JOIN canon.source_head h ON h.source_id = s.id
        LEFT JOIN canon.revision v ON v.id = h.published_revision_id
        WHERE s.id = source AND s.last_complete_revision IS DISTINCT FROM v.source_revision
    ) THEN
        RAISE EXCEPTION 'Quellenkopf und vollständiger Kontrollpunkt müssen gemeinsam veröffentlicht werden';
    END IF;
    IF EXISTS (SELECT 1 FROM raw.run r WHERE r.source_id = source AND r.status = 'succeeded'
        AND NOT EXISTS (SELECT 1 FROM canon.revision v JOIN canon.source_head h ON h.source_id = v.source_id
            WHERE v.raw_run_id = r.id AND h.published_revision_id >= v.id)) THEN
        RAISE EXCEPTION 'Abgeschlossene Quellenläufe müssen veröffentlicht sein';
    END IF;
    RETURN NULL;
END
$checkpoint$;
CREATE CONSTRAINT TRIGGER source_checkpoint AFTER INSERT OR UPDATE ON raw.source DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION canon.check_checkpoint();
CREATE CONSTRAINT TRIGGER head_checkpoint AFTER INSERT OR UPDATE ON canon.source_head DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION canon.check_checkpoint();
CREATE FUNCTION raw.check_completed_run() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $completed$
BEGIN
    IF NEW.status = 'succeeded' AND NOT EXISTS (
        SELECT 1 FROM canon.revision v JOIN canon.source_head h ON h.source_id = v.source_id
        JOIN raw.source s ON s.id = v.source_id WHERE v.raw_run_id = NEW.id
            AND h.published_revision_id = v.id AND s.last_complete_revision = v.source_revision
    ) THEN
        RAISE EXCEPTION 'Laufabschluss benötigt die atomare Veröffentlichung';
    END IF;
    RETURN NULL;
END
$completed$;
CREATE CONSTRAINT TRIGGER completed_run AFTER UPDATE ON raw.run DEFERRABLE INITIALLY DEFERRED
    FOR EACH ROW EXECUTE FUNCTION raw.check_completed_run();

CREATE VIEW serve.current_fact AS
SELECT f.*, s.section, s.unit, s.value_type, s.display_order, v.source_revision
FROM canon.fact f JOIN canon.stat s ON s.key = f.stat_key
JOIN canon.source_head h ON h.source_id = f.source_id
JOIN canon.revision v ON v.id = f.valid_from_revision_id
WHERE f.valid_from_revision_id <= h.published_revision_id
    AND (f.valid_to_revision_id IS NULL OR f.valid_to_revision_id > h.published_revision_id);
CREATE VIEW serve.current_alias AS
SELECT a.*, v.source_revision FROM canon.alias a JOIN canon.source_head h ON h.source_id = a.source_id
JOIN canon.revision v ON v.id = a.valid_from_revision_id
WHERE a.valid_from_revision_id <= h.published_revision_id
    AND (a.valid_to_revision_id IS NULL OR a.valid_to_revision_id > h.published_revision_id);
CREATE VIEW serve.current_relation AS
SELECT r.*, v.source_revision FROM canon.relation r JOIN canon.source_head h ON h.source_id = r.source_id
JOIN canon.revision v ON v.id = r.valid_from_revision_id
WHERE r.valid_from_revision_id <= h.published_revision_id
    AND (r.valid_to_revision_id IS NULL OR r.valid_to_revision_id > h.published_revision_id);

CREATE FUNCTION serve.atomic_fact(entity_id bigint, stat_key text, value_num numeric, value_text text, value_json jsonb,
    unit text, condition jsonb, receipt_id bigint, source_revision text) RETURNS jsonb
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $fact$
    SELECT jsonb_build_object('entity_id', entity_id, 'stat_key', stat_key,
        'value', coalesce(to_jsonb(value_num), to_jsonb(value_text), value_json), 'unit', unit,
        'condition', condition, 'citation', receipt_id::text, 'patch', NULL, 'source_revision', source_revision)
$fact$;

CREATE MATERIALIZED VIEW serve.hero_profile AS
SELECT e.id AS entity_id, e.key, jsonb_build_object(
    'entity_id', e.id, 'kind', e.kind, 'key', e.key, 'status', 'unknown',
    'citation', proof.id::text, 'patch', NULL, 'source_revision', identity_revision.source_revision,
    'facts', identity_facts.rows, 'truncated', identity_facts.omitted_count > 0, 'omitted_count', identity_facts.omitted_count
) AS identity,
jsonb_build_object('facts', stats.rows, 'truncated', stats.omitted_count > 0, 'omitted_count', stats.omitted_count) AS stats,
jsonb_build_object('facts', weapon.rows, 'truncated', weapon.omitted_count > 0, 'omitted_count', weapon.omitted_count) AS weapon,
jsonb_build_object('links', abilities.rows, 'truncated', abilities.omitted_count > 0, 'omitted_count', abilities.omitted_count) AS abilities
FROM canon.entity e JOIN canon.receipt proof ON proof.id = e.identity_receipt_id
JOIN canon.revision identity_revision ON identity_revision.id = proof.revision_id
JOIN canon.source_head identity_head ON identity_head.source_id = proof.source_id AND identity_head.published_revision_id >= proof.revision_id
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(serve.atomic_fact(f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json, f.unit, f.condition, f.receipt_id, f.source_revision)
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id), '[]') AS rows,
        coalesce(max(f.total), 0) - count(*) AS omitted_count
    FROM (SELECT f.*, count(*) OVER () AS total FROM serve.current_fact f WHERE f.entity_id = e.id AND f.section = 'identity'
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id LIMIT 4) f
) identity_facts
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(serve.atomic_fact(f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json, f.unit, f.condition, f.receipt_id, f.source_revision)
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id), '[]') AS rows,
        coalesce(max(f.total), 0) - count(*) AS omitted_count
    FROM (SELECT f.*, count(*) OVER () AS total FROM serve.current_fact f WHERE f.entity_id = e.id AND f.section = 'stats'
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id LIMIT 20) f
) stats
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(serve.atomic_fact(f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json, f.unit, f.condition, f.receipt_id, f.source_revision)
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id), '[]') AS rows,
        coalesce(max(f.total), 0) - count(*) AS omitted_count
    FROM (SELECT f.*, count(*) OVER () AS total FROM serve.current_fact f WHERE f.entity_id = e.id AND f.section = 'weapon'
        ORDER BY f.display_order, f.stat_key, f.source_id, f.id LIMIT 12) f
) weapon
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(jsonb_build_object('entity_id', r.to_entity_id, 'key', r.ability_key, 'slot', r.slot,
        'citation', r.receipt_id::text, 'patch', NULL, 'source_revision', r.source_revision,
        'identity', jsonb_build_object('citation', r.identity_receipt_id::text, 'patch', NULL, 'source_revision', r.identity_source_revision),
        'facts', facts.rows, 'truncated', facts.omitted_count > 0, 'omitted_count', facts.omitted_count)
        ORDER BY r.slot, r.source_id, r.id), '[]') AS rows,
        coalesce(max(r.total), 0) - count(*) AS omitted_count
    FROM (SELECT r.*, ability.key AS ability_key, ability.identity_receipt_id, av.source_revision AS identity_source_revision,
            count(*) OVER () AS total
        FROM serve.current_relation r
        JOIN canon.entity ability ON ability.id = r.to_entity_id
        JOIN canon.receipt ap ON ap.id = ability.identity_receipt_id
        JOIN canon.revision av ON av.id = ap.revision_id
        JOIN canon.source_head ah ON ah.source_id = ap.source_id AND ah.published_revision_id >= ap.revision_id
        WHERE r.from_entity_id = e.id AND r.kind = 'has_ability'
        ORDER BY r.slot, r.source_id, r.id LIMIT 4) r
    CROSS JOIN LATERAL (
        SELECT coalesce(jsonb_agg(serve.atomic_fact(f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json, f.unit, f.condition, f.receipt_id, f.source_revision)
            ORDER BY f.display_order, f.stat_key, f.source_id, f.id), '[]') AS rows,
            coalesce(max(f.total), 0) - count(*) AS omitted_count
        FROM (SELECT f.*, count(*) OVER () AS total FROM serve.current_fact f WHERE f.entity_id = r.to_entity_id
            ORDER BY f.display_order, f.stat_key, f.source_id, f.id LIMIT 8) f
    ) facts
) abilities
WHERE e.kind = 'hero';
CREATE UNIQUE INDEX hero_profile_entity_idx ON serve.hero_profile(entity_id);

CREATE FUNCTION serve.refresh_hero_profile() RETURNS void LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog, pg_temp AS $refresh$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Spielwertveröffentlichungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    REFRESH MATERIALIZED VIEW CONCURRENTLY serve.hero_profile;
END
$refresh$;
CREATE FUNCTION canon.refresh_on_publish() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $refresh$
BEGIN
    PERFORM serve.refresh_hero_profile();
    RETURN NULL;
END
$refresh$;
CREATE TRIGGER head_refresh AFTER INSERT OR UPDATE ON canon.source_head FOR EACH ROW EXECUTE FUNCTION canon.refresh_on_publish();

CREATE FUNCTION ops.is_trace_list(value jsonb, citations boolean) RETURNS boolean
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $trace$
    SELECT CASE WHEN jsonb_typeof(value) = 'array' AND octet_length(value::text) <= 2400 THEN
        (SELECT count(*) <= CASE WHEN citations THEN 100 ELSE 8 END
            AND coalesce(bool_and(jsonb_typeof(v) = 'string' AND (v #>> '{}') ~
                CASE WHEN citations THEN '^[1-9][0-9]{0,18}$' ELSE '^[a-z][a-z0-9_]{0,63}$' END), true)
        FROM jsonb_array_elements(value) e(v)) ELSE false END
$trace$;
CREATE FUNCTION ops.guard_lease() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $lease$
BEGIN
    IF NEW.epoch < OLD.epoch OR (NEW.run_id <> OLD.run_id AND NEW.epoch <= OLD.epoch) THEN
        RAISE EXCEPTION 'Ein neuer Lease-Eigentümer benötigt einen höheren Zaunwert';
    END IF;
    RETURN NEW;
END
$lease$;
CREATE TRIGGER lease_guard BEFORE UPDATE ON ops.job_lease FOR EACH ROW EXECUTE FUNCTION ops.guard_lease();

CREATE TABLE ops.answer_trace (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    question_sha256 bytea NOT NULL CHECK (octet_length(question_sha256) = 32),
    tool_calls jsonb NOT NULL CHECK (ops.is_trace_list(tool_calls, false)),
    citation_ids jsonb NOT NULL CHECK (ops.is_trace_list(citation_ids, true)),
    status text NOT NULL CHECK (status IN ('answered', 'insufficient_evidence', 'out_of_domain', 'failed')),
    tokens jsonb NOT NULL CHECK (ops.is_numeric_counts(tokens)),
    latency_ms integer NOT NULL CHECK (latency_ms >= 0),
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX answer_trace_recorded_idx ON ops.answer_trace(recorded_at DESC);

REVOKE ALL ON ALL TABLES IN SCHEMA raw, canon, docs, serve, ops FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON ALL SEQUENCES IN SCHEMA raw, canon, docs, serve, ops FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON ALL FUNCTIONS IN SCHEMA raw, canon, docs, serve, ops FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON SCHEMA raw, canon, docs, serve, ops FROM brain_tool, brain_ingest, brain_service;
GRANT USAGE ON SCHEMA canon, docs, serve TO brain_tool;
GRANT SELECT ON ALL TABLES IN SCHEMA canon, docs, serve TO brain_tool;
GRANT USAGE ON SCHEMA raw, canon, serve, ops TO brain_ingest;
GRANT SELECT ON ALL TABLES IN SCHEMA raw, canon, serve, ops TO brain_ingest;
GRANT INSERT (kind, locator, enabled, last_complete_revision) ON raw.source TO brain_ingest;
GRANT INSERT (source_id, external_id, revision, fetched_at, content_sha256, media_type, body) ON raw.object TO brain_ingest;
GRANT INSERT (source_id, job_run_id, base_revision, target_revision, extractor, extractor_version, status, last_completed_path) ON raw.run TO brain_ingest;
GRANT INSERT (source_id, raw_run_id, source_revision, extractor, extractor_version, observed_at) ON canon.revision TO brain_ingest;
GRANT INSERT (raw_object_id, source_id, revision_id, locator_kind, locator, value_sha256, extractor, extractor_version) ON canon.receipt TO brain_ingest;
GRANT INSERT (kind, key, status, identity_receipt_id) ON canon.entity TO brain_ingest;
GRANT INSERT (entity_id, stat_key, source_id, source_path, condition, value_num, value_text, value_json, receipt_id,
    valid_from_revision_id, valid_to_revision_id, valid_from_patch, valid_to_patch) ON canon.fact TO brain_ingest;
GRANT INSERT (entity_id, alias_norm, lang, receipt_id, source_id, source_path, valid_from_revision_id, valid_to_revision_id) ON canon.alias TO brain_ingest;
GRANT INSERT (from_entity_id, kind, to_entity_id, slot, receipt_id, source_id, source_path, valid_from_revision_id, valid_to_revision_id) ON canon.relation TO brain_ingest;
GRANT INSERT (job, extractor, extractor_version, started_at, finished_at, status, counts, error_code) ON ops.job_run TO brain_ingest;
GRANT INSERT ON raw.run_object, canon.source_head, canon.stat, ops.job_lease TO brain_ingest;
GRANT UPDATE (enabled, last_complete_revision) ON raw.source TO brain_ingest;
GRANT UPDATE (job_run_id, status, last_completed_path) ON raw.run TO brain_ingest;
GRANT UPDATE (parse_status) ON raw.run_object TO brain_ingest;
GRANT UPDATE (published_revision_id) ON canon.source_head TO brain_ingest;
GRANT UPDATE (valid_to_revision_id) ON canon.fact, canon.alias, canon.relation TO brain_ingest;
GRANT UPDATE (finished_at, status, counts, error_code) ON ops.job_run TO brain_ingest;
GRANT UPDATE (run_id, epoch, expires_at) ON ops.job_lease TO brain_ingest;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA raw, canon TO brain_ingest;
GRANT USAGE ON SEQUENCE ops.job_run_id_seq TO brain_ingest;
GRANT EXECUTE ON FUNCTION canon.is_atomic_json(jsonb), serve.atomic_fact(bigint, text, numeric, text, jsonb, text, jsonb, bigint, text),
    serve.refresh_hero_profile() TO brain_ingest;
GRANT EXECUTE ON FUNCTION ops.is_numeric_counts(jsonb) TO brain_ingest;
GRANT EXECUTE ON FUNCTION ops.is_numeric_counts(jsonb), ops.is_trace_list(jsonb, boolean) TO brain_service;
GRANT USAGE ON SCHEMA ops TO brain_service;
GRANT INSERT (question_sha256, tool_calls, citation_ids, status, tokens, latency_ms, recorded_at) ON ops.answer_trace TO brain_service;
GRANT USAGE ON SEQUENCE ops.answer_trace_id_seq TO brain_service;
ALTER DEFAULT PRIVILEGES FOR ROLE brain_migrate IN SCHEMA canon, docs, serve GRANT SELECT ON TABLES TO brain_tool;

COMMIT;
