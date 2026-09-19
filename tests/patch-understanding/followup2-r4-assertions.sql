-- Assertions fuer R4: ein Event wechselt seine Patchidentitaet (Quelle A -> B).
-- Auf alter + zweiter Migration allein bleibt der Review zu A draft, weil der
-- Trigger nur die neue Zugehoerigkeit invalidiert. Nach der dritten Migration
-- werden beide Reviews needs_revalidation und der Speicherschutz erkennt die
-- Entfernung aus A.
DO $test$
DECLARE
    a_scope_before bigint;
    a_scope_after bigint;
    move_revision bigint;
BEGIN
    INSERT INTO patchnotes.changelog_posts VALUES
        (301,'Patch A','https://steamcommunity.com/a/301','2026-09-16T00:00:00Z','A body'),
        (302,'Patch B','https://steamcommunity.com/a/302','2026-09-16T00:00:00Z','B body');
    INSERT INTO brain.patch_events(event_hash,patch_external_id,patch_title,patch_url,posted_at,source_kind,entity_type,entity_name,raw_line,old_value,new_value,created_at)
        VALUES('mv','https://steamcommunity.com/a/301','Patch A','https://steamcommunity.com/a/301','2026-09-16T00:00:00Z','steam','hero','X','X: line','','','2026-09-16T00:00:00Z');
    INSERT INTO brain.patch_review_runs(patch_external_id,context_sha256,prompt_version,model,context,report)
        VALUES('patch_301','h','v','none','{}','{}'),('patch_302','h','v','none','{}','{}');

    -- A-Scope-Revision vor dem Wechsel (alle Revisionen der je unter A liegenden Events).
    SELECT COALESCE(max(revision_id),0) INTO a_scope_before FROM brain.patch_evidence_revisions r
      WHERE r.source_table='patch_events' AND r.source_key IN (
          SELECT DISTINCT e.source_key FROM brain.patch_evidence_revisions e
          WHERE e.source_table='patch_events' AND e.payload->>'patch_external_id'='https://steamcommunity.com/a/301');

    -- Event wandert von A nach B (gleicher event_hash, keine ID-Umschreibung).
    UPDATE brain.patch_events SET patch_external_id='https://steamcommunity.com/a/302', patch_url='https://steamcommunity.com/a/302' WHERE event_hash='mv';

    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_301')='needs_revalidation',
        'alter Quellreview A muss nach dem Identitaetswechsel needs_revalidation sein';
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_302')='needs_revalidation',
        'neuer Quellreview B muss nach dem Identitaetswechsel needs_revalidation sein';

    -- Speicherschutz: die kanonische Revision im A-Umfang hebt sich auf die
    -- Move-Revision an, erkennt also die Entfernung aus A.
    SELECT max(revision_id) INTO move_revision FROM brain.patch_evidence_revisions
      WHERE source_table='patch_events' AND source_key='mv';
    SELECT COALESCE(max(revision_id),0) INTO a_scope_after FROM brain.patch_evidence_revisions r
      WHERE r.source_table='patch_events' AND r.source_key IN (
          SELECT DISTINCT e.source_key FROM brain.patch_evidence_revisions e
          WHERE e.source_table='patch_events' AND e.payload->>'patch_external_id'='https://steamcommunity.com/a/301');
    ASSERT a_scope_after = move_revision,
        'A-Umfang muss die Move-Revision sehen (Entfernung aus A erkannt)';
    ASSERT a_scope_after > a_scope_before,
        'A-Umfang-Revision muss durch den Wechsel ansteigen';
END
$test$;
