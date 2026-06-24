#!/usr/bin/env python3
from __future__ import annotations

import argparse
import collections
import json
import re
import sqlite3
from pathlib import Path
from typing import Any, Callable


DEFAULT_DB = Path("data/deadlock_brain.sqlite3")


def connect_read_only(db_path: Path) -> sqlite3.Connection:
    uri = f"file:{db_path.resolve().as_posix()}?mode=ro"
    conn = sqlite3.connect(uri, uri=True)
    conn.row_factory = sqlite3.Row
    return conn


def scalar(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> int:
    row = conn.execute(sql, params).fetchone()
    return int((row[0] if row else 0) or 0)


def rows(conn: sqlite3.Connection, sql: str, params: tuple[Any, ...] = ()) -> list[dict[str, Any]]:
    return [dict(row) for row in conn.execute(sql, params).fetchall()]


def norm(value: Any) -> str:
    return re.sub(r"\s+", " ", str(value or "").strip().casefold().replace("_", " ")).strip()


def has(pattern: str, text: str) -> bool:
    return re.search(pattern, text, re.IGNORECASE) is not None


GENERAL_PATTERNS: list[tuple[str, Callable[[dict[str, Any]], bool], str]] = [
    (
        "unresolved_subject",
        lambda row: bool(str(row.get("subject") or "").strip()),
        "Subject vorhanden, aber Alias/Canonical-Index kann es nicht eindeutig als Hero/Item/Ability aufloesen.",
    ),
    (
        "global_objective_economy",
        lambda row: has(
            r"\b(souls?|urn|troopers?|guardians?|walkers?|patron|mid boss|jungles?|neutrals?|lanes?|orbs?|flex slots?|breakables?|rejuv|shrines?|crystal|bounty|sinners?|sacrifice|base guardians?)\b",
            str(row.get("normalized_line") or ""),
        ),
        "Game-System-/Objective-/Economy-Zeilen ohne natuerlichen Hero- oder Item-Besitzer.",
    ),
    (
        "global_all_hero_or_stat_rule",
        lambda row: has(
            r"\b(all heroes|globally|weapon tree|spirit power|sprint|respawn|comeback|team vs team|healing reduction|slow percentage|min speed)\b",
            str(row.get("normalized_line") or ""),
        )
        or bool(re.search(r"\d", str(row.get("normalized_line") or ""))),
        "Globale Regeln oder numerische System-Aenderungen ohne Einzel-Entity.",
    ),
    (
        "map_movement_structure",
        lambda row: has(
            r"\b(jump pad|ziplines?|wall jump|walls?|bridge|teleporters?|rope|mantle|camera|lighting|kiosk|structures?|vents?|air vents|map|spawn)\b",
            str(row.get("normalized_line") or ""),
        ),
        "Map-, Movement- oder Struktur-Regeln statt Hero/Item.",
    ),
    (
        "ui_settings_audio",
        lambda row: has(
            r"\b(options?|settings?|ui|hud|audio|sound|ping|minimap|spectating|categories|console command|shop music|builds? feature)\b",
            str(row.get("normalized_line") or ""),
        ),
        "UI/Settings/Audio/Tooling-Zeilen.",
    ),
    (
        "bugfix_no_owner",
        lambda row: str(row.get("change_type") or "") == "bugfix"
        or str(row.get("normalized_line") or "").casefold().startswith(("fixed ", "fix ")),
        "Bugfix ohne eindeutigen Besitzer.",
    ),
]


def classify_general(row: dict[str, Any]) -> tuple[str, str]:
    for name, predicate, reason in GENERAL_PATTERNS:
        if predicate(row):
            return name, reason
    return "other_general", "Nicht durch die einfachen, reproduzierbaren Muster abgedeckt."


def print_kv_table(title: str, data: dict[str, Any]) -> None:
    print(f"\n### {title}\n")
    print("| Kennzahl | Wert |")
    print("|---|---:|")
    for key, value in data.items():
        print(f"| {key} | {value} |")


def print_rows(title: str, data: list[dict[str, Any]], columns: list[str]) -> None:
    print(f"\n### {title}\n")
    if not data:
        print("Keine Zeilen.")
        return
    print("| " + " | ".join(columns) + " |")
    print("|" + "|".join("---" for _ in columns) + "|")
    for row in data:
        rendered = []
        for column in columns:
            value = row.get(column)
            text = "" if value is None else re.sub(r"\s+", " ", str(value)).strip()
            if len(text) > 140:
                text = text[:137] + "..."
            rendered.append(text.replace("|", "\\|"))
        print("| " + " | ".join(rendered) + " |")


def table_counts(conn: sqlite3.Connection) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for row in conn.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"):
        table = str(row["name"])
        try:
            result[table] = scalar(conn, f'SELECT COUNT(*) FROM "{table}"')
        except sqlite3.OperationalError as exc:
            result[table] = f"ERR: {exc}"
    return result


def general_patterns(conn: sqlite3.Connection) -> tuple[collections.Counter[str], dict[str, str], dict[str, list[dict[str, Any]]]]:
    pattern_counts: collections.Counter[str] = collections.Counter()
    reasons: dict[str, str] = {}
    examples: dict[str, list[dict[str, Any]]] = collections.defaultdict(list)
    for row in rows(
        conn,
        """
        SELECT id, patch_title, section, subject, change_type, normalized_line
        FROM patch_events
        WHERE entity_type='general'
        ORDER BY id
        """,
    ):
        name, reason = classify_general(row)
        pattern_counts[name] += 1
        reasons[name] = reason
        if len(examples[name]) < 3:
            examples[name].append(row)
    return pattern_counts, reasons, examples


def claim_reason_counts(conn: sqlite3.Connection) -> tuple[collections.Counter[str], list[dict[str, Any]], collections.Counter[tuple[str, str]]]:
    reasons: collections.Counter[str] = collections.Counter()
    unresolved_examples: list[dict[str, Any]] = []
    unresolved_by_type: collections.Counter[tuple[str, str]] = collections.Counter()
    for row in rows(
        conn,
        """
        SELECT id, entity_type, entity_name, claim_type, status, verifier_json, claim_text
        FROM youtube_learning_claims
        ORDER BY id
        """,
    ):
        try:
            verifier = json.loads(str(row.get("verifier_json") or "{}"))
        except json.JSONDecodeError:
            verifier = {}
        row_reasons = verifier.get("reasons")
        if not isinstance(row_reasons, list):
            row_reasons = []
        for reason in row_reasons:
            reasons[str(reason)] += 1
        if "entity_not_resolved" in row_reasons:
            unresolved_by_type[(str(row.get("entity_type") or "(null)"), str(row.get("claim_type") or ""))] += 1
            if len(unresolved_examples) < 12:
                unresolved_examples.append(row)
    return reasons, unresolved_examples, unresolved_by_type


def build_resolution(conn: sqlite3.Connection) -> dict[str, int]:
    hero_norms = set()
    for row in conn.execute("SELECT canonical_name FROM entities WHERE entity_type='hero'"):
        hero_norms.add(norm(row["canonical_name"]))
    for row in conn.execute(
        """
        SELECT a.alias_norm
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE e.entity_type='hero'
        """
    ):
        hero_norms.add(str(row["alias_norm"]))

    item_norms = set()
    for row in conn.execute(
        """
        SELECT canonical_name
        FROM entities
        WHERE entity_type IN ('item','item_special','ability','ability_internal','weapon_or_internal')
        """
    ):
        item_norms.add(norm(row["canonical_name"]))
    for row in conn.execute(
        """
        SELECT a.alias_norm
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE e.entity_type IN ('item','item_special','ability','ability_internal','weapon_or_internal')
        """
    ):
        item_norms.add(str(row["alias_norm"]))

    hero_unresolved = 0
    item_refs = 0
    item_unresolved = 0
    for row in rows(conn, "SELECT hero_name, item_names_json FROM learned_builds"):
        if norm(row.get("hero_name")) not in hero_norms:
            hero_unresolved += 1
        try:
            item_names = json.loads(str(row.get("item_names_json") or "[]"))
        except json.JSONDecodeError:
            item_names = []
        for item_name in item_names if isinstance(item_names, list) else []:
            item_refs += 1
            if norm(item_name) not in item_norms:
                item_unresolved += 1

    return {
        "learned_builds": scalar(conn, "SELECT COUNT(*) FROM learned_builds"),
        "build_hero_names_unresolved": hero_unresolved,
        "build_item_refs": item_refs,
        "build_item_refs_unresolved": item_unresolved,
    }


def orphan_checks(conn: sqlite3.Connection) -> dict[str, int]:
    checks = {
        "patch_event_enrichments_without_event": """
            SELECT COUNT(*)
            FROM patch_event_enrichments pee
            LEFT JOIN patch_events pe ON pe.id=pee.patch_event_id
            WHERE pe.id IS NULL
        """,
        "entity_lineage_without_event": """
            SELECT COUNT(*)
            FROM entity_lineage el
            LEFT JOIN patch_events pe ON pe.id=el.patch_event_id
            WHERE pe.id IS NULL
        """,
        "hero_stat_values_without_profile": """
            SELECT COUNT(*)
            FROM hero_stat_values hsv
            LEFT JOIN hero_stat_profiles hsp ON hsp.id=hsv.profile_id
            WHERE hsp.id IS NULL
        """,
        "youtube_claims_without_video": """
            SELECT COUNT(*)
            FROM youtube_learning_claims c
            LEFT JOIN youtube_videos v ON v.video_id=c.video_id
            WHERE v.video_id IS NULL
        """,
        "build_learning_notes_without_build": """
            SELECT COUNT(*)
            FROM build_learning_notes n
            LEFT JOIN learned_builds b ON b.id=n.learned_build_id
            WHERE n.learned_build_id IS NOT NULL AND b.id IS NULL
        """,
    }
    result = {name: scalar(conn, sql) for name, sql in checks.items()}
    result["foreign_key_check_rows"] = len(conn.execute("PRAGMA foreign_key_check").fetchall())
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description="Read-only Deadlock Brain top-down gap report.")
    parser.add_argument("--db", type=Path, default=DEFAULT_DB, help="SQLite DB path, opened with mode=ro.")
    args = parser.parse_args()

    conn = connect_read_only(args.db)

    print(f"# Read-only Top-Down Gap Report\n\nDB: `{args.db}`")
    print_kv_table(
        "Kernzahlen",
        {
            "tables": scalar(conn, "SELECT COUNT(*) FROM sqlite_master WHERE type='table'"),
            "entities": scalar(conn, "SELECT COUNT(*) FROM entities"),
            "entity_aliases": scalar(conn, "SELECT COUNT(*) FROM entity_aliases"),
            "patch_events": scalar(conn, "SELECT COUNT(*) FROM patch_events"),
            "patch_events_section_null": scalar(conn, "SELECT COUNT(*) FROM patch_events WHERE section IS NULL"),
            "patch_events_entity_general": scalar(conn, "SELECT COUNT(*) FROM patch_events WHERE entity_type='general'"),
            "youtube_learning_claims": scalar(conn, "SELECT COUNT(*) FROM youtube_learning_claims"),
            "learned_builds": scalar(conn, "SELECT COUNT(*) FROM learned_builds"),
        },
    )

    print_rows(
        "Entities nach Typ",
        rows(conn, "SELECT entity_type, COUNT(*) AS count FROM entities GROUP BY entity_type ORDER BY count DESC"),
        ["entity_type", "count"],
    )
    print_rows(
        "Patch-Events nach Entity-Typ",
        rows(conn, "SELECT entity_type, COUNT(*) AS count FROM patch_events GROUP BY entity_type ORDER BY count DESC"),
        ["entity_type", "count"],
    )
    print_rows(
        "Patch-Events ohne section nach Entity-Typ",
        rows(
            conn,
            """
            SELECT entity_type, COUNT(*) AS count
            FROM patch_events
            WHERE section IS NULL
            GROUP BY entity_type
            ORDER BY count DESC
            """,
        ),
        ["entity_type", "count"],
    )

    pattern_counts, reasons, examples = general_patterns(conn)
    print("\n### General-Patch-Events nach Muster\n")
    print("| Muster | Anzahl | Ursache |")
    print("|---|---:|---|")
    for name, count in pattern_counts.most_common():
        print(f"| {name} | {count} | {reasons.get(name, '')} |")
    for name, sample_rows in examples.items():
        print_rows(f"Beispiele: {name}", sample_rows, ["id", "patch_title", "subject", "change_type", "normalized_line"])

    print_rows(
        "Unparsed/low-confidence Enrichments nach Entity-Typ und Change-Typ",
        rows(
            conn,
            """
            SELECT pe.entity_type, pe.change_type, COUNT(*) AS count
            FROM patch_event_enrichments pee
            JOIN patch_events pe ON pe.id=pee.patch_event_id
            WHERE pee.confidence=0 OR pee.flags_json LIKE '%unparsed%'
            GROUP BY pe.entity_type, pe.change_type
            ORDER BY count DESC
            """,
        ),
        ["entity_type", "change_type", "count"],
    )

    print_kv_table(
        "Hero-Stats Coverage",
        {
            "hero_stat_profiles": scalar(conn, "SELECT COUNT(*) FROM hero_stat_profiles"),
            "hero_stat_profiles_entity_null": scalar(conn, "SELECT COUNT(*) FROM hero_stat_profiles WHERE entity_id IS NULL"),
            "hero_stat_values": scalar(conn, "SELECT COUNT(*) FROM hero_stat_values"),
            "hero_stat_values_entity_null": scalar(conn, "SELECT COUNT(*) FROM hero_stat_values WHERE entity_id IS NULL"),
            "sheet_heroes_stats": scalar(conn, "SELECT COUNT(*) FROM sheet_heroes_stats"),
            "sheet_heroes_stats_entity_null": scalar(conn, "SELECT COUNT(*) FROM sheet_heroes_stats WHERE entity_id IS NULL"),
            "sheet_raw_heroes": scalar(conn, "SELECT COUNT(*) FROM sheet_raw_heroes"),
            "sheet_raw_heroes_entity_null": scalar(conn, "SELECT COUNT(*) FROM sheet_raw_heroes WHERE entity_id IS NULL"),
            "sheet_hero_rankings": scalar(conn, "SELECT COUNT(*) FROM sheet_hero_rankings"),
            "sheet_hero_rankings_entity_null": scalar(conn, "SELECT COUNT(*) FROM sheet_hero_rankings WHERE entity_id IS NULL"),
        },
    )
    print_rows(
        "Hero-Stat-Profile ohne Entity",
        rows(
            conn,
            """
            SELECT id, snapshot_id, hero_name, source, external_id, row_number
            FROM hero_stat_profiles
            WHERE entity_id IS NULL
            ORDER BY hero_name
            """,
        ),
        ["id", "snapshot_id", "hero_name", "row_number"],
    )

    reasons, unresolved_examples, unresolved_by_type = claim_reason_counts(conn)
    print_rows(
        "YouTube-Claims nach Status",
        rows(conn, "SELECT status, COUNT(*) AS count FROM youtube_learning_claims GROUP BY status ORDER BY count DESC"),
        ["status", "count"],
    )
    print("\n### YouTube-Verifier-Gruende\n")
    print("| Grund | Anzahl |")
    print("|---|---:|")
    for reason, count in reasons.most_common():
        print(f"| {reason} | {count} |")
    print("\n### YouTube `entity_not_resolved` nach Typ\n")
    print("| entity_type | claim_type | Anzahl |")
    print("|---|---|---:|")
    for (entity_type, claim_type), count in unresolved_by_type.most_common():
        print(f"| {entity_type} | {claim_type} | {count} |")
    print_rows(
        "YouTube-Beispiele entity_not_resolved",
        unresolved_examples,
        ["id", "entity_type", "entity_name", "claim_type", "status", "claim_text"],
    )

    print_kv_table("Build-Resolution", build_resolution(conn))
    print_kv_table("Orphan-/FK-Checks", orphan_checks(conn))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
