SELECT jsonb_build_object(
    'entity_id',entity_id,'stat_key',stat_key,'value',value,
    'unit',unit,'condition',condition,'citation',receipt_id::text,
    'source_revision',source_revision,'source',source,'patch',NULL,'timer',key
)
FROM serve.timer
WHERE key=$1
ORDER BY stat_key,source,id
LIMIT 33
