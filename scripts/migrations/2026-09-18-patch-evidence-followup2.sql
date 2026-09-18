-- Dritte Migration zur Patch-Evidenz (R4). Aendert die erste und zweite
-- Migration nicht; ein bereits ausgerollter Stand bleibt kompatibel. Sie
-- ersetzt nur brain.capture_patch_evidence (CREATE OR REPLACE) und ist
-- wiederholbar. Setzt patch_review_canonical_key aus der zweiten Migration voraus.
--
-- R4: Wechselt ein bestehendes Event seine patch_external_id von Quelle A nach B
-- (gleicher event_hash), musste bisher nur der neue kanonische Reviewschluessel
-- gesperrt/invalidiert werden. Jetzt werden bei UPDATE beide Zugehoerigkeiten
-- (OLD und NEW) behandelt: beide Reviews werden needs_revalidation, die Sperren
-- sind deterministisch geordnet, und die Event-ID wird nicht umgeschrieben.
BEGIN;
SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '60s';
SET LOCAL TimeZone = 'UTC';

CREATE OR REPLACE FUNCTION brain.capture_patch_evidence()
RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog
SET TimeZone = 'UTC'
AS $function$
DECLARE
    row_payload jsonb;
    key_text text;
    hash_text text;
    new_state text;
    previous_hash text;
    previous_state text;
    canonical_keys text[];
    canonical_key text;
BEGIN
    IF NOT ((TG_TABLE_SCHEMA = 'brain' AND TG_TABLE_NAME = 'patch_events') OR (TG_TABLE_SCHEMA = 'patchnotes' AND TG_TABLE_NAME = 'changelog_posts')) THEN
        RAISE EXCEPTION 'Unexpected patch evidence trigger source';
    END IF;
    IF TG_OP = 'DELETE' THEN
        row_payload := to_jsonb(OLD);
        new_state := 'deleted';
    ELSE
        row_payload := to_jsonb(NEW);
        new_state := 'present';
    END IF;
    key_text := CASE WHEN TG_TABLE_NAME = 'patch_events'
        THEN row_payload->>'event_hash' ELSE row_payload->>'id' END;
    IF key_text IS NULL OR key_text = '' THEN
        RAISE EXCEPTION 'Patch evidence requires a stable source key';
    END IF;
    -- Alte und neue Zugehoerigkeit sammeln: bei UPDATE kann ein Event seine
    -- Quelle wechseln; beide kanonischen Reviewschluessel muessen behandelt werden.
    IF TG_OP = 'UPDATE' THEN
        canonical_keys := ARRAY[
            brain.patch_review_canonical_key(TG_TABLE_NAME, to_jsonb(NEW)),
            brain.patch_review_canonical_key(TG_TABLE_NAME, to_jsonb(OLD))
        ];
    ELSE
        canonical_keys := ARRAY[brain.patch_review_canonical_key(TG_TABLE_NAME, row_payload)];
    END IF;
    -- Deterministisch ordnen und deduplizieren, damit die Sperrreihenfolge stabil
    -- ist und keine Deadlocks zwischen gleichzeitigen Quellen entstehen.
    SELECT array_agg(k ORDER BY k) INTO canonical_keys
      FROM (SELECT DISTINCT unnest(canonical_keys) AS k) d
      WHERE k IS NOT NULL AND k <> '';
    IF canonical_keys IS NOT NULL THEN
        FOREACH canonical_key IN ARRAY canonical_keys LOOP
            PERFORM pg_advisory_xact_lock(hashtextextended('brain.patch_review:' || canonical_key, 0));
        END LOOP;
    END IF;
    PERFORM pg_advisory_xact_lock(hashtextextended('brain.patch_evidence:' || TG_TABLE_NAME || ':' || key_text, 0));
    hash_text := brain.patch_evidence_payload_hash(row_payload);
    SELECT content_hash, state INTO previous_hash, previous_state
      FROM brain.patch_evidence_revisions
      WHERE source_table = TG_TABLE_NAME AND source_key = key_text
      ORDER BY revision_id DESC LIMIT 1;
    IF previous_hash IS DISTINCT FROM hash_text OR previous_state IS DISTINCT FROM new_state THEN
        INSERT INTO brain.patch_evidence_revisions(
            source_table, source_key, content_hash, state, observation_kind,
            source_published_at, payload
        ) VALUES (
            TG_TABLE_NAME, key_text, hash_text, new_state, 'live',
            brain.patch_evidence_timestamp(row_payload->>'posted_at'), row_payload
        );
        IF canonical_keys IS NOT NULL THEN
            UPDATE brain.patch_review_runs SET status = 'needs_revalidation'
              WHERE status = 'draft'
                AND patch_external_id = ANY(canonical_keys);
        END IF;
    END IF;
    IF TG_OP = 'DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END
$function$;
REVOKE ALL ON FUNCTION brain.capture_patch_evidence() FROM PUBLIC;
COMMIT;
