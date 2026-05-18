from __future__ import annotations

import json
from typing import Any

from deadlock_brain.http import HttpClient
from deadlock_brain.storage import BrainStore


SOURCE = "deadlock_assets_api"
BASE_URL = "https://assets.deadlock-api.com"


ENDPOINTS = {
    "items": "/v2/items",
    "heroes": "/v2/heroes?only_active=true",
    "heroes_all": "/v2/heroes",
    "raw_items": "/raw/items",
    "raw_heroes": "/raw/heroes",
    "ranks": "/v2/ranks",
    "colors": "/v1/colors",
    "build_tags": "/v2/build-tags",
    "npc_units": "/v2/npc-units",
}


def pull_assets(store: BrainStore, http: HttpClient, *, kinds: list[str] | None = None) -> dict[str, Any]:
    selected = kinds or ["items", "heroes", "raw_items", "raw_heroes"]
    summary: dict[str, Any] = {"endpoints": {}, "snapshots": 0}
    for kind in selected:
        if kind not in ENDPOINTS:
            raise ValueError(f"Unbekannter Assets-Endpoint: {kind}")
        url = f"{BASE_URL}{ENDPOINTS[kind]}"
        payload = http.get_json(url, cache_ttl_seconds=3600)
        raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
        raw_path = store.write_raw(source=SOURCE, external_id=kind, content=raw, suffix="json")
        document_id = store.upsert_source_document(
            source=SOURCE,
            external_id=kind,
            title=f"Deadlock Assets API {kind}",
            url=url,
            content_type="application/json",
            raw_path=raw_path,
            content=raw,
            metadata={"endpoint": ENDPOINTS[kind]},
        )

        snapshots = _snapshots_for(kind, payload)
        count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
        summary["endpoints"][kind] = {"url": url, "items": _payload_len(payload), "snapshots": count}
        summary["snapshots"] += count
    return summary


def _payload_len(payload: Any) -> int:
    if isinstance(payload, list):
        return len(payload)
    if isinstance(payload, dict):
        return len(payload)
    return 1


def _snapshots_for(kind: str, payload: Any) -> list[dict[str, Any]]:
    if not isinstance(payload, list):
        return []
    if kind in {"items", "raw_items"}:
        entity_type = "item_or_ability"
    elif kind in {"heroes", "raw_heroes"}:
        entity_type = "hero"
    elif kind == "ranks":
        entity_type = "rank"
    elif kind == "build_tags":
        entity_type = "build_tag"
    elif kind == "npc_units":
        entity_type = "npc_unit"
    else:
        entity_type = kind

    snapshots = []
    for entry in payload:
        if not isinstance(entry, dict):
            continue
        external_id = str(entry.get("id") or entry.get("class_name") or entry.get("name") or len(snapshots))
        canonical_name = entry.get("name") or entry.get("class_name")
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": entity_type,
                "external_id": external_id,
                "canonical_name": str(canonical_name) if canonical_name else None,
                "payload": entry,
            }
        )
    return snapshots
