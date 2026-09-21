-- Assertions fuer die kanonische Patchidentitaet (Korrektur 2 / R2).
-- Ereignisse liegen unter der Quellen-URL, der Review unter patch_<id>. Auf der
-- ALTEN Migration allein muss die Invalidierung scheitern, weil der alte Trigger
-- Entwuerfe nach der rohen URL statt nach dem kanonischen Schluessel markiert.
-- Nach der Folgemigration passieren alle Assertions.
DO $test$
BEGIN
    -- Realitaetsnah: changelog-ID 285, Ereignisse tragen die Quellen-URL.
    INSERT INTO patchnotes.changelog_posts
        VALUES(285,'Minor Update - 09-16-2026','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','2026-09-16T20:16:43Z','Body v1');
    INSERT INTO brain.patch_events(event_hash,patch_external_id,patch_title,patch_url,posted_at,source_kind,entity_type,entity_name,raw_line,old_value,new_value,created_at)
        VALUES('u1','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','Minor Update - 09-16-2026','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','2026-09-16T20:16:43Z','steam','hero','Abrams','Abrams: Base gun damage increased by 5%','','','2026-09-16T20:16:43Z');
    -- Der Reviewentwurf ist unter dem kanonischen Schluessel patch_285 gespeichert.
    INSERT INTO brain.patch_review_runs(patch_external_id,context_sha256,prompt_version,model,context,report)
        VALUES('patch_285','h','v','none','{}','{}');

    -- Eine Aenderung am URL-basierten Ereignis muss denselben kanonischen Review
    -- als pruefbeduerftig markieren.
    UPDATE brain.patch_events SET new_value='changed',raw_line='Abrams: Base gun damage increased by 3%' WHERE event_hash='u1';
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_285')='needs_revalidation',
        'URL-basierte Ereignisaenderung muss den kanonischen patch_<id>-Review invalidieren';

    -- Zuruecksetzen, dann muss auch die Quelländerung denselben Review treffen.
    UPDATE brain.patch_review_runs SET status='draft' WHERE patch_external_id='patch_285';
    UPDATE patchnotes.changelog_posts SET raw_content='Body v2' WHERE id=285;
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_285')='needs_revalidation',
        'Quelländerung muss denselben kanonischen Review invalidieren';

    -- Der kanonische Schluessel loest die gespeicherte URL genau auf patch_285 auf.
    ASSERT brain.patch_review_canonical_key('patch_events',(SELECT to_jsonb(pe) FROM brain.patch_events pe WHERE event_hash='u1'))='patch_285',
        'kanonischer Schluessel loest die Quellen-URL auf patch_285 auf';

    -- Mehrdeutige/unbekannte Zuordnung bleibt sichtbar (roher Schluessel), kein Raten.
    ASSERT brain.patch_review_canonical_key('patch_events','{"patch_external_id":"https://unknown.example/none"}'::jsonb)='https://unknown.example/none',
        'unbekannte Quellen-URL behaelt den rohen Schluessel';
END
$test$;
