\set ON_ERROR_STOP on

BEGIN;

CREATE TEMP TABLE _ingest_change_type_recomputed ON COMMIT DROP AS
WITH
-- Sync with rust/crates/dbrain-normalize/src/patch.rs POSITIVE_STATS.
positive_stats(stat) AS (
    VALUES
        ('damage'),
        ('dps'),
        ('health'),
        ('regen'),
        ('resistance'),
        ('resist'),
        ('range'),
        ('radius'),
        ('speed'),
        ('sprint'),
        ('fire rate'),
        ('spirit power'),
        ('lifesteal'),
        ('duration'),
        ('stamina'),
        ('ammo'),
        ('barrier'),
        ('heal'),
        ('healing'),
        ('scaling'),
        ('souls'),
        ('bounty')
),
-- Sync with rust/crates/dbrain-normalize/src/patch.rs NEGATIVE_STATS.
negative_stats(stat) AS (
    VALUES
        ('cooldown'),
        ('recharge'),
        ('delay'),
        ('cost'),
        ('falloff')
),
numeric_events AS (
    SELECT
        id,
        change_type AS old_change_type,
        lower(coalesce(normalized_line, raw_line, '')) AS lower_line,
        regexp_replace(old_value, '[^0-9.+-]', '', 'g')::numeric AS old_number,
        regexp_replace(new_value, '[^0-9.+-]', '', 'g')::numeric AS new_number
    FROM brain.patch_events
    WHERE change_type IN ('buff', 'nerf')
      AND old_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
      AND new_value ~ '^\s*"?[+-]?(?:\d+(?:\.\d+)?|\.\d+)\s*[[:alpha:]%/]*"?\s*$'
),
classified AS (
    SELECT
        numeric_events.*,
        EXISTS (
            SELECT 1
            FROM positive_stats
            WHERE lower_line LIKE '%' || stat || '%'
        ) AS mentions_positive_stat,
        EXISTS (
            SELECT 1
            FROM negative_stats
            WHERE lower_line LIKE '%' || stat || '%'
        ) AS mentions_negative_stat
    FROM numeric_events
),
recomputed AS (
    SELECT
        id,
        old_change_type,
        old_number,
        new_number,
        CASE
            WHEN new_number > old_number
                THEN CASE WHEN mentions_negative_stat AND NOT mentions_positive_stat THEN 'nerf' ELSE 'buff' END
            WHEN new_number < old_number
                THEN CASE WHEN mentions_negative_stat AND NOT mentions_positive_stat THEN 'buff' ELSE 'nerf' END
        END AS new_change_type
    FROM classified
    WHERE old_number <> new_number
)
SELECT id, old_change_type, new_change_type
FROM recomputed
WHERE new_change_type IS NOT NULL;

CREATE TEMP TABLE _ingest_change_type_candidates ON COMMIT DROP AS
SELECT id, old_change_type, new_change_type
FROM _ingest_change_type_recomputed
WHERE old_change_type <> new_change_type;

CREATE TEMP TABLE _ingest_change_type_counts(metric text, detail text, value bigint) ON COMMIT DROP;

INSERT INTO _ingest_change_type_counts(metric, detail, value)
SELECT 'change_type_recomputed_before', old_change_type || '->' || new_change_type, count(*)
FROM _ingest_change_type_recomputed
WHERE new_change_type IS NOT NULL
GROUP BY old_change_type, new_change_type;

INSERT INTO _ingest_change_type_counts(metric, detail, value)
SELECT 'change_type_update_candidates_before', old_change_type || '->' || new_change_type, count(*)
FROM _ingest_change_type_candidates
GROUP BY old_change_type, new_change_type;

DO $$
DECLARE
    expected bigint;
    actual bigint;
    remaining bigint;
BEGIN
    SELECT count(*) INTO expected FROM _ingest_change_type_candidates;
    IF expected > 5000 THEN
        RAISE EXCEPTION 'change_type candidate count % exceeds sanity ceiling 5000', expected;
    END IF;

    WITH updated AS (
        UPDATE brain.patch_events pe
        SET change_type = c.new_change_type
        FROM _ingest_change_type_candidates c
        WHERE pe.id = c.id
          AND pe.change_type = c.old_change_type
        RETURNING 1
    )
    SELECT count(*) INTO actual FROM updated;

    IF actual <> expected THEN
        RAISE EXCEPTION 'change_type updated %, expected %', actual, expected;
    END IF;

    SELECT count(*) INTO remaining
    FROM _ingest_change_type_recomputed c
    JOIN brain.patch_events pe ON pe.id = c.id
    WHERE pe.change_type <> c.new_change_type;

    IF remaining <> 0 THEN
        RAISE EXCEPTION 'change_type remaining contradictions after update: %', remaining;
    END IF;

    INSERT INTO _ingest_change_type_counts(metric, detail, value)
    VALUES
        ('change_type_updated', NULL, actual),
        ('change_type_remaining_after', NULL, remaining);
END $$;

SELECT metric, detail, value
FROM _ingest_change_type_counts
ORDER BY metric, detail NULLS FIRST;

COMMIT;

BEGIN;

CREATE TEMP TABLE _ingest_posted_at_candidates ON COMMIT DROP AS
WITH null_events AS (
    SELECT
        id,
        patch_external_id,
        patch_title,
        patch_url,
        lower(trim(patch_title)) AS patch_title_key,
        substring(patch_title FROM '(\d{2}-\d{2}-\d{4})') AS title_date
    FROM brain.patch_events
    WHERE posted_at IS NULL
),
peer_external AS (
    SELECT patch_external_id, min(posted_at) AS posted_at
    FROM brain.patch_events
    WHERE posted_at IS NOT NULL
    GROUP BY patch_external_id
),
peer_title AS (
    SELECT lower(trim(patch_title)) AS patch_title_key, min(posted_at) AS posted_at
    FROM brain.patch_events
    WHERE posted_at IS NOT NULL
      AND patch_title IS NOT NULL
    GROUP BY 1
),
changelog_url AS (
    SELECT url, min(posted_at) AS posted_at
    FROM patchnotes.changelog_posts
    WHERE posted_at IS NOT NULL
      AND url IS NOT NULL
    GROUP BY url
),
changelog_title AS (
    SELECT lower(trim(title)) AS patch_title_key, min(posted_at) AS posted_at
    FROM patchnotes.changelog_posts
    WHERE posted_at IS NOT NULL
      AND title IS NOT NULL
    GROUP BY 1
),
resolved AS (
    SELECT
        n.id,
        COALESCE(
            pe.posted_at,
            pt.posted_at,
            cu.posted_at,
            ct.posted_at,
            CASE
                WHEN n.title_date ~ '^(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])-\d{4}$'
                THEN to_date(n.title_date, 'MM-DD-YYYY')::timestamptz
            END
        ) AS new_posted_at
    FROM null_events n
    LEFT JOIN peer_external pe ON pe.patch_external_id IS NOT DISTINCT FROM n.patch_external_id
    LEFT JOIN peer_title pt ON pt.patch_title_key = n.patch_title_key
    LEFT JOIN changelog_url cu ON cu.url = n.patch_url
    LEFT JOIN changelog_title ct ON ct.patch_title_key = n.patch_title_key
)
SELECT id, new_posted_at
FROM resolved
WHERE new_posted_at IS NOT NULL;

CREATE TEMP TABLE _ingest_posted_at_counts(metric text, detail text, value bigint) ON COMMIT DROP;

INSERT INTO _ingest_posted_at_counts(metric, detail, value)
VALUES
    ('posted_at_null_before', NULL, (SELECT count(*) FROM brain.patch_events WHERE posted_at IS NULL)),
    ('posted_at_fillable_before', NULL, (SELECT count(*) FROM _ingest_posted_at_candidates));

DO $$
DECLARE
    expected bigint;
    actual bigint;
    null_before bigint;
    null_after bigint;
BEGIN
    SELECT count(*) INTO expected FROM _ingest_posted_at_candidates;
    SELECT count(*) INTO null_before FROM brain.patch_events WHERE posted_at IS NULL;
    IF expected > null_before THEN
        RAISE EXCEPTION 'posted_at candidates % exceed NULL rows %', expected, null_before;
    END IF;
    IF expected > 8000 THEN
        RAISE EXCEPTION 'posted_at candidate count % exceeds sanity ceiling 8000', expected;
    END IF;

    WITH updated AS (
        UPDATE brain.patch_events pe
        SET posted_at = c.new_posted_at
        FROM _ingest_posted_at_candidates c
        WHERE pe.id = c.id
          AND pe.posted_at IS NULL
        RETURNING 1
    )
    SELECT count(*) INTO actual FROM updated;

    IF actual <> expected THEN
        RAISE EXCEPTION 'posted_at updated %, expected %', actual, expected;
    END IF;

    SELECT count(*) INTO null_after FROM brain.patch_events WHERE posted_at IS NULL;

    INSERT INTO _ingest_posted_at_counts(metric, detail, value)
    VALUES
        ('posted_at_updated', NULL, actual),
        ('posted_at_null_after', NULL, null_after);
END $$;

SELECT metric, detail, value
FROM _ingest_posted_at_counts
ORDER BY metric, detail NULLS FIRST;

COMMIT;
