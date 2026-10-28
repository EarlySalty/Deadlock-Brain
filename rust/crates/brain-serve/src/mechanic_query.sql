SELECT jsonb_build_object(
    'stat_key',stat_key,'value',value,'condition',condition,
    'citation',receipt_id::text,'source_revision',source_revision,'patch',NULL
)
FROM serve.game_rule
WHERE key=$1
ORDER BY entity_id,id
LIMIT 401
