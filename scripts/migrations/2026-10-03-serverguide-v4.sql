BEGIN;
-- Ein konkreter Auftrag ist kein Nutzerprofil. Sperrmarker bleiben nach Payloadabbau erhalten.
CREATE TABLE IF NOT EXISTS brain.guide_action_grants (
    action_id TEXT PRIMARY KEY CHECK (length(action_id) BETWEEN 1 AND 128),
    turn_id TEXT CHECK (length(turn_id) BETWEEN 1 AND 128),
    actor_id BIGINT NOT NULL CHECK (actor_id > 0),
    guild_id BIGINT NOT NULL CHECK (guild_id > 0),
    source_channel_id BIGINT CHECK (source_channel_id > 0),
    source_thread_id BIGINT CHECK (source_thread_id > 0),
    source_event_type TEXT CHECK (source_event_type IN ('message', 'button', 'command')),
    source_surface TEXT NOT NULL CHECK (source_surface IN ('dm', 'public')),
    message_id BIGINT NOT NULL CHECK (message_id > 0),
    privacy_epoch BIGINT NOT NULL CHECK (privacy_epoch >= 0),
    kind TEXT NOT NULL CHECK (kind = 'deadlock_access_invite'),
    steam_id64 BIGINT CHECK (steam_id64 > 0),
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    friend_task_id BIGINT,
    invite_task_id BIGINT,
    friend_dispatch_reserved BOOLEAN NOT NULL DEFAULT FALSE,
    invite_dispatch_reserved BOOLEAN NOT NULL DEFAULT FALSE,
    status TEXT NOT NULL DEFAULT 'queued' CHECK (status IN (
        'queued', 'friend_request_sent', 'waiting_for_acceptance', 'invite_sent',
        'already_has_access', 'failed', 'unknown', 'expired', 'cancelled'
    )),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    UNIQUE (guild_id, actor_id, message_id)
);
REVOKE ALL ON brain.guide_action_grants FROM PUBLIC;
CREATE INDEX IF NOT EXISTS guide_action_grants_actor_idx
    ON brain.guide_action_grants(actor_id, guild_id);
CREATE INDEX IF NOT EXISTS guide_action_grants_expiry_idx
    ON brain.guide_action_grants(expires_at) WHERE steam_id64 IS NOT NULL;
ALTER TABLE brain.guide_turn_claims ADD COLUMN IF NOT EXISTS source_surface TEXT;
ALTER TABLE brain.guide_turn_claims ADD COLUMN IF NOT EXISTS source_channel_id TEXT;
ALTER TABLE brain.guide_turn_claims ADD COLUMN IF NOT EXISTS source_thread_id TEXT;
ALTER TABLE brain.guide_turn_claims ADD COLUMN IF NOT EXISTS source_message_id TEXT;
ALTER TABLE brain.guide_turn_claims ADD COLUMN IF NOT EXISTS source_event_type TEXT;
-- Alte Claims erhalten keine erfundene Herkunft und autorisieren keine neuen Aktionen.
COMMIT;
