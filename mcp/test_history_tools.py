import importlib.util
from pathlib import Path

import pytest

spec = importlib.util.spec_from_file_location("history_tools", Path(__file__).with_name("history_tools.py"))
history = importlib.util.module_from_spec(spec)
spec.loader.exec_module(history)


def fake_query(calls, changes=None, count=None):
    changes = [] if changes is None else changes
    def query(sql, variables):
        calls.append((sql, variables))
        return [{"match_count": len(changes) if count is None else count,
                 "earliest_matching_patch": None, "latest_matching_patch": None,
                 "changes": changes}]
    return query


def test_aliases_resolve_without_interpolating_entity():
    calls = []
    name = "Holliday'; DROP TABLE brain.patch_events; --"
    result = history.lookup_changes(fake_query(calls), "reload stun", name)
    sql, variables = calls[0]
    assert name not in sql
    assert variables["entity"] == name
    assert "brain.entity_aliases" in sql
    assert "SELECT name FROM requested_names" in sql
    assert "term0" in sql and "term1" in sql
    assert result["next_offset"] is None


def test_percent_underscore_and_escape_are_literals():
    calls = []
    history.lookup_changes(fake_query(calls), "35% x_y hello!")
    sql, variables = calls[0]
    assert variables["term0"] == "%35!%%"
    assert variables["term1"] == "%x!_y%"
    assert variables["term2"] == "%hello!!%"
    assert "ESCAPE '!'" in sql


@pytest.mark.parametrize("value", ["20260918", "2026-02-30", "bad", "", "2026-09-18T12:00:00"])
def test_invalid_dates_are_rejected(value):
    with pytest.raises(ValueError):
        history.lookup_changes(fake_query([]), "reload", since=value)


def test_date_window_and_earliest_scope():
    calls = []
    result = history.lookup_changes(fake_query(calls), "reload", since="2026-09-01", until="2026-09-18")
    sql, _ = calls[0]
    assert "patch_date >= :'since'::date" in sql
    assert "patch_date <= :'until'::date" in sql
    assert "Suchfenster" in result["coverage"]
    with pytest.raises(ValueError):
        history.lookup_changes(fake_query([]), "reload", since="2026-09-19", until="2026-09-18")


def test_pagination_and_bounded_limit():
    calls = []
    result = history.lookup_changes(fake_query(calls, [{"raw_line": "x"}], 5), "reload", limit=9999, offset=2)
    assert calls[0][1]["limit"] == 200
    assert result["next_offset"] == 3
    assert "NULLS LAST" in calls[0][0]
    result = history.lookup_changes(fake_query([], [], 5), "reload", offset=10)
    assert result["next_offset"] is None


def test_index_time_is_not_patch_time_or_first_ever_change():
    result = history.lookup_changes(fake_query([]), "reload")
    assert "Rebuilds" in result["date_semantics"]["first_indexed_at"]
    assert "Live-Zeitpunkt" in result["date_semantics"]["patch_date"]
    assert "keine Garantie" in result["coverage"]


@pytest.mark.parametrize("value", ["", " " * 5, "a" * 257, " ".join(str(i) for i in range(13))])
def test_invalid_queries(value):
    with pytest.raises(ValueError):
        history.lookup_changes(fake_query([]), value)


def test_invalid_database_response_is_not_reported_as_empty():
    with pytest.raises(RuntimeError):
        history.lookup_changes(lambda *_: [], "reload")


def test_registration_and_insight_authority():
    tools = {}
    class MCP:
        def tool(self, **kwargs):
            def decorate(fn):
                tools[kwargs["name"]] = fn
                return fn
            return decorate
    history.register_tools(MCP(), lambda *_: [])
    assert set(tools) == {"change_lookup", "patch_insight", "video_evidence"}
    result = tools["patch_insight"]("https://example.test/patch")
    assert result["found"] is False
    assert result["current_patch_verified"] is False


def test_video_evidence_checks_current_transcript_hash():
    calls = []
    def query(sql, variables):
        calls.append((sql, variables))
        return []
    result = history.read_video_evidence(query, "ZWm7ixeWjbQ", "35%")
    assert "t.content_hash=e.transcript_hash" in calls[0][0]
    assert calls[0][1]["text"] == "%35!%%"
    assert result["found"] is False
    assert "not_visual" in result["evidence_type"]


@pytest.mark.parametrize("video_id", ["../private", "", "a" * 129, "äbc"])
def test_bad_video_ids_are_rejected(video_id):
    with pytest.raises(ValueError):
        history.read_video_evidence(lambda *_: [], video_id)


def test_insight_read_detects_source_revisions_without_mutating_history():
    calls = []
    def query(sql, variables):
        calls.append((sql, variables))
        return []
    history.read_insight(query, "https://example.test/patch")
    sql = calls[0][0]
    assert "source_revision_current" in sql
    assert "newer_indexed_patch_exists" in sql
    assert "ELSE 'stale'" in sql
    assert "UPDATE" not in sql
    assert "DELETE" not in sql
