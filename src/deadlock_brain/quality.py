from __future__ import annotations

import json
import re
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias
from deadlock_brain.patch_parser import normalize_key
from deadlock_brain.sources.assets_api import SOURCE as ASSETS_SOURCE


SAMPLE_LIMIT = 20
GENERAL_ENTITY_SCAN_LIMIT = 50
LOW_CONFIDENCE_THRESHOLD = 0.5
PUBLIC_ENTITY_TYPES = {"hero", "item", "item_special", "ability"}


def run_quality_checks(conn) -> dict[str, Any]:
    """Run local data quality checks against the normalized Deadlock tables.

    The checks are intentionally read-only and use the already-ingested local
    assets snapshots/entities as the Deadlock API basis.
    """

    checks = [
        _check_required_tables(conn),
        _check_entity_counts(conn),
        _check_alias_collisions(conn),
        _check_patch_events_unknown_entities(conn),
        _check_enrichment_table(conn),
        _check_lineage_table(conn),
        _check_legacy_entities_table(conn),
        _check_low_confidence_enrichments(conn),
        _check_sheet_profiles_without_entity(conn),
        _check_general_events_with_known_entity_names(conn),
    ]
    severity_rank = {"ok": 0, "info": 1, "warning": 2, "error": 3}
    worst = max((severity_rank.get(check["severity"], 0) for check in checks), default=0)
    return {
        "ok": worst < severity_rank["warning"],
        "summary": {
            "checks": len(checks),
            "warnings": sum(1 for check in checks if check["severity"] == "warning"),
            "errors": sum(1 for check in checks if check["severity"] == "error"),
        },
        "checks": checks,
    }


def _check_required_tables(conn) -> dict[str, Any]:
    required = {
        "source_documents",
        "entity_snapshots",
        "entities",
        "entity_aliases",
        "patch_events",
    }
    missing = sorted(table for table in required if not _table_exists(conn, table))
    if missing:
        return _result(
            "error",
            "required_tables",
            f"Missing required normalized tables: {', '.join(missing)}",
            rows=len(missing),
            samples=[{"table": table} for table in missing],
        )
    return _result("ok", "required_tables", "Required normalized tables are present.")


def _check_entity_counts(conn) -> dict[str, Any]:
    if not _tables_exist(conn, "entity_snapshots", "entities"):
        return _result("error", "entity_counts", "Cannot check entity counts because required tables are missing.")

    asset_snapshots = _scalar(
        conn,
        """
        SELECT COUNT(*)
        FROM entity_snapshots
        WHERE source=? AND entity_type IN ('hero', 'item_or_ability', 'rank')
        """,
        (ASSETS_SOURCE,),
    )
    entity_counts = {
        str(row["entity_type"]): int(row["count"])
        for row in _query(
            conn,
            """
            SELECT entity_type, COUNT(*) AS count
            FROM entities
            GROUP BY entity_type
            """,
        )
    }
    public_total = sum(entity_counts.get(entity_type, 0) for entity_type in PUBLIC_ENTITY_TYPES)
    issues: list[dict[str, Any]] = []

    if asset_snapshots == 0:
        issues.append({"metric": "asset_snapshots", "value": 0, "expected": "> 0"})
    if public_total == 0:
        issues.append({"metric": "public_entities", "value": 0, "expected": "> 0"})
    for entity_type, minimum in (("hero", 10), ("item", 20), ("ability", 20)):
        value = entity_counts.get(entity_type, 0)
        if value < minimum:
            issues.append({"metric": f"entities.{entity_type}", "value": value, "expected": f">= {minimum}"})
    if asset_snapshots and public_total > asset_snapshots * 2:
        issues.append(
            {
                "metric": "public_entities_to_asset_snapshots",
                "value": public_total,
                "expected": f"not more than {asset_snapshots * 2}",
                "asset_snapshots": asset_snapshots,
            }
        )

    severity = "warning" if issues else "ok"
    message = "Entity counts look plausible." if not issues else "Entity counts are outside expected local snapshot bounds."
    return _result(
        severity,
        "entity_counts",
        message,
        rows=len(issues),
        samples=issues,
        details={"asset_snapshots": asset_snapshots, "entity_counts": dict(sorted(entity_counts.items()))},
    )


def _check_alias_collisions(conn) -> dict[str, Any]:
    if not _tables_exist(conn, "entity_aliases", "entities"):
        return _result("error", "alias_collisions", "Cannot check alias collisions because required tables are missing.")

    rows = _query(
        conn,
        """
        SELECT a.alias_norm, COUNT(DISTINCT a.entity_id) AS entities,
               GROUP_CONCAT(DISTINCT e.entity_type || ':' || e.canonical_name) AS targets,
               GROUP_CONCAT(DISTINCT a.alias) AS aliases
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE a.alias_norm <> ''
        GROUP BY a.alias_norm
        HAVING COUNT(DISTINCT a.entity_id) > 1
        ORDER BY entities DESC, a.alias_norm
        LIMIT ?
        """,
        (SAMPLE_LIMIT,),
    )
    total = _scalar(
        conn,
        """
        SELECT COUNT(*)
        FROM (
          SELECT alias_norm
          FROM entity_aliases
          WHERE alias_norm <> ''
          GROUP BY alias_norm
          HAVING COUNT(DISTINCT entity_id) > 1
        )
        """,
    )
    severity = "warning" if total else "ok"
    return _result(
        severity,
        "alias_collisions",
        "Alias collisions found across multiple entities." if total else "No alias collisions found.",
        rows=total,
        samples=rows,
    )


def _check_patch_events_unknown_entities(conn) -> dict[str, Any]:
    if not _tables_exist(conn, "patch_events", "entities", "entity_aliases"):
        return _result(
            "error",
            "patch_events_unknown_entities",
            "Cannot check patch events because required tables are missing.",
        )

    known = _known_entity_names(conn)
    unknown: list[dict[str, Any]] = []
    for row in _query(
        conn,
        """
        SELECT id, patch_external_id, entity_type, entity_name, raw_line
        FROM patch_events
        WHERE entity_type <> 'general' AND entity_name IS NOT NULL AND TRIM(entity_name) <> ''
        ORDER BY id
        """,
    ):
        key = (str(row["entity_type"]), normalize_alias(str(row["entity_name"])))
        if key not in known:
            unknown.append(row)
            if len(unknown) >= SAMPLE_LIMIT:
                break

    total = 0
    for row in _query(
        conn,
        """
        SELECT entity_type, entity_name
        FROM patch_events
        WHERE entity_type <> 'general' AND entity_name IS NOT NULL AND TRIM(entity_name) <> ''
        """,
    ):
        if (str(row["entity_type"]), normalize_alias(str(row["entity_name"]))) not in known:
            total += 1

    severity = "warning" if total else "ok"
    return _result(
        severity,
        "patch_events_unknown_entities",
        "Patch events reference entities that are absent from normalized entities/aliases."
        if total
        else "Patch event entity references resolve to normalized entities or aliases.",
        rows=total,
        samples=unknown,
    )


def _check_enrichment_table(conn) -> dict[str, Any]:
    if not _table_exists(conn, "patch_event_enrichments"):
        return _result(
            "warning",
            "missing_enrichment_table",
            "Missing patch_event_enrichments table; enrichment quality cannot be evaluated.",
        )
    return _result("ok", "missing_enrichment_table", "patch_event_enrichments table is present.")


def _check_lineage_table(conn) -> dict[str, Any]:
    if not _table_exists(conn, "entity_lineage"):
        return _result(
            "warning",
            "missing_lineage_table",
            "Missing entity_lineage table; renamed/reworked legacy names are not linked.",
        )
    total = _scalar(conn, "SELECT COUNT(*) FROM entity_lineage")
    rows = _query(
        conn,
        """
        SELECT relation_type, COUNT(*) AS count
        FROM entity_lineage
        GROUP BY relation_type
        ORDER BY relation_type
        """,
    )
    severity = "warning" if total == 0 else "ok"
    return _result(
        severity,
        "lineage_table",
        "entity_lineage is populated." if total else "entity_lineage exists but has no rows.",
        rows=total,
        samples=rows,
    )


def _check_legacy_entities_table(conn) -> dict[str, Any]:
    if not _table_exists(conn, "legacy_entities"):
        return _result(
            "warning",
            "missing_legacy_entities_table",
            "Missing legacy_entities table; old removed names outside lineage are not modeled.",
        )
    total = _scalar(conn, "SELECT COUNT(*) FROM legacy_entities")
    suspect = _scalar(conn, "SELECT COUNT(*) FROM legacy_entities WHERE status='suspect_parser_subject'")
    samples = _query(
        conn,
        """
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM legacy_entities
        ORDER BY confidence DESC, event_count DESC, canonical_name
        LIMIT ?
        """,
        (SAMPLE_LIMIT,),
    )
    suspect_samples = _query(
        conn,
        """
        SELECT legacy_type, canonical_name, event_count, confidence, status
        FROM legacy_entities
        WHERE status='suspect_parser_subject'
        ORDER BY canonical_name
        LIMIT ?
        """,
        (SAMPLE_LIMIT,),
    )
    return _result(
        "warning" if suspect else ("ok" if total else "warning"),
        "legacy_entities_table",
        "legacy_entities has suspect parser subjects." if suspect else ("legacy_entities is populated." if total else "legacy_entities exists but has no rows."),
        rows=total,
        samples=(suspect_samples or samples),
        details={"suspect_parser_subjects": suspect},
    )


def _check_low_confidence_enrichments(conn) -> dict[str, Any]:
    if not _table_exists(conn, "patch_event_enrichments"):
        return _result(
            "info",
            "low_confidence_unparsed_enrichments",
            "Skipped because patch_event_enrichments table is missing.",
        )
    rows = _query(
        conn,
        """
        SELECT pee.patch_event_id, pee.confidence, pee.flags_json, pe.patch_external_id,
               pe.entity_type, pe.entity_name, pe.normalized_line
        FROM patch_event_enrichments pee
        LEFT JOIN patch_events pe ON pe.id=pee.patch_event_id
        WHERE pee.confidence < ? OR pee.flags_json LIKE '%unparsed%'
        ORDER BY pee.confidence ASC, pee.patch_event_id ASC
        LIMIT ?
        """,
        (LOW_CONFIDENCE_THRESHOLD, SAMPLE_LIMIT),
    )
    total = _scalar(
        conn,
        """
        SELECT COUNT(*)
        FROM patch_event_enrichments
        WHERE confidence < ? OR flags_json LIKE '%unparsed%'
        """,
        (LOW_CONFIDENCE_THRESHOLD,),
    )
    severity = "warning" if total else "ok"
    return _result(
        severity,
        "low_confidence_unparsed_enrichments",
        "Low-confidence or unparsed patch event enrichments found."
        if total
        else "No low-confidence or unparsed enrichments found.",
        rows=total,
        samples=rows,
        details={"confidence_threshold": LOW_CONFIDENCE_THRESHOLD},
    )


def _check_sheet_profiles_without_entity(conn) -> dict[str, Any]:
    if not _table_exists(conn, "hero_stat_profiles"):
        return _result("info", "sheet_profiles_without_entity", "Skipped because hero_stat_profiles table is missing.")
    rows = _query(
        conn,
        """
        SELECT id, snapshot_id, hero_name, source, external_id, row_number
        FROM hero_stat_profiles
        WHERE entity_id IS NULL
        ORDER BY hero_name
        LIMIT ?
        """,
        (SAMPLE_LIMIT,),
    )
    total = _scalar(conn, "SELECT COUNT(*) FROM hero_stat_profiles WHERE entity_id IS NULL")
    severity = "warning" if total else "ok"
    return _result(
        severity,
        "sheet_profiles_without_entity",
        "Sheet profiles without normalized hero entity found."
        if total
        else "All sheet profiles are linked to a normalized hero entity.",
        rows=total,
        samples=rows,
    )


def _check_general_events_with_known_entity_names(conn) -> dict[str, Any]:
    if not _tables_exist(conn, "patch_events", "entities", "entity_aliases"):
        return _result(
            "error",
            "general_events_with_known_entity_names",
            "Cannot scan general events because required tables are missing.",
        )

    scan_names = _entity_scan_names(conn)
    samples: list[dict[str, Any]] = []
    total = 0
    for row in _query(
        conn,
        """
        SELECT id, patch_external_id, section, raw_line, normalized_line
        FROM patch_events
        WHERE entity_type='general'
        ORDER BY id
        """,
    ):
        text_key = f" {normalize_key(str(row['raw_line'] or '') + ' ' + str(row['normalized_line'] or ''))} "
        matches = [
            {"entity_type": entity_type, "entity_name": entity_name, "matched_name": name}
            for name_key, entity_type, entity_name, name in scan_names
            if re.search(rf"\b{re.escape(name_key)}\b", text_key)
        ]
        if not matches:
            continue
        total += 1
        if len(samples) < GENERAL_ENTITY_SCAN_LIMIT:
            sample = dict(row)
            sample["matches"] = matches[:5]
            samples.append(sample)

    severity = "warning" if total else "ok"
    return _result(
        severity,
        "general_events_with_known_entity_names",
        "General patch events mention known local API entities and may be misclassified."
        if total
        else "No general patch events with known local API entity names found.",
        rows=total,
        samples=samples[:SAMPLE_LIMIT],
    )


def _known_entity_names(conn) -> set[tuple[str, str]]:
    known: set[tuple[str, str]] = set()
    for row in _query(conn, "SELECT entity_type, canonical_name FROM entities"):
        known.add((str(row["entity_type"]), normalize_alias(str(row["canonical_name"]))))
    for row in _query(
        conn,
        """
        SELECT e.entity_type, a.alias_norm
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        """,
    ):
        known.add((str(row["entity_type"]), str(row["alias_norm"])))
    if _table_exists(conn, "entity_lineage"):
        for row in _query(conn, "SELECT source_entity_type, source_name_norm, target_entity_type, target_name_norm FROM entity_lineage"):
            _add_lineage_known_name(known, row.get("source_entity_type"), row.get("source_name_norm"))
            _add_lineage_known_name(known, row.get("target_entity_type"), row.get("target_name_norm"))
    if _table_exists(conn, "legacy_entities"):
        for row in _query(
            conn,
            """
            SELECT observed_entity_type, name_norm
            FROM legacy_entities
            """,
        ):
            known.add((str(row["observed_entity_type"]), str(row["name_norm"])))
    return known


def _add_lineage_known_name(known: set[tuple[str, str]], entity_type: Any, name_norm: Any) -> None:
    name = str(name_norm or "").strip()
    if not name:
        return
    if entity_type:
        known.add((str(entity_type), name))
        return
    for fallback_type in ("item", "item_special", "ability", "hero"):
        known.add((fallback_type, name))


def _entity_scan_names(conn) -> list[tuple[str, str, str, str]]:
    by_key: dict[tuple[str, str, str], str] = {}
    for row in _query(
        conn,
        """
        SELECT entity_type, canonical_name
        FROM entities
        WHERE source=? AND entity_type IN ('hero', 'item', 'item_special', 'ability')
        """,
        (ASSETS_SOURCE,),
    ):
        _add_scan_name(by_key, str(row["entity_type"]), str(row["canonical_name"]), str(row["canonical_name"]))
    for row in _query(
        conn,
        """
        SELECT e.entity_type, e.canonical_name, a.alias
        FROM entity_aliases a
        JOIN entities e ON e.id=a.entity_id
        WHERE a.source=? AND e.entity_type IN ('hero', 'item', 'item_special', 'ability')
        """,
        (ASSETS_SOURCE,),
    ):
        _add_scan_name(by_key, str(row["entity_type"]), str(row["canonical_name"]), str(row["alias"]))
    if _table_exists(conn, "entity_lineage"):
        for row in _query(
            conn,
            """
            SELECT source_entity_type, source_name, target_entity_type, target_name
            FROM entity_lineage
            WHERE relation_type IN ('rename', 'replaced_by')
            """,
        ):
            _add_scan_name(by_key, str(row.get("source_entity_type") or "legacy"), str(row.get("source_name") or ""), str(row.get("source_name") or ""))
            _add_scan_name(by_key, str(row.get("target_entity_type") or "legacy"), str(row.get("target_name") or ""), str(row.get("target_name") or ""))
    if _table_exists(conn, "legacy_entities"):
        for row in _query(
            conn,
            """
            SELECT legacy_type, canonical_name
            FROM legacy_entities
            WHERE confidence >= 0.5
            """,
        ):
            _add_scan_name(by_key, str(row["legacy_type"]), str(row["canonical_name"]), str(row["canonical_name"]))
    return sorted(
        ((key[0], key[1], key[2], value) for key, value in by_key.items()),
        key=lambda item: len(item[0]),
        reverse=True,
    )


def _add_scan_name(by_key: dict[tuple[str, str, str], str], entity_type: str, entity_name: str, name: str) -> None:
    name_key = normalize_key(name)
    if len(name_key) < 4 or name_key.isdigit():
        return
    if name_key in {"hero", "item", "ability", "weapon", "melee"}:
        return
    by_key.setdefault((name_key, entity_type, entity_name), name)


def _table_exists(conn, table: str) -> bool:
    row = conn.execute(
        "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
        (table,),
    ).fetchone()
    return row is not None


def _tables_exist(conn, *tables: str) -> bool:
    return all(_table_exists(conn, table) for table in tables)


def _query(conn, sql: str, params: tuple[Any, ...] = ()) -> list[dict[str, Any]]:
    cursor = conn.execute(sql, params)
    columns = [column[0] for column in cursor.description or []]
    return [_row_to_dict(row, columns) for row in cursor.fetchall()]


def _scalar(conn, sql: str, params: tuple[Any, ...] = ()) -> int:
    row = conn.execute(sql, params).fetchone()
    if row is None:
        return 0
    value = row[0]
    return int(value or 0)


def _row_to_dict(row, columns: list[str]) -> dict[str, Any]:
    if hasattr(row, "keys"):
        return {key: _json_value(row[key]) for key in row.keys()}
    return {key: _json_value(row[index]) for index, key in enumerate(columns)}


def _json_value(value: Any) -> Any:
    if isinstance(value, bytes):
        return value.decode("utf-8", errors="replace")
    if isinstance(value, (str, int, float)) or value is None:
        return value
    return str(value)


def _result(
    severity: str,
    check: str,
    message: str,
    *,
    rows: int = 0,
    samples: list[dict[str, Any]] | None = None,
    details: dict[str, Any] | None = None,
) -> dict[str, Any]:
    result: dict[str, Any] = {
        "severity": severity,
        "check": check,
        "message": message,
        "rows": rows,
        "samples": samples or [],
    }
    if details is not None:
        result["details"] = _clean_details(details)
    return result


def _clean_details(value: Any) -> Any:
    if isinstance(value, dict):
        return {str(key): _clean_details(item) for key, item in value.items()}
    if isinstance(value, list):
        return [_clean_details(item) for item in value]
    try:
        json.dumps(value)
    except TypeError:
        return str(value)
    return value
