WITH latest AS MATERIALIZED (
    SELECT m.* FROM serve.meta_stat m
    WHERE m.entity_id=(SELECT s.entity_id FROM serve.meta_stat_set s WHERE s.stat_set=$1)
        AND m.stat_key=$2
    ORDER BY m.run_id DESC LIMIT 1
), points AS (
    SELECT 0 AS position,'latest' AS snapshot,l.* FROM latest l
    UNION ALL
    SELECT b.position,b.snapshot,h.* FROM latest l
    CROSS JOIN (VALUES (1,'24h',interval '24 hours'),(2,'7d',interval '7 days'),
        (3,'30d',interval '30 days')) b(position,snapshot,back)
    CROSS JOIN LATERAL (
        SELECT m.* FROM serve.meta_stat m
        WHERE m.entity_id=l.entity_id AND m.stat_key=$2 AND m.snapshot_at<=l.snapshot_at-b.back
        ORDER BY m.run_id DESC LIMIT 1
    ) h
)
SELECT jsonb_build_object(
    'stat_key',stat_key,'snapshot',snapshot,
    'snapshot_at',to_char(snapshot_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS"Z"'),
    'value',value,
    'condition',jsonb_build_object('matches_since',CASE WHEN captured THEN to_char(
            date_trunc('day',source_revision::timestamptz AT TIME ZONE 'UTC')-interval '30 days',
            'YYYY-MM-DD"T"HH24:MI:SS"Z"') END,
        'matches_until',CASE WHEN captured THEN source_revision END,
        'game_mode','normal','match_mode','source_default','bucket','no_bucket'),
    'citation',receipt_id::text,'source_revision',source_revision,'source',source,'patch',NULL
)
FROM (
    SELECT p.*,p.source_revision ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:00:00Z$' AS captured
    FROM points p
) points ORDER BY position
LIMIT 4
