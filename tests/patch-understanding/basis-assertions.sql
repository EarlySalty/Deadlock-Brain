DO $test$
DECLARE
    payload jsonb := jsonb_build_object('event_hash', 'same', 'line_index', 1, 'patch_snapshot_id', 2, 'legacy_patch_snapshot_id', 3);
BEGIN
    ASSERT brain.patch_evidence_payload_hash(payload)
        <> brain.patch_evidence_payload_hash(jsonb_set(payload, '{patch_snapshot_id}', 'null'::jsonb)),
        'patch_snapshot_id changes must create a content revision';
    ASSERT brain.patch_evidence_payload_hash(payload)
        <> brain.patch_evidence_payload_hash(jsonb_set(payload, '{legacy_patch_snapshot_id}', 'null'::jsonb)),
        'legacy_patch_snapshot_id changes must create a content revision';
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE context_sha256 = 'seed') = 'needs_revalidation',
        'migration must invalidate drafts created before basis hashes were corrected';

    INSERT INTO brain.patch_review_runs(patch_external_id, context_sha256, prompt_version, model, context, report)
    VALUES ('patch_1', 'basis-update', 'test', 'none', '{}', '{}');
    UPDATE brain.patch_events
    SET patch_snapshot_id = COALESCE(patch_snapshot_id, 0) + 1
    WHERE patch_external_id = 'patch_1'
       OR patch_external_id = (SELECT url FROM patchnotes.changelog_posts WHERE id = 1);
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE context_sha256 = 'basis-update') = 'needs_revalidation',
        'changing the parse basis must invalidate the patch review';
END
$test$;
