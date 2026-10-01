-- Assertions fuer die first_observed_at-Semantik (Korrektur 1).
-- Auf der ALTEN Migration allein muessen diese Assertions scheitern:
-- der alte knowledge_events-Trigger uebernimmt die zuletzt protokollierte
-- Beobachtung statt der fruehesten, und patch_history_v1 hat kein
-- first_observed_at. Nach der Folgemigration passieren sie.
DO $test$
DECLARE
    base_obs timestamptz;
BEGIN
    -- Frueheste Beobachtung des urspruenglichen e1-Inhalts (Baseline-Revision).
    SELECT observed_at INTO base_obs FROM brain.patch_evidence_revisions
      WHERE source_table='patch_events' AND source_key='e1'
      ORDER BY revision_id ASC LIMIT 1;

    -- Loeschen und identischer Reimport, dann echte Inhaltsaenderung, dann
    -- Rueckkehr zum urspruenglichen Inhalt.
    DELETE FROM brain.patch_events WHERE event_hash='e1';
    INSERT INTO brain.patch_events(event_hash,patch_external_id,patch_title,patch_url,posted_at,source_kind,entity_type,entity_name,raw_line,old_value,new_value,created_at)
    VALUES('e1','patch_1','Synthetic fixture','https://example.test/patch','2020-01-01T00:00:00Z','steam','mechanic','Synthetic','Synthetic 35 to 40','35','40','2020-02-01T00:00:00Z');
    UPDATE brain.patch_events SET new_value='45',raw_line='Synthetic 35 to 45' WHERE event_hash='e1';
    UPDATE brain.patch_events SET new_value='40',raw_line='Synthetic 35 to 40' WHERE event_hash='e1';

    -- knowledge_events muss die frueheste Beobachtung des aktuellen (wieder
    -- hergestellten) Inhalts uebernehmen. Der alte Trigger nimmt die letzte.
    INSERT INTO brain.knowledge_events VALUES('patch:e1','patch_event',now(),now(),now(),'{}');
    ASSERT (SELECT observed_at FROM brain.knowledge_events WHERE event_hash='patch:e1') = base_obs,
        'knowledge_events.observed_at muss die fruehste Beobachtung des aktuellen Inhalts sein';
    ASSERT (SELECT (metadata->>'earlier_observation_unknown')::boolean FROM brain.knowledge_events WHERE event_hash='patch:e1'),
        'zurueckgekehrter Baseline-Inhalt bleibt earlier_observation_unknown';

    -- View: first_observed_at des wiederhergestellten Inhalts = Baseline-Zeit.
    ASSERT (SELECT first_observed_at FROM brain.patch_history_v1
            WHERE event_hash='e1' AND state='present' AND new_value='40'
            ORDER BY revision_id DESC LIMIT 1) = base_obs,
        'Rueckkehr zu frueherem Inhalt behaelt die fruehste Beobachtung';
    ASSERT (SELECT earlier_observation_unknown FROM brain.patch_history_v1
            WHERE event_hash='e1' AND state='present' AND new_value='40'
            ORDER BY revision_id DESC LIMIT 1),
        'Baseline-Unsicherheit bleibt nach Reimport erhalten';

    -- Eine echte Inhaltsaenderung hat ihre eigene, spaetere Erstbeobachtung.
    ASSERT (SELECT first_observed_at FROM brain.patch_history_v1
            WHERE event_hash='e1' AND new_value='45'
            ORDER BY revision_id DESC LIMIT 1) > base_obs,
        'eine Inhaltsaenderung hat eine eigene Erstbeobachtung';
    ASSERT NOT (SELECT earlier_observation_unknown FROM brain.patch_history_v1
                WHERE event_hash='e1' AND new_value='45'
                ORDER BY revision_id DESC LIMIT 1),
        'frisch beobachtete Inhaltsaenderung ist nicht baseline-unbekannt';

    -- known-at: ein historischer Treffer benutzt keine spaetere Revision.
    ASSERT (SELECT count(*) FROM brain.patch_history_v1
            WHERE first_observed_at IS NOT NULL AND first_observed_at > observed_at) = 0,
        'first_observed_at darf nie nach der annotierten Revision liegen';
END
$test$;
