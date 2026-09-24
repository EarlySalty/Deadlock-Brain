-- Test-only, atomic, schema-only bootstrap. Never apply to production.
\set ON_ERROR_STOP on
BEGIN;
SET LOCAL lock_timeout = '10s';
DO $guard$
BEGIN
  IF current_database() !~ '(^|_)(ci|test)(_|$)' THEN
    RAISE EXCEPTION 'Refusing non-test database: use a name containing a ci/test segment';
  END IF;
  IF EXISTS (
    SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid = c.relnamespace
    WHERE n.nspname = 'brain' AND c.relkind IN ('r', 'p', 'v', 'm')
  ) THEN
    RAISE EXCEPTION 'Refusing existing brain relations: bootstrap requires a fresh test database';
  END IF;
END
$guard$;
\ir ../../schema/vendor/dl-central-db/0012_brain_knowledge_timeline.sql
\ir ../../schema/vendor/dl-central-db/0013_brain_insight_records.sql
\ir ../../schema/vendor/dl-central-db/2026070410_brain_ingestion_tables.sql
\ir ../../schema/vendor/dl-central-db/2026071401_brain_feeder_runs.sql
\ir ../../schema/vendor/dl-central-db/2026071660_brain_plan.sql
\ir ../../schema/vendor/dl-central-db/2026071661_brain_plan_rejected_items.sql
\ir ../../scripts/migrations/2026-09-12-reasoner.sql
\ir ../../scripts/migrations/2026-09-16-population.sql
COMMIT;
