from __future__ import annotations

import csv
import json
import io
import re
from typing import Any

from deadlock_brain.http import HttpClient
from deadlock_brain.storage import BrainStore


SOURCE = "deadlock_stats_sheet"


def sheet_csv_url(sheet_id: str, gid: str) -> str:
    return f"https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv&gid={gid}"


def sheet_pubhtml_url(sheet_id: str) -> str:
    return f"https://docs.google.com/spreadsheets/d/{sheet_id}/pubhtml"


def pull_sheet(
    store: BrainStore,
    http: HttpClient,
    *,
    sheet_id: str,
    gid: str | None,
    all_tabs: bool = False,
    cache_ttl_seconds: int = 1800,
) -> dict[str, Any]:
    tabs = discover_sheet_tabs(http, sheet_id=sheet_id, cache_ttl_seconds=cache_ttl_seconds) if all_tabs else []
    if not tabs:
        safe_gid = str(gid or "0").strip()
        tabs = [{"name": "sheet", "gid": safe_gid}]
    summaries = [
        _pull_single_sheet(store, http, sheet_id=sheet_id, tab=tab, cache_ttl_seconds=cache_ttl_seconds)
        for tab in tabs
    ]
    return {
        "sheet_id": sheet_id,
        "tabs": len(summaries),
        "tab_names": [summary.get("sheet_name") for summary in summaries],
        "rows": sum(int(summary.get("rows") or 0) for summary in summaries),
        "hero_snapshots": sum(int(summary.get("hero_snapshots") or 0) for summary in summaries),
        "item_snapshots": sum(int(summary.get("item_snapshots") or 0) for summary in summaries),
        "generic_row_snapshots": sum(int(summary.get("generic_row_snapshots") or 0) for summary in summaries),
        "summaries": summaries,
    }


def discover_sheet_tabs(http: HttpClient, *, sheet_id: str, cache_ttl_seconds: int = 1800) -> list[dict[str, str]]:
    url = sheet_pubhtml_url(sheet_id)
    result = http.get(url, cache_ttl_seconds=cache_ttl_seconds, timeout=45)
    tabs = []
    for match in re.finditer(r'items\.push\(\{name:\s*"((?:[^"\\]|\\.)*)".*?gid:\s*"(-?\d+)"', result.text, re.S):
        name = _decode_js_string(match.group(1)).strip()
        gid = match.group(2).strip()
        if name and gid and {"name": name, "gid": gid} not in tabs:
            tabs.append({"name": name, "gid": gid})
    return tabs


def _pull_single_sheet(
    store: BrainStore,
    http: HttpClient,
    *,
    sheet_id: str,
    tab: dict[str, str],
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    gid = str(tab.get("gid") or "0")
    sheet_name = str(tab.get("name") or gid)
    url = sheet_csv_url(sheet_id, gid)
    result = http.get(url, cache_ttl_seconds=cache_ttl_seconds, timeout=45)
    raw_path = store.write_raw(source=SOURCE, external_id=f"gid_{gid}", content=result.content, suffix="csv")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id=f"{sheet_id}:{gid}",
        title=f"Deadlock community stats sheet - {sheet_name}",
        url=url,
        content_type="text/csv",
        raw_path=raw_path,
        content=result.content,
        metadata={"sheet_id": sheet_id, "gid": gid, "sheet_name": sheet_name, "from_cache": result.from_cache},
    )

    rows = list(csv.reader(io.StringIO(result.text)))
    header_index = _find_header_row(rows)
    snapshots = []
    generic_snapshots = []
    item_snapshots = []
    if header_index is not None:
        headers = [h.strip() for h in rows[header_index]]
        for idx, row in enumerate(rows[header_index + 1 :], start=header_index + 2):
            record = _row_to_record(headers, row)
            hero_name = _hero_name_from_record(record)
            item_name = _item_name_from_record(record)
            payload = {"row_number": idx, "sheet_name": sheet_name, "gid": gid, "values": record}
            snapshots.append(
                {
                    "source": SOURCE,
                    "entity_type": "stats_sheet_row",
                    "external_id": f"{gid}:{idx}",
                    "canonical_name": hero_name or item_name or f"{sheet_name} row {idx}",
                    "payload": payload,
                }
            )
            generic_snapshots.append(snapshots[-1])
            if hero_name:
                snapshots.append(
                    {
                        "source": SOURCE,
                        "entity_type": "hero_stats_sheet",
                        "external_id": f"{gid}:{hero_name.lower().replace(' ', '_')}",
                        "canonical_name": hero_name,
                        "payload": payload,
                    }
                )
            if item_name:
                snapshots.append(
                    {
                        "source": SOURCE,
                        "entity_type": "item_stats_sheet",
                        "external_id": f"{gid}:{item_name.lower().replace(' ', '_')}",
                        "canonical_name": item_name,
                        "payload": payload,
                    }
                )
                item_snapshots.append(snapshots[-1])
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {
        "url": url,
        "sheet_name": sheet_name,
        "gid": gid,
        "rows": len(rows),
        "header_row": header_index + 1 if header_index is not None else None,
        "snapshots": count,
        "hero_snapshots": sum(1 for snapshot in snapshots if snapshot["entity_type"] == "hero_stats_sheet"),
        "item_snapshots": len(item_snapshots),
        "generic_row_snapshots": len(generic_snapshots),
    }


def _find_header_row(rows: list[list[str]]) -> int | None:
    for idx, row in enumerate(rows):
        normalized = [cell.strip().lower() for cell in row]
        if (
            "hero name" in normalized
            or "hero name" in " ".join(normalized)
            or "hero" in normalized
            or "name" in normalized
            or "game name" in normalized
            or "code name" in normalized
            or "souls" in normalized
            or "weapon" in normalized
        ):
            return idx
    # Fallback: erste nicht-leere Zeile als Header verwenden, damit kein Tab leer bleibt
    for idx, row in enumerate(rows):
        if any(cell.strip() for cell in row):
            return idx
    return None


def _row_to_record(headers: list[str], row: list[str]) -> dict[str, str]:
    record: dict[str, str] = {}
    used: set[str] = set()
    for idx, header in enumerate(headers):
        key = header.strip() or f"Column {idx + 1}"
        if key in used:
            suffix = 2
            while f"{key} {suffix}" in used:
                suffix += 1
            key = f"{key} {suffix}"
        used.add(key)
        record[key] = row[idx].strip() if idx < len(row) else ""
    return record


def _hero_name_from_record(record: dict[str, str]) -> str:
    if str(record.get("disabled") or "").strip().casefold() in {"true", "1", "yes"}:
        return ""
    for key in ("Hero Name", "hero name", "name"):
        value = str(record.get(key) or "").strip()
        if value and _looks_like_name(value):
            return value
    return ""


def _item_name_from_record(record: dict[str, str]) -> str:
    for key in ("game name", "Game Name", "Item", "item", "code name", "Code Name"):
        value = str(record.get(key) or "").strip()
        if value and _looks_like_name(value):
            return value
    return ""


def _looks_like_name(value: str) -> bool:
    lowered = value.casefold()
    if lowered in {"true", "false", "name", "hero", "game name", "code name"}:
        return False
    if "tiermaker" in lowered or re.search(r"\([<>]?\d", lowered):
        return False
    return any(ch.isalpha() for ch in value)


def _decode_js_string(value: str) -> str:
    try:
        return json.loads(f'"{value}"')
    except json.JSONDecodeError:
        return value.replace("\\/", "/")
