-- Expliziter Owner-Migrationslauf, niemals beim normalen Dienststart.
BEGIN;
CREATE TABLE IF NOT EXISTS brain.maintenance_jobs_v1 (
    id text PRIMARY KEY,
    repo_id text NOT NULL,
    idempotency_key text NOT NULL UNIQUE,
    spec_json jsonb NOT NULL,
    status text NOT NULL CHECK (status IN ('planned','discovered_policy_pending','source_review','author','reviewer','publish','activated','failed','superseded')),
    checkpoint_json jsonb NOT NULL DEFAULT '{"artifact_refs":{},"review":null,"publication":null,"activation":null}',
    attempts integer NOT NULL DEFAULT 0 CHECK (attempts BETWEEN 0 AND 100),
    error_code text,
    owner text,
    fence bigint NOT NULL DEFAULT 0 CHECK (fence >= 0),
    lease_until timestamptz,
    available_at timestamptz NOT NULL DEFAULT now(),
    superseded_by text REFERENCES brain.maintenance_jobs_v1(id),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CHECK ((owner IS NULL) = (lease_until IS NULL)),
    CHECK (status NOT IN ('activated','failed','superseded') OR owner IS NULL),
    CHECK ((status = 'superseded') = (superseded_by IS NOT NULL)),
    CHECK (length(id) BETWEEN 1 AND 512 AND length(repo_id) BETWEEN 1 AND 512)
);
CREATE INDEX IF NOT EXISTS maintenance_jobs_v1_claim_idx ON brain.maintenance_jobs_v1(available_at,created_at)
    WHERE status IN ('planned','source_review','author','reviewer','publish');
CREATE TABLE IF NOT EXISTS brain.maintenance_sources_v1 (
    repo_id text PRIMARY KEY,
    registration_json jsonb NOT NULL,
    updated_at timestamptz NOT NULL DEFAULT now()
);
COMMENT ON TABLE brain.maintenance_jobs_v1 IS 'Eigene erneuerbare Worker-Leases mit monotoner Fence gegen abgelaufene Worker. Quell- und Deploy-Revisionen bleiben vom Aktivierungsnachweis getrennt.';
COMMIT;
