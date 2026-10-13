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
    'condition',jsonb_build_object('window','rolling_30_days_before_snapshot',
        'game_mode','normal','match_mode','source_default','bucket','no_bucket'),
    'citation',receipt_id::text,'source_revision',source_revision,'source',source,'patch',NULL
)
FROM points ORDER BY position
LIMIT 4
