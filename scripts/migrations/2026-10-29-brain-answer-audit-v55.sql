BEGIN;

DO $owner$
BEGIN
    IF current_user <> 'brain_migrate' THEN
        RAISE EXCEPTION 'Antwortprüfung-Migration benötigt brain_migrate';
    END IF;
END
$owner$;

SET LOCAL lock_timeout='5000ms';

CREATE TABLE ops.answer_probe_run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    started_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    finished_at timestamptz,
    release text CHECK (release ~ '^[0-9a-f]{7,64}$'),
    probe_set_sha256 text NOT NULL CHECK (probe_set_sha256 ~ '^[0-9a-f]{64}$'),
    probes integer NOT NULL CHECK (probes BETWEEN 0 AND 1000),
    passed integer CHECK (passed>=0),
    partial integer CHECK (partial>=0),
    failed integer CHECK (failed>=0),
    unavailable integer CHECK (unavailable>=0),
    baseline_run bigint REFERENCES ops.answer_probe_run(id),
    regressions integer CHECK (regressions>=0),
    unavailable_spike boolean,
    status text NOT NULL CHECK (status IN ('running','ok','regressed','degraded'))
);

CREATE INDEX answer_probe_run_started ON ops.answer_probe_run (started_at);

CREATE TABLE ops.answer_probe_result (
    run_id bigint NOT NULL REFERENCES ops.answer_probe_run(id) ON DELETE CASCADE,
    probe_id text NOT NULL CHECK (probe_id ~ '^[A-Z][A-Z0-9_-]{0,31}$'),
    kind text NOT NULL CHECK (kind ~ '^[a-z][a-z0-9_]{0,31}$'),
    topic text NOT NULL CHECK (char_length(topic)<=300),
    final_status text NOT NULL CHECK (final_status ~ '^[a-z][a-z0-9_]{0,63}$'),
    http integer NOT NULL CHECK (http BETWEEN 0 AND 999),
    grade text NOT NULL CHECK (grade IN ('pass','partial','fail','unavailable')),
    hit_groups integer NOT NULL CHECK (hit_groups>=0),
    total_groups integer NOT NULL CHECK (total_groups>=0),
    latency_ms integer NOT NULL CHECK (latency_ms>=0),
    answer_text text CHECK (char_length(answer_text)<=8000),
    baseline_grade text CHECK (baseline_grade IN ('pass','partial','fail','unavailable')),
    regressed boolean NOT NULL DEFAULT false,
    recorded_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    PRIMARY KEY (run_id,probe_id)
);

CREATE TABLE ops.answer_audit_run (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    audit_day date NOT NULL,
    since timestamptz NOT NULL,
    ran_at timestamptz NOT NULL DEFAULT clock_timestamp(),
    traces integer NOT NULL CHECK (traces>=0),
    claims integer NOT NULL CHECK (claims>=0),
    flagged_traces integer NOT NULL CHECK (flagged_traces>=0),
    flags integer NOT NULL CHECK (flags>=0),
    probe_run bigint REFERENCES ops.answer_probe_run(id)
);

CREATE INDEX answer_audit_run_day ON ops.answer_audit_run (audit_day,id);

CREATE TABLE ops.answer_audit_flag (
    run_id bigint NOT NULL REFERENCES ops.answer_audit_run(id) ON DELETE CASCADE,
    ordinal integer NOT NULL CHECK (ordinal>=0),
    trace_id bigint NOT NULL,
    trace_at timestamptz NOT NULL,
    kind text NOT NULL CHECK (kind IN ('wrong_patch_date','stale_value','stale_history','unsourced_change','unsourced_number')),
    claim text NOT NULL CHECK (char_length(claim)<=600),
    expected text NOT NULL CHECK (char_length(expected)<=600),
    source text NOT NULL CHECK (char_length(source)<=300),
    PRIMARY KEY (run_id,ordinal)
);

CREATE INDEX answer_audit_flag_trace ON ops.answer_audit_flag (trace_id);

REVOKE ALL ON ops.answer_probe_run,ops.answer_probe_result,ops.answer_audit_run,ops.answer_audit_flag FROM PUBLIC;
GRANT SELECT,INSERT,UPDATE ON ops.answer_probe_run TO brain_ingest;
GRANT SELECT,INSERT,DELETE ON ops.answer_probe_result,ops.answer_audit_flag TO brain_ingest;
GRANT SELECT,INSERT,DELETE ON ops.answer_audit_run TO brain_ingest;

COMMIT;
