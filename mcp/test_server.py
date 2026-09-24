#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import subprocess
from unittest.mock import patch
from urllib.parse import urlparse

import pytest


ROOT = Path(__file__).resolve().parent
SERVER_PATH = ROOT / "server.py"


def load_server():
    spec = importlib.util.spec_from_file_location("dl_brain_mcp_server", SERVER_PATH)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    # Import must never invoke the real production secret loader.
    with patch("subprocess.run", side_effect=FileNotFoundError):
        spec.loader.exec_module(module)
    return module


def test_patch_history_uses_exact_entity_match():
    server = load_server()
    calls = []
    original = server._query_rows
    try:
        server._query_rows = lambda sql, variables: calls.append((sql, variables)) or []
        assert server.patch_history("Holliday") == []
    finally:
        server._query_rows = original

    sql, variables = calls[0]
    assert "lower(entity_name) = lower(:'entity')" in sql
    assert "entity_name ILIKE" not in sql
    assert "ORDER BY patch_date DESC" in sql
    assert sql.rstrip().endswith("ORDER BY patch_date, stat_name")
    assert variables["entity"] == "Holliday"


def test_postgres_uri_dsn_is_passed_via_libpq_environment():
    server = load_server()
    env = server._psql_env(
        "postgresql://user:pw%21@db.example.test:5433/deadlock?sslmode=require",
        {"PATH": "/bin"},
    )
    assert env["PGHOST"] == "db.example.test"
    assert env["PGPORT"] == "5433"
    assert env["PGUSER"] == "user"
    assert env["PGPASSWORD"] == "pw!"
    assert env["PGDATABASE"] == "deadlock"
    assert env["PGSSLMODE"] == "require"
    assert "postgresql://" not in " ".join(env.values())


def test_query_rows_sends_sql_via_stdin_for_psql_variables():
    server = load_server()
    calls = []
    original_run = server.subprocess.run
    original_dsn = server._DSN

    class Result:
        returncode = 0
        stdout = "[]"
        stderr = ""

    def fake_run(command, **kwargs):
        calls.append((command, kwargs))
        return Result()

    try:
        server._DSN = "postgresql://user:pw@db.example.test/deadlock"
        server.subprocess.run = fake_run
        assert server._query_rows("SELECT 1 AS ok LIMIT :limit", {"limit": 1}) == []
    finally:
        server.subprocess.run = original_run
        server._DSN = original_dsn

    command, kwargs = calls[0]
    assert "-c" not in command
    assert "SELECT 1 AS ok LIMIT :limit" in kwargs["input"]


def test_database_contract_requires_isolated_infrastructure():
    dsn = os.environ["BRAIN_TEST_DATABASE_URL"]
    parsed = urlparse(dsn)
    assert parsed.hostname in {"127.0.0.1", "localhost", "::1"}
    assert set(parsed.path.lstrip("/").split("_")) & {"ci", "test"}
    server = load_server()
    server._DSN = dsn
    server._DSN_ERROR = None
    # Synthetic fixtures on the genuine versioned schema and runtime view.
    fixture = """
    WITH inserted AS (
      INSERT INTO brain.patch_events (
        legacy_patch_snapshot_id, patch_external_id, patch_title, patch_url,
        source_kind, posted_at, line_index, entity_type, entity_name, change_type,
        raw_line, normalized_line, old_value, new_value, confidence, event_hash, created_at
      ) SELECT 0, 'ci-mcp-' || ord, 'CI fixture ' || ord, 'https://example.invalid/' || ord,
        'ci', day::timestamptz, 0, 'hero', 'Holliday', 'changed',
        'Powder Keg spirit scaling changed', 'Powder Keg spirit scaling changed',
        old, new, 1.0, 'ci-mcp-' || ord, '2026-07-10'::timestamptz
      FROM (VALUES
        (1, '2025-09-04', '1.4', '1.6'), (2, '2026-06-12', '1.6', '1.4'),
        (3, '2026-06-30', '1.4', '1.2'), (4, '2026-07-09', '1.2', '1.05'),
        (5, '2026-07-10', '1.05', '1.0')
      ) AS data(ord, day, old, new)
      RETURNING id, old_value, new_value
    ) INSERT INTO brain.patch_event_enrichments (
      patch_event_id, legacy_patch_event_id, ability_name, stat_name,
      old_value, new_value, confidence, created_at, updated_at
    ) SELECT id, 0, 'Powder Keg', 'spirit scaling', old_value, new_value,
      1.0, now(), now() FROM inserted;
    """
    cleanup = "DELETE FROM brain.patch_events WHERE event_hash LIKE 'ci-mcp-%';"

    def execute(sql):
        subprocess.run(
            ["psql", dsn, "-X", "-v", "ON_ERROR_STOP=1"],
            input=sql, text=True, capture_output=True, check=True,
        )

    execute(cleanup)
    try:
        execute(fixture)
        history = server.patch_history("Holliday")
        assert len(history) == 5
        assert [(row["patch_date"], row["old_value"], row["new_value"])
                for row in history[:4]] == [
            ("2025-09-04", "1.4", "1.6"), ("2026-06-12", "1.6", "1.4"),
            ("2026-06-30", "1.4", "1.2"), ("2026-07-09", "1.2", "1.05"),
        ]
        assert len(server.list_patches(5)) == 5
        assert len(server.patch_search("Powder Keg")) == 5
        assert server.entity_summary("Holliday")["change_count"] == 5
        assert server.patch_history("Holliday' OR 1=1 --") == []
        assert server._query_rows(
            "SELECT current_setting('transaction_read_only') AS read_only"
        ) == [{"read_only": "on"}]
    finally:
        execute(cleanup)


@pytest.mark.parametrize("value", ["", "not-a-date", "2026-13-40"])
def test_invalid_dates_are_rejected(value):
    with pytest.raises(ValueError):
        load_server()._require_date(value)


def test_unavailable_database_is_an_error_not_empty_success():
    server = load_server()
    server._DSN = None
    with pytest.raises(RuntimeError):
        server.patch_history("Holliday")
