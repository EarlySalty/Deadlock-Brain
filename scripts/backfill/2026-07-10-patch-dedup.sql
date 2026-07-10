BEGIN;

DROP TABLE IF EXISTS _patch_event_dedup_counts;
DROP TABLE IF EXISTS _patch_event_dedup_stats;
DROP TABLE IF EXISTS _patch_event_dedup_candidates;

CREATE TEMP TABLE _patch_event_dedup_counts (
    action text PRIMARY KEY,
    rows bigint NOT NULL
);

CREATE TEMP TABLE _patch_event_dedup_candidates AS
WITH raw_base AS (
    SELECT
        pe.id,
        pe.confidence,
        pee.patch_event_id IS NOT NULL AS has_enrichment,
        lower(regexp_replace(TRIM(BOTH FROM coalesce(pe.patch_title, '')), '\s+', ' ', 'g')) AS patch_title_key,
        pe.entity_type,
        pe.entity_name,
        CASE
            WHEN pee.ability_name IS NULL THEN NULL::text
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM pee.ability_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM pee.ability_name), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM pee.ability_name)
        END AS ability_name_raw,
        CASE
            WHEN pee.stat_name IS NULL THEN NULL::text
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM pee.stat_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM pee.stat_name), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM pee.stat_name)
        END AS stat_name_raw,
        COALESCE(NULLIF(TRIM(BOTH FROM pee.old_value), ''), NULLIF(TRIM(BOTH FROM pe.old_value), '')) AS old_value,
        COALESCE(NULLIF(TRIM(BOTH FROM pee.new_value), ''), NULLIF(TRIM(BOTH FROM pe.new_value), '')) AS new_value,
        CASE
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line))
        END AS cleaned_source_line
    FROM brain.patch_events pe
    LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id = pe.id
), keyed AS (
    SELECT
        id,
        confidence,
        has_enrichment,
        patch_title_key,
        coalesce(entity_type, '') AS entity_type_key,
        coalesce(entity_name, '') AS entity_name_key,
        coalesce(NULLIF(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''), '') AS ability_name_key,
        coalesce(NULLIF(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''), '') AS stat_name_key,
        coalesce(NULLIF(TRIM(BOTH FROM old_value), ''), '') AS old_value_key,
        coalesce(NULLIF(TRIM(BOTH FROM new_value), ''), '') AS new_value_key,
        CASE
            WHEN stat_name_raw IS NOT NULL AND old_value IS NOT NULL AND new_value IS NOT NULL THEN concat_ws(
                ' ',
                NULLIF(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                NULLIF(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                'changed from',
                old_value,
                'to',
                new_value
            )
            ELSE regexp_replace(regexp_replace(cleaned_source_line, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g')
        END AS raw_line_key
    FROM raw_base
), ranked AS (
    SELECT
        id,
        first_value(id) OVER dedup_order AS keep_id,
        row_number() OVER dedup_order AS rn,
        count(*) OVER dedup_group AS group_size
    FROM keyed
    WINDOW
        dedup_group AS (
            PARTITION BY patch_title_key, entity_type_key, entity_name_key, ability_name_key, stat_name_key, old_value_key, new_value_key, raw_line_key
        ),
        dedup_order AS (
            dedup_group ORDER BY confidence DESC NULLS LAST, has_enrichment DESC, id ASC
        )
)
SELECT id AS delete_id, keep_id
FROM ranked
WHERE group_size > 1
  AND rn > 1;

CREATE INDEX ON _patch_event_dedup_candidates(delete_id);
CREATE INDEX ON _patch_event_dedup_candidates(keep_id);

CREATE TEMP TABLE _patch_event_dedup_stats AS
WITH raw_base AS (
    SELECT
        pe.id,
        pe.confidence,
        pee.patch_event_id IS NOT NULL AS has_enrichment,
        lower(regexp_replace(TRIM(BOTH FROM coalesce(pe.patch_title, '')), '\s+', ' ', 'g')) AS patch_title_key,
        pe.entity_type,
        pe.entity_name,
        CASE
            WHEN pee.ability_name IS NULL THEN NULL::text
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM pee.ability_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM pee.ability_name), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM pee.ability_name)
        END AS ability_name_raw,
        CASE
            WHEN pee.stat_name IS NULL THEN NULL::text
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM pee.stat_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM pee.stat_name), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM pee.stat_name)
        END AS stat_name_raw,
        COALESCE(NULLIF(TRIM(BOTH FROM pee.old_value), ''), NULLIF(TRIM(BOTH FROM pe.old_value), '')) AS old_value,
        COALESCE(NULLIF(TRIM(BOTH FROM pee.new_value), ''), NULLIF(TRIM(BOTH FROM pe.new_value), '')) AS new_value,
        CASE
            WHEN pe.entity_name IS NOT NULL
                 AND lower(left(TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN TRIM(BOTH FROM substr(TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 2))
            ELSE TRIM(BOTH FROM COALESCE(pe.normalized_line, pe.raw_line))
        END AS cleaned_source_line
    FROM brain.patch_events pe
    LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id = pe.id
), keyed AS (
    SELECT
        id,
        confidence,
        has_enrichment,
        patch_title_key,
        coalesce(entity_type, '') AS entity_type_key,
        coalesce(entity_name, '') AS entity_name_key,
        coalesce(NULLIF(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''), '') AS ability_name_key,
        coalesce(NULLIF(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''), '') AS stat_name_key,
        coalesce(NULLIF(TRIM(BOTH FROM old_value), ''), '') AS old_value_key,
        coalesce(NULLIF(TRIM(BOTH FROM new_value), ''), '') AS new_value_key,
        CASE
            WHEN stat_name_raw IS NOT NULL AND old_value IS NOT NULL AND new_value IS NOT NULL THEN concat_ws(
                ' ',
                NULLIF(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                NULLIF(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                'changed from',
                old_value,
                'to',
                new_value
            )
            ELSE regexp_replace(regexp_replace(cleaned_source_line, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g')
        END AS raw_line_key
    FROM raw_base
), ranked AS (
    SELECT
        id,
        row_number() OVER dedup_order AS rn,
        count(*) OVER dedup_group AS group_size
    FROM keyed
    WINDOW
        dedup_group AS (
            PARTITION BY patch_title_key, entity_type_key, entity_name_key, ability_name_key, stat_name_key, old_value_key, new_value_key, raw_line_key
        ),
        dedup_order AS (
            dedup_group ORDER BY confidence DESC NULLS LAST, has_enrichment DESC, id ASC
        )
)
SELECT
    count(*) FILTER (WHERE rn = 1 AND group_size > 1) AS dup_groups,
    count(*) FILTER (WHERE rn > 1) AS delete_candidates,
    (SELECT count(*) FROM brain.patch_events) AS events_before,
    (SELECT count(*) FROM brain.patch_events) - count(*) FILTER (WHERE rn > 1) AS expected_events_after
FROM ranked;

DO $$
DECLARE
    candidate_count bigint;
BEGIN
    SELECT count(*) INTO candidate_count FROM _patch_event_dedup_candidates;
    IF candidate_count > 24000 THEN
        RAISE EXCEPTION 'patch_event dedup abort: % delete candidates exceed safety ceiling 24000', candidate_count;
    END IF;
END $$;

INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'delete_candidates', count(*) FROM _patch_event_dedup_candidates;

WITH deleted AS (
    DELETE FROM brain.patch_event_enrichments ref
    USING _patch_event_dedup_candidates c
    WHERE ref.patch_event_id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'patch_event_enrichments_deleted', count(*) FROM deleted;

WITH mapped AS (
    SELECT
        ref.id,
        c.keep_id,
        ref.relation_type,
        ref.source_name_norm,
        coalesce(ref.target_name_norm, '') AS target_name_norm,
        coalesce(ref.owner_name_norm, '') AS owner_name_norm,
        EXISTS (
            SELECT 1
            FROM brain.entity_lineage keep
            WHERE keep.patch_event_id = c.keep_id
              AND keep.relation_type = ref.relation_type
              AND keep.source_name_norm = ref.source_name_norm
              AND coalesce(keep.target_name_norm, '') = coalesce(ref.target_name_norm, '')
              AND coalesce(keep.owner_name_norm, '') = coalesce(ref.owner_name_norm, '')
        ) AS has_keep_row
    FROM brain.entity_lineage ref
    JOIN _patch_event_dedup_candidates c ON ref.patch_event_id = c.delete_id
), ranked AS (
    SELECT
        id,
        has_keep_row,
        row_number() OVER (
            PARTITION BY keep_id, relation_type, source_name_norm, target_name_norm, owner_name_norm
            ORDER BY id
        ) AS rn
    FROM mapped
), deleted AS (
    DELETE FROM brain.entity_lineage ref
    USING ranked r
    WHERE ref.id = r.id
      AND (r.has_keep_row OR r.rn > 1)
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'entity_lineage_redundant_deleted', count(*) FROM deleted;

WITH updated AS (
    UPDATE brain.entity_lineage ref
    SET patch_event_id = c.keep_id,
        updated_at = now()
    FROM _patch_event_dedup_candidates c
    WHERE ref.patch_event_id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'entity_lineage_repointed', count(*) FROM updated;

WITH updated AS (
    UPDATE brain.knowledge_events ref
    SET patch_event_id = c.keep_id,
        updated_at = now()
    FROM _patch_event_dedup_candidates c
    WHERE ref.patch_event_id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'knowledge_events_repointed', count(*) FROM updated;

WITH updated AS (
    UPDATE brain.legacy_entities ref
    SET first_patch_event_id = c.keep_id,
        updated_at = now()
    FROM _patch_event_dedup_candidates c
    WHERE ref.first_patch_event_id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'legacy_entities_first_repointed', count(*) FROM updated;

WITH updated AS (
    UPDATE brain.legacy_entities ref
    SET last_patch_event_id = c.keep_id,
        updated_at = now()
    FROM _patch_event_dedup_candidates c
    WHERE ref.last_patch_event_id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'legacy_entities_last_repointed', count(*) FROM updated;

WITH deleted AS (
    DELETE FROM brain.patch_events pe
    USING _patch_event_dedup_candidates c
    WHERE pe.id = c.delete_id
    RETURNING 1
)
INSERT INTO _patch_event_dedup_counts(action, rows)
SELECT 'patch_events_deleted', count(*) FROM deleted;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM brain.entity_lineage ref
        LEFT JOIN brain.patch_events pe ON pe.id = ref.patch_event_id
        WHERE ref.patch_event_id IS NOT NULL AND pe.id IS NULL
    ) THEN
        RAISE EXCEPTION 'patch_event dedup abort: orphan in brain.entity_lineage.patch_event_id';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM brain.knowledge_events ref
        LEFT JOIN brain.patch_events pe ON pe.id = ref.patch_event_id
        WHERE ref.patch_event_id IS NOT NULL AND pe.id IS NULL
    ) THEN
        RAISE EXCEPTION 'patch_event dedup abort: orphan in brain.knowledge_events.patch_event_id';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM brain.legacy_entities ref
        LEFT JOIN brain.patch_events pe ON pe.id = ref.first_patch_event_id
        WHERE ref.first_patch_event_id IS NOT NULL AND pe.id IS NULL
    ) THEN
        RAISE EXCEPTION 'patch_event dedup abort: orphan in brain.legacy_entities.first_patch_event_id';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM brain.legacy_entities ref
        LEFT JOIN brain.patch_events pe ON pe.id = ref.last_patch_event_id
        WHERE ref.last_patch_event_id IS NOT NULL AND pe.id IS NULL
    ) THEN
        RAISE EXCEPTION 'patch_event dedup abort: orphan in brain.legacy_entities.last_patch_event_id';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM brain.patch_event_enrichments ref
        LEFT JOIN brain.patch_events pe ON pe.id = ref.patch_event_id
        WHERE ref.patch_event_id IS NOT NULL AND pe.id IS NULL
    ) THEN
        RAISE EXCEPTION 'patch_event dedup abort: orphan in brain.patch_event_enrichments.patch_event_id';
    END IF;
END $$;

DO $$
DECLARE
    diff_count bigint;
BEGIN
    WITH expected(old_value, new_value) AS (
        VALUES ('1.4'::text, '1.6'::text), ('1.6', '1.4'), ('1.4', '1.2'), ('1.2', '1.05')
    ), actual AS (
        SELECT old_value, new_value
        FROM brain.patch_changes
        WHERE entity_name = 'Holliday'
          AND lower(raw_line) LIKE '%powder keg%'
          AND lower(raw_line) LIKE '%spirit scaling%'
          AND (old_value, new_value) IN (SELECT old_value, new_value FROM expected)
    ), diff AS (
        (SELECT old_value, new_value FROM expected EXCEPT SELECT old_value, new_value FROM actual)
        UNION ALL
        (SELECT old_value, new_value FROM actual EXCEPT SELECT old_value, new_value FROM expected)
    )
    SELECT count(*) INTO diff_count
    FROM diff;

    IF diff_count <> 0 OR (
        SELECT count(*)
        FROM brain.patch_changes
        WHERE entity_name = 'Holliday'
          AND lower(raw_line) LIKE '%powder keg%'
          AND lower(raw_line) LIKE '%spirit scaling%'
          AND (old_value, new_value) IN (
              VALUES ('1.4'::text, '1.6'::text), ('1.6', '1.4'), ('1.4', '1.2'), ('1.2', '1.05')
          )
    ) <> 4 THEN
        RAISE EXCEPTION 'patch_event dedup abort: Holliday Powder Keg spirit scaling regression';
    END IF;
END $$;

SELECT 'V2_PRE' AS section, dup_groups, delete_candidates, events_before, expected_events_after
FROM _patch_event_dedup_stats;

SELECT 'REPOINT' AS section, action, rows
FROM _patch_event_dedup_counts
ORDER BY action;

COMMIT;
