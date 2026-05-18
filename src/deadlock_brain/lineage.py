from __future__ import annotations

import json
import re
import sqlite3
import time
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias


RENAMED_TO_RE = re.compile(r"^(?:now\s+)?renamed\s+to\s+(?P<new>.+?)(?:[.;,)]|$)", re.IGNORECASE)
RENAMED_X_TO_Y_RE = re.compile(r"^(?:renamed\s+)?(?P<old>.+?)\s+renamed\s+to\s+(?P<new>.+?)(?:[.;,)]|$)", re.IGNORECASE)
RENAMED_OLD_TO_NEW_RE = re.compile(r"^renamed\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[.;,)]|$)", re.IGNORECASE)
REPLACED_WITH_RE = re.compile(r"^replaced\s+(?P<old>.+?)\s+with\s+(?P<new>.+?)(?:[.;,)]|$)", re.IGNORECASE)
REWORKED_RE = re.compile(r"^(?P<name>.+?)\s+(?:has\s+been\s+)?reworked\b", re.IGNORECASE)

LINEAGE_NAME_LIMIT = 120


def build_entity_lineage(conn: sqlite3.Connection, rebuild: bool = False) -> dict[str, Any]:
    """Build deterministic rename/rework lineage rows from existing patch events."""
    ensure_entity_lineage_table(conn)
    before_total = _count_lineage(conn)
    deleted = 0
    if rebuild:
        cursor = conn.execute("DELETE FROM entity_lineage")
        deleted = int(cursor.rowcount if cursor.rowcount is not None else 0)

    events = conn.execute(
        """
        SELECT id, patch_snapshot_id, patch_external_id, patch_title, patch_url,
               source_kind, posted_at, line_index, section, entity_type,
               entity_name, subject, change_type, normalized_line, raw_line
        FROM patch_events
        WHERE
          change_type IN ('changed', 'rework', 'removed', 'added')
          OR lower(normalized_line) LIKE '%rename%'
          OR lower(normalized_line) LIKE '%rework%'
          OR lower(normalized_line) LIKE '%replaced%'
        ORDER BY id ASC
        """
    ).fetchall()

    processed = 0
    inserted = 0
    by_relation: dict[str, int] = {}
    for row in events:
        event = _row_to_dict(row)
        candidates = extract_lineage_candidates(event)
        processed += 1
        for candidate in candidates:
            if _insert_lineage(conn, candidate):
                inserted += 1
                relation = str(candidate["relation_type"])
                by_relation[relation] = by_relation.get(relation, 0) + 1

    conn.commit()
    return {
        "events_processed": processed,
        "lineage_inserted": inserted,
        "lineage_before": before_total,
        "lineage_total": _count_lineage(conn),
        "deleted_before_build": deleted,
        "by_relation": dict(sorted(by_relation.items())),
        "rebuild": rebuild,
    }


def ensure_entity_lineage_table(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS entity_lineage (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          patch_event_id INTEGER NOT NULL,
          relation_type TEXT NOT NULL,
          source_entity_type TEXT,
          source_name TEXT NOT NULL,
          source_name_norm TEXT NOT NULL,
          target_entity_type TEXT,
          target_name TEXT,
          target_name_norm TEXT,
          owner_entity_type TEXT,
          owner_name TEXT,
          owner_name_norm TEXT,
          confidence REAL NOT NULL,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(patch_event_id) REFERENCES patch_events(id) ON DELETE CASCADE
        );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_entity_lineage_unique
          ON entity_lineage(
            patch_event_id,
            relation_type,
            source_name_norm,
            COALESCE(target_name_norm, ''),
            COALESCE(owner_name_norm, '')
          );

        CREATE INDEX IF NOT EXISTS idx_entity_lineage_source
          ON entity_lineage(source_name_norm);

        CREATE INDEX IF NOT EXISTS idx_entity_lineage_target
          ON entity_lineage(target_name_norm);

        CREATE INDEX IF NOT EXISTS idx_entity_lineage_owner
          ON entity_lineage(owner_name_norm);
        """
    )
    conn.commit()


def extract_lineage_candidates(event: dict[str, Any]) -> list[dict[str, Any]]:
    text = _clean_line(event.get("normalized_line") or event.get("raw_line") or "")
    if not text:
        return []
    event_id = int(event["id"])
    entity_type = _lineage_entity_type(event.get("entity_type"))
    entity_name = _clean_name(event.get("entity_name") or event.get("subject"))
    owner_type = entity_type if entity_type == "hero" else None
    owner_name = entity_name if owner_type else None
    candidates: list[dict[str, Any]] = []

    match = RENAMED_TO_RE.match(text)
    if match and entity_name:
        candidates.append(
            _candidate(
                event_id=event_id,
                relation_type="rename",
                source_type=entity_type,
                source_name=entity_name,
                target_type=entity_type,
                target_name=_clean_name(match.group("new")),
                confidence=0.95,
                event=event,
            )
        )
        return candidates

    for pattern in (RENAMED_X_TO_Y_RE, RENAMED_OLD_TO_NEW_RE):
        match = pattern.match(text)
        if match:
            old_name = _clean_name(match.group("old"))
            new_name = _clean_name(match.group("new"))
            candidates.append(
                _candidate(
                    event_id=event_id,
                    relation_type="rename",
                    source_type=_scoped_child_type(entity_type),
                    source_name=old_name,
                    target_type=_scoped_child_type(entity_type),
                    target_name=new_name,
                    owner_type=owner_type,
                    owner_name=owner_name,
                    confidence=0.9,
                    event=event,
                )
            )
            return candidates

    match = REPLACED_WITH_RE.match(text)
    if match:
        candidates.append(
            _candidate(
                event_id=event_id,
                relation_type="replaced_by",
                source_type=entity_type,
                source_name=_clean_name(match.group("old")),
                target_type=entity_type,
                target_name=_clean_name(match.group("new")),
                confidence=0.75,
                event=event,
            )
        )

    if _is_rework_event(event, text):
        reworked_name = entity_name
        match = REWORKED_RE.match(text)
        if match and not entity_name:
            reworked_name = _clean_name(match.group("name"))
        if reworked_name:
            candidates.append(
                _candidate(
                    event_id=event_id,
                    relation_type="rework",
                    source_type=entity_type,
                    source_name=reworked_name,
                    target_type=entity_type,
                    target_name=reworked_name,
                    owner_type=owner_type if reworked_name != owner_name else None,
                    owner_name=owner_name if reworked_name != owner_name else None,
                    confidence=0.72 if "reworked" in text.casefold() else 0.65,
                    event=event,
                )
            )

    return [candidate for candidate in candidates if _valid_candidate(candidate)]


def related_names_for_query(conn: sqlite3.Connection, query: str, best_match: dict[str, Any] | None = None) -> list[dict[str, Any]]:
    """Return lineage-related names that should be searched together."""
    if not _table_exists(conn, "entity_lineage"):
        return []
    names = {query.strip()}
    if best_match and best_match.get("canonical_name"):
        names.add(str(best_match["canonical_name"]).strip())
    norms = sorted({normalize_alias(name) for name in names if name})
    if not norms:
        return []
    placeholders = ",".join("?" for _ in norms)
    rows = _fetch_all_dicts(
        conn,
        f"""
        SELECT *
        FROM entity_lineage
        WHERE source_name_norm IN ({placeholders})
           OR target_name_norm IN ({placeholders})
           OR owner_name_norm IN ({placeholders})
        ORDER BY confidence DESC, id ASC
        LIMIT ?
        """,
        (*norms, *norms, *norms, LINEAGE_NAME_LIMIT),
    )
    return [_decode_metadata(row) for row in rows]


def lineage_lookup_names(conn: sqlite3.Connection, query: str, best_match: dict[str, Any] | None = None) -> list[str]:
    names = {query.strip()}
    if best_match and best_match.get("canonical_name"):
        names.add(str(best_match["canonical_name"]).strip())
    for row in related_names_for_query(conn, query, best_match):
        for key in ("source_name", "target_name", "owner_name"):
            value = str(row.get(key) or "").strip()
            if value:
                names.add(value)
    return sorted(name for name in names if name)


def _candidate(
    *,
    event_id: int,
    relation_type: str,
    source_type: str | None,
    source_name: str,
    target_type: str | None,
    target_name: str | None,
    confidence: float,
    event: dict[str, Any],
    owner_type: str | None = None,
    owner_name: str | None = None,
) -> dict[str, Any]:
    return {
        "patch_event_id": event_id,
        "relation_type": relation_type,
        "source_entity_type": source_type,
        "source_name": source_name,
        "source_name_norm": normalize_alias(source_name),
        "target_entity_type": target_type,
        "target_name": target_name,
        "target_name_norm": normalize_alias(target_name or "") or None,
        "owner_entity_type": owner_type,
        "owner_name": owner_name,
        "owner_name_norm": normalize_alias(owner_name or "") or None,
        "confidence": confidence,
        "metadata": {
            "patch_title": event.get("patch_title"),
            "patch_url": event.get("patch_url"),
            "posted_at": event.get("posted_at"),
            "source_kind": event.get("source_kind"),
            "line": event.get("normalized_line") or event.get("raw_line"),
        },
    }


def _insert_lineage(conn: sqlite3.Connection, candidate: dict[str, Any]) -> bool:
    now = int(time.time())
    cursor = conn.execute(
        """
        INSERT INTO entity_lineage(
          patch_event_id, relation_type, source_entity_type, source_name, source_name_norm,
          target_entity_type, target_name, target_name_norm, owner_entity_type, owner_name,
          owner_name_norm, confidence, metadata_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO NOTHING
        """,
        (
            candidate["patch_event_id"],
            candidate["relation_type"],
            candidate["source_entity_type"],
            candidate["source_name"],
            candidate["source_name_norm"],
            candidate["target_entity_type"],
            candidate["target_name"],
            candidate["target_name_norm"],
            candidate["owner_entity_type"],
            candidate["owner_name"],
            candidate["owner_name_norm"],
            candidate["confidence"],
            json.dumps(candidate.get("metadata") or {}, ensure_ascii=True, sort_keys=True),
            now,
            now,
        ),
    )
    return cursor.rowcount > 0


def _valid_candidate(candidate: dict[str, Any]) -> bool:
    source = str(candidate.get("source_name") or "").strip()
    target = str(candidate.get("target_name") or "").strip()
    if len(source) < 2 or len(source) > 120:
        return False
    if target and len(target) > 120:
        return False
    return source.casefold() not in {"it", "this", "the item", "the ability"}


def _lineage_entity_type(value: Any) -> str | None:
    text = str(value or "").strip()
    return None if text == "general" else text or None


def _scoped_child_type(parent_type: str | None) -> str | None:
    if parent_type in {"hero", "hero_internal"}:
        return "ability"
    return parent_type


def _is_rework_event(event: dict[str, Any], text: str) -> bool:
    lower = text.casefold()
    return str(event.get("change_type") or "").casefold() == "rework" or "reworked" in lower or "rescaled" in lower


def _clean_line(value: Any) -> str:
    return re.sub(r"\s+", " ", str(value or "").strip())


def _clean_name(value: Any) -> str:
    text = _clean_line(value)
    text = re.sub(r"\s*\([^)]*\)\s*$", "", text).strip()
    return text.strip(" :-\t\"'")


def _count_lineage(conn: sqlite3.Connection) -> int:
    row = conn.execute("SELECT COUNT(*) AS count FROM entity_lineage").fetchone()
    return int(row["count"] if hasattr(row, "keys") else row[0])


def _row_to_dict(row: sqlite3.Row) -> dict[str, Any]:
    return {key: row[key] for key in row.keys()}


def _table_exists(conn: sqlite3.Connection, name: str) -> bool:
    return conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (name,)).fetchone() is not None


def _fetch_all_dicts(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> list[dict[str, Any]]:
    cursor = conn.execute(sql, params)
    columns = [column[0] for column in cursor.description or []]
    rows = cursor.fetchall()
    if rows and hasattr(rows[0], "keys"):
        return [{key: row[key] for key in row.keys()} for row in rows]
    return [{key: row[index] for index, key in enumerate(columns)} for row in rows]


def _decode_metadata(row: dict[str, Any]) -> dict[str, Any]:
    metadata = row.pop("metadata_json", "{}")
    try:
        row["metadata"] = json.loads(str(metadata))
    except json.JSONDecodeError:
        row["metadata"] = {}
    return row
