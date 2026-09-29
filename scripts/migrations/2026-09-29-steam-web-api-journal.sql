CREATE SCHEMA IF NOT EXISTS brain;

CREATE TABLE IF NOT EXISTS brain.steam_web_api_pending_observations (
  reservation_id bigint PRIMARY KEY,
  caller text NOT NULL,
  dispatch_started boolean NOT NULL DEFAULT false,
  answered boolean NOT NULL DEFAULT false,
  http_status smallint,
  retry_after text,
  reserved_at timestamptz NOT NULL DEFAULT now(),
  answered_at timestamptz,
  CHECK (answered OR (http_status IS NULL AND retry_after IS NULL AND answered_at IS NULL))
);
