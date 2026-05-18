from __future__ import annotations

import json
import sqlite3
import time
import urllib.parse
from typing import Any

from deadlock_brain.http import HttpClient
from deadlock_brain.storage import BrainStore


SOURCE = "statlocker"
BASE_URL = "https://statlocker.gg"
DEFAULT_RANKS = "rank_10,rank_11"


def pull_statlocker(
    store: BrainStore,
    http: HttpClient,
    *,
    kinds: list[str] | None = None,
    patch: str | None = None,
    hero: str = "all",
    min_sample_size: int = 500,
    rank: str = DEFAULT_RANKS,
    leaderboard_page: int = 1,
    leaderboard_page_size: int = 100,
    account_id: str | None = None,
    match_id: str | None = None,
    hero_id: str | None = None,
    players_from_leaderboard: int = 5,
    matches_per_player: int = 6,
    include_match_details: bool = False,
    include_build_analysis: bool = False,
    game_mode: str | None = None,
    delay_seconds: float = 1.0,
    cache_ttl_seconds: int = 21600,
) -> dict[str, Any]:
    selected = kinds or ["wpa-patches", "wpa-items", "leaderboard"]
    summary: dict[str, Any] = {"endpoints": {}, "snapshots": 0}
    latest_patch = patch

    if "wpa-patches" in selected or "wpa-items" in selected:
        patches_summary, latest = _pull_wpa_patches(store, http, cache_ttl_seconds=cache_ttl_seconds)
        summary["endpoints"]["wpa-patches"] = patches_summary
        summary["snapshots"] += int(patches_summary.get("snapshots", 0))
        latest_patch = latest_patch or latest

    if "wpa-items" in selected:
        if not latest_patch:
            raise RuntimeError("Kein Statlocker WPA-Patch gefunden.")
        items_summary = _pull_wpa_items(
            store,
            http,
            patch=_normalize_patch(latest_patch),
            hero=_normalize_hero(hero),
            min_sample_size=min_sample_size,
            rank=rank,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        summary["endpoints"]["wpa-items"] = items_summary
        summary["snapshots"] += int(items_summary.get("snapshots", 0))

    if "leaderboard" in selected:
        leaderboard_summary = _pull_leaderboard(
            store,
            http,
            page=leaderboard_page,
            page_size=leaderboard_page_size,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        summary["endpoints"]["leaderboard"] = leaderboard_summary
        summary["snapshots"] += int(leaderboard_summary.get("snapshots", 0))

    if "player-profile" in selected:
        if not account_id:
            raise ValueError("--account-id ist fuer player-profile erforderlich.")
        profile_summary = _pull_player_profile(store, http, account_id=account_id, cache_ttl_seconds=cache_ttl_seconds)
        summary["endpoints"]["player-profile"] = profile_summary
        summary["snapshots"] += int(profile_summary.get("snapshots", 0))

    if "player-matches" in selected:
        if not account_id:
            raise ValueError("--account-id ist fuer player-matches erforderlich.")
        matches_summary = _pull_player_matches(
            store,
            http,
            account_id=account_id,
            limit=matches_per_player,
            game_mode=game_mode,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        summary["endpoints"]["player-matches"] = _without_rows(matches_summary)
        summary["snapshots"] += int(matches_summary.get("snapshots", 0))
        match_rows = list(matches_summary.get("match_rows") or [])
        if include_match_details:
            details_summary = _pull_match_details_for_rows(
                store,
                http,
                match_rows,
                cache_ttl_seconds=cache_ttl_seconds,
                delay_seconds=delay_seconds,
            )
            summary["endpoints"]["match-detail"] = details_summary
            summary["snapshots"] += int(details_summary.get("snapshots", 0))
        if include_build_analysis:
            build_summary = _pull_build_analysis_for_rows(
                store,
                http,
                match_rows,
                cache_ttl_seconds=cache_ttl_seconds,
                delay_seconds=delay_seconds,
            )
            summary["endpoints"]["player-build-analysis"] = build_summary
            summary["snapshots"] += int(build_summary.get("snapshots", 0))

    if "match-detail" in selected:
        if not match_id:
            raise ValueError("--match-id ist fuer match-detail erforderlich.")
        detail_summary = _pull_match_detail(store, http, match_id=match_id, cache_ttl_seconds=cache_ttl_seconds)
        summary["endpoints"]["match-detail"] = detail_summary
        summary["snapshots"] += int(detail_summary.get("snapshots", 0))

    if "player-build-analysis" in selected:
        if not account_id or not hero_id:
            raise ValueError("--account-id und --hero-id sind fuer player-build-analysis erforderlich.")
        build_summary = _pull_player_build_analysis(
            store,
            http,
            account_id=account_id,
            hero_id=hero_id,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        summary["endpoints"]["player-build-analysis"] = build_summary
        summary["snapshots"] += int(build_summary.get("snapshots", 0))

    if "leaderboard-player-matches" in selected:
        player_summary = _pull_leaderboard_player_matches(
            store,
            http,
            players_from_leaderboard=players_from_leaderboard,
            matches_per_player=matches_per_player,
            include_match_details=include_match_details,
            include_build_analysis=include_build_analysis,
            game_mode=game_mode,
            delay_seconds=delay_seconds,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        summary["endpoints"]["leaderboard-player-matches"] = player_summary
        summary["snapshots"] += int(player_summary.get("snapshots", 0))

    known = {
        "wpa-patches",
        "wpa-items",
        "leaderboard",
        "player-profile",
        "player-matches",
        "match-detail",
        "player-build-analysis",
        "leaderboard-player-matches",
    }
    unknown = sorted(set(selected) - known)
    if unknown:
        raise ValueError(f"Unbekannte Statlocker-Kinds: {', '.join(unknown)}")
    return summary


def _pull_wpa_patches(store: BrainStore, http: HttpClient, *, cache_ttl_seconds: int) -> tuple[dict[str, Any], str | None]:
    url = f"{BASE_URL}/api/info/wpa-patches"
    payload = _get_statlocker_json(http, url, referer=f"{BASE_URL}/vision/wpa", cache_ttl_seconds=cache_ttl_seconds)
    rows = payload if isinstance(payload, list) else []
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    raw_path = store.write_raw(source=SOURCE, external_id="wpa-patches", content=raw, suffix="json")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id="wpa-patches",
        title="Statlocker WPA patches",
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata={"cache_ttl_seconds": cache_ttl_seconds},
    )
    snapshots = []
    latest_patch = None
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            continue
        patch_id = str(row.get("minorPatchId") or "").strip()
        if not patch_id:
            continue
        latest_patch = latest_patch or f"patch_{patch_id}"
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": "statlocker_wpa_patch",
                "external_id": f"patch_{patch_id}",
                "canonical_name": f"patch_{patch_id}",
                "payload": _with_source_metadata(row, source_url=url),
            }
        )
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {"url": url, "patches": len(rows), "snapshots": count, "latest_patch": latest_patch}, latest_patch


def _pull_wpa_items(
    store: BrainStore,
    http: HttpClient,
    *,
    patch: str,
    hero: str,
    min_sample_size: int,
    rank: str,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    params = {
        "hero": hero,
        "tier": "all",
        "rank": rank,
        "category": "all",
        "gameState": "all",
        "purchaseTime": "all",
        "teamComp": "Average Comp",
        "buildType": "all",
        "patch": patch,
        "minSampleSize": str(max(1, int(min_sample_size))),
        "searchTerm": "",
        "sortBy": "wpa",
    }
    query = urllib.parse.urlencode(params)
    url = f"{BASE_URL}/api/info/wpa-filtered-items?{query}"
    referer = f"{BASE_URL}/vision/wpa?min={min_sample_size}&mode=items-heroes&patch={patch}"
    payload = _get_statlocker_json(http, url, referer=referer, cache_ttl_seconds=cache_ttl_seconds)
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    external_id = f"wpa-items:{patch}:{hero}:min{min_sample_size}:rank{rank}"
    raw_path = store.write_raw(source=SOURCE, external_id=external_id, content=raw, suffix="json")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id=external_id,
        title=f"Statlocker WPA items {patch} {hero}",
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata={"patch": patch, "hero": hero, "min_sample_size": min_sample_size, "rank": rank},
    )
    items = payload.get("items") if isinstance(payload, dict) else []
    items = items if isinstance(items, list) else []
    snapshots = []
    for item in items:
        if not isinstance(item, dict):
            continue
        item_name = str(item.get("item") or "").strip()
        if not item_name:
            continue
        item_payload = _with_source_metadata(
            item,
            source_url=url,
            patch=patch,
            requested_hero=hero,
            min_sample_size=min_sample_size,
            rank=rank,
            signal_kind="item_wpa",
        )
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": "statlocker_wpa_item",
                "external_id": f"{patch}:{hero}:{item_name}",
                "canonical_name": item_name,
                "payload": item_payload,
            }
        )
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {"url": url, "patch": patch, "hero": hero, "items": len(items), "snapshots": count}


def _pull_leaderboard(
    store: BrainStore,
    http: HttpClient,
    *,
    page: int,
    page_size: int,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    safe_page = max(1, int(page))
    safe_page_size = max(1, min(int(page_size), 100))
    query = urllib.parse.urlencode({"page": safe_page, "pageSize": safe_page_size, "version": 2})
    url = f"{BASE_URL}/api/leaderboard/get-pp-rankings/?{query}"
    payload = _get_statlocker_json(http, url, referer=f"{BASE_URL}/statlocker-leaderboard", cache_ttl_seconds=cache_ttl_seconds)
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    external_id = f"leaderboard:global:page{safe_page}:size{safe_page_size}:v2"
    raw_path = store.write_raw(source=SOURCE, external_id=external_id, content=raw, suffix="json")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id=external_id,
        title=f"Statlocker leaderboard page {safe_page}",
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata={"page": safe_page, "page_size": safe_page_size, "version": 2},
    )
    rows = payload.get("data") if isinstance(payload, dict) else []
    rows = rows if isinstance(rows, list) else []
    snapshots = []
    for row in rows:
        if not isinstance(row, dict):
            continue
        account_id = str(row.get("accountId") or "").strip()
        if not account_id:
            continue
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": "statlocker_leaderboard_player",
                "external_id": account_id,
                "canonical_name": str(row.get("name") or account_id),
                "payload": _with_source_metadata(row, source_url=url, leaderboard_page=safe_page),
            }
        )
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {
        "url": url,
        "page": payload.get("page") if isinstance(payload, dict) else safe_page,
        "total_count": payload.get("totalCount") if isinstance(payload, dict) else None,
        "players": len(rows),
        "snapshots": count,
    }


def _pull_player_profile(
    store: BrainStore,
    http: HttpClient,
    *,
    account_id: str,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    safe_account_id = _safe_required_id(account_id, "account_id")
    url = f"{BASE_URL}/api/profile/steam-profile/{urllib.parse.quote(safe_account_id)}"
    payload = _get_statlocker_json(http, url, referer=f"{BASE_URL}/profile/{safe_account_id}", cache_ttl_seconds=cache_ttl_seconds)
    document_id = _store_json_document(
        store,
        external_id=f"player-profile:{safe_account_id}",
        title=f"Statlocker player profile {safe_account_id}",
        url=url,
        payload=payload,
        metadata={"account_id": safe_account_id, "cache_ttl_seconds": cache_ttl_seconds},
    )
    profile_name = _first_string(payload, ("name", "personaName", "personaname", "steamName", "displayName")) or safe_account_id
    count = store.insert_many_snapshots(
        [
            {
                "source": SOURCE,
                "entity_type": "statlocker_player_profile",
                "external_id": safe_account_id,
                "canonical_name": profile_name,
                "payload": _with_source_metadata(payload, source_url=url, account_id=safe_account_id),
            }
        ],
        source_document_id=document_id,
    )
    return {"url": url, "account_id": safe_account_id, "snapshots": count}


def _pull_player_matches(
    store: BrainStore,
    http: HttpClient,
    *,
    account_id: str,
    limit: int,
    game_mode: str | None,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    safe_account_id = _safe_required_id(account_id, "account_id")
    safe_limit = max(1, min(int(limit), 50))
    params: dict[str, Any] = {"offset": 0, "limit": safe_limit}
    mode_value = _game_mode_value(game_mode)
    if mode_value:
        params["gameMode"] = mode_value
    query = urllib.parse.urlencode(params)
    url = f"{BASE_URL}/api/profile/data/matches/{urllib.parse.quote(safe_account_id)}/concise?{query}"
    payload = _get_statlocker_json(http, url, referer=f"{BASE_URL}/profile/{safe_account_id}", cache_ttl_seconds=cache_ttl_seconds)
    rows = _extract_rows(payload)
    external_id = f"player-matches:{safe_account_id}:limit{safe_limit}:{mode_value or 'all'}"
    document_id = _store_json_document(
        store,
        external_id=external_id,
        title=f"Statlocker player matches {safe_account_id}",
        url=url,
        payload=payload,
        metadata={"account_id": safe_account_id, "limit": safe_limit, "game_mode": game_mode},
    )
    snapshots = []
    match_rows = []
    for index, row in enumerate(rows):
        if not isinstance(row, dict):
            continue
        match_id = _match_id_from_payload(row) or f"row{index}"
        hero_id = _hero_id_from_payload(row)
        row_payload = _with_source_metadata(
            row,
            source_url=url,
            account_id=safe_account_id,
            match_id=match_id,
            hero_id=hero_id,
            signal_kind="player_match",
        )
        snapshots.append(
            {
                "source": SOURCE,
                "entity_type": "statlocker_player_match",
                "external_id": f"{safe_account_id}:{match_id}",
                "canonical_name": f"{safe_account_id}:{match_id}",
                "payload": row_payload,
            }
        )
        match_rows.append({"account_id": safe_account_id, "match_id": str(match_id), "hero_id": hero_id, "payload": row_payload})
    count = store.insert_many_snapshots(snapshots, source_document_id=document_id)
    return {"url": url, "account_id": safe_account_id, "matches": len(match_rows), "snapshots": count, "match_rows": match_rows}


def _pull_match_detail(
    store: BrainStore,
    http: HttpClient,
    *,
    match_id: str,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    safe_match_id = _safe_required_id(match_id, "match_id")
    url = f"{BASE_URL}/api/match/{urllib.parse.quote(safe_match_id)}"
    payload = _get_statlocker_json(http, url, referer=f"{BASE_URL}/match/{safe_match_id}", cache_ttl_seconds=cache_ttl_seconds)
    document_id = _store_json_document(
        store,
        external_id=f"match-detail:{safe_match_id}",
        title=f"Statlocker match detail {safe_match_id}",
        url=url,
        payload=payload,
        metadata={"match_id": safe_match_id, "cache_ttl_seconds": cache_ttl_seconds},
    )
    count = store.insert_many_snapshots(
        [
            {
                "source": SOURCE,
                "entity_type": "statlocker_match_detail",
                "external_id": safe_match_id,
                "canonical_name": safe_match_id,
                "payload": _with_source_metadata(payload, source_url=url, match_id=safe_match_id),
            }
        ],
        source_document_id=document_id,
    )
    return {"url": url, "match_id": safe_match_id, "snapshots": count}


def _pull_player_build_analysis(
    store: BrainStore,
    http: HttpClient,
    *,
    account_id: str,
    hero_id: str,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    safe_account_id = _safe_required_id(account_id, "account_id")
    safe_hero_id = _safe_required_id(hero_id, "hero_id")
    url = f"{BASE_URL}/api/info/player-build-analysis/{urllib.parse.quote(safe_account_id)}/{urllib.parse.quote(safe_hero_id)}"
    payload = _get_statlocker_json(
        http,
        url,
        referer=f"{BASE_URL}/profile/{safe_account_id}",
        cache_ttl_seconds=cache_ttl_seconds,
    )
    document_id = _store_json_document(
        store,
        external_id=f"player-build-analysis:{safe_account_id}:{safe_hero_id}",
        title=f"Statlocker player build analysis {safe_account_id} hero {safe_hero_id}",
        url=url,
        payload=payload,
        metadata={"account_id": safe_account_id, "hero_id": safe_hero_id, "cache_ttl_seconds": cache_ttl_seconds},
    )
    count = store.insert_many_snapshots(
        [
            {
                "source": SOURCE,
                "entity_type": "statlocker_player_build_analysis",
                "external_id": f"{safe_account_id}:{safe_hero_id}",
                "canonical_name": f"{safe_account_id}:{safe_hero_id}",
                "payload": _with_source_metadata(payload, source_url=url, account_id=safe_account_id, hero_id=safe_hero_id),
            }
        ],
        source_document_id=document_id,
    )
    return {"url": url, "account_id": safe_account_id, "hero_id": safe_hero_id, "snapshots": count}


def _pull_leaderboard_player_matches(
    store: BrainStore,
    http: HttpClient,
    *,
    players_from_leaderboard: int,
    matches_per_player: int,
    include_match_details: bool,
    include_build_analysis: bool,
    game_mode: str | None,
    delay_seconds: float,
    cache_ttl_seconds: int,
) -> dict[str, Any]:
    players = _latest_leaderboard_accounts(store.conn, limit=players_from_leaderboard)
    snapshots = 0
    results = []
    for index, player in enumerate(players):
        account_id = str(player.get("account_id") or "")
        if not account_id:
            continue
        player_result: dict[str, Any] = {"account_id": account_id, "name": player.get("name")}
        profile_summary = _pull_player_profile(store, http, account_id=account_id, cache_ttl_seconds=cache_ttl_seconds)
        snapshots += int(profile_summary.get("snapshots", 0))
        player_result["profile"] = profile_summary
        if delay_seconds > 0:
            time.sleep(float(delay_seconds))
        matches_summary = _pull_player_matches(
            store,
            http,
            account_id=account_id,
            limit=matches_per_player,
            game_mode=game_mode,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        snapshots += int(matches_summary.get("snapshots", 0))
        player_result["matches"] = _without_rows(matches_summary)
        match_rows = list(matches_summary.get("match_rows") or [])
        if include_match_details:
            if delay_seconds > 0:
                time.sleep(float(delay_seconds))
            details_summary = _pull_match_details_for_rows(
                store,
                http,
                match_rows,
                cache_ttl_seconds=cache_ttl_seconds,
                delay_seconds=delay_seconds,
            )
            snapshots += int(details_summary.get("snapshots", 0))
            player_result["match_details"] = details_summary
        if include_build_analysis:
            if delay_seconds > 0:
                time.sleep(float(delay_seconds))
            build_summary = _pull_build_analysis_for_rows(
                store,
                http,
                match_rows,
                cache_ttl_seconds=cache_ttl_seconds,
                delay_seconds=delay_seconds,
            )
            snapshots += int(build_summary.get("snapshots", 0))
            player_result["build_analysis"] = build_summary
        results.append(player_result)
        if delay_seconds > 0 and index < len(players) - 1:
            time.sleep(float(delay_seconds))
    return {
        "players_selected": len(players),
        "matches_per_player": max(1, min(int(matches_per_player), 50)),
        "include_match_details": bool(include_match_details),
        "include_build_analysis": bool(include_build_analysis),
        "snapshots": snapshots,
        "players": results,
    }


def _pull_match_details_for_rows(
    store: BrainStore,
    http: HttpClient,
    rows: list[dict[str, Any]],
    *,
    cache_ttl_seconds: int,
    delay_seconds: float,
) -> dict[str, Any]:
    snapshots = 0
    pulled = []
    match_ids = _unique_match_ids(rows)
    for index, match_id in enumerate(match_ids):
        detail = _pull_match_detail(store, http, match_id=match_id, cache_ttl_seconds=cache_ttl_seconds)
        snapshots += int(detail.get("snapshots", 0))
        pulled.append({"match_id": match_id, "snapshots": detail.get("snapshots", 0)})
        if delay_seconds > 0 and index < len(match_ids) - 1:
            time.sleep(float(delay_seconds))
    return {"matches": len(pulled), "snapshots": snapshots, "pulled": pulled}


def _pull_build_analysis_for_rows(
    store: BrainStore,
    http: HttpClient,
    rows: list[dict[str, Any]],
    *,
    cache_ttl_seconds: int,
    delay_seconds: float,
) -> dict[str, Any]:
    pairs = sorted({(str(row.get("account_id") or ""), str(row.get("hero_id") or "")) for row in rows if row.get("account_id") and row.get("hero_id")})
    snapshots = 0
    pulled = []
    for index, (account_id, hero_id) in enumerate(pairs):
        build = _pull_player_build_analysis(
            store,
            http,
            account_id=account_id,
            hero_id=hero_id,
            cache_ttl_seconds=cache_ttl_seconds,
        )
        snapshots += int(build.get("snapshots", 0))
        pulled.append({"account_id": account_id, "hero_id": hero_id, "snapshots": build.get("snapshots", 0)})
        if delay_seconds > 0 and index < len(pairs) - 1:
            time.sleep(float(delay_seconds))
    return {"pairs": len(pulled), "snapshots": snapshots, "pulled": pulled}


def _get_statlocker_json(http: HttpClient, url: str, *, referer: str, cache_ttl_seconds: int) -> Any:
    result = http.get(
        url,
        cache_ttl_seconds=cache_ttl_seconds,
        timeout=30,
        headers={
            "Accept": "application/json",
            "Origin": BASE_URL,
            "Referer": referer,
        },
    )
    return json.loads(result.text)


def _store_json_document(
    store: BrainStore,
    *,
    external_id: str,
    title: str,
    url: str,
    payload: Any,
    metadata: dict[str, Any],
) -> int:
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    raw_path = store.write_raw(source=SOURCE, external_id=external_id, content=raw, suffix="json")
    return store.upsert_source_document(
        source=SOURCE,
        external_id=external_id,
        title=title,
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata=metadata,
    )


def _latest_leaderboard_accounts(conn: sqlite3.Connection, *, limit: int) -> list[dict[str, Any]]:
    rows = conn.execute(
        """
        SELECT external_id, canonical_name, payload_json, MAX(fetched_at) AS latest_fetched_at
        FROM entity_snapshots
        WHERE source=? AND entity_type='statlocker_leaderboard_player'
        GROUP BY external_id
        ORDER BY CAST(json_extract(payload_json, '$.rank') AS INTEGER) ASC,
                 CAST(json_extract(payload_json, '$.leaderboard_page') AS INTEGER) ASC,
                 latest_fetched_at DESC
        LIMIT ?
        """,
        (SOURCE, max(1, min(int(limit), 50))),
    ).fetchall()
    players = []
    for row in rows:
        payload = _loads_json(row["payload_json"], fallback={})
        players.append(
            {
                "account_id": str(row["external_id"]),
                "name": row["canonical_name"],
                "rank": payload.get("rank") if isinstance(payload, dict) else None,
                "payload": payload,
            }
        )
    return players


def _extract_rows(payload: Any) -> list[Any]:
    if isinstance(payload, list):
        return payload
    if not isinstance(payload, dict):
        return []
    for key in ("matchHistory", "match_history", "data", "matches", "rows", "results", "items"):
        value = payload.get(key)
        if isinstance(value, list):
            return value
    return []


def _match_id_from_payload(payload: Any) -> str | None:
    if not isinstance(payload, dict):
        return None
    for key in ("match_id", "matchId", "id", "match"):
        value = payload.get(key)
        if value is not None and str(value).strip():
            return str(value).strip()
    metadata = payload.get("_deadlock_brain") if isinstance(payload.get("_deadlock_brain"), dict) else {}
    value = metadata.get("match_id")
    return str(value).strip() if value is not None and str(value).strip() else None


def _hero_id_from_payload(payload: Any) -> str | None:
    if not isinstance(payload, dict):
        return None
    for key in ("hero_id", "heroId", "player_hero_id", "playerHeroId", "hero"):
        value = payload.get(key)
        if value is not None and str(value).strip():
            return str(value).strip()
    metadata = payload.get("_deadlock_brain") if isinstance(payload.get("_deadlock_brain"), dict) else {}
    value = metadata.get("hero_id")
    return str(value).strip() if value is not None and str(value).strip() else None


def _unique_match_ids(rows: list[dict[str, Any]]) -> list[str]:
    result = []
    seen = set()
    for row in rows:
        match_id = _match_id_from_payload(row.get("payload")) or str(row.get("match_id") or "").strip()
        if match_id and match_id not in seen:
            seen.add(match_id)
            result.append(match_id)
    return result


def _game_mode_value(game_mode: str | None) -> str | None:
    raw = str(game_mode or "").strip().casefold()
    if not raw or raw == "all":
        return None
    if raw in {"standard", "normal", "ranked"}:
        return "1"
    if raw in {"brawl", "street-brawl", "street_brawl"}:
        return "4"
    return raw


def _safe_required_id(value: str, label: str) -> str:
    raw = str(value or "").strip()
    if not raw:
        raise ValueError(f"{label} fehlt.")
    return raw


def _first_string(payload: Any, keys: tuple[str, ...]) -> str | None:
    if not isinstance(payload, dict):
        return None
    for key in keys:
        value = payload.get(key)
        if value is not None and str(value).strip():
            return str(value).strip()
    return None


def _without_rows(summary: dict[str, Any]) -> dict[str, Any]:
    return {key: value for key, value in summary.items() if key != "match_rows"}


def _loads_json(value: Any, *, fallback: Any) -> Any:
    if isinstance(value, (dict, list)):
        return value
    try:
        return json.loads(str(value or ""))
    except json.JSONDecodeError:
        return fallback


def _normalize_patch(patch: str) -> str:
    raw = str(patch or "").strip()
    if not raw:
        return raw
    return raw if raw.startswith("patch_") else f"patch_{raw}"


def _normalize_hero(hero: str) -> str:
    raw = str(hero or "all").strip()
    if not raw or raw.casefold() == "all":
        return "all"
    return raw.replace(" & ", "_and_").replace("&", "and").replace(" ", "_")


def _with_source_metadata(payload: Any, **metadata: Any) -> dict[str, Any]:
    copied = dict(payload) if isinstance(payload, dict) else {"value": payload}
    copied["_deadlock_brain"] = metadata
    return copied
