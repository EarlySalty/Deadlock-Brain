BEGIN;

DO $tool$
BEGIN
    IF EXISTS(SELECT 1 FROM pg_roles WHERE rolname='brain_tool') THEN
        GRANT EXECUTE ON FUNCTION serve.site_generation() TO brain_tool;
    END IF;
END
$tool$;

COMMIT;
