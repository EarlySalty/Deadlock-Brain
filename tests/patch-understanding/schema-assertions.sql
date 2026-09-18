DO $test$
BEGIN
    ASSERT (SELECT count(*) FROM brain.patch_evidence_revisions)=2;
    ASSERT (SELECT bool_and(earlier_observation_unknown) FROM brain.patch_history_v1);
    ASSERT (SELECT bool_and(observed_at > source_published_at) FROM brain.patch_history_v1);
    ASSERT brain.patch_evidence_timestamp('not-a-date') IS NULL;
    UPDATE brain.patch_events SET created_at=now() WHERE event_hash='e1';
    ASSERT (SELECT created_at FROM brain.patch_events WHERE event_hash='e1')='2020-02-01T00:00:00Z'::timestamptz;
    ASSERT (SELECT count(*) FROM brain.patch_evidence_revisions)=2;
    UPDATE brain.patch_events SET new_value='45',raw_line='Synthetic 35 to 45' WHERE event_hash='e1';
    ASSERT (SELECT count(*) FROM brain.patch_evidence_revisions)=3;
    ASSERT (SELECT observation_kind FROM brain.patch_evidence_revisions ORDER BY revision_id DESC LIMIT 1)='live';
    INSERT INTO brain.knowledge_events VALUES('patch:e1','patch_event','2020-01-01T00:00:00Z','2020-01-01T00:00:00Z','2020-01-01T00:00:00Z','{}');
    ASSERT (SELECT effective_from IS NULL FROM brain.knowledge_events WHERE event_hash='patch:e1');
    ASSERT (SELECT observed_at > occurred_at FROM brain.knowledge_events WHERE event_hash='patch:e1');
    INSERT INTO brain.patch_review_runs(patch_external_id,context_sha256,prompt_version,model,context,report)
    VALUES('patch_1','hash','test','none','{}','{}'),('patch_2','other','test','none','{}','{}');
    UPDATE patchnotes.changelog_posts SET raw_content='Synthetic source revision' WHERE id=1;
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_1')='needs_revalidation';
    ASSERT (SELECT status FROM brain.patch_review_runs WHERE patch_external_id='patch_2')='draft';
    DELETE FROM brain.patch_events WHERE event_hash='e1';
    ASSERT (SELECT count(*) FROM brain.patch_evidence_revisions WHERE source_table='patch_events')=3;
    ASSERT (SELECT state FROM brain.patch_evidence_revisions WHERE source_table='patch_events' ORDER BY revision_id DESC LIMIT 1)='deleted';
    ASSERT (SELECT count(*) FROM brain.patch_history_v1 WHERE raw_line='Synthetic 35 to 40')=1;
    INSERT INTO brain.youtube_videos VALUES('test_video','{"transcript_evidence_hash":"raw2"}');
    INSERT INTO brain.youtube_transcripts(video_id,source_kind,transcript_text)
    VALUES('test_video','youtube_caption_manual','same text');
    INSERT INTO brain.youtube_transcript_evidence(video_id,raw_sha256,text_sha256,language,source_kind,raw_caption_json,segments)
    VALUES('test_video','raw1',encode(sha256(convert_to('same text','UTF8')),'hex'),'en','youtube_caption_manual','{}','[{"source_event_index":0,"start_ms":100,"end_ms":200,"text":"same text","pieces":[]}]'),
          ('test_video','raw2',encode(sha256(convert_to('same text','UTF8')),'hex'),'en','youtube_caption_manual','{}','[{"source_event_index":0,"start_ms":300,"end_ms":null,"text":"same text","pieces":[]}]');
    ASSERT (SELECT count(*) FROM brain.youtube_transcript_evidence)=2;
    ASSERT (SELECT start_ms FROM brain.youtube_caption_segments_v1)=300;
    ASSERT (SELECT end_ms IS NULL FROM brain.youtube_caption_segments_v1);
END
$test$;
