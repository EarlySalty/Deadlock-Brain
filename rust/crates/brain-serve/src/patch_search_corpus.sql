WITH catalog AS MATERIALIZED (
        SELECT c.* FROM serve.p4_patch_catalog c
        LEFT JOIN serve.patch_ledger_member l ON l.patch_id=c.patch_id
        WHERE (l.disposition IS NULL OR l.disposition='primary')
            AND ($1::text IS NULL OR coalesce(l.ledger_released_at,c.released_at) >= $1::timestamptz)
            AND ($2::text IS NULL OR coalesce(l.ledger_released_at,c.released_at) <= $2::timestamptz)
    ), lines AS MATERIALIZED (
        SELECT f.entity_id,f.raw_object_id,f.valid_from_run,f.id,f.receipt_id,f.value_text,
            CASE WHEN f.stat_key COLLATE "C" < 'patch_line_r'
                    THEN f.stat_key ~ '^patch_change_r[0-9a-f]+_[0-9]+$' AND strpos(substr(f.stat_key,15),'_')=65
                WHEN f.stat_key COLLATE "C" < 'patch_note_'
                    THEN f.stat_key ~ '^patch_line_r[0-9a-f]+_[0-9]+_[0-9]+$' AND strpos(substr(f.stat_key,13),'_')=65
                ELSE f.stat_key ~ '^patch_note_[0-9]+_[0-9]+$' END AS shaped
        FROM serve.current_fact f
        WHERE f.entity_id=ANY(ARRAY(SELECT c.patch_id FROM catalog c))
            AND (f.stat_key COLLATE "C" >= 'patch_change_r' AND f.stat_key COLLATE "C" < 'patch_change_s'
                OR f.stat_key COLLATE "C" >= 'patch_line_r' AND f.stat_key COLLATE "C" < 'patch_line_s'
                OR f.stat_key COLLATE "C" >= 'patch_note_' AND f.stat_key COLLATE "C" < 'patch_note`')
    ), facts AS MATERIALIZED (
        SELECT c.patch_id,c.label,c.released_at,c.official_receipt_id,c.label_receipt_id,
            c.released_at_receipt_id,c.url_receipt_id,c.extractor,c.raw_object_id,c.ingest_run_id,
            f.id,f.receipt_id,f.value_text::text AS body
        FROM catalog c JOIN lines f ON f.entity_id=c.patch_id AND f.raw_object_id=c.raw_object_id
            AND f.valid_from_run=c.ingest_run_id
        WHERE f.shaped
    ), matched AS MATERIALIZED (
        SELECT f.*,payload.value
        FROM facts f CROSS JOIN LATERAL (
            SELECT CASE WHEN pg_input_is_valid(f.body,'jsonb')
                THEN f.body::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
        ) payload
        WHERE jsonb_typeof(payload.value)='object'
            AND jsonb_typeof(payload.value->'line_text')='string' AND btrim(payload.value->>'line_text')<>''
    ) SELECT jsonb_build_object('patch_id',m.patch_id,'label',m.label,'released_at',m.released_at,
        'official_receipt_id',m.official_receipt_id,'label_receipt_id',m.label_receipt_id,
        'released_at_receipt_id',m.released_at_receipt_id,'url_receipt_id',m.url_receipt_id,
        'extractor',m.extractor,'raw_object_id',m.raw_object_id,'ingest_run_id',m.ingest_run_id,
        'id',m.id,'receipt_id',m.receipt_id,'value',m.value),lower(m.value->>'line_text')
    FROM matched m ORDER BY m.id
