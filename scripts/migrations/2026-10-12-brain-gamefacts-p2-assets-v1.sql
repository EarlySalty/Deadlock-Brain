BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' OR to_regclass('canon.relation') IS NULL
        OR to_regclass('serve.current_fact') IS NULL THEN
        RAISE EXCEPTION 'Assets-Migration benötigt den Migrationseigentümer und das P1-Spielwertschema';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
END
$owner$;

ALTER TABLE canon.relation
    DROP CONSTRAINT relation_kind_check,
    DROP CONSTRAINT relation_slot_check,
    ALTER COLUMN slot DROP NOT NULL,
    ADD CONSTRAINT relation_kind_check CHECK (kind IN ('has_ability', 'component_of', 'upgrades_to')),
    ADD CONSTRAINT relation_slot_check CHECK (
        (kind = 'has_ability' AND slot IS NOT NULL AND slot BETWEEN 1 AND 4)
        OR (kind IN ('component_of', 'upgrades_to') AND slot IS NULL)),
    ADD CONSTRAINT relation_item_endpoint_check CHECK (kind = 'has_ability' OR from_entity_id <> to_entity_id);
CREATE UNIQUE INDEX relation_active_item_edge_p2_idx ON canon.relation(source_id, from_entity_id, kind, to_entity_id)
    WHERE valid_to_revision_id IS NULL AND kind IN ('component_of', 'upgrades_to');
CREATE INDEX relation_item_endpoint_p2_idx ON canon.relation(to_entity_id, kind, from_entity_id, source_id, valid_from_revision_id)
    INCLUDE (valid_to_revision_id, receipt_id) WHERE kind IN ('component_of', 'upgrades_to');

CREATE OR REPLACE FUNCTION canon.guard_assertion() RETURNS trigger LANGUAGE plpgsql SET search_path = pg_catalog, pg_temp
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
        IF NEW.kind = 'has_ability' THEN
            IF NOT EXISTS (
                SELECT 1 FROM canon.entity h, canon.entity a WHERE h.id = NEW.from_entity_id AND h.kind = 'hero'
                    AND a.id = NEW.to_entity_id AND a.kind = 'ability'
            ) THEN
                RAISE EXCEPTION 'Fähigkeitsverknüpfungen benötigen Held und Fähigkeit';
            END IF;
        ELSIF NEW.kind IN ('component_of', 'upgrades_to') THEN
            IF NEW.from_entity_id = NEW.to_entity_id OR NOT EXISTS (
                SELECT 1 FROM canon.entity f, canon.entity t WHERE f.id = NEW.from_entity_id AND f.kind = 'item'
                    AND t.id = NEW.to_entity_id AND t.kind = 'item'
            ) THEN
                RAISE EXCEPTION 'Itemverknüpfungen benötigen zwei unterschiedliche Items';
            END IF;
        END IF;
    END IF;
    RETURN NEW;
END
$guard$;

CREATE FUNCTION serve.fact_selection_condition_p2(stat_key text, condition jsonb) RETURNS jsonb
LANGUAGE sql IMMUTABLE STRICT PARALLEL SAFE SET search_path = pg_catalog, pg_temp AS $condition$
    WITH upgrade AS (
        SELECT starts_with(stat_key, 'ability.')
            AND condition -> 'upgrade_property' = to_jsonb(substr(stat_key, 9))
            AND condition -> 'upgrade_tier' IN ('1'::jsonb, '2'::jsonb, '3'::jsonb)
            AND condition -> 'operation' = '"delta"'::jsonb AS redundant
    ), removable AS (
        SELECT key FROM jsonb_each(CASE WHEN jsonb_typeof(condition) = 'object' THEN condition ELSE '{}'::jsonb END)
        WHERE jsonb_typeof(value) = 'string' AND starts_with(value #>> '{}', '/')
            AND ((key IN ('scale_class_locator', 'scale_type_locator', 'scale_value_locator',
                'scale_stat_0_locator', 'scale_stat_1_locator', 'scale_stat_2_locator', 'scale_stat_3_locator',
                'usage_flag_0_locator', 'usage_flag_1_locator', 'usage_flag_2_locator', 'usage_flag_3_locator')
                AND jsonb_typeof(condition -> substr(key, 1, length(key) - 8)) = 'string')
            OR (key = 'upgrade_property_locator' AND (SELECT redundant FROM upgrade))
            OR (key = 'scale_locator' AND EXISTS (
                SELECT 1 FROM jsonb_each(condition) qualifier
                WHERE qualifier.key IN ('scale_class', 'scale_type', 'scale_value',
                    'scale_stat_0', 'scale_stat_1', 'scale_stat_2', 'scale_stat_3')
                    AND jsonb_typeof(qualifier.value) = 'string')))
        UNION ALL SELECT 'upgrade_property' WHERE (SELECT redundant FROM upgrade)
    )
    SELECT condition - coalesce((SELECT array_agg(key) FROM removable), ARRAY[]::text[])
$condition$;

CREATE VIEW serve.selected_fact_p2 AS
WITH candidates AS (
    SELECT f AS fact, p.extractor, selection.condition,
        min(f.condition::text) OVER same_source AS first_condition,
        max(f.condition::text) OVER same_source AS last_condition
    FROM serve.current_fact f JOIN canon.receipt p ON p.id = f.receipt_id
    CROSS JOIN LATERAL (VALUES (CASE WHEN p.extractor IN ('brain-assets', 'hero-ability-atoms')
        THEN serve.fact_selection_condition_p2(f.stat_key, f.condition) ELSE f.condition END)) selection(condition)
    WINDOW same_source AS (PARTITION BY f.entity_id, f.stat_key, f.source_id, selection.condition)
), qualified AS (
    SELECT c.*, bool_or(c.first_condition <> c.last_condition) OVER (
        PARTITION BY (c.fact).entity_id, (c.fact).stat_key, c.condition) AS ambiguous
    FROM candidates c
)
SELECT DISTINCT ON ((c.fact).entity_id, (c.fact).stat_key, selection.condition) (c.fact).*
FROM qualified c
CROSS JOIN LATERAL (VALUES (CASE WHEN c.ambiguous
    THEN (c.fact).condition ELSE c.condition END)) selection(condition)
ORDER BY (c.fact).entity_id, (c.fact).stat_key, selection.condition,
    CASE WHEN (c.fact).value_type = 'text' THEN CASE WHEN c.extractor = 'brain-assets' THEN 0 ELSE 1 END
        ELSE CASE WHEN c.extractor = 'brain-assets' THEN 1 ELSE 0 END END,
    (c.fact).source_id, (c.fact).valid_from_revision_id DESC, (c.fact).source_path, (c.fact).receipt_id, (c.fact).id;

CREATE VIEW serve.selected_relation_p2 AS
SELECT DISTINCT ON (r.from_entity_id, r.kind, r.slot,
    CASE WHEN r.kind = 'has_ability' THEN 0::bigint ELSE r.to_entity_id END) r.*
FROM serve.current_relation r JOIN canon.receipt p ON p.id = r.receipt_id
JOIN canon.entity f ON f.id = r.from_entity_id
JOIN canon.receipt fp ON fp.id = f.identity_receipt_id
JOIN canon.source_head fh ON fh.source_id = fp.source_id AND fh.published_revision_id >= fp.revision_id
JOIN canon.entity t ON t.id = r.to_entity_id
JOIN canon.receipt tp ON tp.id = t.identity_receipt_id
JOIN canon.source_head th ON th.source_id = tp.source_id AND th.published_revision_id >= tp.revision_id
ORDER BY r.from_entity_id, r.kind, r.slot,
    CASE WHEN r.kind = 'has_ability' THEN 0::bigint ELSE r.to_entity_id END,
    CASE WHEN p.extractor = 'brain-assets' THEN 1 ELSE 0 END,
    r.source_id, r.valid_from_revision_id DESC, r.source_path, r.receipt_id, r.id;

CREATE VIEW serve.asset_card_p2 AS
SELECT e.id AS entity_id, e.key, e.kind, jsonb_build_object(
    'identity', identity.payload,
    'facts', jsonb_build_object('facts', facts.rows, 'omitted_count', facts.omitted_count, 'truncated', facts.omitted_count > 0),
    'links', jsonb_build_object('links', links.rows, 'omitted_count', links.omitted_count, 'truncated', links.omitted_count > 0)
) AS card
FROM canon.entity e JOIN canon.receipt proof ON proof.id = e.identity_receipt_id
JOIN canon.revision identity_revision ON identity_revision.id = proof.revision_id
JOIN canon.source_head identity_head ON identity_head.source_id = proof.source_id AND identity_head.published_revision_id >= proof.revision_id
CROSS JOIN LATERAL (
    SELECT jsonb_build_object('entity_id', e.id, 'kind', e.kind, 'key', e.key, 'status', e.status,
        'citation', proof.id::text, 'patch', NULL, 'source_revision', identity_revision.source_revision,
        'condition', '{}'::jsonb) AS payload
) identity
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(payload ORDER BY ordinal) FILTER (WHERE ordinal <= 8 AND bytes <= 3500), '[]') AS rows,
        count(*) - count(*) FILTER (WHERE ordinal <= 8 AND bytes <= 3500) AS omitted_count
    FROM (
        SELECT payload, row_number() OVER ordering AS ordinal,
            sum(octet_length(payload::text) + 2) OVER ordering AS bytes
        FROM (
            SELECT r.kind, r.from_entity_id, r.to_entity_id, r.slot, r.id,
                jsonb_build_object('kind', r.kind, 'from_entity_id', r.from_entity_id, 'to_entity_id', r.to_entity_id,
                    'entity_id', endpoint.id, 'key', endpoint.key, 'slot', r.slot,
                    'citation', r.receipt_id::text, 'patch', NULL, 'source_revision', r.source_revision, 'condition', '{}'::jsonb,
                    'identity', jsonb_build_object('entity_id', endpoint.id, 'kind', endpoint.kind, 'key', endpoint.key,
                        'citation', ep.id::text, 'patch', NULL, 'source_revision', ev.source_revision, 'condition', '{}'::jsonb)) AS payload
            FROM serve.selected_relation_p2 r
            JOIN canon.entity endpoint ON endpoint.id = CASE WHEN r.from_entity_id = e.id THEN r.to_entity_id ELSE r.from_entity_id END
            JOIN canon.receipt ep ON ep.id = endpoint.identity_receipt_id
            JOIN canon.revision ev ON ev.id = ep.revision_id
            WHERE (e.kind = 'ability' AND r.kind = 'has_ability' AND r.to_entity_id = e.id)
                OR (e.kind = 'item' AND r.kind IN ('component_of', 'upgrades_to') AND (r.from_entity_id = e.id OR r.to_entity_id = e.id))
        ) assertions
        WINDOW ordering AS (ORDER BY kind, slot, from_entity_id, to_entity_id, id ROWS UNBOUNDED PRECEDING)
    ) bounded
) links
CROSS JOIN LATERAL (
    SELECT jsonb_build_object(
        'identity', identity.payload,
        'facts', jsonb_build_object('facts', '[]'::jsonb, 'omitted_count', 9223372036854775807::bigint, 'truncated', false),
        'links', jsonb_build_object('links', links.rows, 'omitted_count', links.omitted_count, 'truncated', links.omitted_count > 0)
    ) AS card,
    jsonb_build_array(proof.id::text) || coalesce((
        SELECT jsonb_agg(citation) FROM jsonb_array_elements(links.rows) link
        CROSS JOIN LATERAL (VALUES (link -> 'citation'), (link -> 'identity' -> 'citation')) ids(citation)
    ), '[]'::jsonb) AS evidence_ids
) shell
CROSS JOIN LATERAL (
    SELECT least(14000, 15000 - octet_length(shell.card::text)) AS fact_bytes,
        16384 - octet_length(jsonb_build_object('call_id', repeat('"', 512), 'name', 'damage_calculate',
            'result', jsonb_build_object('card', shell.card), 'evidence_ids', shell.evidence_ids, 'is_error', false)::text) AS envelope_bytes
) budget
CROSS JOIN LATERAL (
    SELECT coalesce(jsonb_agg(payload ORDER BY ordinal) FILTER (WHERE bytes <= budget.fact_bytes AND envelope_bytes <= budget.envelope_bytes), '[]') AS rows,
        count(*) - count(*) FILTER (WHERE bytes <= budget.fact_bytes AND envelope_bytes <= budget.envelope_bytes) AS omitted_count
    FROM (
        SELECT payload, row_number() OVER ordering AS ordinal,
            sum(octet_length(payload::text) + 2) OVER ordering AS bytes,
            sum(octet_length(payload::text) + octet_length(to_jsonb(citation)::text) + 4) OVER ordering AS envelope_bytes
        FROM (
            SELECT f.display_order, f.stat_key, f.condition, f.id, f.receipt_id::text AS citation,
                CASE WHEN f.stat_key = 'item.cost' THEN 0
                    WHEN starts_with(f.stat_key, e.kind || '.description_') THEN 1
                    WHEN f.condition ? 'upgrade_tier' THEN 2
                    WHEN f.section <> 'identity' AND (f.value_num IS NULL OR f.value_num <> 0) THEN 3
                    WHEN f.stat_key IN ('identity.name', 'name', 'name_en', 'name_de') THEN 4
                    WHEN f.section <> 'identity' THEN 5
                    ELSE 6 END AS priority,
                serve.atomic_fact(f.entity_id, f.stat_key, f.value_num, f.value_text, f.value_json,
                    f.unit, f.condition, f.receipt_id, f.source_revision) AS payload
            FROM serve.selected_fact_p2 f WHERE f.entity_id = e.id
        ) assertions
        WINDOW ordering AS (ORDER BY priority, display_order, stat_key, condition, id ROWS UNBOUNDED PRECEDING)
    ) bounded
) facts
WHERE e.kind IN ('ability', 'item');

CREATE MATERIALIZED VIEW serve.item_card AS SELECT entity_id, key, card FROM serve.asset_card_p2 WHERE kind = 'item';
CREATE UNIQUE INDEX item_card_entity_p2_idx ON serve.item_card(entity_id);
CREATE MATERIALIZED VIEW serve.ability_card AS SELECT entity_id, key, card FROM serve.asset_card_p2 WHERE kind = 'ability';
CREATE UNIQUE INDEX ability_card_entity_p2_idx ON serve.ability_card(entity_id);

CREATE FUNCTION serve.refresh_assets_p2() RETURNS void LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog, pg_temp AS $refresh$
BEGIN
    IF current_setting('transaction_isolation') <> 'read committed' THEN
        RAISE EXCEPTION 'Spielwertveröffentlichungen benötigen READ COMMITTED' USING ERRCODE = '0A000';
    END IF;
    PERFORM pg_advisory_xact_lock(742110026114::bigint);
    REFRESH MATERIALIZED VIEW CONCURRENTLY serve.item_card;
    REFRESH MATERIALIZED VIEW CONCURRENTLY serve.ability_card;
END
$refresh$;
CREATE FUNCTION canon.refresh_assets_on_publish_p2() RETURNS trigger LANGUAGE plpgsql
SET search_path = pg_catalog, pg_temp AS $refresh$
BEGIN
    PERFORM serve.refresh_assets_p2();
    RETURN NULL;
END
$refresh$;
CREATE TRIGGER head_refresh_assets_p2 AFTER INSERT OR UPDATE ON canon.source_head
    FOR EACH STATEMENT EXECUTE FUNCTION canon.refresh_assets_on_publish_p2();

REVOKE ALL ON FUNCTION serve.fact_selection_condition_p2(text, jsonb), serve.refresh_assets_p2(), canon.refresh_assets_on_publish_p2() FROM PUBLIC, brain_tool, brain_ingest, brain_service;
GRANT EXECUTE ON FUNCTION serve.fact_selection_condition_p2(text, jsonb) TO brain_tool, brain_ingest;
GRANT EXECUTE ON FUNCTION serve.refresh_assets_p2() TO brain_ingest;
GRANT SELECT ON serve.selected_fact_p2, serve.selected_relation_p2, serve.asset_card_p2, serve.item_card, serve.ability_card TO brain_tool, brain_ingest;

COMMIT;
