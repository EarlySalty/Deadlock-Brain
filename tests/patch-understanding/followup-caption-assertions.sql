-- Assertions fuer den Caption-Text-SHA-Abgleich (Korrektur 2).
-- Auf der ALTEN Migration allein muss die Assertion "geaenderter Text liefert
-- keine alten Segmente" scheitern, weil die alte View nur den raw_sha256-Zeiger
-- prueft. Nach der Folgemigration passieren alle Assertions.
DO $test$
BEGIN
    INSERT INTO brain.youtube_videos VALUES('capvid','{"transcript_evidence_hash":"rawA"}');
    INSERT INTO brain.youtube_transcripts(video_id,source_kind,transcript_text)
    VALUES('capvid','youtube_caption_manual','hello world');
    INSERT INTO brain.youtube_transcript_evidence(video_id,raw_sha256,text_sha256,language,source_kind,raw_caption_json,segments)
    VALUES('capvid','rawA',encode(sha256(convert_to('hello world','UTF8')),'hex'),'en','youtube_caption_manual','{}',
           '[{"source_event_index":0,"start_ms":500,"end_ms":600,"text":"hello world","pieces":[]}]');

    -- Passender Text: Segment erscheint.
    ASSERT (SELECT start_ms FROM brain.youtube_caption_segments_v1 WHERE video_id='capvid')=500,
        'passender Transkript-Text liefert seine Segmente';

    -- Text aendert sich, der Zeiger nennt weiter die alte Evidenz: nichts erscheint.
    UPDATE brain.youtube_transcripts SET transcript_text='different text' WHERE video_id='capvid';
    ASSERT (SELECT count(*) FROM brain.youtube_caption_segments_v1 WHERE video_id='capvid')=0,
        'geaenderter Transkript-Text liefert keine alten Zeitsegmente';

    -- Neue Evidenz fuer den neuen Text mit eigener Zeit ist eine neue Fassung.
    INSERT INTO brain.youtube_transcript_evidence(video_id,raw_sha256,text_sha256,language,source_kind,raw_caption_json,segments)
    VALUES('capvid','rawB',encode(sha256(convert_to('different text','UTF8')),'hex'),'en','youtube_caption_manual','{}',
           '[{"source_event_index":0,"start_ms":900,"end_ms":null,"text":"different text","pieces":[]}]');
    UPDATE brain.youtube_videos SET metadata='{"transcript_evidence_hash":"rawB"}' WHERE video_id='capvid';
    ASSERT (SELECT start_ms FROM brain.youtube_caption_segments_v1 WHERE video_id='capvid')=900,
        'die neue Evidenzfassung erscheint, sobald Text und Zeiger uebereinstimmen';

    -- Gleicher Text mit neuer Zeitzuordnung bleibt eine eigene Evidenzfassung.
    INSERT INTO brain.youtube_transcript_evidence(video_id,raw_sha256,text_sha256,language,source_kind,raw_caption_json,segments)
    VALUES('capvid','rawC',encode(sha256(convert_to('different text','UTF8')),'hex'),'en','youtube_caption_manual','{}',
           '[{"source_event_index":0,"start_ms":1300,"end_ms":1400,"text":"different text","pieces":[]}]');
    UPDATE brain.youtube_videos SET metadata='{"transcript_evidence_hash":"rawC"}' WHERE video_id='capvid';
    ASSERT (SELECT start_ms FROM brain.youtube_caption_segments_v1 WHERE video_id='capvid')=1300,
        'gleicher Text mit neuer Zeit ist eine per Zeiger gewaehlte neue Fassung';
    ASSERT (SELECT count(*) FROM brain.youtube_transcript_evidence WHERE video_id='capvid')=3,
        'aeltere Evidenzfassungen bleiben erhalten';
END
$test$;
