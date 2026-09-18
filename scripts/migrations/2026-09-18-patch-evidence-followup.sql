-- Folgemigration zu 2026-09-18-patch-evidence.sql.
-- Aendert die urspruengliche Migration nicht, damit ein bereits ausgerollter
-- Stand kompatibel bleibt. Sie ersetzt nur Views, Funktionen und die
-- Triggerlogik (CREATE OR REPLACE) und ist wiederholbar. Sie loescht oder
-- ueberschreibt keine Quellrevisionen; vorhandene abgeleitete
-- Knowledge-Event-Zeitwerte werden bewusst ueber den Trigger neu berechnet.
--
-- Korrektur 1: Die frueheste belegte Beobachtung desselben Ereignisinhalts
--   bleibt erhalten (unveraenderter Reimport, Loeschen/Reimport, Rueckkehr zu
--   einem frueheren Inhalt). Inhaltsaenderungen erhalten eine eigene
--   Erstbeobachtung. baseline und earlier_observation_unknown bleiben erhalten.
--   View und Materialisierung liefern eine konsistente first_observed_at-Semantik.
-- Korrektur 2: youtube_caption_segments_v1 prueft zusaetzlich den SHA-256 des
--   aktuellen transcript_text gegen e.text_sha256; anderer Text liefert keine
--   alten Zeitsegmente, gleicher Text mit neuer Zeit bleibt neue Evidenzfassung.
-- Korrektur 3: Patchidentitaet konsistent aufloesen. Ereignisse liegen unter der
--   Quellen-URL, Quellzeilen unter der internen ID. capture_patch_evidence
--   sperrt und invalidiert Entwuerfe jetzt ueber einen kanonischen Reviewschluessel
--   (patch_<changelog_id>, aufgeloest ueber die URL). Mehrdeutige Zuordnungen
--   bleiben sichtbar (roher Schluessel) statt nach Titel geraten zu werden.
BEGIN;
SET LOCAL lock_timeout = '5s';
SET LOCAL statement_timeout = '60s';
SET LOCAL TimeZone = 'UTC';

-- Patchhistorie: first_observed_at ist die frueheste PRESENT-Beobachtung des
-- gleichen content_hash. earlier_observation_unknown folgt der ERSTEN
-- Beobachtung dieses Inhalts, nicht der aktuellen Revision. Damit bleibt die
-- Baseline-Unsicherheit auch nach einem Reimport erhalten. Die Revisionszeit
-- (observed_at) bleibt separat. first_observed_at liegt nie nach observed_at,
-- weil das Minimum ueber PRESENT-Revisionen die aktuelle Revision einschliesst
-- oder eine fruehere PRESENT-Revision desselben Inhalts ist; ein spaeterer
-- known-at-Treffer benutzt also keine Information aus einer spaeteren Revision.
CREATE OR REPLACE VIEW brain.patch_history_v1 AS
WITH present_first AS (
    SELECT source_table, source_key, content_hash,
           min(observed_at) AS first_observed_at,
           (array_agg(observation_kind ORDER BY revision_id ASC))[1] AS first_observation_kind
    FROM brain.patch_evidence_revisions
    WHERE state = 'present'
    GROUP BY source_table, source_key, content_hash
)
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
       COALESCE(pf.first_observation_kind = 'baseline', r.observation_kind = 'baseline') AS earlier_observation_unknown,
       pf.first_observed_at
FROM brain.patch_evidence_revisions r
LEFT JOIN present_first pf
    ON pf.source_table = r.source_table
   AND pf.source_key = r.source_key
   AND pf.content_hash = r.content_hash
WHERE r.source_table = 'patch_events';

-- Untertitelsegmente: nur ausliefern, wenn der aktuelle Transkript-Text zur
-- Evidenzfassung passt. Der Zeiger v.metadata->>'transcript_evidence_hash'
-- waehlt die aktuelle Rohfassung; der Text-SHA-Abgleich verhindert, dass ein
-- geaenderter Text weiterhin alte Zeitsegmente liefert.
CREATE OR REPLACE VIEW brain.youtube_caption_segments_v1 AS
SELECT e.video_id, e.raw_sha256, e.text_sha256, e.language, e.source_kind,
       e.observed_at, (part->>'source_event_index')::bigint AS source_event_index,
       (part->>'start_ms')::bigint AS start_ms,
       (part->>'duration_ms')::bigint AS duration_ms,
       (part->>'end_ms')::bigint AS end_ms,
       part->>'text' AS segment_text, part->'pieces' AS pieces
FROM brain.youtube_transcript_evidence e
JOIN brain.youtube_transcripts t ON t.video_id = e.video_id AND t.source_kind = e.source_kind
    AND encode(sha256(convert_to(t.transcript_text, 'UTF8')), 'hex') = e.text_sha256
JOIN brain.youtube_videos v ON v.video_id = e.video_id
    AND v.metadata->>'transcript_evidence_hash' = e.raw_sha256
CROSS JOIN LATERAL jsonb_array_elements(e.segments) part;

-- knowledge_events erhaelt observed_at = frueheste Beobachtung des aktuellen
-- Inhalts (nicht die zuletzt protokollierte Revision). occurred_at bleibt das
-- Quelldatum. earlier_observation_unknown folgt der Erstbeobachtung.
CREATE OR REPLACE FUNCTION brain.correct_patch_knowledge_times()
RETURNS trigger LANGUAGE plpgsql SECURITY DEFINER
SET search_path = pg_catalog
AS $function$
DECLARE
    current_content record;
    first_observation record;
    source_key_text text;
BEGIN
    IF TG_TABLE_SCHEMA <> 'brain' OR TG_TABLE_NAME <> 'knowledge_events' THEN
        RAISE EXCEPTION 'Unexpected knowledge time trigger source';
    END IF;
    IF NEW.event_source = 'patch_event' THEN
        source_key_text := substring(NEW.event_hash FROM 7);
        SELECT r.content_hash, r.source_published_at INTO current_content
        FROM brain.patch_evidence_revisions r
        WHERE r.source_table = 'patch_events' AND r.source_key = source_key_text
          AND r.state = 'present'
        ORDER BY r.revision_id DESC LIMIT 1;
        IF FOUND THEN
            SELECT min(r.observed_at) AS first_observed_at,
                   (array_agg(r.observation_kind ORDER BY r.revision_id ASC))[1] AS first_observation_kind
            INTO first_observation
            FROM brain.patch_evidence_revisions r
            WHERE r.source_table = 'patch_events' AND r.source_key = source_key_text
              AND r.state = 'present' AND r.content_hash = current_content.content_hash;
            NEW.observed_at := first_observation.first_observed_at;
            NEW.occurred_at := current_content.source_published_at;
            NEW.effective_from := NULL;
            NEW.metadata := COALESCE(NEW.metadata, '{}'::jsonb) || jsonb_build_object(
                'observed_at_semantics', 'first_observation_of_current_content',
                'occurred_at_semantics', 'source_publication_not_confirmed_game_rollout',
                'earlier_observation_unknown', first_observation.first_observation_kind = 'baseline'
            );
        END IF;
    END IF;
    RETURN NEW;
END
$function$;
REVOKE ALL ON FUNCTION brain.correct_patch_knowledge_times() FROM PUBLIC;

-- Bestehende patch_event-Materialisierungen mit der korrigierten Semantik neu
-- berechnen (feuert den BEFORE-UPDATE-Trigger, idempotent).
UPDATE brain.knowledge_events SET metadata = metadata WHERE event_source = 'patch_event';

-- Kanonischer Reviewschluessel: Quellzeilen tragen die interne ID, Ereignisse die
-- Quellen-URL. Beide muessen auf denselben Review zeigen. Die URL wird auf genau
-- eine changelog-ID aufgeloest; bei 0 oder mehreren Treffern bleibt der rohe
-- Schluessel erhalten, damit eine mehrdeutige Zuordnung sichtbar bleibt.
CREATE OR REPLACE FUNCTION brain.patch_review_canonical_key(p_source_table text, p_payload jsonb)
RETURNS text LANGUAGE plpgsql STABLE
SET search_path = pg_catalog
AS $function$
DECLARE
    match_count integer;
    resolved text;
BEGIN
    IF p_source_table = 'changelog_posts' THEN
        RETURN 'patch_' || (p_payload->>'id');
    END IF;
    SELECT count(*), min('patch_' || c.id::text)
      INTO match_count, resolved
      FROM patchnotes.changelog_posts c
      WHERE c.url = p_payload->>'patch_external_id';
    IF match_count = 1 THEN
        RETURN resolved;
    END IF;
    RETURN p_payload->>'patch_external_id';
END
$function$;
REVOKE ALL ON FUNCTION brain.patch_review_canonical_key(text, jsonb) FROM PUBLIC;

-- capture_patch_evidence neu: identische Revisionslogik, aber Sperre und
-- Entwurfs-Invalidierung ueber den kanonischen Reviewschluessel.
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
    canonical_key := brain.patch_review_canonical_key(TG_TABLE_NAME, row_payload);
    PERFORM pg_advisory_xact_lock(hashtextextended('brain.patch_review:' || canonical_key, 0));
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
            AND patch_external_id = canonical_key;
    END IF;
    IF TG_OP = 'DELETE' THEN RETURN OLD; END IF;
    RETURN NEW;
END
$function$;
REVOKE ALL ON FUNCTION brain.capture_patch_evidence() FROM PUBLIC;

REVOKE ALL ON brain.patch_history_v1, brain.youtube_caption_segments_v1 FROM PUBLIC;
COMMIT;
