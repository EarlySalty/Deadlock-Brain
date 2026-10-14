WITH stat_set AS MATERIALIZED (
    SELECT s.entity_id,s.path FROM serve.meta_stat_set s WHERE s.stat_set=$1
), captures AS MATERIALIZED (
    SELECT c.raw_object_id,c.run_id FROM serve.meta_stat_capture c
    WHERE c.path=(SELECT s.path FROM stat_set s)
    ORDER BY c.capture_key DESC,c.raw_object_id DESC LIMIT 1000
), applications AS MATERIALIZED (
    SELECT s.entity_id,a.run_id,a.snapshot_at FROM stat_set s
    CROSS JOIN LATERAL (
        SELECT m.run_id,m.snapshot_at FROM serve.meta_stat m
        WHERE m.entity_id=s.entity_id AND m.stat_key=$2
        ORDER BY m.run_id DESC LIMIT 1000
    ) a
), history AS MATERIALIZED (
    SELECT a.entity_id,a.run_id,a.snapshot_at FROM applications a
    UNION
    SELECT s.entity_id,present.run_id,present.snapshot_at FROM captures c
    CROSS JOIN stat_set s
    CROSS JOIN LATERAL (
        SELECT m.run_id,m.snapshot_at FROM serve.meta_stat m
        WHERE m.entity_id=s.entity_id AND m.stat_key=$2 AND m.run_id=c.run_id
            AND m.raw_object_id=c.raw_object_id
        LIMIT 1
    ) present
), latest AS MATERIALIZED (
    SELECT h.* FROM history h ORDER BY h.snapshot_at DESC,h.run_id DESC LIMIT 1
), picks AS (
    SELECT 0 AS position,'latest' AS snapshot,l.entity_id,l.run_id FROM latest l
    UNION ALL
    SELECT b.position,b.snapshot,c.entity_id,c.run_id FROM latest l
    CROSS JOIN (VALUES (1,'24h',interval '24 hours'),(2,'7d',interval '7 days'),
        (3,'30d',interval '30 days')) b(position,snapshot,back)
    CROSS JOIN LATERAL (
        SELECT h.* FROM history h
        WHERE h.snapshot_at<=l.snapshot_at-b.back
        ORDER BY h.snapshot_at DESC,h.run_id DESC LIMIT 1
    ) c
)
SELECT jsonb_build_object(
    'stat_key',m.stat_key,'snapshot',p.snapshot,
    'snapshot_at',to_char(m.snapshot_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS"Z"'),
    'value',m.value,
    'condition',jsonb_build_object('matches_since',to_char(
            date_trunc('day',m.snapshot_at AT TIME ZONE 'UTC')-interval '30 days',
            'YYYY-MM-DD"T"HH24:MI:SS"Z"'),
        'matches_until',to_char(m.snapshot_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS"Z"'),
        'game_mode','normal','match_mode','source_default','bucket','no_bucket'),
    'citation',m.receipt_id::text,'source_revision',m.source_revision,'source',m.source,'patch',NULL
)
FROM picks p
CROSS JOIN LATERAL (
    SELECT v.* FROM serve.meta_stat v
    WHERE v.entity_id=p.entity_id AND v.stat_key=$2 AND v.run_id=p.run_id LIMIT 1
) m
ORDER BY p.position
LIMIT 4
