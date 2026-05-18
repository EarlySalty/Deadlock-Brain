from __future__ import annotations

import json
import re
import sqlite3
import time
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias


_COLUMN_PLACEHOLDER_RE = re.compile(r"^column\s+\d+$", re.IGNORECASE)
_NON_KEY_CHARS_RE = re.compile(r"[^a-z0-9]+")
_SEPARATOR_CHARS = {"|", "l", "I"}
_HUMAN_HERO_ALIAS_KINDS = {"canonical", "snapshot_name"}


def normalize_sheet_stats(conn: sqlite3.Connection, rebuild: bool = False) -> dict[str, Any]:
    """Normalize hero stat snapshots from the community Google Sheet.

    The input is read from existing ``entity_snapshots`` rows with
    ``entity_type='hero_stats_sheet'``. Output tables are owned by this module
    and are created on demand.
    """

    _ensure_tables(conn)
    deleted = _clear_tables(conn) if rebuild else {"hero_stat_values": 0, "hero_stat_profiles": 0}

    hero_index = _build_hero_index(conn)
    rows = conn.execute(
        """
        SELECT id, source, external_id, canonical_name, payload_hash, payload_json
        FROM entity_snapshots
        WHERE entity_type='hero_stats_sheet'
        ORDER BY id
        """
    ).fetchall()

    profiles = 0
    values = 0
    skipped_columns = 0
    skipped_snapshots = 0
    unmatched_heroes: set[str] = set()
    seen_stat_keys: set[str] = set()

    for row in rows:
        snapshot_id = int(row[0])
        source = str(row[1])
        external_id = str(row[2])
        snapshot_name = str(row[3] or "").strip()
        payload_hash = str(row[4])
        try:
            payload = json.loads(str(row[5]))
        except json.JSONDecodeError:
            skipped_snapshots += 1
            continue
        if not isinstance(payload, dict):
            skipped_snapshots += 1
            continue

        raw_values = payload.get("values")
        if not isinstance(raw_values, dict):
            skipped_snapshots += 1
            continue

        hero_name = str(raw_values.get("Hero Name") or snapshot_name).strip()
        if not hero_name:
            skipped_snapshots += 1
            continue

        entity_id = hero_index.get(normalize_alias(hero_name))
        if entity_id is None:
            unmatched_heroes.add(hero_name)

        profile_id = _upsert_profile(
            conn,
            snapshot_id=snapshot_id,
            entity_id=entity_id,
            hero_name=hero_name,
            source=source,
            external_id=external_id,
            payload_hash=payload_hash,
            row_number=_int_or_none(payload.get("row_number")),
        )
        profiles += 1

        stat_keys_for_profile: set[str] = set()
        for label, raw_value in raw_values.items():
            if not _is_stat_column(label, raw_value):
                skipped_columns += 1
                continue

            stat_key = _stat_key(str(label))
            if not stat_key:
                skipped_columns += 1
                continue
            stat_key = _dedupe_stat_key(stat_key, stat_keys_for_profile)
            stat_keys_for_profile.add(stat_key)
            seen_stat_keys.add(stat_key)

            raw_text = str(raw_value).strip()
            numeric_value = _parse_number(raw_text)
            if _upsert_value(
                conn,
                profile_id=profile_id,
                entity_id=entity_id,
                hero_name=hero_name,
                stat_key=stat_key,
                stat_label=str(label).strip(),
                numeric_value=numeric_value,
                raw_value=raw_text,
            ):
                values += 1

    conn.commit()
    return {
        "deleted": deleted,
        "snapshots": len(rows),
        "profiles": profiles,
        "values": values,
        "stat_keys": len(seen_stat_keys),
        "skipped_snapshots": skipped_snapshots,
        "skipped_columns": skipped_columns,
        "unmatched_heroes": sorted(unmatched_heroes),
    }


def _ensure_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS hero_stat_profiles (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          entity_id INTEGER,
          hero_name TEXT NOT NULL,
          source TEXT NOT NULL,
          external_id TEXT NOT NULL,
          payload_hash TEXT NOT NULL,
          row_number INTEGER,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_hero_stat_profiles_entity
          ON hero_stat_profiles(entity_id);

        CREATE INDEX IF NOT EXISTS idx_hero_stat_profiles_hero
          ON hero_stat_profiles(hero_name);

        CREATE TABLE IF NOT EXISTS hero_stat_values (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          profile_id INTEGER NOT NULL,
          entity_id INTEGER,
          hero_name TEXT NOT NULL,
          stat_key TEXT NOT NULL,
          stat_label TEXT NOT NULL,
          numeric_value REAL,
          raw_value TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(profile_id, stat_key),
          FOREIGN KEY(profile_id) REFERENCES hero_stat_profiles(id) ON DELETE CASCADE,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );

        CREATE INDEX IF NOT EXISTS idx_hero_stat_values_entity_key
          ON hero_stat_values(entity_id, stat_key);

        CREATE INDEX IF NOT EXISTS idx_hero_stat_values_key
          ON hero_stat_values(stat_key);
        """
    )


def _clear_tables(conn: sqlite3.Connection) -> dict[str, int]:
    value_cursor = conn.execute("DELETE FROM hero_stat_values")
    profile_cursor = conn.execute("DELETE FROM hero_stat_profiles")
    return {
        "hero_stat_values": int(value_cursor.rowcount),
        "hero_stat_profiles": int(profile_cursor.rowcount),
    }


def _build_hero_index(conn: sqlite3.Connection) -> dict[str, int]:
    candidates: dict[str, set[int]] = {}
    entity_rows = conn.execute(
        """
        SELECT id, canonical_name
        FROM entities
        WHERE entity_type='hero'
        """
    ).fetchall()
    for entity_id, canonical_name in entity_rows:
        _add_index_candidate(candidates, normalize_alias(str(canonical_name)), int(entity_id))

    alias_rows = conn.execute(
        """
        SELECT entity_id, alias, alias_norm, alias_kind
        FROM entity_aliases
        WHERE entity_id IN (SELECT id FROM entities WHERE entity_type='hero')
        """
    ).fetchall()
    for entity_id, alias, alias_norm, alias_kind in alias_rows:
        if alias_kind not in _HUMAN_HERO_ALIAS_KINDS:
            continue
        if _looks_like_internal_hero_alias(str(alias)):
            continue
        _add_index_candidate(candidates, str(alias_norm), int(entity_id))

    return {alias: next(iter(ids)) for alias, ids in candidates.items() if len(ids) == 1}


def _add_index_candidate(candidates: dict[str, set[int]], alias_norm: str, entity_id: int) -> None:
    if alias_norm:
        candidates.setdefault(alias_norm, set()).add(entity_id)


def _looks_like_internal_hero_alias(value: str) -> bool:
    lowered = value.strip().casefold()
    return lowered.startswith("hero_") or lowered.startswith("hero ")


def _upsert_profile(
    conn: sqlite3.Connection,
    *,
    snapshot_id: int,
    entity_id: int | None,
    hero_name: str,
    source: str,
    external_id: str,
    payload_hash: str,
    row_number: int | None,
) -> int:
    now = int(time.time())
    conn.execute(
        """
        INSERT INTO hero_stat_profiles(
          snapshot_id, entity_id, hero_name, source, external_id, payload_hash,
          row_number, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?)
        ON CONFLICT(snapshot_id) DO UPDATE SET
          entity_id=excluded.entity_id,
          hero_name=excluded.hero_name,
          source=excluded.source,
          external_id=excluded.external_id,
          payload_hash=excluded.payload_hash,
          row_number=excluded.row_number,
          updated_at=excluded.updated_at
        """,
        (snapshot_id, entity_id, hero_name, source, external_id, payload_hash, row_number, now, now),
    )
    row = conn.execute(
        "SELECT id FROM hero_stat_profiles WHERE snapshot_id=?",
        (snapshot_id,),
    ).fetchone()
    return int(row[0])


def _upsert_value(
    conn: sqlite3.Connection,
    *,
    profile_id: int,
    entity_id: int | None,
    hero_name: str,
    stat_key: str,
    stat_label: str,
    numeric_value: float | None,
    raw_value: str,
) -> bool:
    now = int(time.time())
    cursor = conn.execute(
        """
        INSERT INTO hero_stat_values(
          profile_id, entity_id, hero_name, stat_key, stat_label, numeric_value,
          raw_value, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?)
        ON CONFLICT(profile_id, stat_key) DO UPDATE SET
          entity_id=excluded.entity_id,
          hero_name=excluded.hero_name,
          stat_label=excluded.stat_label,
          numeric_value=excluded.numeric_value,
          raw_value=excluded.raw_value,
          updated_at=excluded.updated_at
        """,
        (profile_id, entity_id, hero_name, stat_key, stat_label, numeric_value, raw_value, now, now),
    )
    return cursor.rowcount > 0


def _is_stat_column(label: Any, raw_value: Any) -> bool:
    label_text = str(label or "").strip()
    value_text = str(raw_value or "").strip()
    if not label_text or not value_text:
        return False
    if label_text.casefold() in {"hero name", "hero labs"}:
        return False
    if _COLUMN_PLACEHOLDER_RE.match(label_text):
        return False
    if _is_separator_label(label_text):
        return False
    return True


def _is_separator_label(label: str) -> bool:
    compact = re.sub(r"\s+", "", label)
    if len(compact) < 3:
        return False
    return set(compact) <= _SEPARATOR_CHARS and len(set(compact.casefold())) == 1


def _stat_key(label: str) -> str:
    key = _NON_KEY_CHARS_RE.sub("_", label.strip().casefold()).strip("_")
    return re.sub(r"_+", "_", key)


def _dedupe_stat_key(stat_key: str, used_keys: set[str]) -> str:
    if stat_key not in used_keys:
        return stat_key
    suffix = 2
    while f"{stat_key}_{suffix}" in used_keys:
        suffix += 1
    return f"{stat_key}_{suffix}"


def _parse_number(value: str) -> float | None:
    normalized = value.strip().replace(",", "")
    if normalized.endswith("%"):
        normalized = normalized[:-1].strip()
    if normalized.startswith("+"):
        normalized = normalized[1:].strip()
    if not re.fullmatch(r"-?(?:\d+(?:\.\d*)?|\.\d+)", normalized):
        return None
    try:
        return float(normalized)
    except ValueError:
        return None


def _int_or_none(value: Any) -> int | None:
    try:
        return int(value)
    except (TypeError, ValueError):
        return None
