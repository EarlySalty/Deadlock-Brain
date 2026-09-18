BEGIN;
SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '60s';
SET LOCAL TimeZone = 'UTC';

CREATE TABLE IF NOT EXISTS brain.patch_evidence_revisions (
    revision_id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_table text NOT NULL CHECK (source_table IN ('changelog_posts', 'patch_events')),
    source_key text NOT NULL,
    content_hash text NOT NULL,
    state text NOT NULL CHECK (state IN ('present', 'deleted')),
    observation_kind text NOT NULL CHECK (observation_kind IN ('baseline', 'live')),
    observed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    source_published_at timestamptz,
    payload jsonb NOT NULL,
    UNIQUE (source_table, source_key, revision_id)
);
CREATE INDEX IF NOT EXISTS patch_evidence_key_revision_idx
    ON brain.patch_evidence_revisions (source_table, source_key, revision_id DESC);
CREATE INDEX IF NOT EXISTS patch_evidence_patch_revision_idx
    ON brain.patch_evidence_revisions (source_table, (payload->>'patch_external_id'), revision_id DESC);
CREATE INDEX IF NOT EXISTS patch_evidence_published_idx
    ON brain.patch_evidence_revisions (source_published_at, revision_id);

CREATE TABLE IF NOT EXISTS brain.youtube_transcript_evidence (
    video_id text NOT NULL REFERENCES brain.youtube_videos(video_id) ON DELETE CASCADE,
    raw_sha256 text NOT NULL,
    text_sha256 text NOT NULL,
    language text NOT NULL,
    source_kind text NOT NULL,
    raw_caption_json text NOT NULL,
    segments jsonb NOT NULL CHECK (jsonb_typeof(segments) = 'array'),
    observed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (video_id, source_kind, raw_sha256)
);

CREATE TABLE IF NOT EXISTS brain.patch_review_runs (
    run_id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    patch_external_id text NOT NULL,
    context_sha256 text NOT NULL,
    prompt_version text NOT NULL,
    model text NOT NULL,
    status text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'needs_revalidation', 'rejected')),
    context jsonb NOT NULL,
    report jsonb NOT NULL,
    generated_at timestamptz NOT NULL DEFAULT clock_timestamp()
);
CREATE INDEX IF NOT EXISTS patch_review_patch_idx
    ON brain.patch_review_runs (patch_external_id, run_id DESC);

CREATE OR REPLACE FUNCTION brain.patch_evidence_payload_hash(row_payload jsonb)
RETURNS text LANGUAGE sql IMMUTABLE STRICT
SET search_path = pg_catalog
AS $function$
    SELECT encode(sha256(convert_to((row_payload - ARRAY[
        'id', 'created_at', 'updated_at', 'fetched_at', 'patch_snapshot_id',
        'legacy_patch_snapshot_id', 'metadata', 'event_hash'
    ])::text, 'UTF8')), 'hex')
$function$;

CREATE OR REPLACE FUNCTION brain.patch_evidence_timestamp(value text)
RETURNS timestamptz LANGUAGE plpgsql STABLE
SET search_path = pg_catalog
SET TimeZone = 'UTC'
AS $function$
BEGIN
    RETURN NULLIF(value, '')::timestamptz;
EXCEPTION WHEN invalid_datetime_format OR datetime_field_overflow THEN
    RETURN NULL;
END
$function$;

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
    PERFORM pg_advisory_xact_lock(hashtextextended('brain.patch_review:' || CASE WHEN TG_TABLE_NAME='patch_events' THEN row_payload->>'patch_external_id' ELSE 'patch_' || key_text END, 0));
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
        UPDATE brain.patch_review_runs SET status = 'needs_revalidation'
          WHERE status = 'draft'
            AND patch_external_id = CASE WHEN TG_TABLE_NAME = 'patch_events'
                THEN row_payload->>'patch_external_id' ELSE 'patch_' || key_text END;
    END IF;
    IF TG_OP = 'DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END
$function$;
REVOKE ALL ON FUNCTION brain.capture_patch_evidence() FROM PUBLIC;

LOCK TABLE patchnotes.changelog_posts, brain.patch_events IN SHARE ROW EXCLUSIVE MODE;

INSERT INTO brain.patch_evidence_revisions(
    source_table, source_key, content_hash, state, observation_kind, source_published_at, payload
)
SELECT 'changelog_posts', p.id::text, brain.patch_evidence_payload_hash(to_jsonb(p)),
       'present', 'baseline', brain.patch_evidence_timestamp(to_jsonb(p)->>'posted_at'), to_jsonb(p)
FROM patchnotes.changelog_posts p
WHERE NOT EXISTS (
    SELECT 1 FROM brain.patch_evidence_revisions r
    WHERE r.source_table = 'changelog_posts' AND r.source_key = p.id::text
);
INSERT INTO brain.patch_evidence_revisions(
    source_table, source_key, content_hash, state, observation_kind, source_published_at, payload
)
SELECT 'patch_events', p.event_hash, brain.patch_evidence_payload_hash(to_jsonb(p)),
       'present', 'baseline', brain.patch_evidence_timestamp(to_jsonb(p)->>'posted_at'), to_jsonb(p)
FROM brain.patch_events p
WHERE NOT EXISTS (
    SELECT 1 FROM brain.patch_evidence_revisions r
    WHERE r.source_table = 'patch_events' AND r.source_key = p.event_hash
);

DROP TRIGGER IF EXISTS brain_capture_patch_source ON patchnotes.changelog_posts;
CREATE TRIGGER brain_capture_patch_source
AFTER INSERT OR UPDATE OR DELETE ON patchnotes.changelog_posts
FOR EACH ROW EXECUTE FUNCTION brain.capture_patch_evidence();
DROP TRIGGER IF EXISTS brain_capture_patch_event ON brain.patch_events;
CREATE TRIGGER brain_capture_patch_event
AFTER INSERT OR UPDATE OR DELETE ON brain.patch_events
FOR EACH ROW EXECUTE FUNCTION brain.capture_patch_evidence();

CREATE OR REPLACE VIEW brain.patch_history_v1 AS
SELECT r.revision_id, r.source_key AS event_hash, r.content_hash,
       r.source_published_at, r.observed_at, r.observation_kind, r.state,
       r.payload->>'patch_external_id' AS patch_external_id,
       r.payload->>'patch_title' AS patch_title,
       r.payload->>'patch_url' AS source_url,
       r.payload->>'source_kind' AS source_kind,
       r.payload->>'entity_type' AS entity_type,
       r.payload->>'entity_name' AS entity_name,
       r.payload->>'subject' AS subject,
       r.payload->>'change_type' AS change_type,
       r.payload->>'raw_line' AS raw_line,
       r.payload->>'old_value' AS old_value,
       r.payload->>'new_value' AS new_value,
       'source_publication_not_confirmed_game_rollout'::text AS time_semantics,
       (r.observation_kind = 'baseline') AS earlier_observation_unknown
FROM brain.patch_evidence_revisions r
WHERE r.source_table = 'patch_events';

CREATE OR REPLACE VIEW brain.youtube_caption_segments_v1 AS
SELECT e.video_id, e.raw_sha256, e.text_sha256, e.language, e.source_kind,
       e.observed_at, (part->>'source_event_index')::bigint AS source_event_index,
       (part->>'start_ms')::bigint AS start_ms,
       (part->>'duration_ms')::bigint AS duration_ms,
       (part->>'end_ms')::bigint AS end_ms,
       part->>'text' AS segment_text, part->'pieces' AS pieces
FROM brain.youtube_transcript_evidence e
JOIN brain.youtube_transcripts t ON t.video_id=e.video_id AND t.source_kind=e.source_kind
JOIN brain.youtube_videos v ON v.video_id=e.video_id
    AND v.metadata->>'transcript_evidence_hash'=e.raw_sha256
CROSS JOIN LATERAL jsonb_array_elements(e.segments) part;

CREATE OR REPLACE FUNCTION brain.preserve_patch_created_at()
RETURNS trigger LANGUAGE plpgsql
SET search_path = pg_catalog
AS $function$
BEGIN
    NEW.created_at := OLD.created_at;
    RETURN NEW;
END
$function$;
DROP TRIGGER IF EXISTS brain_preserve_patch_created_at ON brain.patch_events;
CREATE TRIGGER brain_preserve_patch_created_at BEFORE UPDATE ON brain.patch_events
FOR EACH ROW EXECUTE FUNCTION brain.preserve_patch_created_at();

CREATE OR REPLACE FUNCTION brain.correct_patch_knowledge_times()
RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog
AS $function$
DECLARE
    observation record;
BEGIN
    IF TG_TABLE_SCHEMA <> 'brain' OR TG_TABLE_NAME <> 'knowledge_events' THEN
        RAISE EXCEPTION 'Unexpected knowledge time trigger source';
    END IF;
    IF NEW.event_source = 'patch_event' THEN
        SELECT r.observed_at, r.observation_kind, r.source_published_at INTO observation
        FROM brain.patch_evidence_revisions r
        WHERE r.source_table='patch_events' AND r.source_key=substring(NEW.event_hash FROM 7)
          AND r.state='present'
        ORDER BY r.revision_id DESC LIMIT 1;
        IF FOUND THEN
            NEW.observed_at := observation.observed_at;
            NEW.occurred_at := observation.source_published_at;
            NEW.effective_from := NULL;
            NEW.metadata := COALESCE(NEW.metadata, '{}'::jsonb) || jsonb_build_object(
                'observed_at_semantics','revision_observation',
                'occurred_at_semantics','source_publication_not_confirmed_game_rollout',
                'earlier_observation_unknown',observation.observation_kind='baseline'
            );
        END IF;
    END IF;
    RETURN NEW;
END
$function$;
REVOKE ALL ON FUNCTION brain.correct_patch_knowledge_times() FROM PUBLIC;
DROP TRIGGER IF EXISTS brain_correct_patch_knowledge_times ON brain.knowledge_events;
CREATE TRIGGER brain_correct_patch_knowledge_times
BEFORE INSERT OR UPDATE ON brain.knowledge_events
FOR EACH ROW EXECUTE FUNCTION brain.correct_patch_knowledge_times();
UPDATE brain.knowledge_events SET metadata=metadata WHERE event_source='patch_event';

REVOKE ALL ON brain.patch_evidence_revisions, brain.youtube_transcript_evidence,
    brain.patch_review_runs, brain.patch_history_v1, brain.youtube_caption_segments_v1 FROM PUBLIC;
COMMIT;
