from __future__ import annotations

import json
import sqlite3
from datetime import datetime, timezone
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias
from deadlock_brain.legacy_entities import legacy_lookup_names
from deadlock_brain.lineage import lineage_lookup_names, related_names_for_query

MAX_EVENTS = 500
MAX_ALIASES = 80
MAX_SHEET_ROWS_PER_SOURCE = 5
MIN_ENTITY_MATCH_SCORE = 100

_embedding_model = None

def _get_embedding_model():
    global _embedding_model
    if _embedding_model is None:
        from sentence_transformers import SentenceTransformer
        _embedding_model = SentenceTransformer('all-MiniLM-L6-v2')
    return _embedding_model

def get_embedding(text: str) -> list[float]:
    model = _get_embedding_model()
    return model.encode(text).tolist()

def search_mechanic_notes(conn: sqlite3.Connection, query: str, limit: int = 5) -> list[dict[str, Any]]:
    if not _table_exists(conn, "mechanic_notes") or not _table_exists(conn, "vector_embeddings"):
        return []
    
    query_emb = get_embedding(query)
    emb_bytes = json.dumps(query_emb).encode('utf-8')
    
    # Needs struct pack for sqlite-vec but sqlite-vec also accepts json array in newer versions or binary.
    # Actually sqlite-vec `vec_f32` or similar might be needed. 
    # We will use `json_array` representation which vec0 might understand if cast, or we just format it.
    
    # We can use vec_f32 function if available or just string cast.
    # Let's pass it as a JSON string and see if vec0 accepts it in vec_distance_l2
    try:
        cursor = conn.execute(
            """
            SELECT
                mn.id, mn.title, mn.content, mn.source, mn.category,
                vec_distance_L2(ve.embedding, ?) AS distance
            FROM vector_embeddings ve
            JOIN mechanic_notes mn ON mn.rowid = ve.rowid
            ORDER BY distance ASC
            LIMIT ?
            """,
            (json.dumps(query_emb), limit)
        )
        return [dict(row) for row in cursor.fetchall()]
    except Exception as e:
        print(f"Warnung: Vektor-Suche fehlgeschlagen: {e}")
        # Fallback to text search
        cursor = conn.execute(
            "SELECT id, title, content, source, category FROM mechanic_notes WHERE title LIKE ? OR content LIKE ? LIMIT ?",
            (f"%{query}%", f"%{query}%", limit)
        )
        rows = cursor.fetchall()
        return [{"id": r[0], "title": r[1], "content": r[2], "source": r[3], "category": r[4], "distance": 0.0} for r in rows]

def build_entity_context(conn: sqlite3.Connection, query: str, limit_events: int = 30) -> dict[str, Any]:
    """Build a compact, deterministic context bundle for one entity-oriented query."""
    query = query.strip()
    query_norm = normalize_alias(query)
    limit = max(1, min(int(limit_events), MAX_EVENTS))

    best_match = _find_best_entity_match(conn, query, query_norm)
    fallback_used = best_match is None
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
    )
    event_ids = [int(event["id"]) for event in events if event.get("id") is not None]
    event_hashes = [str(event["event_hash"]) for event in events if event.get("event_hash")]

    return {
        "query": query,
        "query_norm": query_norm,
        "best_match": best_match,
        "aliases": aliases,
        "lineage": lineage,
        "patch_events": events,
        "enrichments": _load_enrichments(conn, event_ids=event_ids, event_hashes=event_hashes),
        "sheet_stats": _load_sheet_stats(conn, query=query, best_match=best_match, aliases=aliases),
        "fallback": {
            "used": fallback_used,
            "strategy": "patch_events.entity_name LIKE" if fallback_used else None,
        },
    }


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
    metadata = _loads_json_object(row.pop("metadata_json", None))
    row["metadata"] = metadata
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
) -> list[dict[str, Any]]:
    if not _table_exists(conn, "patch_events"):
        return []

    params: list[Any] = []
    where: str
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

    params.append(MAX_EVENTS)
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
        ORDER BY patch_snapshot_id DESC, line_index
        LIMIT ?
        """,
        tuple(params),
    )
    for row in rows:
        row["metadata"] = _loads_json_object(row.pop("metadata_json", None))
    rows.sort(key=_event_sort_key, reverse=True)
    return rows[:limit]


def _event_lookup_names(best_match: dict[str, Any], aliases: list[dict[str, Any]]) -> list[str]:
    names = {str(best_match["canonical_name"]).casefold()}
    for alias in aliases:
        alias_kind = str(alias.get("alias_kind") or "")
        if alias_kind in {"canonical", "snapshot_name", "class_name_short"}:
            value = str(alias.get("alias") or "").strip()
            if value:
                names.add(value.casefold())
    return sorted(names)


def _load_enrichments(
    conn: sqlite3.Connection,
    *,
    event_ids: list[int],
    event_hashes: list[str],
) -> dict[str, Any]:
    table = "patch_event_enrichments"
    if not _table_exists(conn, table):
        return {"available": False, "rows": []}

    columns = _table_columns(conn, table)
    rows: list[dict[str, Any]] = []
    if "patch_event_id" in columns and event_ids:
        rows.extend(_select_by_values(conn, table, "patch_event_id", event_ids))
    elif "event_id" in columns and event_ids:
        rows.extend(_select_by_values(conn, table, "event_id", event_ids))
    elif "event_hash" in columns and event_hashes:
        rows.extend(_select_by_values(conn, table, "event_hash", event_hashes))

    return {
        "available": True,
        "rows": [_decode_json_fields(row) for row in _dedupe_rows(rows)],
    }


def _load_sheet_stats(
    conn: sqlite3.Connection,
    *,
    query: str,
    best_match: dict[str, Any] | None,
    aliases: list[dict[str, Any]],
) -> dict[str, Any]:
    names = _sheet_lookup_names(query=query, best_match=best_match, aliases=aliases)
    sources = []

    snapshot_rows = _load_sheet_snapshot_stats(conn, names)
    if snapshot_rows:
        sources.append({"source": "entity_snapshots", "rows": snapshot_rows})

    for table in _candidate_sheet_tables(conn):
        rows = _load_matching_sheet_table_rows(conn, table, names)
        if rows:
            sources.append({"source": table, "rows": rows})

    return {"available": bool(sources), "sources": sources}


def _load_sheet_snapshot_stats(conn: sqlite3.Connection, names: list[str]) -> list[dict[str, Any]]:
    if not _table_exists(conn, "entity_snapshots"):
        return []
    lowered = [name.casefold() for name in names]
    placeholders = ",".join("?" for _ in lowered)
    if not placeholders:
        return []
    rows = _fetch_all_dicts(
        conn,
        f"""
        SELECT id, source, entity_type, external_id, canonical_name, payload_json, fetched_at
        FROM entity_snapshots
        WHERE
          (source='deadlock_stats_sheet' OR entity_type='hero_stats_sheet')
          AND lower(canonical_name) IN ({placeholders})
        ORDER BY fetched_at DESC, id DESC
        LIMIT ?
        """,
        (*lowered, MAX_SHEET_ROWS_PER_SOURCE),
    )
    for row in rows:
        payload = _loads_json_object(row.pop("payload_json", None))
        row["values"] = payload.get("values", payload)
    return rows


def _candidate_sheet_tables(conn: sqlite3.Connection) -> list[str]:
    rows = _fetch_all_dicts(
        conn,
        """
        SELECT name
        FROM sqlite_master
        WHERE type='table'
          AND name NOT LIKE 'sqlite_%'
          AND name NOT IN (
            'source_documents',
            'source_runs',
            'entity_snapshots',
            'entities',
            'entity_aliases',
            'patch_events',
            'patch_event_enrichments'
          )
        ORDER BY name
        """,
    )
    return [
        str(row["name"])
        for row in rows
        if "sheet" in str(row["name"]).casefold() or "stat" in str(row["name"]).casefold()
    ]


def _load_matching_sheet_table_rows(conn: sqlite3.Connection, table: str, names: list[str]) -> list[dict[str, Any]]:
    columns = _table_columns(conn, table)
    if not columns:
        return []

    preferred = [
        column
        for column in columns
        if column.casefold() in {"entity_name", "canonical_name", "hero_name", "hero", "name"}
    ]
    text_columns = preferred or columns[: min(len(columns), 8)]
    clauses = []
    params: list[Any] = []
    for column in text_columns:
        for name in names:
            clauses.append(f"{_quote_identifier(column)} LIKE ?")
            params.append(f"%{name}%")
    if not clauses:
        return []

    params.append(MAX_SHEET_ROWS_PER_SOURCE)
    rows = _fetch_all_dicts(
        conn,
        f"""
        SELECT *
        FROM {_quote_identifier(table)}
        WHERE {" OR ".join(clauses)}
        LIMIT ?
        """,
        tuple(params),
    )
    return [_decode_json_fields(row) for row in rows]


def _sheet_lookup_names(
    *,
    query: str,
    best_match: dict[str, Any] | None,
    aliases: list[dict[str, Any]],
) -> list[str]:
    names = {query.strip()}
    if best_match and best_match.get("canonical_name"):
        names.add(str(best_match["canonical_name"]).strip())
    for alias in aliases:
        alias_kind = str(alias.get("alias_kind") or "")
        if alias_kind in {"canonical", "snapshot_name", "class_name_short"}:
            value = str(alias.get("alias") or "").strip()
            if value and not value.isdigit():
                names.add(value)
    return sorted(name for name in names if name)


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


def _event_sort_key(event: dict[str, Any]) -> tuple[float, int, int]:
    return (
        _date_sort_value(event.get("posted_at")),
        _int_or_zero(event.get("patch_snapshot_id")),
        _int_or_zero(event.get("line_index")),
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


def _int_or_zero(value: Any) -> int:
    try:
        return int(value)
    except (TypeError, ValueError):
        return 0


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


def _decode_json_fields(row: dict[str, Any]) -> dict[str, Any]:
    decoded = dict(row)
    for key, value in list(decoded.items()):
        if not key.endswith("_json") or not isinstance(value, str):
            continue
        decoded[key.removesuffix("_json")] = _loads_json_object(value)
        del decoded[key]
    return decoded


def _dedupe_rows(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    seen: set[tuple[tuple[str, str], ...]] = set()
    deduped = []
    for row in rows:
        key = tuple(sorted((str(k), repr(v)) for k, v in row.items()))
        if key in seen:
            continue
        seen.add(key)
        deduped.append(row)
    return deduped


def _split_group_concat(value: Any) -> list[str]:
    if not value:
        return []
    return sorted({part for part in str(value).split(",") if part})


def _quote_identifier(value: str) -> str:
    return '"' + value.replace('"', '""') + '"'
