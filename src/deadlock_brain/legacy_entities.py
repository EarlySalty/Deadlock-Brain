from __future__ import annotations

import json
import re
import sqlite3
import time
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias


LEGACY_TYPES = {
    "hero": "legacy_hero",
    "item": "legacy_item",
    "item_special": "legacy_item",
    "ability": "legacy_ability",
    "ability_internal": "legacy_ability",
}

SUSPICIOUS_PREFIXES = (
    "added ",
    "removed ",
    "improved ",
    "the following ",
)


def build_legacy_entities(conn: sqlite3.Connection, rebuild: bool = False) -> dict[str, Any]:
    """Build legacy entities seen in patch notes but absent from current API entities."""
    ensure_legacy_entities_table(conn)
    before_total = _count_legacy(conn)
    deleted = 0
    if rebuild:
        cursor = conn.execute("DELETE FROM legacy_entities")
        deleted = int(cursor.rowcount if cursor.rowcount is not None else 0)

    known = _known_names(conn)
    grouped: dict[tuple[str, str], dict[str, Any]] = {}
    rows = _fetch_all_dicts(
        conn,
        """
        SELECT id, patch_external_id, patch_title, patch_url, source_kind, posted_at,
               entity_type, entity_name, raw_line, normalized_line
        FROM patch_events
        WHERE entity_type <> 'general'
          AND entity_name IS NOT NULL
          AND TRIM(entity_name) <> ''
        ORDER BY id ASC
        """,
    )
    for row in rows:
        entity_type = str(row.get("entity_type") or "")
        name = _clean_name(row.get("entity_name"))
        name_norm = normalize_alias(name)
        if not name_norm or (entity_type, name_norm) in known:
            continue
        legacy_type = LEGACY_TYPES.get(entity_type, f"legacy_{entity_type or 'entity'}")
        key = (legacy_type, name_norm)
        entry = grouped.setdefault(
            key,
            {
                "legacy_type": legacy_type,
                "canonical_name": name,
                "name_norm": name_norm,
                "observed_entity_type": entity_type,
                "first_patch_event_id": row.get("id"),
                "last_patch_event_id": row.get("id"),
                "first_seen_at": row.get("posted_at"),
                "last_seen_at": row.get("posted_at"),
                "event_count": 0,
                "confidence": _name_confidence(name, row),
                "status": "legacy_candidate",
                "samples": [],
            },
        )
        entry["event_count"] += 1
        entry["last_patch_event_id"] = row.get("id")
        entry["last_seen_at"] = row.get("posted_at") or entry.get("last_seen_at")
        entry["confidence"] = max(float(entry["confidence"]), _name_confidence(name, row))
        if len(entry["samples"]) < 5:
            entry["samples"].append(
                {
                    "patch_event_id": row.get("id"),
                    "patch_title": row.get("patch_title"),
                    "patch_url": row.get("patch_url"),
                    "posted_at": row.get("posted_at"),
                    "line": row.get("normalized_line") or row.get("raw_line"),
                }
            )

    inserted = 0
    by_type: dict[str, int] = {}
    for entry in grouped.values():
        if entry["confidence"] < 0.5:
            entry["status"] = "suspect_parser_subject"
        if _insert_legacy(conn, entry):
            inserted += 1
            by_type[entry["legacy_type"]] = by_type.get(entry["legacy_type"], 0) + 1

    conn.commit()
    return {
        "legacy_inserted": inserted,
        "legacy_before": before_total,
        "legacy_total": _count_legacy(conn),
        "deleted_before_build": deleted,
        "by_type": dict(sorted(by_type.items())),
        "rebuild": rebuild,
    }


def ensure_legacy_entities_table(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS legacy_entities (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          legacy_type TEXT NOT NULL,
          canonical_name TEXT NOT NULL,
          name_norm TEXT NOT NULL,
          observed_entity_type TEXT NOT NULL,
          first_patch_event_id INTEGER,
          last_patch_event_id INTEGER,
          first_seen_at TEXT,
          last_seen_at TEXT,
          event_count INTEGER NOT NULL,
          confidence REAL NOT NULL,
          status TEXT NOT NULL,
          samples_json TEXT NOT NULL DEFAULT '[]',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(legacy_type, name_norm),
          FOREIGN KEY(first_patch_event_id) REFERENCES patch_events(id) ON DELETE SET NULL,
          FOREIGN KEY(last_patch_event_id) REFERENCES patch_events(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_legacy_entities_name
          ON legacy_entities(name_norm);

        CREATE INDEX IF NOT EXISTS idx_legacy_entities_type
          ON legacy_entities(legacy_type, status);
        """
    )
    conn.commit()


def legacy_lookup_names(conn: sqlite3.Connection, query: str) -> list[str]:
    if not _table_exists(conn, "legacy_entities"):
        return []
    query_norm = normalize_alias(query)
    if not query_norm:
        return []
    rows = _fetch_all_dicts(
        conn,
        """
        SELECT canonical_name
        FROM legacy_entities
        WHERE name_norm=? OR name_norm LIKE ?
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT 40
        """,
        (query_norm, f"%{query_norm}%"),
    )
    return [str(row["canonical_name"]) for row in rows if row.get("canonical_name")]


def _known_names(conn: sqlite3.Connection) -> set[tuple[str, str]]:
    known: set[tuple[str, str]] = set()
    if _table_exists(conn, "entities"):
        for row in _fetch_all_dicts(conn, "SELECT entity_type, canonical_name FROM entities"):
            known.add((str(row["entity_type"]), normalize_alias(str(row["canonical_name"]))))
    if _table_exists(conn, "entity_aliases"):
        for row in _fetch_all_dicts(
            conn,
            """
            SELECT e.entity_type, a.alias_norm
            FROM entity_aliases a
            JOIN entities e ON e.id=a.entity_id
            """,
        ):
            known.add((str(row["entity_type"]), str(row["alias_norm"])))
    if _table_exists(conn, "entity_lineage"):
        for row in _fetch_all_dicts(conn, "SELECT source_entity_type, source_name_norm, target_entity_type, target_name_norm FROM entity_lineage"):
            for entity_type, name_norm in (
                (row.get("source_entity_type"), row.get("source_name_norm")),
                (row.get("target_entity_type"), row.get("target_name_norm")),
            ):
                if name_norm:
                    if entity_type:
                        known.add((str(entity_type), str(name_norm)))
                    else:
                        for fallback_type in ("hero", "item", "item_special", "ability"):
                            known.add((fallback_type, str(name_norm)))
    return known


def _insert_legacy(conn: sqlite3.Connection, entry: dict[str, Any]) -> bool:
    now = int(time.time())
    cursor = conn.execute(
        """
        INSERT INTO legacy_entities(
          legacy_type, canonical_name, name_norm, observed_entity_type,
          first_patch_event_id, last_patch_event_id, first_seen_at, last_seen_at,
          event_count, confidence, status, samples_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(legacy_type, name_norm) DO UPDATE SET
          canonical_name=excluded.canonical_name,
          observed_entity_type=excluded.observed_entity_type,
          first_patch_event_id=excluded.first_patch_event_id,
          last_patch_event_id=excluded.last_patch_event_id,
          first_seen_at=excluded.first_seen_at,
          last_seen_at=excluded.last_seen_at,
          event_count=excluded.event_count,
          confidence=excluded.confidence,
          status=excluded.status,
          samples_json=excluded.samples_json,
          updated_at=excluded.updated_at
        """,
        (
            entry["legacy_type"],
            entry["canonical_name"],
            entry["name_norm"],
            entry["observed_entity_type"],
            entry["first_patch_event_id"],
            entry["last_patch_event_id"],
            entry["first_seen_at"],
            entry["last_seen_at"],
            entry["event_count"],
            entry["confidence"],
            entry["status"],
            json.dumps(entry["samples"], ensure_ascii=True, sort_keys=True),
            now,
            now,
        ),
    )
    return cursor.rowcount > 0


def _name_confidence(name: str, row: dict[str, Any]) -> float:
    lowered = name.casefold()
    if not name or len(name) > 70:
        return 0.2
    if lowered.startswith(SUSPICIOUS_PREFIXES) or lowered.endswith(" from"):
        return 0.25
    if "(" in name and ")" not in name:
        return 0.25
    if any(token in lowered for token in (" following ", " abilities have ", " balance note")):
        return 0.3
    words = name.split()
    if len(words) > 6:
        return 0.35
    if row.get("entity_type") in {"hero", "item", "item_special", "ability"}:
        return 0.74
    return 0.55


def _clean_name(value: Any) -> str:
    return re.sub(r"\s+", " ", str(value or "").strip()).strip(" :-\t")


def _count_legacy(conn: sqlite3.Connection) -> int:
    row = conn.execute("SELECT COUNT(*) FROM legacy_entities").fetchone()
    return int(row[0] if row else 0)


def _table_exists(conn: sqlite3.Connection, name: str) -> bool:
    return conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (name,)).fetchone() is not None


def _fetch_all_dicts(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> list[dict[str, Any]]:
    cursor = conn.execute(sql, params)
    columns = [column[0] for column in cursor.description or []]
    rows = cursor.fetchall()
    if rows and hasattr(rows[0], "keys"):
        return [{key: row[key] for key in row.keys()} for row in rows]
    return [{key: row[index] for index, key in enumerate(columns)} for row in rows]
