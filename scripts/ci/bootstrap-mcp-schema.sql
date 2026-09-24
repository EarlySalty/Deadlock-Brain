\set ON_ERROR_STOP on
BEGIN;
DO $guard$
BEGIN
  IF current_database() !~ '(^|_)(ci|test)(_|$)' THEN
    RAISE EXCEPTION 'MCP fixture requires an isolated test database';
  END IF;
END
$guard$;
CREATE SCHEMA IF NOT EXISTS patchnotes;
\ir ../../schema/vendor/dl-central-db/changelog_posts.sql
\ir ../../rust/crates/dbrain-sources/src/patch_changes.sql
COMMIT;
