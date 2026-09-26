-- C11 metadata only; run through brain-migrate, not service startup.
-- The migrator inserts the supported version ONLY AFTER validating existing data,
-- inside the same transaction as all pending DDL. No guessed version/backfill here.
BEGIN;
CREATE TABLE IF NOT EXISTS brain.core_schema_version (
    singleton boolean PRIMARY KEY DEFAULT true CHECK (singleton),
    schema_version integer NOT NULL,
    store_contract text NOT NULL,
    installed_at timestamptz NOT NULL DEFAULT now()
);
COMMIT;
