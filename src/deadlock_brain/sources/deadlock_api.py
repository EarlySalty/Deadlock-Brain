from __future__ import annotations

import json
import urllib.parse
from typing import Any

from deadlock_brain.http import HttpClient
from deadlock_brain.storage import BrainStore


SOURCE = "deadlock_api"
BASE_URL = "https://api.deadlock-api.com"


def pull_match_metadata(
    store: BrainStore,
    http: HttpClient,
    *,
    match_ids: list[str],
    account_ids: list[str] | None = None,
    hero_ids: list[str] | None = None,
    include_player_items: bool = True,
    include_player_info: bool = True,
    include_player_stats: bool = True,
    include_player_death_details: bool = True,
    include_objectives: bool = True,
    cache_ttl_seconds: int = 21600,
) -> dict[str, Any]:
    safe_match_ids = _safe_ids(match_ids)
    if not safe_match_ids:
        raise ValueError("match_ids fehlt.")
    params: dict[str, Any] = {
        "match_ids": ",".join(safe_match_ids),
        "include_info": "true",
        "include_player_items": str(bool(include_player_items)).lower(),
        "include_player_info": str(bool(include_player_info)).lower(),
        "include_player_stats": str(bool(include_player_stats)).lower(),
        "include_player_death_details": str(bool(include_player_death_details)).lower(),
        "include_objectives": str(bool(include_objectives)).lower(),
        "limit": str(len(safe_match_ids)),
    }
    if account_ids:
        params["account_ids"] = ",".join(_safe_ids(account_ids))
    if hero_ids:
        params["hero_ids"] = ",".join(_safe_ids(hero_ids))
    url = f"{BASE_URL}/v1/matches/metadata?{urllib.parse.urlencode(params)}"
    payload = _get_deadlock_api_json(http, url, cache_ttl_seconds=cache_ttl_seconds)
    rows = payload if isinstance(payload, list) else []
    external_id = f"match-metadata:{','.join(safe_match_ids)}"
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    raw_path = store.write_raw(source=SOURCE, external_id=external_id, content=raw, suffix="json")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id=external_id,
        title=f"Deadlock API match metadata {','.join(safe_match_ids)}",
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata={
            "match_ids": safe_match_ids,
            "account_ids": _safe_ids(account_ids or []),
            "hero_ids": _safe_ids(hero_ids or []),
            "cache_ttl_seconds": cache_ttl_seconds,
            "include_player_items": include_player_items,
            "include_player_stats": include_player_stats,
            "include_player_death_details": include_player_death_details,
            "include_objectives": include_objectives,
        },
    )
    snapshots = []
    for row in rows:
        if not isinstance(row, dict):
            continue
        match_id = str(row.get("match_id") or row.get("matchId") or "").strip()
        if not match_id:
            continue
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": "deadlock_api_match_metadata",
                "external_id": match_id,
                "canonical_name": match_id,
                "payload": _with_source_metadata(row, source_url=url, match_id=match_id),
            }
        )
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {"url": url, "matches": len(rows), "snapshots": count}


def _get_deadlock_api_json(http: HttpClient, url: str, *, cache_ttl_seconds: int) -> Any:
    result = http.get(
        url,
        cache_ttl_seconds=cache_ttl_seconds,
        timeout=60,
        headers={
            "Accept": "application/json",
            "Referer": "https://deadlock-api.com/",
        },
    )
    return json.loads(result.text)


def _safe_ids(values: list[str]) -> list[str]:
    result = []
    for value in values:
        raw = str(value or "").strip()
        if raw and raw not in result:
            result.append(raw)
    return result


def _with_source_metadata(payload: Any, **metadata: Any) -> dict[str, Any]:
    copied = dict(payload) if isinstance(payload, dict) else {"value": payload}
    copied["_deadlock_brain"] = metadata
    return copied
