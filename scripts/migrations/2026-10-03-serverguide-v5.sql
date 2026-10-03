BEGIN;
-- Dispatchsperren überstehen Widerruf, Payloadabbau und einen Dienstneustart.
CREATE OR REPLACE FUNCTION brain.guide_action_guard() RETURNS trigger
LANGUAGE plpgsql SET search_path=pg_catalog AS $$
BEGIN
    IF NEW.actor_id IS DISTINCT FROM OLD.actor_id
       OR NEW.guild_id IS DISTINCT FROM OLD.guild_id
       OR NEW.message_id IS DISTINCT FROM OLD.message_id
       OR NEW.privacy_epoch IS DISTINCT FROM OLD.privacy_epoch
       OR NEW.kind IS DISTINCT FROM OLD.kind
       OR NEW.source_surface IS DISTINCT FROM OLD.source_surface
       OR NEW.expires_at IS DISTINCT FROM OLD.expires_at
       OR NEW.action_id IS DISTINCT FROM OLD.action_id THEN
        RAISE EXCEPTION 'Die Herkunft eines Einladungsauftrags ist unveränderlich';
    END IF;
    IF (OLD.revoked_at IS NOT NULL AND (NEW.revoked_at IS NULL OR NEW.revoked_at < OLD.revoked_at))
       OR (OLD.friend_dispatch_reserved AND NOT NEW.friend_dispatch_reserved)
       OR (OLD.invite_dispatch_reserved AND NOT NEW.invite_dispatch_reserved) THEN
        RAISE EXCEPTION 'Eine Einladungsversandsperre kann nicht aufgehoben werden';
    END IF;
    IF (NEW.steam_id64 IS NOT NULL AND NEW.steam_id64 IS DISTINCT FROM OLD.steam_id64)
       OR (NEW.turn_id IS NOT NULL AND NEW.turn_id IS DISTINCT FROM OLD.turn_id)
       OR (NEW.source_channel_id IS NOT NULL AND NEW.source_channel_id IS DISTINCT FROM OLD.source_channel_id)
       OR (NEW.source_thread_id IS NOT NULL AND NEW.source_thread_id IS DISTINCT FROM OLD.source_thread_id)
       OR (NEW.source_event_type IS NOT NULL AND NEW.source_event_type IS DISTINCT FROM OLD.source_event_type) THEN
        RAISE EXCEPTION 'Abgebaute oder korrigierte Einladungsdaten dürfen nicht wiederhergestellt werden';
    END IF;
    RETURN NEW;
END;
$$;
REVOKE ALL ON FUNCTION brain.guide_action_guard() FROM PUBLIC;
DROP TRIGGER IF EXISTS guide_action_guard ON brain.guide_action_grants;
CREATE TRIGGER guide_action_guard BEFORE UPDATE ON brain.guide_action_grants
FOR EACH ROW EXECUTE FUNCTION brain.guide_action_guard();
COMMIT;
