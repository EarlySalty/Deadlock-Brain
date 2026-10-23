BEGIN;

CREATE OR REPLACE FUNCTION serve.site_hero_abilities(requested_key text)
RETURNS TABLE(slot integer,ability jsonb,link jsonb,facts jsonb)
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $abilities$
    SELECT s.slot,a.identity,s.link,COALESCE(f.rows,'[]'::jsonb)
    FROM canon.entity h
    CROSS JOIN LATERAL (
        SELECT s.slot,s.to_entity_id,s.link FROM serve.p2_hero_slot s
        WHERE s.from_entity_id=h.id
        OFFSET 0
    ) s
    CROSS JOIN LATERAL (
        SELECT i.identity FROM serve.entity_identity i
        WHERE i.entity_id=s.to_entity_id AND i.kind='ability' LIMIT 1
    ) a
    CROSS JOIN LATERAL (
        WITH facts AS MATERIALIZED (
            SELECT x.stat_key::text AS stat_key,x.value_text::text AS value_text,
                x.raw_object_id,x.valid_from_run,x.receipt_id
            FROM serve.current_fact x
            WHERE x.entity_id=s.to_entity_id AND x.source='assets'
                AND x.stat_key::text LIKE 'p2\_%' ESCAPE '\'
        ), tagged AS MATERIALIZED (
            SELECT substr(x.stat_key,6,16) AS tag,x.raw_object_id,x.valid_from_run,
                substr(x.stat_key,23) AS suffix,x.value_text,x.receipt_id,left(x.stat_key,4) AS family
            FROM facts x
        )
        SELECT jsonb_object_agg(d.stat_key,d.label) AS labels
        FROM (
            SELECT DISTINCT ON (v.suffix) v.suffix AS stat_key,btrim(l.value_text) AS label
            FROM tagged v
            JOIN tagged g ON g.family='p2_q' AND g.tag=v.tag
                AND g.raw_object_id=v.raw_object_id AND g.valid_from_run=v.valid_from_run
                AND g.suffix='source_language' AND g.value_text='de'
            JOIN tagged l ON l.family='p2_q' AND l.tag=v.tag
                AND l.raw_object_id=v.raw_object_id AND l.valid_from_run=v.valid_from_run
                AND l.suffix='label'
            WHERE v.family='p2_v' AND v.suffix ~ '^[a-z][a-z0-9_]*$'
                AND NULLIF(btrim(l.value_text),'') IS NOT NULL
            ORDER BY v.suffix,l.receipt_id DESC
            LIMIT 400
        ) d
    ) de
    CROSS JOIN LATERAL (
        SELECT jsonb_agg(q.fact ORDER BY q.stat_key,q.fact::text) AS rows
        FROM (
            SELECT c.stat_key,
                CASE WHEN de.labels ? c.stat_key::text
                    THEN (c.fact-'qualifiers')||jsonb_build_object('label_de',de.labels->c.stat_key::text)
                    ELSE c.fact-'qualifiers'
                END AS fact
            FROM serve.p2_fact c
            WHERE c.entity_id=s.to_entity_id
                AND c.stat_key NOT LIKE 'identity\_%' ESCAPE '\'
            ORDER BY c.stat_key,c.fact::text LIMIT 200
        ) q
    ) f
    WHERE h.key=requested_key AND h.kind='hero' AND serve.hero_is_playable(h.id)
    ORDER BY s.slot
$abilities$;
REVOKE ALL ON FUNCTION serve.site_hero_abilities(text) FROM PUBLIC;

DROP FUNCTION serve.site_hero_patch_changes(text);
CREATE FUNCTION serve.site_hero_patch_changes(requested_key text)
RETURNS TABLE(patch_label text,released_at timestamptz,line_text text,citation text,source_revision text,ordinal bigint)
LANGUAGE sql STABLE STRICT SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $changes$
    SELECT r.patch_label,r.released_at,r.line_text,r.citation,r.source_revision,
        row_number() OVER (ORDER BY r.released_at DESC NULLS LAST,r.patch_id,r.statement_index,r.change_id)
    FROM (
        SELECT btrim(p.label) AS patch_label,p.released_at,p.patch_id,c.line_text,
            c.receipt_id::text AS citation,c.source_revision,c.change_id,
            CASE WHEN jsonb_typeof(c.statement_index)='number' THEN (c.statement_index)::text::numeric END AS statement_index
        FROM canon.entity h
        CROSS JOIN LATERAL (
            SELECT c.change_id,c.patch_id,c.line_text,c.receipt_id,c.source_revision,c.statement_index
            FROM serve.p4_patch_change c
            WHERE c.entity_id=h.id
            OFFSET 0
        ) c
        JOIN serve.p4_patch_catalog p ON p.patch_id=c.patch_id
        WHERE h.key=requested_key AND h.kind='hero' AND serve.hero_is_playable(h.id)
            AND NULLIF(btrim(c.line_text),'') IS NOT NULL
            AND EXISTS(SELECT 1 FROM serve.published_receipt r WHERE r.id=c.receipt_id)
    ) r
    ORDER BY r.released_at DESC NULLS LAST,r.patch_id,r.statement_index,r.change_id
    LIMIT 160
$changes$;
REVOKE ALL ON FUNCTION serve.site_hero_patch_changes(text) FROM PUBLIC;

DO $site$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_site') THEN
        GRANT EXECUTE ON FUNCTION serve.site_hero_abilities(text),
            serve.site_hero_patch_changes(text) TO brain_site;
    END IF;
END
$site$;

COMMIT;
