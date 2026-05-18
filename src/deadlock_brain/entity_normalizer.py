from __future__ import annotations

import json
import re
from dataclasses import dataclass, field
from typing import Any

from deadlock_brain.sources.assets_api import SOURCE as ASSETS_SOURCE
from deadlock_brain.storage import BrainStore


PUBLIC_ITEM_TYPES = {"upgrade"}
PUBLIC_ITEM_SLOTS = {"weapon", "vitality", "spirit"}
INTERNAL_NAME_PREFIXES = (
    "ability_",
    "citadel_",
    "hero_",
    "item_",
    "melee_",
    "upgrade_",
    "weapon_",
)
INTERNAL_ENTITY_TYPES = {"hero_internal", "ability_internal", "weapon_or_internal"}


@dataclass
class EntityCandidate:
    entity_type: str
    canonical_name: str
    external_id: str | None
    snapshot_id: int | None
    source: str
    metadata: dict[str, Any] = field(default_factory=dict)
    aliases: set[tuple[str, str]] = field(default_factory=set)


@dataclass(frozen=True)
class AssetContext:
    active_hero_ids: set[str] = field(default_factory=set)
    internal_hero_ids: set[str] = field(default_factory=set)
    active_hero_tokens: set[str] = field(default_factory=set)
    internal_hero_tokens: set[str] = field(default_factory=set)
    public_item_names_by_class: dict[str, str] = field(default_factory=dict)
    special_item_names_by_class: dict[str, str] = field(default_factory=dict)
    active_ability_names_by_class: dict[str, str] = field(default_factory=dict)
    internal_ability_names_by_class: dict[str, str] = field(default_factory=dict)


def normalize_entities(store: BrainStore, *, rebuild: bool = False) -> dict[str, Any]:
    if rebuild:
        deleted = store.clear_entities()
    else:
        deleted = {"aliases": 0, "entities": 0}

    candidates = _collect_asset_candidates(store)
    inserted = 0
    aliases = 0
    skipped = 0
    by_type: dict[str, int] = {}

    for candidate in candidates.values():
        if not candidate.canonical_name:
            skipped += 1
            continue
        entity_id = store.upsert_entity(
            entity_type=candidate.entity_type,
            canonical_name=candidate.canonical_name,
            primary_external_id=candidate.external_id,
            source=candidate.source,
            first_snapshot_id=candidate.snapshot_id,
            metadata=candidate.metadata,
        )
        inserted += 1
        by_type[candidate.entity_type] = by_type.get(candidate.entity_type, 0) + 1
        for alias, alias_kind in sorted(candidate.aliases):
            alias_norm = normalize_alias(alias)
            if not alias_norm:
                continue
            if store.upsert_entity_alias(
                entity_id=entity_id,
                alias=alias,
                alias_norm=alias_norm,
                alias_kind=alias_kind,
                source=candidate.source,
                external_id=candidate.external_id,
                snapshot_id=candidate.snapshot_id,
            ):
                aliases += 1

    return {
        "deleted": deleted,
        "entities": inserted,
        "aliases": aliases,
        "skipped": skipped,
        "by_type": dict(sorted(by_type.items())),
    }


def normalize_alias(value: str) -> str:
    return re.sub(r"\s+", " ", value.strip().casefold().replace("_", " "))


def _collect_asset_candidates(store: BrainStore) -> dict[tuple[str, str], EntityCandidate]:
    rows = store.conn.execute(
        """
        SELECT es.id, es.entity_type, es.external_id, es.canonical_name,
               es.payload_json, sd.external_id AS document_kind
        FROM entity_snapshots es
        LEFT JOIN source_documents sd ON sd.id=es.source_document_id
        WHERE es.source=?
        ORDER BY
          CASE sd.external_id
            WHEN 'heroes' THEN 0
            WHEN 'items' THEN 1
            WHEN 'raw_heroes' THEN 2
            WHEN 'raw_items' THEN 3
            ELSE 4
          END,
          es.id
        """,
        (ASSETS_SOURCE,),
    ).fetchall()

    context = _build_asset_context(rows)
    candidates: dict[tuple[str, str], EntityCandidate] = {}
    class_index: dict[tuple[str, str], tuple[str, str]] = {}
    for row in rows:
        try:
            payload = json.loads(row["payload_json"])
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue

        entity_type = _classify_snapshot(row["entity_type"], payload, context)
        canonical_name = _canonical_name(entity_type, row["canonical_name"], payload, context)
        if not canonical_name:
            continue

        class_name = str(payload.get("class_name") or "").strip()
        class_key = (entity_type, class_name.casefold()) if class_name else None
        key = class_index.get(class_key) if class_key else None
        if key is None:
            key = (entity_type, canonical_name)
        metadata = _metadata_for(row["document_kind"], payload)
        candidate = candidates.get(key)
        if candidate is None:
            candidate = EntityCandidate(
                entity_type=entity_type,
                canonical_name=canonical_name,
                external_id=str(row["external_id"]) if row["external_id"] is not None else None,
                snapshot_id=int(row["id"]),
                source=ASSETS_SOURCE,
                metadata=metadata,
            )
            candidates[key] = candidate
            if class_key:
                class_index[class_key] = key
        else:
            candidate.metadata = _merge_metadata(candidate.metadata, metadata)
            if class_key:
                class_index[class_key] = key

        _add_aliases(candidate, row["external_id"], row["canonical_name"], payload)

    return candidates


def _build_asset_context(rows) -> AssetContext:
    active_hero_ids: set[str] = set()
    internal_hero_ids: set[str] = set()
    active_hero_tokens: set[str] = set()
    internal_hero_tokens: set[str] = set()
    public_item_names_by_class: dict[str, str] = {}
    special_item_names_by_class: dict[str, str] = {}
    active_ability_names_by_class: dict[str, str] = {}
    internal_ability_names_by_class: dict[str, str] = {}

    for row in rows:
        try:
            payload = json.loads(row["payload_json"])
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue
        if row["entity_type"] == "hero":
            hero_id = str(payload.get("id") or row["external_id"] or "").strip()
            class_token = _strip_prefix(payload.get("class_name"), "hero_")
            if _is_active_hero(payload):
                if hero_id:
                    active_hero_ids.add(hero_id)
                if class_token:
                    active_hero_tokens.add(class_token.casefold())
            else:
                if hero_id:
                    internal_hero_ids.add(hero_id)
                if class_token:
                    internal_hero_tokens.add(class_token.casefold())
    for row in rows:
        try:
            payload = json.loads(row["payload_json"])
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue
        payload_type = str(payload.get("type") or "").casefold()
        class_name = str(payload.get("class_name") or "").strip().casefold()
        if _is_public_item(payload_type, payload):
            class_name = str(payload.get("class_name") or "").strip().casefold()
            name = str(payload.get("name") or row["canonical_name"] or "").strip()
            if class_name and name:
                if _is_special_item(payload):
                    special_item_names_by_class[class_name] = name
                else:
                    public_item_names_by_class[class_name] = name
        elif payload_type == "ability":
            name = str(payload.get("name") or row["canonical_name"] or "").strip()
            if class_name and name:
                if _is_active_ability(payload, active_hero_ids, active_hero_tokens):
                    active_ability_names_by_class[class_name] = name
                elif name and not _looks_internal_name(name):
                    internal_ability_names_by_class[class_name] = name

    return AssetContext(
        active_hero_ids=active_hero_ids,
        internal_hero_ids=internal_hero_ids,
        active_hero_tokens=active_hero_tokens,
        internal_hero_tokens=internal_hero_tokens,
        public_item_names_by_class=public_item_names_by_class,
        special_item_names_by_class=special_item_names_by_class,
        active_ability_names_by_class=active_ability_names_by_class,
        internal_ability_names_by_class=internal_ability_names_by_class,
    )


def _classify_snapshot(snapshot_type: str, payload: dict[str, Any], context: AssetContext) -> str:
    payload_type = str(payload.get("type") or "").casefold()
    if snapshot_type == "hero" or payload_type == "hero":
        return "hero" if _is_active_hero(payload) else "hero_internal"
    if snapshot_type == "rank" or payload_type == "rank":
        return "rank"
    class_name = str(payload.get("class_name") or "").strip().casefold()
    if payload_type == "ability":
        if class_name in context.active_ability_names_by_class:
            return "ability"
        if class_name in context.internal_ability_names_by_class:
            return "ability_internal"
        return "ability" if _is_active_ability(payload, context.active_hero_ids, context.active_hero_tokens) else "ability_internal"
    if class_name in context.public_item_names_by_class:
        return "item"
    if class_name in context.special_item_names_by_class:
        return "item_special"
    if _is_public_item(payload_type, payload):
        return "item_special" if _is_special_item(payload) else "item"
    return "weapon_or_internal"


def _is_active_hero(payload: dict[str, Any]) -> bool:
    return (
        payload.get("disabled") is not True
        and payload.get("player_selectable") is True
        and payload.get("in_development") is not True
        and payload.get("prerelease_only") is not True
        and payload.get("assigned_players_only") is not True
    )


def _is_active_ability(payload: dict[str, Any], active_hero_ids: set[str], active_hero_tokens: set[str]) -> bool:
    name = str(payload.get("name") or "").strip()
    if not name or _looks_internal_name(name):
        return False
    hero = str(payload.get("hero") or "").strip()
    if hero:
        return hero in active_hero_ids
    class_token = _hero_token_from_ability_class(payload.get("class_name"))
    if class_token:
        return class_token in active_hero_tokens
    return False


def _hero_token_from_ability_class(value: Any) -> str | None:
    if not value:
        return None
    text = str(value).casefold()
    for prefix in ("citadel_ability_", "ability_"):
        if text.startswith(prefix):
            rest = text[len(prefix) :]
            token = rest.split("_", 1)[0]
            return token or None
    return None


def _is_public_item(payload_type: str, payload: dict[str, Any]) -> bool:
    if payload_type not in PUBLIC_ITEM_TYPES:
        return False
    if payload.get("disabled") is True:
        return False
    if payload.get("shopable") is not True:
        return False
    if str(payload.get("item_slot_type") or "").casefold() not in PUBLIC_ITEM_SLOTS:
        return False
    name = str(payload.get("name") or "")
    if _looks_internal_name(name) and not payload.get("shop_image"):
        return False
    return bool(payload.get("item_tier") or payload.get("cost") or payload.get("shop_image"))


def _is_special_item(payload: dict[str, Any]) -> bool:
    try:
        tier = int(payload.get("item_tier") or 0)
    except (TypeError, ValueError):
        tier = 0
    try:
        cost = int(payload.get("cost") or 0)
    except (TypeError, ValueError):
        cost = 0
    return tier > 4 or cost >= 9000


def _canonical_name(
    entity_type: str,
    snapshot_name: Any,
    payload: dict[str, Any],
    context: AssetContext,
) -> str | None:
    class_name = str(payload.get("class_name") or "").strip().casefold()
    if entity_type == "item" and class_name in context.public_item_names_by_class:
        return context.public_item_names_by_class[class_name]
    if entity_type == "item_special" and class_name in context.special_item_names_by_class:
        return context.special_item_names_by_class[class_name]
    if entity_type == "ability" and class_name in context.active_ability_names_by_class:
        return context.active_ability_names_by_class[class_name]
    if entity_type == "ability_internal" and class_name in context.internal_ability_names_by_class:
        return context.internal_ability_names_by_class[class_name]
    name = str(payload.get("name") or snapshot_name or payload.get("class_name") or "").strip()
    if not name:
        return None
    if entity_type in {"hero", "hero_internal", "item", "item_special", "ability", "ability_internal", "rank"}:
        return name
    return str(payload.get("class_name") or name).strip() or None


def _add_aliases(candidate: EntityCandidate, external_id: Any, snapshot_name: Any, payload: dict[str, Any]) -> None:
    if candidate.entity_type not in INTERNAL_ENTITY_TYPES:
        _add_alias(candidate, candidate.canonical_name, "canonical")
        _add_alias(candidate, snapshot_name, "snapshot_name")
    _add_alias(candidate, external_id, "external_id")
    _add_alias(candidate, payload.get("id"), "external_id")
    _add_alias(candidate, payload.get("class_name"), "class_name")


def _add_alias(candidate: EntityCandidate, value: Any, alias_kind: str) -> None:
    if value is None:
        return
    alias = str(value).strip()
    if alias:
        candidate.aliases.add((alias, alias_kind))


def _strip_prefix(value: Any, prefix: str) -> str | None:
    if not value:
        return None
    text = str(value)
    if text.startswith(prefix):
        return text[len(prefix) :]
    return None


def _looks_internal_name(name: str) -> bool:
    stripped = name.strip()
    if not stripped:
        return True
    lowered = stripped.casefold()
    return any(lowered.startswith(prefix) for prefix in INTERNAL_NAME_PREFIXES)


def _metadata_for(document_kind: str | None, payload: dict[str, Any]) -> dict[str, Any]:
    keys = (
        "type",
        "ability_type",
        "activation",
        "item_slot_type",
        "item_tier",
        "cost",
        "shopable",
        "disabled",
        "hero",
    )
    metadata = {key: payload[key] for key in keys if key in payload}
    if document_kind:
        metadata["document_kinds"] = [document_kind]
    return metadata


def _merge_metadata(existing: dict[str, Any], new: dict[str, Any]) -> dict[str, Any]:
    merged = dict(existing)
    existing_kinds = set(merged.get("document_kinds") or [])
    new_kinds = set(new.get("document_kinds") or [])
    if existing_kinds or new_kinds:
        merged["document_kinds"] = sorted(existing_kinds | new_kinds)
    for key, value in new.items():
        if key != "document_kinds" and key not in merged:
            merged[key] = value
    return merged
