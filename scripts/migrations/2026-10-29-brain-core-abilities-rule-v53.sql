BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Spielregel-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE OR REPLACE VIEW serve.game_rule AS
SELECT a.alias_norm AS key,e.id AS entity_id,f.stat_key,
    f.value_text::jsonb->'value' AS value,
    f.value_text::jsonb->>'unit' AS unit,
    f.value_text::jsonb->'condition' AS condition,
    f.receipt_id,f.source,f.source_revision,f.id
FROM serve.current_alias a
CROSS JOIN LATERAL (
    SELECT e.id,e.source,e.raw_object_id FROM serve.current_entity e
    WHERE e.id=a.entity_id AND e.raw_object_id=a.raw_object_id AND e.source=a.source
        AND e.kind='rule'
    OFFSET 0
) e
CROSS JOIN LATERAL (
    SELECT f.* FROM serve.current_fact f
    WHERE f.entity_id=e.id AND f.raw_object_id=e.raw_object_id AND f.source=e.source
        AND f.value_text IS NOT NULL
    OFFSET 0
) f
WHERE a.lang='und'
    AND a.alias_norm IN (
    'mechanic_lane_troopers', 'mechanic_soul_sharing', 'mechanic_denizens', 'mechanic_objectives',
    'mechanic_mid_boss', 'mechanic_rejuvenator', 'mechanic_hero_kills', 'mechanic_respawn',
    'mechanic_urn', 'mechanic_breakables', 'mechanic_sinners_sacrifice', 'mechanic_item_tiers',
    'mechanic_levels', 'mechanic_street_brawl', 'mechanic_core_abilities');

REVOKE ALL ON serve.game_rule FROM PUBLIC;
GRANT SELECT ON serve.game_rule TO brain_tool;

COMMIT;
