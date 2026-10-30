BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Patch-Zeilen benötigen brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE TABLE canon.patch_line (
    ledger_id bigint NOT NULL REFERENCES ops.patch_ledger(ledger_id) DEFERRABLE INITIALLY DEFERRED,
    line_no integer NOT NULL CHECK (line_no>=0),
    piece integer NOT NULL CHECK (piece>=0),
    section text NOT NULL CHECK (btrim(section)<>''),
    entity_type text NOT NULL CHECK (entity_type IN ('hero','ability','item','objective','general')),
    entity_id bigint REFERENCES canon.entity(id),
    entity_name text CHECK (btrim(entity_name)<>''),
    ability_id bigint REFERENCES canon.entity(id),
    ability_name text CHECK (btrim(ability_name)<>''),
    alias_used text CHECK (btrim(alias_used)<>''),
    change_type text CHECK (change_type IN ('buff','nerf','changed','added','removed','rework','bugfix','rename')),
    stat_name text CHECK (btrim(stat_name)<>''),
    old_value text CHECK (btrim(old_value)<>''),
    new_value text CHECK (btrim(new_value)<>''),
    raw_line text NOT NULL CHECK (btrim(raw_line)<>''),
    source text NOT NULL CHECK (source='extract' OR source='manual' OR source ~ '^relink-consensus-[0-9]{4}-[0-9]{2}-[0-9]{2}$'),
    correction_ref text CHECK (btrim(correction_ref)<>''),
    needs_review boolean NOT NULL DEFAULT false,
    run_id bigint REFERENCES ops.ingest_run(id),
    PRIMARY KEY (ledger_id,line_no,piece),
    CONSTRAINT patch_line_general_unbound CHECK (entity_type<>'general' OR entity_id IS NULL),
    CONSTRAINT patch_line_ability_only CHECK (entity_type='ability' OR (ability_id IS NULL AND ability_name IS NULL)),
    CONSTRAINT patch_line_ability_named CHECK (entity_type<>'ability' OR ability_name IS NOT NULL),
    CONSTRAINT patch_line_values_paired CHECK ((old_value IS NULL)=(new_value IS NULL)),
    CONSTRAINT patch_line_no_change_words CHECK (
        coalesce(lower(btrim(stat_name)),'') NOT IN ('buff','nerf','changed','added','removed','rework','bugfix','rename','fixed','fix')
        AND coalesce(lower(btrim(old_value)),'') NOT IN ('buff','nerf','changed','added','removed','rework','bugfix','rename','fixed','fix')
        AND coalesce(lower(btrim(new_value)),'') NOT IN ('buff','nerf','changed','added','removed','rework','bugfix','rename','fixed','fix')),
    CONSTRAINT patch_line_correction_ref CHECK ((source='extract')=(correction_ref IS NULL)),
    CONSTRAINT patch_line_extract_run CHECK (source<>'extract' OR run_id IS NOT NULL)
);
CREATE INDEX patch_line_entity ON canon.patch_line (entity_id,ledger_id) WHERE entity_id IS NOT NULL;
CREATE INDEX patch_line_ability ON canon.patch_line (ability_id,ledger_id) WHERE ability_id IS NOT NULL;
CREATE INDEX patch_line_type ON canon.patch_line (entity_type,ledger_id);

CREATE VIEW serve.patch_change_v2 AS
SELECT l.ledger_id,l.line_no,l.piece,p.patch_id,p.label,p.kind,p.released_at,p.source_url,
    p.released_at_receipt_id,p.source_revision,
    l.section,l.entity_type,l.entity_id,l.entity_name,l.ability_id,l.ability_name,l.alias_used,
    l.change_type,l.stat_name,l.old_value,l.new_value,l.raw_line,l.source,l.correction_ref,
    l.needs_review
FROM canon.patch_line l
JOIN serve.patch_ledger p ON p.ledger_id=l.ledger_id;

CREATE OR REPLACE FUNCTION serve.site_generation()
RETURNS text
LANGUAGE plpgsql STABLE SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $generation$
BEGIN
    RETURN md5(concat_ws('|',
        (SELECT string_agg(r.id::text,',' ORDER BY r.id) FROM ops.ingest_run r
            WHERE r.status='succeeded'
                AND EXISTS(SELECT 1 FROM canon.receipt c WHERE c.ingest_run_id=r.id)),
        (SELECT count(*)::text||':'||COALESCE(sum(q.receipt_id),0)::text||':'
                ||count(*) FILTER (WHERE q.cause='audited')::text
            FROM canon.receipt_quarantine q),
        (SELECT count(*)::text||':'||COALESCE(sum(a.raw_object_id),0)::text||':'
                ||COALESCE(sum(a.run_id),0)::text
            FROM ops.receipt_audit_rejection a),
        (SELECT md5(string_agg(e.id::text||':'||e.kind::text||':'||e.key::text,',' ORDER BY e.id))
            FROM canon.entity e),
        (SELECT 'ledger:'||md5(COALESCE(string_agg(concat_ws(':',l.ledger_id,l.dated_post_id,
                extract(epoch FROM l.released_at),l.kind,l.label,l.source_url),',' ORDER BY l.ledger_id),''))
            FROM ops.patch_ledger l),
        (SELECT 'posts:'||md5(COALESCE(string_agg(concat_ws(':',m.post_id,m.ledger_id,m.disposition),
                ',' ORDER BY m.post_id),''))
            FROM ops.patch_ledger_post m),
        (SELECT 'lines:'||count(*)::text||':'||COALESCE(sum(hashtext(concat_ws(chr(31),
                l.ledger_id,l.line_no,l.piece,l.section,l.entity_type,l.entity_id,l.entity_name,
                l.ability_id,l.ability_name,l.alias_used,l.change_type,l.stat_name,l.old_value,
                l.new_value,l.raw_line,l.source,l.correction_ref,l.needs_review))::bigint),0)::text
            FROM canon.patch_line l)
    ));
END
$generation$;

REVOKE ALL ON canon.patch_line,serve.patch_change_v2 FROM PUBLIC;
GRANT SELECT,INSERT,UPDATE,DELETE ON canon.patch_line TO brain_ingest;
GRANT SELECT ON serve.patch_change_v2 TO brain_tool;

COMMIT;
