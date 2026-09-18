-- Realitaetsnahe, isolierte Fixture fuer den Smoke-Lauf des normalen
-- patch-review-CLI-Binaries. Nur fuer eine leere Wegwerf-Testdatenbank.
-- Ereignisse liegen unter der Quellen-URL (nicht patch_1); Snapshots tragen den
-- realen Typ item_or_ability. Beide Migrationen muessen darauf sauber anwenden.
CREATE SCHEMA brain;
CREATE SCHEMA patchnotes;

-- Von beiden Migrationen referenzierte Tabellen.
CREATE TABLE brain.youtube_videos(video_id text PRIMARY KEY, metadata jsonb DEFAULT '{}');
CREATE TABLE brain.youtube_transcripts(video_id text PRIMARY KEY, source_kind text NOT NULL, transcript_text text, content_hash text);
CREATE TABLE brain.knowledge_events(
    event_hash text PRIMARY KEY, event_source text, observed_at timestamptz,
    occurred_at timestamptz, effective_from timestamptz, metadata jsonb DEFAULT '{}'
);
CREATE TABLE patchnotes.changelog_posts(id bigint PRIMARY KEY, title text, url text, posted_at timestamptz, raw_content text);
CREATE TABLE brain.patch_events(
    id bigserial PRIMARY KEY, event_hash text UNIQUE NOT NULL, patch_external_id text,
    patch_title text, patch_url text, posted_at timestamptz, source_kind text,
    entity_type text, entity_name text, subject text, change_type text,
    raw_line text, old_value text, new_value text, line_index int DEFAULT 0,
    patch_snapshot_id bigint, created_at timestamptz DEFAULT now(), metadata jsonb DEFAULT '{}'
);

-- Von der CLI (build_context) gelesene Snapshot-Tabellen.
CREATE TABLE brain.source_documents(id bigserial PRIMARY KEY, url text);
CREATE TABLE brain.entity_snapshots(
    id bigserial PRIMARY KEY, source text, entity_type text, external_id text,
    canonical_name text, payload_hash text, payload jsonb, fetched_at timestamptz,
    source_document_id bigint REFERENCES brain.source_documents(id)
);

INSERT INTO patchnotes.changelog_posts
    VALUES(1,'Minor Update - 09-16-2026','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','2026-09-16T20:16:43Z','Real body text long enough to pass the non-empty check.');
INSERT INTO brain.patch_events(event_hash,patch_external_id,patch_title,patch_url,posted_at,source_kind,entity_type,entity_name,raw_line,old_value,new_value,line_index,created_at)
VALUES
 ('s1','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','Minor Update - 09-16-2026','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','2026-09-16T20:16:43Z','steam','hero','Abrams','Abrams: Base gun damage increased by 5%','','',0,'2026-09-16T20:16:43Z'),
 ('s2','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','Minor Update - 09-16-2026','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','2026-09-16T20:16:43Z','steam','item_or_ability','Veil Walker','Veil Walker: cooldown reduced','','',1,'2026-09-16T20:16:43Z');

INSERT INTO brain.source_documents(url) VALUES('https://assets.deadlock-api.com/v2/items');
INSERT INTO brain.entity_snapshots(source,entity_type,external_id,canonical_name,payload_hash,payload,fetched_at,source_document_id)
VALUES
 ('deadlock_assets_api','item_or_ability','veil_walker','Veil Walker','ph1','{"cooldown":30}','2026-09-15T00:00:00Z',1),
 ('deadlock_assets_api','item_or_ability','abrams_gun','Abrams Gun','ph2','{"dmg":100}','2026-09-15T00:00:00Z',1);

-- Eventbasis (R6): patchnote-Snapshot, auf dem die Events beruhen; raw_content
-- stimmt mit changelog_posts ueberein (abgeschlossener Import). Steam-URL, damit
-- er nicht in die Assets-Auswahl faellt.
INSERT INTO brain.source_documents(url) VALUES('https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435');
INSERT INTO brain.entity_snapshots(source,entity_type,external_id,canonical_name,payload_hash,payload,fetched_at,source_document_id)
VALUES('deadlock_patchnotes_db','patchnote','https://steamcommunity.com/games/1422450/announcements/detail/698776157349216435','Minor Update - 09-16-2026','phpatch',
 jsonb_build_object('id','1','raw_content','Real body text long enough to pass the non-empty check.'),'2026-09-16T20:16:43Z',2);
UPDATE brain.patch_events SET patch_snapshot_id=(SELECT id FROM brain.entity_snapshots WHERE entity_type='patchnote') WHERE event_hash IN ('s1','s2');
