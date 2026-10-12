BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' OR NOT EXISTS (SELECT 1 FROM ops.gamefacts_schema_version WHERE version = 1) THEN
        RAISE EXCEPTION 'Dokumentmigration benötigt den Migrationseigentümer und das Spielwertschema';
    END IF;
END
$owner$;

CREATE FUNCTION docs.valid_heading_path(value text[]) RETURNS boolean
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $heading$
    SELECT cardinality(value) <= 16 AND (cardinality(value) = 0 OR array_ndims(value) = 1 AND array_lower(value, 1) = 1)
        AND coalesce((SELECT bool_and(v IS NOT NULL AND octet_length(v) BETWEEN 1 AND 256) FROM unnest(value) v), true)
$heading$;

CREATE FUNCTION docs.valid_entity_ids(value bigint[]) RETURNS boolean
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $entities$
    SELECT cardinality(value) <= 64 AND (cardinality(value) = 0 OR array_ndims(value) = 1 AND array_lower(value, 1) = 1)
        AND coalesce((SELECT bool_and(v IS NOT NULL AND v > 0) FROM unnest(value) v), true)
$entities$;

CREATE FUNCTION docs.section_tsv(heading_path text[], body text) RETURNS tsvector
LANGUAGE sql IMMUTABLE PARALLEL SAFE SET search_path = pg_catalog, pg_temp
AS $fts$
    SELECT setweight(to_tsvector('pg_catalog.german'::regconfig, array_to_string(heading_path, ' ')), 'A')
        || setweight(to_tsvector('pg_catalog.simple'::regconfig, array_to_string(heading_path, ' ')), 'A')
        || setweight(to_tsvector('pg_catalog.german'::regconfig, body), 'B')
        || setweight(to_tsvector('pg_catalog.simple'::regconfig, body), 'B')
$fts$;

CREATE TABLE docs.page (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_id bigint NOT NULL REFERENCES raw.source(id) ON DELETE RESTRICT,
    path text NOT NULL CHECK (octet_length(path) BETWEEN 1 AND 2048
        AND path ~ '^(public|internal)/.+[.]html$' AND path !~ '(^|/)[.]{1,2}(/|$)' AND path !~ '//|\\'),
    title text NOT NULL CHECK (octet_length(title) BETWEEN 1 AND 512),
    lang text NOT NULL CHECK (lang ~ '^[A-Za-z]{2,8}(-[A-Za-z0-9]{1,8})?$' AND length(lang) <= 16),
    scope text GENERATED ALWAYS AS (CASE WHEN path LIKE 'public/%' THEN 'public' ELSE 'internal' END) STORED NOT NULL,
    raw_object_id bigint NOT NULL,
    valid_from_revision_id bigint NOT NULL,
    valid_to_revision_id bigint,
    CHECK (valid_to_revision_id IS NULL OR valid_to_revision_id > valid_from_revision_id),
    FOREIGN KEY (raw_object_id, source_id) REFERENCES raw.object(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_from_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT,
    FOREIGN KEY (valid_to_revision_id, source_id) REFERENCES canon.revision(id, source_id) ON DELETE RESTRICT,
    UNIQUE (source_id, path, valid_from_revision_id)
);
CREATE UNIQUE INDEX page_active_idx ON docs.page(source_id, path) WHERE valid_to_revision_id IS NULL;
CREATE INDEX page_source_history_idx ON docs.page(source_id, path, valid_from_revision_id) INCLUDE (valid_to_revision_id);
CREATE INDEX page_object_idx ON docs.page(raw_object_id, source_id);
CREATE INDEX page_from_idx ON docs.page(valid_from_revision_id, source_id);
CREATE INDEX page_to_idx ON docs.page(valid_to_revision_id, source_id);

CREATE TABLE docs.section (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    page_id bigint NOT NULL REFERENCES docs.page(id) ON DELETE RESTRICT,
    ordinal integer NOT NULL CHECK (ordinal BETWEEN 0 AND 4095),
    heading_path text[] NOT NULL DEFAULT '{}' CHECK (docs.valid_heading_path(heading_path)),
    body text NOT NULL CHECK (octet_length(body) BETWEEN 1 AND 262144),
    entity_ids bigint[] NOT NULL DEFAULT '{}' CHECK (docs.valid_entity_ids(entity_ids)),
    url text NOT NULL CHECK (octet_length(url) BETWEEN 1 AND 2048 AND url ~ '^https://[^[:space:]]+$'),
    receipt_id bigint NOT NULL REFERENCES canon.receipt(id) ON DELETE RESTRICT,
    tsv tsvector GENERATED ALWAYS AS (docs.section_tsv(heading_path, body)) STORED NOT NULL,
    UNIQUE (page_id, ordinal)
);
CREATE INDEX section_receipt_idx ON docs.section(receipt_id);
CREATE INDEX section_tsv_idx ON docs.section USING gin(tsv);
CREATE INDEX section_entity_ids_idx ON docs.section USING gin(entity_ids);

CREATE FUNCTION docs.serialize_write() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $serialize$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Dokumentänderungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    RETURN NULL;
END
$serialize$;
CREATE TRIGGER page_write_lock BEFORE INSERT OR UPDATE OR DELETE ON docs.page FOR EACH STATEMENT EXECUTE FUNCTION docs.serialize_write();
CREATE TRIGGER section_write_lock BEFORE INSERT OR UPDATE OR DELETE ON docs.section FOR EACH STATEMENT EXECUTE FUNCTION docs.serialize_write();

CREATE FUNCTION docs.guard_page() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $page$
DECLARE revision bigint;
BEGIN
    IF TG_OP = 'DELETE' THEN
        RAISE EXCEPTION 'Dokumentversionen dürfen nicht gelöscht werden';
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.valid_to_revision_id IS NOT NULL OR NEW.valid_to_revision_id IS NULL
            OR (to_jsonb(NEW) - ARRAY['valid_to_revision_id', 'scope'])
                IS DISTINCT FROM (to_jsonb(OLD) - ARRAY['valid_to_revision_id', 'scope']) THEN
            RAISE EXCEPTION 'Eine Dokumentversion darf nur einmal geschlossen werden';
        END IF;
        revision := NEW.valid_to_revision_id;
    ELSE
        IF NEW.valid_to_revision_id IS NOT NULL OR NOT EXISTS (
            SELECT 1 FROM raw.object o WHERE o.id = NEW.raw_object_id AND o.source_id = NEW.source_id
                AND o.external_id = NEW.path AND octet_length(o.body) <= 8388608
        ) THEN
            RAISE EXCEPTION 'Neue Dokumentversion benötigt ein passendes begrenztes Rohobjekt';
        END IF;
        revision := NEW.valid_from_revision_id;
        IF EXISTS (SELECT 1 FROM docs.page p WHERE p.source_id = NEW.source_id AND p.path = NEW.path
            AND (p.valid_to_revision_id IS NULL OR p.valid_to_revision_id > revision)) THEN
            RAISE EXCEPTION 'Dokumentversionen dürfen sich nicht überlappen';
        END IF;
    END IF;
    IF revision <= coalesce((SELECT published_revision_id FROM canon.source_head WHERE source_id = NEW.source_id), 0)
        OR NOT EXISTS (
            SELECT 1 FROM canon.revision v JOIN raw.run r ON r.id = v.raw_run_id
            JOIN raw.run_object ro ON ro.run_id = r.id AND ro.path = NEW.path
            WHERE v.id = revision AND v.source_id = NEW.source_id AND r.status <> 'succeeded'
                AND (TG_OP = 'UPDATE' OR ro.action = 'upsert' AND ro.object_id = NEW.raw_object_id)
            FOR UPDATE OF r
        ) THEN
        RAISE EXCEPTION 'Dokumentänderung benötigt einen geplanten Pfad im offenen Folgestand';
    END IF;
    RETURN NEW;
END
$page$;
CREATE TRIGGER page_guard BEFORE INSERT OR UPDATE OR DELETE ON docs.page FOR EACH ROW EXECUTE FUNCTION docs.guard_page();

CREATE FUNCTION docs.guard_section() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $section$
DECLARE page docs.page%ROWTYPE;
BEGIN
    IF TG_OP <> 'INSERT' THEN
        RAISE EXCEPTION 'Dokumentabschnitte und ihre Belege sind unveränderlich';
    END IF;
    SELECT * INTO page FROM docs.page WHERE id = NEW.page_id FOR UPDATE;
    IF NOT FOUND OR page.valid_to_revision_id IS NOT NULL OR NOT EXISTS (
        SELECT 1 FROM canon.receipt c JOIN canon.revision v ON v.id = c.revision_id
        JOIN raw.run r ON r.id = v.raw_run_id
        WHERE c.id = NEW.receipt_id AND c.source_id = page.source_id AND c.raw_object_id = page.raw_object_id
            AND c.revision_id = page.valid_from_revision_id AND r.status <> 'succeeded'
        FOR UPDATE OF r
    ) THEN
        RAISE EXCEPTION 'Abschnittbeleg muss zum Rohobjekt, zur Quelle und zur offenen Seitenrevision gehören';
    END IF;
    IF EXISTS (SELECT 1 FROM unnest(NEW.entity_ids) entity_id WHERE NOT EXISTS (
        SELECT 1 FROM canon.entity e WHERE e.id = entity_id
    )) OR octet_length(NEW.body) + coalesce((SELECT sum(octet_length(body)) FROM docs.section WHERE page_id = page.id), 0) > 8388608 THEN
        RAISE EXCEPTION 'Abschnitt überschreitet die Seitengrenze oder verweist auf eine unbekannte Entität';
    END IF;
    RETURN NEW;
END
$section$;
CREATE TRIGGER section_guard BEFORE INSERT OR UPDATE OR DELETE ON docs.section FOR EACH ROW EXECUTE FUNCTION docs.guard_section();

CREATE FUNCTION docs.guard_publication() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
AS $publish$
BEGIN
    IF EXISTS (
        SELECT 1 FROM canon.revision v JOIN raw.run_object ro ON ro.run_id = v.raw_run_id
        WHERE v.id = NEW.published_revision_id AND ro.path ~ '^(public|internal)/.+[.]html$' AND (
            EXISTS (SELECT 1 FROM docs.page p WHERE p.source_id = NEW.source_id AND p.path = ro.path
                AND p.valid_from_revision_id < v.id AND (p.valid_to_revision_id IS NULL OR p.valid_to_revision_id > v.id))
            OR ro.action = 'delete' AND EXISTS (SELECT 1 FROM docs.page p WHERE p.source_id = NEW.source_id
                AND p.path = ro.path AND p.valid_from_revision_id = v.id)
            OR ro.action = 'upsert' AND NOT EXISTS (SELECT 1 FROM docs.page p WHERE p.source_id = NEW.source_id
                AND p.path = ro.path AND p.valid_from_revision_id = v.id AND p.raw_object_id = ro.object_id
                AND EXISTS (SELECT 1 FROM docs.section s WHERE s.page_id = p.id))
        )
    ) THEN
        RAISE EXCEPTION 'Dokumentveröffentlichung benötigt vollständig ersetzte oder geschlossene Seiten mit Abschnitten';
    END IF;
    RETURN NEW;
END
$publish$;
CREATE TRIGGER docs_publication_guard BEFORE INSERT OR UPDATE ON canon.source_head FOR EACH ROW EXECUTE FUNCTION docs.guard_publication();

CREATE VIEW docs.current_section AS
SELECT s.id AS section_id, p.id AS page_id, p.source_id, p.path, p.title, p.lang, p.scope,
    s.ordinal, s.heading_path, s.body, s.entity_ids, s.url, s.receipt_id,
    v.source_revision, v.id AS revision_id, s.tsv
FROM docs.page p JOIN docs.section s ON s.page_id = p.id
JOIN canon.source_head h ON h.source_id = p.source_id
JOIN canon.revision v ON v.id = p.valid_from_revision_id
WHERE p.valid_from_revision_id <= h.published_revision_id
    AND (p.valid_to_revision_id IS NULL OR p.valid_to_revision_id > h.published_revision_id);

REVOKE ALL ON TABLE docs.page, docs.section, docs.current_section FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON SEQUENCE docs.page_id_seq, docs.section_id_seq FROM PUBLIC, brain_tool, brain_ingest, brain_service;
REVOKE ALL ON FUNCTION docs.valid_heading_path(text[]), docs.valid_entity_ids(bigint[]), docs.section_tsv(text[], text),
    docs.serialize_write(), docs.guard_page(), docs.guard_section(), docs.guard_publication() FROM PUBLIC, brain_tool, brain_ingest, brain_service;
GRANT USAGE ON SCHEMA docs TO brain_ingest;
GRANT SELECT ON docs.page, docs.section, docs.current_section TO brain_tool, brain_ingest;
GRANT INSERT (source_id, path, title, lang, raw_object_id, valid_from_revision_id) ON docs.page TO brain_ingest;
GRANT UPDATE (valid_to_revision_id) ON docs.page TO brain_ingest;
GRANT INSERT (page_id, ordinal, heading_path, body, entity_ids, url, receipt_id) ON docs.section TO brain_ingest;
GRANT USAGE ON SEQUENCE docs.page_id_seq, docs.section_id_seq TO brain_ingest;
GRANT EXECUTE ON FUNCTION docs.valid_heading_path(text[]), docs.valid_entity_ids(bigint[]), docs.section_tsv(text[], text) TO brain_ingest;

COMMIT;
