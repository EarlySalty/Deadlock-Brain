WITH selected AS (
    SELECT entity_id,identity FROM serve.entity_identity
    WHERE entity_id=ANY($1::bigint[]) AND kind IN('hero','item')
        AND (kind<>'hero' OR serve.hero_is_playable(entity_id))
    ORDER BY entity_id LIMIT 4
), facts AS MATERIALIZED (
    SELECT entity_id,stat_key,fact,total FROM (
        SELECT e.entity_id,f.stat_key,f.fact,
            count(*) OVER(PARTITION BY e.entity_id,f.stat_key) AS total,
            row_number() OVER(PARTITION BY e.entity_id,f.stat_key ORDER BY f.fact::text) AS ord
        FROM selected e CROSS JOIN LATERAL (
            SELECT stat_key,fact FROM serve.p2_fact
            WHERE entity_id=e.entity_id AND stat_key=ANY($2::text[])
        ) f
    ) bounded WHERE ord<=96
), cells AS (
    SELECT e.entity_id,s.stat_key,f.total,f.rows FROM selected e
    CROSS JOIN unnest($2::text[]) s(stat_key)
    CROSS JOIN LATERAL (
        SELECT COALESCE(max(total),0) AS total,jsonb_agg(fact ORDER BY fact::text) AS rows
        FROM facts WHERE entity_id=e.entity_id AND stat_key=s.stat_key
    ) f
), ranked AS (
    SELECT fact,row_number() OVER(ORDER BY entity_id,stat_key,fact::text) AS ord,
        sum(octet_length(fact::text)) OVER(ORDER BY entity_id,stat_key,fact::text) AS bytes
    FROM cells CROSS JOIN LATERAL jsonb_array_elements(COALESCE(rows,'[]'::jsonb)) f(fact)
)
SELECT jsonb_build_object(
    'entities',COALESCE((SELECT jsonb_agg(identity ORDER BY entity_id) FROM selected),'[]'::jsonb),
    'stats',jsonb_build_object('facts',COALESCE((SELECT jsonb_agg(fact ORDER BY ord)
        FROM ranked WHERE ord<=96 AND bytes<=10240),'[]'::jsonb),'truncated',false,'omitted_count',0),
    'cells',COALESCE((SELECT jsonb_agg(jsonb_build_object('entity_id',entity_id,'stat_key',stat_key,
        'total_count',total,'status','missing','omitted_count',0) ORDER BY entity_id,stat_key)
        FROM cells),'[]'::jsonb))
