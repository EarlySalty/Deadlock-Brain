WITH matched AS MATERIALIZED (
        SELECT * FROM jsonb_to_recordset($2::jsonb) AS m(patch_id bigint,label text,released_at timestamptz,
            official_receipt_id bigint,label_receipt_id bigint,released_at_receipt_id bigint,url_receipt_id bigint,
            extractor text,raw_object_id bigint,ingest_run_id bigint,id bigint,receipt_id bigint,value jsonb,hits bigint)
    ), rarity AS MATERIALIZED (
        SELECT t.term,ln(($3::float8+2)/(1+t.lines))/ln($3::float8+2) AS weight
        FROM unnest($1::text[],$4::bigint[]) t(term,lines)
    ), distinct_lines AS (
        SELECT DISTINCT ON (m.patch_id,regexp_replace(btrim(m.value->>'line_text'),'^[-*•][[:space:]]*',''))
            m.* FROM matched m WHERE m.hits>0
        ORDER BY m.patch_id,regexp_replace(btrim(m.value->>'line_text'),'^[-*•][[:space:]]*',''),m.id
    ), ranked AS MATERIALIZED (
        SELECT d.*,sc.score FROM distinct_lines d CROSS JOIN LATERAL (
            SELECT COALESCE(max(power(g.hits,1.75)*(CASE WHEN g.changed THEN 1.75 ELSE 1 END)
                /(1+ln(1+g.words::float8/20))),0) AS score
            FROM (SELECT (SELECT sum(CASE WHEN t ~ '^[0-9]' THEN (CASE WHEN lower(x.seg) ~ ('(^|[^0-9.,])'||t||'(?![.,]?[0-9])') THEN w.weight ELSE 0 END)
                        WHEN lower(x.seg) ~ ('\m'||t) THEN w.weight
                        WHEN strpos(lower(x.seg),t)>0 THEN 0.5*w.weight ELSE 0 END)::float8
                    FROM unnest($1::text[]) t JOIN rarity w ON w.term=t) AS hits,
                cardinality(regexp_split_to_array(btrim(x.seg),'\s+')) AS words,
                (jsonb_typeof(d.value->'new') IN ('number','string')
                    OR x.seg ~* '\mfrom\s+[-+]?[0-9.][^;]{0,48}?\mto\s+[-+]?[0-9.]') AS changed
            FROM unnest(regexp_split_to_array(d.value->>'line_text','\s+-\s+')) x(seg)) g
            WHERE g.hits>0
        ) sc
        ORDER BY sc.score DESC,d.released_at DESC,d.patch_id DESC,d.id DESC LIMIT 60
    ), scored AS (
        SELECT * FROM ranked m
        WHERE EXISTS(SELECT 1 FROM serve.published_receipt r WHERE r.id=m.receipt_id
            AND r.extractor=m.extractor AND r.raw_object_id=m.raw_object_id
            AND r.ingest_run_id=m.ingest_run_id
            AND r.locator_kind='json_pointer' AND r.locator='/raw_content')
    ) SELECT jsonb_build_object('id',s.id,'entity_id',NULL::bigint,'section',s.value->'section',
        'subject',s.value->'subject','line_text',s.value->>'line_text','stat_label',s.value->'stat_label',
        'old',s.value->'old','new',s.value->'new','patch',s.label,'released_at',s.released_at,
        'citation',s.receipt_id::text,'head_citation',s.official_receipt_id::text,
        'label_citation',s.label_receipt_id::text,'released_at_citation',s.released_at_receipt_id::text,
        'url_citation',s.url_receipt_id::text,'binding_citation',NULL::text,'entity_citation',NULL::text,
        'stat_citation',NULL::text,'ledger_id',l.ledger_id,'ledger_released_at',l.ledger_released_at)
    FROM scored s LEFT JOIN serve.patch_ledger_member l ON l.patch_id=s.patch_id
    ORDER BY s.score DESC,s.released_at DESC,s.patch_id DESC,s.id DESC LIMIT 20
