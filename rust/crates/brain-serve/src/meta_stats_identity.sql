SELECT jsonb_build_object(
    'stat_key',f.stat_key,'value',f.value_text,'condition','{}'::jsonb,
    'citation',f.receipt_id::text,'source_revision',f.source_revision,'patch',NULL
)
FROM serve.current_fact f
JOIN serve.current_entity e ON e.id=f.entity_id
WHERE f.entity_id=$1 AND e.kind=$2 AND f.stat_key='identity_game_id'
    AND f.value_text ~ '^[1-9][0-9]{0,18}$'
LIMIT 1
