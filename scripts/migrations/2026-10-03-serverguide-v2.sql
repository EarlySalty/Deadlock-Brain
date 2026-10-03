BEGIN;
ALTER TABLE brain.guide_subjects ADD COLUMN IF NOT EXISTS min_event_id BIGINT NOT NULL DEFAULT 0;
ALTER TABLE brain.guide_feedback_outbox ADD COLUMN IF NOT EXISTS expires_at TIMESTAMPTZ;
UPDATE brain.guide_feedback_outbox SET expires_at=created_at+interval '10 minutes' WHERE expires_at IS NULL;
ALTER TABLE brain.guide_feedback_outbox ALTER COLUMN expires_at SET DEFAULT (now()+interval '10 minutes');
ALTER TABLE brain.guide_feedback_outbox ALTER COLUMN expires_at SET NOT NULL;
COMMIT;
