from __future__ import annotations

import json
import re
import sqlite3
from collections import OrderedDict
from datetime import datetime, timezone
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias
from deadlock_brain.legacy_entities import legacy_lookup_names
from deadlock_brain.lineage import lineage_lookup_names, related_names_for_query


MAX_EVENTS = 2000
MAX_ALIASES = 120
MIN_ENTITY_MATCH_SCORE = 100

IMPACT_KINDS = {
    "numeric_buff",
    "numeric_nerf",
    "functional_change",
    "rework",
    "bugfix",
    "added",
    "removed",
    "unknown",
}
IMPACT_LEVELS = {"high", "medium", "low", "unknown"}

_NUMBER_RE = re.compile(r"[-+]?\d+(?:\.\d+)?")
_BUFF_WORDS = {
    "buff",
    "increased",
    "increase",
    "raised",
    "improved",
    "faster",
    "stronger",
}
_NERF_WORDS = {
    "nerf",
    "reduced",
    "decreased",
    "lowered",
    "slower",
    "weaker",
}
_FUNCTIONAL_WORDS = {
    "now",
    "changed",
    "adjusted",
    "upgrades from",
    "no longer grants",
    "grants",
}
_HIGH_IMPACT_WORDS = {"crash", "exploit", "reworked", "new hero", "new item"}


def build_entity_timeline(
    conn: sqlite3.Connection,
    query: str,
    *,
    limit_events: int = MAX_EVENTS,
    ascending: bool = True,
) -> dict[str, Any]:
    """Build a deterministic patch timeline for one entity-oriented query.

    The result groups matching ``patch_events`` rows by patch snapshot and adds
    ``impact_kind`` plus ``impact_level`` to every event. It only reads existing
    tables and uses ``patch_event_enrichments`` when the table is present.
    """
    query = query.strip()
    query_norm = normalize_alias(query)
    limit = max(1, min(_safe_int(limit_events, default=MAX_EVENTS), MAX_EVENTS))

    best_match = _find_best_entity_match(conn, query, query_norm)
    aliases = _load_aliases(conn, int(best_match["id"])) if best_match and best_match.get("id") is not None else []
    lineage = related_names_for_query(conn, query, best_match)
    lineage_names = sorted(set(lineage_lookup_names(conn, query, best_match)) | set(legacy_lookup_names(conn, query)))
    events = _load_patch_events(
        conn,
        query=query,
        best_match=best_match,
        aliases=aliases,
        lineage_names=lineage_names,
        limit=limit,
        ascending=ascending,
    )
    enrichments = _load_enrichments(conn, [int(event["id"]) for event in events if event.get("id") is not None])

    classified_events = []
    for event in events:
        enrichment = enrichments.get(int(event["id"])) if event.get("id") is not None else None
        impact = classify_patch_event_impact(event, enrichment)
        classified_events.append({**event, "enrichment": enrichment, **impact})

    classified_events.sort(key=_event_sort_key)
    patches = _group_events_by_patch(classified_events, ascending=ascending)
    return {
        "query": query,
        "query_norm": query_norm,
        "best_match": best_match,
        "aliases": aliases,
        "lineage": lineage,
        "patches": patches,
        "event_count": len(classified_events),
        "patch_count": len(patches),
        "impact_summary": _impact_summary(classified_events),
        "enrichments": {
            "available": _table_exists(conn, "patch_event_enrichments"),
            "matched": sum(1 for event in classified_events if event.get("enrichment")),
        },
        "fallback": {
            "used": best_match is None,
            "strategy": "patch_events.entity_name LIKE" if best_match is None else None,
        },
    }


def classify_patch_event_impact(
    event: dict[str, Any],
    enrichment: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Classify one patch event into a stable impact kind and level."""
    enrichment = enrichment or {}
    change_type = str(event.get("change_type") or "").casefold()
    line = str(event.get("normalized_line") or event.get("raw_line") or "")
    lower_line = line.casefold()
    flags = _coerce_flags(enrichment.get("flags"))
    stat_name = str(enrichment.get("stat_name") or "").casefold()
    confidence = _safe_float(enrichment.get("confidence"), default=_safe_float(event.get("confidence"), default=0.0))

    if change_type == "added" or "added" in lower_line or "new item" in lower_line or "new hero" in lower_line:
        return _impact("added", _non_numeric_level("added", lower_line, confidence), confidence, flags)
    if change_type == "removed" or "removed" in lower_line or "no longer" in lower_line or "removed_grant" in flags:
        return _impact("removed", _non_numeric_level("removed", lower_line, confidence), confidence, flags)
    if change_type == "bugfix" or "fixed" in lower_line or "bug" in lower_line:
        return _impact("bugfix", _non_numeric_level("bugfix", lower_line, confidence), confidence, flags)
    if change_type == "rework" or "reworked" in lower_line or "rescaled" in lower_line or "moved from" in lower_line:
        return _impact("rework", _non_numeric_level("rework", lower_line, confidence), confidence, flags)
    if "movement" in flags or stat_name == "item_tier":
        return _impact("rework", _non_numeric_level("rework", lower_line, confidence), confidence, flags)

    old_value = _first_present(enrichment.get("old_value"), event.get("old_value"))
    new_value = _first_present(enrichment.get("new_value"), event.get("new_value"))
    numeric_delta = _numeric_delta(old_value, new_value)

    if change_type == "buff" or any(word in lower_line for word in _BUFF_WORDS) or _has_direction(flags, "increased"):
        kind = "numeric_buff" if numeric_delta is not None else "functional_change"
        return _impact(kind, _level_for_kind(kind, lower_line, confidence, numeric_delta), confidence, flags)
    if change_type == "nerf" or any(word in lower_line for word in _NERF_WORDS) or _has_direction(flags, "reduced"):
        kind = "numeric_nerf" if numeric_delta is not None else "functional_change"
        return _impact(kind, _level_for_kind(kind, lower_line, confidence, numeric_delta), confidence, flags)

    if numeric_delta is not None:
        if numeric_delta["delta"] > 0:
            return _impact("numeric_buff", _level_for_kind("numeric_buff", lower_line, confidence, numeric_delta), confidence, flags)
        if numeric_delta["delta"] < 0:
            return _impact("numeric_nerf", _level_for_kind("numeric_nerf", lower_line, confidence, numeric_delta), confidence, flags)

    if change_type == "changed" or any(word in lower_line for word in _FUNCTIONAL_WORDS):
        return _impact("functional_change", _level_for_kind("functional_change", lower_line, confidence, None), confidence, flags)
    if stat_name in {"upgrades_from"} or (stat_name and (old_value is not None or new_value is not None)):
        return _impact("functional_change", _level_for_kind("functional_change", lower_line, confidence, None), confidence, flags)

    return _impact("unknown", "unknown", confidence, flags)


def _find_best_entity_match(conn: sqlite3.Connection, query: str, query_norm: str) -> dict[str, Any] | None:
    if not query_norm or not _table_exists(conn, "entities") or not _table_exists(conn, "entity_aliases"):
        return None

    like_norm = f"%{query_norm}%"
    rows = _fetch_all_dicts(
        conn,
        """
        SELECT
          e.id,
          e.entity_type,
          e.canonical_name,
          e.primary_external_id,
          e.source,
          e.metadata_json,
          MAX(
            CASE
              WHEN lower(e.canonical_name)=lower(?) THEN 120
              WHEN a.alias_norm=? AND a.alias_kind='canonical' THEN 115
              WHEN a.alias_norm=? THEN 110
              WHEN a.alias_norm LIKE ? THEN 80
              WHEN lower(e.canonical_name) LIKE lower(?) THEN 70
              ELSE 50
            END
          ) AS score,
          GROUP_CONCAT(DISTINCT a.alias_kind) AS matched_alias_kinds
        FROM entities e
        LEFT JOIN entity_aliases a ON a.entity_id=e.id
        WHERE
          lower(e.canonical_name)=lower(?)
          OR lower(e.canonical_name) LIKE lower(?)
          OR a.alias_norm=?
          OR a.alias_norm LIKE ?
        GROUP BY e.id
        ORDER BY score DESC, e.entity_type, length(e.canonical_name), e.canonical_name
        LIMIT 1
        """,
        (
            query,
            query_norm,
            query_norm,
            like_norm,
            f"%{query}%",
            query,
            f"%{query}%",
            query_norm,
            like_norm,
        ),
    )
    if not rows:
        return None

    row = rows[0]
    if int(row["score"] or 0) < MIN_ENTITY_MATCH_SCORE:
        return None
    row["metadata"] = _loads_json_object(row.pop("metadata_json", None))
    row["score"] = int(row["score"] or 0)
    row["matched_alias_kinds"] = _split_group_concat(row.get("matched_alias_kinds"))
    return row


def _load_aliases(conn: sqlite3.Connection, entity_id: int) -> list[dict[str, Any]]:
    rows = _fetch_all_dicts(
        conn,
        """
        SELECT alias, alias_norm, alias_kind, source, external_id, snapshot_id
        FROM entity_aliases
        WHERE entity_id=?
        ORDER BY
          CASE alias_kind
            WHEN 'canonical' THEN 0
            WHEN 'snapshot_name' THEN 1
            WHEN 'class_name' THEN 2
            WHEN 'class_name_short' THEN 3
            WHEN 'external_id' THEN 4
            ELSE 5
          END,
          length(alias),
          alias
        LIMIT ?
        """,
        (entity_id, MAX_ALIASES),
    )
    return rows


def _load_patch_events(
    conn: sqlite3.Connection,
    *,
    query: str,
    best_match: dict[str, Any] | None,
    aliases: list[dict[str, Any]],
    lineage_names: list[str],
    limit: int,
    ascending: bool,
) -> list[dict[str, Any]]:
    if not _table_exists(conn, "patch_events"):
        return []

    params: list[Any] = []
    if best_match:
        names = sorted(set(_event_lookup_names(best_match, aliases)) | {name.casefold() for name in lineage_names})
        placeholders = ",".join("?" for _ in names)
        where = f"lower(entity_name) IN ({placeholders})"
        params.extend(names)
    elif lineage_names:
        names = sorted({name.casefold() for name in lineage_names})
        placeholders = ",".join("?" for _ in names)
        where = f"(lower(entity_name) IN ({placeholders}) OR entity_name LIKE ?)"
        params.extend(names)
        params.append(f"%{query}%")
    else:
        where = "entity_name LIKE ?"
        params.append(f"%{query}%")

    direction = "ASC" if ascending else "DESC"
    params.append(limit)
    rows = _fetch_all_dicts(
        conn,
        f"""
        SELECT
          id,
          patch_snapshot_id,
          patch_external_id,
          patch_title,
          patch_url,
          source_kind,
          posted_at,
          line_index,
          section,
          entity_type,
          entity_name,
          subject,
          change_type,
          raw_line,
          normalized_line,
          old_value,
          new_value,
          confidence,
          metadata_json,
          event_hash,
          created_at
        FROM patch_events
        WHERE {where}
        ORDER BY COALESCE(posted_at, '') {direction}, patch_snapshot_id {direction}, line_index {direction}
        LIMIT ?
        """,
        tuple(params),
    )
    for row in rows:
        row["metadata"] = _loads_json_object(row.pop("metadata_json", None))
    return rows


def _event_lookup_names(best_match: dict[str, Any], aliases: list[dict[str, Any]]) -> list[str]:
    names = {str(best_match["canonical_name"]).casefold()}
    for alias in aliases:
        alias_kind = str(alias.get("alias_kind") or "")
        if alias_kind in {"canonical", "snapshot_name", "class_name_short"}:
            value = str(alias.get("alias") or "").strip()
            if value:
                names.add(value.casefold())
    return sorted(names)


def _load_enrichments(conn: sqlite3.Connection, event_ids: list[int]) -> dict[int, dict[str, Any]]:
    if not event_ids or not _table_exists(conn, "patch_event_enrichments"):
        return {}

    columns = _table_columns(conn, "patch_event_enrichments")
    if "patch_event_id" not in columns:
        return {}

    rows = _select_by_values(conn, "patch_event_enrichments", "patch_event_id", event_ids)
    enrichments: dict[int, dict[str, Any]] = {}
    for row in rows:
        decoded = _decode_json_fields(row)
        patch_event_id = decoded.get("patch_event_id")
        if patch_event_id is None:
            continue
        enrichments[int(patch_event_id)] = decoded
    return enrichments


def _group_events_by_patch(events: list[dict[str, Any]], *, ascending: bool) -> list[dict[str, Any]]:
    grouped: OrderedDict[int, dict[str, Any]] = OrderedDict()
    for event in events:
        patch_snapshot_id = int(event["patch_snapshot_id"])
        patch = grouped.setdefault(
            patch_snapshot_id,
            {
                "patch_snapshot_id": patch_snapshot_id,
                "patch_external_id": event.get("patch_external_id"),
                "title": event.get("patch_title"),
                "url": event.get("patch_url"),
                "date": event.get("posted_at"),
                "source": event.get("source_kind"),
                "events": [],
            },
        )
        patch["events"].append(event)

    patches = list(grouped.values())
    if not ascending:
        patches.reverse()
    return patches


def _impact_summary(events: list[dict[str, Any]]) -> dict[str, Any]:
    by_kind: dict[str, int] = {}
    by_level: dict[str, int] = {}
    for event in events:
        kind = str(event.get("impact_kind") or "unknown")
        level = str(event.get("impact_level") or "unknown")
        by_kind[kind] = by_kind.get(kind, 0) + 1
        by_level[level] = by_level.get(level, 0) + 1
    return {
        "by_kind": dict(sorted(by_kind.items())),
        "by_level": dict(sorted(by_level.items())),
    }


def _event_sort_key(event: dict[str, Any]) -> tuple[float, int, int]:
    return (
        _date_sort_value(event.get("posted_at")),
        _safe_int(event.get("patch_snapshot_id"), default=0),
        _safe_int(event.get("line_index"), default=0),
    )


def _date_sort_value(value: Any) -> float:
    if value in (None, ""):
        return 0.0
    text = str(value).strip()
    try:
        return float(text)
    except ValueError:
        pass

    for fmt in ("%Y-%m-%dT%H:%M:%S%z", "%Y-%m-%d %H:%M:%S%z", "%Y-%m-%d"):
        try:
            parsed = datetime.strptime(text, fmt)
            if parsed.tzinfo is None:
                parsed = parsed.replace(tzinfo=timezone.utc)
            return parsed.timestamp()
        except ValueError:
            continue

    try:
        return datetime.fromisoformat(text).timestamp()
    except ValueError:
        return 0.0


def _impact(kind: str, level: str, confidence: float, flags: list[str]) -> dict[str, Any]:
    if kind not in IMPACT_KINDS:
        kind = "unknown"
    if level not in IMPACT_LEVELS:
        level = "unknown"
    return {
        "impact_kind": kind,
        "impact_level": level,
        "impact_confidence": round(max(0.0, min(confidence, 1.0)), 3),
        "impact_flags": flags,
    }


def _level_for_kind(
    kind: str,
    lower_line: str,
    confidence: float,
    numeric_delta: dict[str, float] | None,
) -> str:
    if confidence <= 0 and kind not in {"functional_change", "bugfix", "added", "removed", "rework"}:
        return "unknown"
    if kind == "bugfix":
        return "medium" if any(word in lower_line for word in {"crash", "exploit"}) else "low"
    if any(word in lower_line for word in _HIGH_IMPACT_WORDS):
        return "high"
    if kind in {"rework", "added", "removed"}:
        return "high"
    if kind == "functional_change":
        return "medium"
    if not numeric_delta:
        return "low"

    relative = abs(numeric_delta.get("relative") or 0.0)
    if relative >= 0.25:
        return "high"
    if relative >= 0.10:
        return "medium"
    return "low"


def _non_numeric_level(kind: str, lower_line: str, confidence: float) -> str:
    return _level_for_kind(kind, lower_line, confidence, None)


def _numeric_delta(old_value: Any, new_value: Any) -> dict[str, float] | None:
    old_number = _extract_number(old_value)
    new_number = _extract_number(new_value)
    if old_number is None or new_number is None:
        return None

    delta = new_number - old_number
    relative = abs(delta / old_number) if old_number else 0.0
    return {"old": old_number, "new": new_number, "delta": delta, "relative": relative}


def _extract_number(value: Any) -> float | None:
    if value is None:
        return None
    match = _NUMBER_RE.search(str(value))
    if not match:
        return None
    try:
        return float(match.group(0))
    except ValueError:
        return None


def _has_direction(flags: list[str], direction: str) -> bool:
    return any(flag == f"direction:{direction}" for flag in flags)


def _first_present(*values: Any) -> Any:
    for value in values:
        if value not in (None, ""):
            return value
    return None


def _coerce_flags(value: Any) -> list[str]:
    if value is None:
        return []
    if isinstance(value, list):
        return sorted(str(item) for item in value)
    if isinstance(value, tuple):
        return sorted(str(item) for item in value)
    return [str(value)]


def _safe_float(value: Any, *, default: float) -> float:
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def _safe_int(value: Any, *, default: int) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def _select_by_values(conn: sqlite3.Connection, table: str, column: str, values: list[Any]) -> list[dict[str, Any]]:
    placeholders = ",".join("?" for _ in values)
    return _fetch_all_dicts(
        conn,
        f"""
        SELECT *
        FROM {_quote_identifier(table)}
        WHERE {_quote_identifier(column)} IN ({placeholders})
        """,
        tuple(values),
    )


def _table_exists(conn: sqlite3.Connection, name: str) -> bool:
    row = _fetch_one_dict(
        conn,
        "SELECT 1 AS found FROM sqlite_master WHERE type='table' AND name=?",
        (name,),
    )
    return row is not None


def _table_columns(conn: sqlite3.Connection, table: str) -> list[str]:
    rows = _fetch_all_dicts(conn, f"PRAGMA table_info({_quote_identifier(table)})")
    return [str(row["name"]) for row in rows]


def _fetch_one_dict(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> dict[str, Any] | None:
    rows = _fetch_all_dicts(conn, sql, params)
    return rows[0] if rows else None


def _fetch_all_dicts(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> list[dict[str, Any]]:
    cursor = conn.execute(sql, params)
    columns = [description[0] for description in cursor.description]
    return [dict(zip(columns, row)) for row in cursor.fetchall()]


def _loads_json_object(value: Any) -> dict[str, Any]:
    if not value:
        return {}
    try:
        parsed = json.loads(str(value))
    except json.JSONDecodeError:
        return {}
    return parsed if isinstance(parsed, dict) else {}


def _loads_json_list(value: Any) -> list[Any]:
    if not value:
        return []
    try:
        parsed = json.loads(str(value))
    except json.JSONDecodeError:
        return []
    return parsed if isinstance(parsed, list) else []


def _decode_json_fields(row: dict[str, Any]) -> dict[str, Any]:
    decoded = dict(row)
    for key, value in list(decoded.items()):
        if not key.endswith("_json") or not isinstance(value, str):
            continue
        target = key.removesuffix("_json")
        decoded[target] = _loads_json_list(value) if target == "flags" else _loads_json_object(value)
        del decoded[key]
    return decoded


def _split_group_concat(value: Any) -> list[str]:
    if not value:
        return []
    return sorted({part for part in str(value).split(",") if part})


def _quote_identifier(value: str) -> str:
    return '"' + value.replace('"', '""') + '"'
