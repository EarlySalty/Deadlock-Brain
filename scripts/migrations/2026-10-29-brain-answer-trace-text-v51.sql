BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Antwortnachweis-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

ALTER TABLE ops.answer_trace
    ADD COLUMN source text CHECK(char_length(source) BETWEEN 1 AND 128),
    ADD COLUMN route text CHECK(route ~ '^[a-z][a-z0-9_:.-]{0,63}$'),
    ADD COLUMN question text CHECK(char_length(question)<=4000),
    ADD COLUMN history jsonb NOT NULL DEFAULT '[]'::jsonb
        CHECK(jsonb_typeof(history)='array' AND jsonb_array_length(history)<=4 AND octet_length(history::text)<=32768),
    ADD COLUMN answer_text text CHECK(char_length(answer_text)<=8000),
    ADD COLUMN not_answered_stage text CHECK(not_answered_stage ~ '^[a-z][a-z0-9_]{0,63}$'),
    ADD COLUMN not_answered_reason text CHECK(not_answered_reason ~ '^[a-z][a-z0-9_]{0,63}$'),
    ADD COLUMN provider_rounds integer CHECK(provider_rounds>=0);

CREATE INDEX answer_trace_recorded_idx ON ops.answer_trace(recorded_at);

GRANT INSERT(source,route,question,history,answer_text,not_answered_stage,not_answered_reason,provider_rounds)
    ON ops.answer_trace TO brain_service;

CREATE FUNCTION ops.purge_answer_trace() RETURNS bigint
LANGUAGE sql VOLATILE SECURITY DEFINER SET search_path=pg_catalog,pg_temp AS $purge$
    WITH purged AS (
        DELETE FROM ops.answer_trace
        WHERE recorded_at < clock_timestamp() - interval '90 days'
        RETURNING 1
    )
    SELECT count(*) FROM purged
$purge$;
REVOKE ALL ON FUNCTION ops.purge_answer_trace() FROM PUBLIC;
GRANT EXECUTE ON FUNCTION ops.purge_answer_trace() TO brain_ingest;

COMMIT;
