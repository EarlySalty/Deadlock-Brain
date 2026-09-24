
CREATE OR REPLACE VIEW brain.patch_changes AS
WITH patch_catalog AS (
    SELECT
        lower(regexp_replace(trim(title), '\s+', ' ', 'g')) AS patch_title_key,
        url AS patch_url,
        min(posted_at)::date AS patch_date
    FROM patchnotes.changelog_posts
    WHERE title IS NOT NULL OR url IS NOT NULL
    GROUP BY 1, 2
), raw_base AS (
    SELECT
        pe.id,
        lower(regexp_replace(trim(pe.patch_title), '\s+', ' ', 'g')) AS patch_title_key,
        trim(pe.patch_title) AS patch_title,
        pe.patch_url,
        pe.posted_at,
        pe.created_at,
        pe.entity_type,
        pe.entity_name,
        CASE
            WHEN pee.ability_name IS NULL THEN NULL
            WHEN lower(left(trim(pee.ability_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(pee.ability_name), length(pe.entity_name) + 2))
            ELSE trim(pee.ability_name)
        END AS ability_name_raw,
        CASE
            WHEN pee.stat_name IS NULL THEN NULL
            WHEN lower(left(trim(pee.stat_name), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(pee.stat_name), length(pe.entity_name) + 2))
            ELSE trim(pee.stat_name)
        END AS stat_name_raw,
        coalesce(nullif(trim(pee.old_value), ''), nullif(trim(pe.old_value), '')) AS old_value,
        coalesce(nullif(trim(pee.new_value), ''), nullif(trim(pe.new_value), '')) AS new_value,
        pe.change_type,
        CASE
            WHEN lower(left(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':')
                THEN trim(substr(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 2))
            ELSE trim(coalesce(pe.normalized_line, pe.raw_line))
        END AS cleaned_source_line,
        lower(left(trim(coalesce(pe.normalized_line, pe.raw_line)), length(pe.entity_name) + 1)) = lower(pe.entity_name || ':') AS hero_prefixed,
        greatest(coalesce(pe.confidence, 0), coalesce(pee.confidence, 0)) AS confidence
    FROM brain.patch_events pe
    LEFT JOIN brain.patch_event_enrichments pee ON pee.patch_event_id = pe.id
), base AS (
    SELECT
        id,
        patch_title_key,
        patch_title,
        patch_url,
        posted_at,
        created_at,
        entity_type,
        entity_name,
        nullif(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), '') AS ability_name,
        nullif(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), '') AS stat_name,
        old_value,
        new_value,
        change_type,
        CASE
            WHEN stat_name_raw IS NOT NULL AND old_value IS NOT NULL AND new_value IS NOT NULL THEN
                concat_ws(
                    ' ',
                    nullif(regexp_replace(regexp_replace(ability_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                    nullif(regexp_replace(regexp_replace(stat_name_raw, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g'), ''),
                    'changed from',
                    old_value,
                    'to',
                    new_value
                )
            ELSE regexp_replace(regexp_replace(cleaned_source_line, '(?i)spirits\s+caling', 'spirit scaling', 'g'), '\s+', ' ', 'g')
        END AS raw_line,
        hero_prefixed,
        confidence
    FROM raw_base
), patch_groups AS (
    SELECT
        patch_title_key,
        (array_agg(patch_title ORDER BY (posted_at IS NULL), posted_at, id))[1] AS patch_title,
        (array_agg(patch_url ORDER BY (patch_url IS NULL), (posted_at IS NULL), posted_at, id))[1] AS patch_url,
        min(posted_at)::date AS posted_at_date,
        min(created_at)::date AS created_at_date
    FROM base
    GROUP BY patch_title_key
), patch_dates AS (
    SELECT
        pg.patch_title_key,
        pg.patch_title,
        coalesce(
            pg.posted_at_date,
            (SELECT min(pc.patch_date) FROM patch_catalog pc WHERE pc.patch_url = pg.patch_url),
            (SELECT min(pc.patch_date) FROM patch_catalog pc WHERE pc.patch_title_key = pg.patch_title_key),
            to_date(substring(pg.patch_title FROM '(\d{2}-\d{2}-\d{4})'), 'MM-DD-YYYY'),
            pg.created_at_date
        ) AS patch_date,
        pg.patch_url
    FROM patch_groups pg
), deduped AS (
    SELECT DISTINCT ON (
        b.patch_title_key,
        b.entity_type,
        b.entity_name,
        coalesce(b.ability_name, ''),
        coalesce(b.stat_name, ''),
        coalesce(b.old_value, ''),
        coalesce(b.new_value, ''),
        b.raw_line
    )
        pd.patch_title,
        pd.patch_date,
        b.entity_type,
        b.entity_name,
        b.ability_name,
        b.stat_name,
        b.old_value,
        b.new_value,
        b.change_type,
        b.raw_line,
        coalesce(b.patch_url, pd.patch_url) AS patch_url,
        b.confidence
    FROM base b
    JOIN patch_dates pd ON pd.patch_title_key = b.patch_title_key
    ORDER BY
        b.patch_title_key,
        b.entity_type,
        b.entity_name,
        coalesce(b.ability_name, ''),
        coalesce(b.stat_name, ''),
        coalesce(b.old_value, ''),
        coalesce(b.new_value, ''),
        b.raw_line,
        b.confidence DESC,
        ((b.ability_name IS NOT NULL)::int + (b.stat_name IS NOT NULL)::int) DESC,
        b.hero_prefixed DESC,
        (b.posted_at IS NULL),
        b.posted_at,
        b.id
), parsed AS (
    SELECT
        d.*,
        CASE
            WHEN d.old_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
             AND d.new_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
                THEN regexp_replace(d.old_value, '[^0-9.+-]', '', 'g')::numeric
        END AS old_number,
        CASE
            WHEN d.old_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
             AND d.new_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
                THEN regexp_replace(d.new_value, '[^0-9.+-]', '', 'g')::numeric
        END AS new_number
    FROM deduped d
)
SELECT
    patch_title,
    patch_date,
    entity_type,
    entity_name,
    ability_name,
    stat_name,
    old_value,
    new_value,
    change_type,
    CASE
        WHEN old_number IS NULL OR new_number IS NULL OR old_number = new_number THEN NULL
        WHEN new_number > old_number THEN 'increase'
        ELSE 'decrease'
    END AS numeric_direction,
    raw_line,
    patch_url,
    confidence
FROM parsed
