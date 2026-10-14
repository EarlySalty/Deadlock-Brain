BEGIN;
CREATE INDEX p2_complete_snapshot_receipts ON canon.receipt(ingest_run_id DESC,id DESC,raw_object_id)
WHERE extractor='v1:17:assets_hero_slots:1:1' AND locator_kind='raw_bytes' AND locator='';
CREATE OR REPLACE VIEW serve.p2_hero_slot_source AS
SELECT p.source_revision AS revision
FROM serve.published_receipt p
WHERE p.source='assets' AND p.path='heroes/en/observed/complete.snapshot'
    AND p.extractor='v1:17:assets_hero_slots:1:1'
    AND p.locator_kind='raw_bytes' AND p.locator=''
    AND p.value_sha256=p.content_sha256
ORDER BY p.ingest_run_id DESC,p.id DESC LIMIT 1;
COMMIT;
