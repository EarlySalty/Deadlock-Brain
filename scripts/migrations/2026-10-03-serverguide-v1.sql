BEGIN;
CREATE TABLE IF NOT EXISTS brain.guide_schema_version (version INTEGER PRIMARY KEY CHECK(version=1));
INSERT INTO brain.guide_schema_version VALUES(1) ON CONFLICT DO NOTHING;
CREATE TABLE IF NOT EXISTS brain.guide_subjects (
 guild_id TEXT NOT NULL, user_id TEXT NOT NULL, epoch BIGINT NOT NULL DEFAULT 0,
 memory_enabled BOOLEAN NOT NULL DEFAULT FALSE, contact_enabled BOOLEAN NOT NULL DEFAULT FALSE,
 globally_opted_out BOOLEAN NOT NULL DEFAULT FALSE, deleted BOOLEAN NOT NULL DEFAULT FALSE,
 profile_json JSONB NOT NULL DEFAULT '{}', history_json JSONB NOT NULL DEFAULT '[]',
 updated_at TIMESTAMPTZ NOT NULL DEFAULT now(), PRIMARY KEY(guild_id,user_id)
);
CREATE TABLE IF NOT EXISTS brain.guide_turn_claims (
 guild_id TEXT NOT NULL, user_id TEXT NOT NULL, request_id TEXT NOT NULL,
 state TEXT NOT NULL CHECK(state IN ('claimed','finished','uncertain')),
 created_at TIMESTAMPTZ NOT NULL DEFAULT now(), PRIMARY KEY(guild_id,user_id,request_id)
);
CREATE TABLE IF NOT EXISTS brain.guide_conversations (
 guild_id TEXT NOT NULL,user_id TEXT NOT NULL,conversation_id TEXT NOT NULL,
 surface TEXT NOT NULL CHECK(surface IN ('dm','public')), state_json JSONB NOT NULL,
 expires_at BIGINT NOT NULL, PRIMARY KEY(guild_id,user_id,conversation_id)
);
CREATE TABLE IF NOT EXISTS brain.guide_feedback_outbox (
 guild_id TEXT NOT NULL,user_id TEXT NOT NULL,delivery_id TEXT NOT NULL,
 destination_channel_id TEXT NOT NULL,text TEXT,
 state TEXT NOT NULL CHECK(state IN ('pending','sent','failed')),
 discord_message_id TEXT,created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 PRIMARY KEY(guild_id,user_id,delivery_id)
);
CREATE TABLE IF NOT EXISTS brain.guide_legacy_imports (
 guild_id TEXT NOT NULL,user_id TEXT NOT NULL,imported_at TIMESTAMPTZ NOT NULL DEFAULT now(),
 PRIMARY KEY(guild_id,user_id)
);
CREATE TABLE IF NOT EXISTS brain.guide_feedback_drafts (
 guild_id TEXT NOT NULL,user_id TEXT NOT NULL,channel_id TEXT NOT NULL,
 message_id TEXT NOT NULL,text TEXT NOT NULL,expires_at BIGINT NOT NULL,
 PRIMARY KEY(guild_id,user_id,channel_id)
);
-- Bestehende Sperren übernehmen. Private Archive werden nicht in Wissen kopiert.
DO $migration$
BEGIN
 IF to_regclass('bot.concierge_profiles') IS NOT NULL THEN
  INSERT INTO brain.guide_subjects(guild_id,user_id,globally_opted_out,deleted)
  SELECT p.guild_id::text,p.user_id::text,p.opted_out OR COALESCE(g.opted_out,false),
   p.forgot_at IS NOT NULL OR g.deleted_at IS NOT NULL
  FROM bot.concierge_profiles p LEFT JOIN core.user_privacy g ON g.user_id=p.user_id
  ON CONFLICT(guild_id,user_id) DO UPDATE SET
   globally_opted_out=brain.guide_subjects.globally_opted_out OR EXCLUDED.globally_opted_out,
   deleted=brain.guide_subjects.deleted OR EXCLUDED.deleted;
 END IF;
END $migration$;
-- Enger kanonischer Schreibweg. Die Laufzeit erhält ausschließlich EXECUTE, keine Quell-DDL.
CREATE OR REPLACE FUNCTION brain.guide_set_server_record(p_guild text,p_revision bigint,p_record jsonb)
RETURNS void LANGUAGE plpgsql SECURITY DEFINER SET search_path=pg_catalog,brain AS $function$
DECLARE src text := 'guide-discord:' || p_guild;
 current_revision bigint;
 current_hash text;
BEGIN
 IF p_guild IS NULL OR p_guild !~ '^[1-9][0-9]{0,18}$' OR p_revision IS NULL OR p_revision<=0 OR p_record IS NULL OR octet_length(p_record::text)>131072
  OR p_record->>'source_id' IS DISTINCT FROM src OR p_record->>'logical_id' IS DISTINCT FROM 'server'
  OR (p_record->>'revision')::bigint IS DISTINCT FROM p_revision OR p_record->>'visibility' IS DISTINCT FROM 'public'
  OR p_record->'metadata'->>'source_class' IS DISTINCT FROM 'server_documentation'
  OR p_record->'allowed_scopes' IS DISTINCT FROM '["bot.public"]'::jsonb
  OR p_record->>'content_hash' IS NULL OR p_record->>'content_hash' !~ '^[0-9a-f]{64}$'
  OR jsonb_typeof(p_record->'content') IS DISTINCT FROM 'string'
  OR jsonb_typeof((p_record->>'content')::jsonb) IS DISTINCT FROM 'object'
  OR ((p_record->>'content')::jsonb)->>'guild_id' IS DISTINCT FROM p_guild
  OR (p_record->>'tombstone')::boolean IS DISTINCT FROM false THEN
  RAISE EXCEPTION 'Guide-Quellvertrag ist nicht freigegeben';
 END IF;
 PERFORM pg_advisory_xact_lock(hashtextextended(src,0));
 SELECT revision,content_hash INTO current_revision,current_hash FROM brain.source_record_heads WHERE source_id=src AND logical_id='server' FOR UPDATE;
 IF current_revision IS NOT NULL AND p_revision<=current_revision THEN
  IF p_revision=current_revision AND current_hash=p_record->>'content_hash' THEN RETURN; END IF;
  RAISE EXCEPTION 'Guide-Quellrevision ist veraltet';
 END IF;
 INSERT INTO brain.source_record_revisions(source_id,logical_id,revision,content_hash,record_json)
 VALUES(src,'server',p_revision,p_record->>'content_hash',p_record);
 INSERT INTO brain.source_record_heads(source_id,logical_id,revision,content_hash,record_json)
 VALUES(src,'server',p_revision,p_record->>'content_hash',p_record)
 ON CONFLICT(source_id,logical_id) DO UPDATE SET revision=EXCLUDED.revision,content_hash=EXCLUDED.content_hash,record_json=EXCLUDED.record_json,updated_at=now();
 -- Gelöschte Regeln und entzogene Rechte dürfen auch in alten Ableitungen nicht bleiben.
 DELETE FROM brain.source_record_revisions WHERE source_id=src AND logical_id='server' AND revision<p_revision;
END $function$;
REVOKE ALL ON FUNCTION brain.guide_set_server_record(text,bigint,jsonb) FROM PUBLIC;
COMMIT;
