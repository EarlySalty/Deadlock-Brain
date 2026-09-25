import asyncio
import importlib.util
import os
from pathlib import Path
import subprocess

import pytest

ROOT = Path(__file__).resolve().parent


def load_server():
    spec = importlib.util.spec_from_file_location("brain_contract_server", ROOT / "server.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_real_sdk_registers_all_shared_tools_without_database(monkeypatch):
    monkeypatch.delenv("DEADLOCK_CENTRAL_DSN", raising=False)
    monkeypatch.setattr(subprocess, "run", lambda *args, **kwargs: pytest.fail("Module import must not read credentials or start a database query"))
    server = load_server()
    tools = asyncio.run(server.mcp.list_tools())
    names = {tool.name for tool in tools}
    assert names == {"patch_history", "patch_search", "list_patches", "entity_summary", "change_lookup", "patch_insight", "video_evidence"}


@pytest.fixture
def scratch_server():
    if os.environ.get("BRAIN_SCRATCH_TEST") != "1":
        pytest.skip("Isolierte Scratch-Postgres ist nicht aktiviert.")
    dsn = os.environ.get("DEADLOCK_CENTRAL_DSN", "")
    assert "@127.0.0.1:" in dsn and dsn.endswith("/brain_contract")
    return load_server()


def test_change_lookup_executes_alias_date_pagination_and_index_time(scratch_server):
    server = scratch_server
    first = server._history_module.lookup_changes(server._query_rows, "cooldown", "Fixture Alias", limit=1)
    assert first["match_count"] == 2
    assert first["earliest_matching_patch"] == "2026-09-01"
    assert first["latest_matching_patch"] == "2026-09-02"
    assert first["changes"][0]["patch_date"] == "2026-09-01"
    assert first["changes"][0]["first_indexed_at"].startswith("2026-09-03")
    assert first["next_offset"] == 1
    second = server._history_module.lookup_changes(server._query_rows, "cooldown", "Fixture Hero", limit=1, offset=1)
    assert second["changes"][0]["old_value"] == "12s"
    assert second["next_offset"] is None
    bounded = server._history_module.lookup_changes(server._query_rows, "cooldown", until="2026-09-01")
    assert bounded["match_count"] == 1
    empty = server._history_module.lookup_changes(server._query_rows, "35%")
    assert empty["match_count"] == 0


def test_new_read_paths_return_empty_from_real_tables(scratch_server):
    server = scratch_server
    assert server._history_module.read_insight(server._query_rows, "https://example.invalid/no-analysis")["found"] is False
    assert server._history_module.read_video_evidence(server._query_rows, "fixture_missing")["found"] is False
