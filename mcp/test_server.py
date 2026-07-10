#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import os
from pathlib import Path


ROOT = Path(__file__).resolve().parent
SERVER_PATH = ROOT / "server.py"


def load_server():
    spec = importlib.util.spec_from_file_location("dl_brain_mcp_server", SERVER_PATH)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
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


def test_brain_sql_rejects_drop_without_db_call():
    server = load_server()
    called = False
    original = server._query_rows
    try:
        def fake_query_rows(sql, variables):
            nonlocal called
            called = True
            return []

        server._query_rows = fake_query_rows
        try:
            server.brain_sql("DROP TABLE brain.patch_events")
        except ValueError as exc:
            assert "nur read-only SELECT erlaubt" in str(exc)
        else:
            raise AssertionError("DROP wurde nicht abgelehnt")
    finally:
        server._query_rows = original

    assert called is False


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


def test_live_contract_if_deadlock_central_dsn_is_present():
    if not os.environ.get("DEADLOCK_CENTRAL_DSN"):
        print("SKIP live DB: DEADLOCK_CENTRAL_DSN nicht gesetzt")
        return

    server = load_server()
    history = server.patch_history("Holliday")
    pairs = {
        (row["patch_date"], str(row["old_value"]), str(row["new_value"]))
        for row in history
        if "powder keg" in str(row.get("raw_line") or "").lower()
        and "spirit scaling" in str(row.get("raw_line") or "").lower()
    }
    assert ("2025-09-04", "1.4", "1.6") in pairs
    assert ("2026-06-12", "1.6", "1.4") in pairs
    assert ("2026-06-30", "1.4", "1.2") in pairs
    assert ("2026-07-09", "1.2", "1.05") in pairs
    assert all(row["entity_name"].lower() == "holliday" for row in server.patch_history("Holliday"))
    assert len(server.list_patches(5)) == 5


if __name__ == "__main__":
    test_patch_history_uses_exact_entity_match()
    test_brain_sql_rejects_drop_without_db_call()
    test_postgres_uri_dsn_is_passed_via_libpq_environment()
    test_query_rows_sends_sql_via_stdin_for_psql_variables()
    test_live_contract_if_deadlock_central_dsn_is_present()
    print("mcp/test_server.py: ok")
