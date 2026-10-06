BEGIN;
SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '60s';
SET LOCAL TimeZone = 'UTC';

CREATE OR REPLACE FUNCTION brain.patch_evidence_payload_hash(row_payload jsonb)
RETURNS text LANGUAGE sql IMMUTABLE STRICT
SET search_path = pg_catalog
AS $function$
    SELECT encode(sha256(convert_to((row_payload - ARRAY[
        'id', 'created_at', 'updated_at', 'fetched_at', 'metadata', 'event_hash'
    ])::text, 'UTF8')), 'hex')
$function$;

UPDATE brain.patch_review_runs
SET status = 'needs_revalidation'
WHERE status = 'draft';

COMMIT;
