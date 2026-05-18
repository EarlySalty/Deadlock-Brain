from __future__ import annotations

import json
import re
from dataclasses import dataclass
from typing import Any

from deadlock_brain.storage import BrainStore, stable_hash_text


SECTION_RE = re.compile(r"^\s*(?:#{1,4}\s*)?(?:\[\s*)?([A-Za-z][A-Za-z0-9 &/+-]{1,60})(?:\s*\])?\s*:?\s*$")
BULLET_RE = re.compile(r"^\s*(?:[-*]\s+|\d+[.)]\s+)(.+?)\s*$")
SUBJECT_RE = re.compile(r"^(?P<subject>\*\*[^*]+\*\*|[^:]{2,80}):\s*(?P<body>.+)$")
FROM_TO_RE = re.compile(r"\bfrom\s+(.+?)\s+to\s+(.+?)(?:[.;,)]|$)", re.IGNORECASE)
QUOTED_FROM_TO_RE = re.compile(r'from\s+"([^"]+)"\s+to\s+"([^"]+)"', re.IGNORECASE)

KNOWN_SECTIONS = {
    "general": "General",
    "items": "Items",
    "item": "Items",
    "heroes": "Heroes",
    "hero": "Heroes",
    "map": "Map",
    "ui": "UI",
    "audio": "Audio",
    "misc": "Misc",
    "street brawl": "Street Brawl",
}

POSITIVE_STATS = (
    "damage",
    "dps",
    "health",
    "regen",
    "resistance",
    "resist",
    "range",
    "radius",
    "speed",
    "sprint",
    "fire rate",
    "spirit power",
    "lifesteal",
    "duration",
    "stamina",
    "ammo",
    "barrier",
    "heal",
    "healing",
    "scaling",
    "souls",
    "bounty",
)

NEGATIVE_STATS = (
    "cooldown",
    "recharge",
    "delay",
    "cost",
    "falloff",
)
GENERIC_INTERNAL_KEYS = {"melee"}


@dataclass(frozen=True)
class EntityIndex:
    heroes: set[str]
    hero_names: dict[str, str]
    hero_internals: set[str]
    hero_internal_names: dict[str, str]
    items: set[str]
    item_names: dict[str, str]
    special_items: set[str]
    special_item_names: dict[str, str]
    abilities: set[str]
    ability_names: dict[str, str]
    internal_abilities: set[str]
    internal_ability_names: dict[str, str]
    internals: set[str]
    internal_names: dict[str, str]

    def canonical(self, name: str, *, section: str | None) -> tuple[str, str | None, float]:
        cleaned = clean_subject(name)
        key = normalize_key(cleaned)
        if not key:
            return "general", None, 0.35
        if key in self.hero_names:
            return "hero", self.hero_names[key], 0.95
        if key in self.hero_internal_names:
            return "hero_internal", self.hero_internal_names[key], 0.75
        if key in self.item_names:
            return "item", self.item_names[key], 0.92
        if key in self.special_item_names:
            return "item_special", self.special_item_names[key], 0.85
        if key in self.ability_names:
            return "ability", self.ability_names[key], 0.9
        if key in self.internal_ability_names:
            return "ability_internal", self.internal_ability_names[key], 0.65
        if key in self.internal_names:
            return "weapon_or_internal", self.internal_names[key], 0.7
        if section and section.lower() == "heroes":
            return "hero", cleaned, 0.78
        if section and section.lower() in {"items", "street brawl"}:
            return "item", cleaned, 0.75
        return "general", cleaned, 0.45


def parse_all_patchnotes(store: BrainStore, *, rebuild: bool = False) -> dict[str, Any]:
    before_total = store.conn.execute("SELECT COUNT(*) AS count FROM patch_events").fetchone()["count"]
    if rebuild:
        deleted = store.clear_patch_events()
    else:
        deleted = 0
    index = build_entity_index(store)
    rows = store.conn.execute(
        """
        SELECT id, external_id, canonical_name, payload_json
        FROM entity_snapshots
        WHERE entity_type='patchnote'
        ORDER BY id ASC
        """
    ).fetchall()
    inserted = 0
    parsed_patches = 0
    skipped_lines = 0
    source_kinds: dict[str, int] = {}
    for row in rows:
        payload = json.loads(row["payload_json"])
        events, skipped = parse_patchnote_snapshot(
            snapshot_id=int(row["id"]),
            external_id=str(row["external_id"]),
            payload=payload,
            index=index,
        )
        skipped_lines += skipped
        source_kind = classify_source_kind(payload.get("url"))
        source_kinds[source_kind] = source_kinds.get(source_kind, 0) + 1
        for event in events:
            if store.insert_patch_event(event):
                inserted += 1
        parsed_patches += 1
    after_total = store.conn.execute("SELECT COUNT(*) AS count FROM patch_events").fetchone()["count"]
    return {
        "patches": parsed_patches,
        "events_inserted": inserted,
        "events_total": after_total,
        "events_before": before_total,
        "skipped_lines": skipped_lines,
        "deleted_before_parse": deleted,
        "source_kinds": source_kinds,
    }


def parse_patchnote_snapshot(
    *,
    snapshot_id: int,
    external_id: str,
    payload: dict[str, Any],
    index: EntityIndex,
) -> tuple[list[dict[str, Any]], int]:
    content = payload.get("raw_content") or payload.get("translated_content") or ""
    title = payload.get("title")
    url = payload.get("url")
    posted_at = payload.get("posted_at")
    source_kind = classify_source_kind(url)
    section: str | None = None
    current_group: str | None = None
    current_group_type: str | None = None
    current_group_confidence = 0.0
    events: list[dict[str, Any]] = []
    skipped = 0

    for line_index, raw_line in iter_patch_lines(content):
        line = raw_line.strip()
        if not line:
            continue

        section_candidate = detect_section(line)
        if section_candidate:
            section = section_candidate
            current_group = None
            current_group_type = None
            current_group_confidence = 0.0
            continue

        bullet_body = bullet_body_from_line(line)
        if bullet_body is None:
            maybe_group = standalone_group_name(line)
            if maybe_group:
                entity_type, canonical, confidence = index.canonical(maybe_group, section=section)
                current_group = canonical or maybe_group
                current_group_type = entity_type
                current_group_confidence = confidence
            else:
                skipped += 1
            continue

        subject, body = split_subject(bullet_body)
        if subject:
            entity_type, entity_name, confidence = index.canonical(subject, section=section)
            if looks_like_group_heading(body):
                current_group = entity_name or clean_subject(subject)
                current_group_type = entity_type
                current_group_confidence = confidence
                body = clean_subject(body)
            else:
                current_group = entity_name if entity_type != "general" else current_group
        elif current_group:
            entity_type = current_group_type or "general"
            entity_name = current_group
            subject = current_group
            body = bullet_body
            confidence = current_group_confidence or 0.7
        else:
            inferred = infer_entities_from_line(bullet_body, index)
            if inferred:
                entity_type, entity_name, confidence = inferred[0]
                subject = entity_name
                body = bullet_body
            else:
                entity_type, entity_name, confidence = "general", None, 0.45
                subject = None
                body = bullet_body

        normalized_line = normalize_patch_line(body)
        if not normalized_line:
            skipped += 1
            continue
        change_type = classify_change_type(normalized_line)
        old_value, new_value = extract_old_new(normalized_line)
        metadata = {
            "source_language": "en" if payload.get("raw_content") else "de",
            "line_subject": subject,
            "original_bullet": bullet_body,
        }
        event_hash = stable_hash_text(
            "|".join(
                [
                    str(snapshot_id),
                    str(line_index),
                    section or "",
                    entity_type,
                    entity_name or "",
                    normalized_line,
                ]
            )
        )
        events.append(
            {
                "patch_snapshot_id": snapshot_id,
                "patch_external_id": external_id,
                "patch_title": title,
                "patch_url": url,
                "source_kind": source_kind,
                "posted_at": posted_at,
                "line_index": line_index,
                "section": section,
                "entity_type": entity_type,
                "entity_name": entity_name,
                "subject": clean_subject(subject) if subject else None,
                "change_type": change_type,
                "raw_line": raw_line,
                "normalized_line": normalized_line,
                "old_value": old_value,
                "new_value": new_value,
                "confidence": confidence,
                "metadata": metadata,
                "event_hash": event_hash,
            }
        )
    return events, skipped


def iter_patch_lines(content: str) -> list[tuple[int, str]]:
    lines: list[tuple[int, str]] = []
    virtual_index = 0
    for raw_index, raw_line in enumerate(content.splitlines(), start=1):
        expanded = expand_inline_bullets(raw_line)
        for part in expanded:
            virtual_index += 1
            # Keep a monotonically increasing line index while preserving the original line in metadata-like form.
            lines.append((virtual_index, part))
    return lines


def expand_inline_bullets(raw_line: str) -> list[str]:
    stripped = raw_line.strip()
    if not stripped:
        return [raw_line]
    if stripped.startswith("- ") and " - " in stripped:
        parts = re.split(r"\s+-\s+(?=[A-Z0-9\"'])", stripped[2:])
        if len(parts) > 1:
            return [f"- {part.strip()}" for part in parts if part.strip()]
    return [raw_line]


def build_entity_index(store: BrainStore) -> EntityIndex:
    normalized = _build_entity_index_from_entities(store)
    if normalized:
        return normalized

    heroes: set[str] = set()
    hero_names: dict[str, str] = {}
    hero_internals: set[str] = set()
    hero_internal_names: dict[str, str] = {}
    items: set[str] = set()
    item_names: dict[str, str] = {}
    special_items: set[str] = set()
    special_item_names: dict[str, str] = {}
    abilities: set[str] = set()
    ability_names: dict[str, str] = {}
    internal_abilities: set[str] = set()
    internal_ability_names: dict[str, str] = {}
    internals: set[str] = set()
    internal_names: dict[str, str] = {}
    rows = store.conn.execute(
        """
        SELECT entity_type, canonical_name, payload_json
        FROM entity_snapshots
        WHERE entity_type IN ('hero', 'item_or_ability', 'hero_stats_sheet')
        """
    ).fetchall()
    for row in rows:
        payload = json.loads(row["payload_json"])
        entity_type = row["entity_type"]
        names = set()
        if row["canonical_name"]:
            names.add(str(row["canonical_name"]))
        if isinstance(payload, dict):
            for key in ("name", "class_name"):
                if payload.get(key):
                    names.add(str(payload[key]))
            values = payload.get("values")
            if isinstance(values, dict) and values.get("Hero Name"):
                names.add(str(values["Hero Name"]))
        for name in names:
            key = normalize_key(name)
            if not key or key.isdigit():
                continue
            if entity_type in {"hero", "hero_stats_sheet"}:
                heroes.add(key)
                hero_names.setdefault(key, clean_subject(name))
            else:
                items.add(key)
                item_names.setdefault(key, clean_subject(name))
    return EntityIndex(
        heroes=heroes,
        hero_names=hero_names,
        hero_internals=hero_internals,
        hero_internal_names=hero_internal_names,
        items=items,
        item_names=item_names,
        special_items=special_items,
        special_item_names=special_item_names,
        abilities=abilities,
        ability_names=ability_names,
        internal_abilities=internal_abilities,
        internal_ability_names=internal_ability_names,
        internals=internals,
        internal_names=internal_names,
    )


def _build_entity_index_from_entities(store: BrainStore) -> EntityIndex | None:
    try:
        rows = store.conn.execute(
            """
            SELECT e.entity_type, e.canonical_name, a.alias
            FROM entities e
            LEFT JOIN entity_aliases a ON a.entity_id=e.id
            WHERE e.entity_type IN (
              'hero', 'hero_internal', 'item', 'item_special',
              'ability', 'ability_internal', 'weapon_or_internal'
            )
            """
        ).fetchall()
    except Exception:
        return None
    if not rows:
        return None

    heroes: set[str] = set()
    hero_names: dict[str, str] = {}
    hero_internals: set[str] = set()
    hero_internal_names: dict[str, str] = {}
    items: set[str] = set()
    item_names: dict[str, str] = {}
    special_items: set[str] = set()
    special_item_names: dict[str, str] = {}
    abilities: set[str] = set()
    ability_names: dict[str, str] = {}
    internal_abilities: set[str] = set()
    internal_ability_names: dict[str, str] = {}
    internals: set[str] = set()
    internal_names: dict[str, str] = {}

    for row in rows:
        canonical = clean_subject(row["canonical_name"])
        names = {canonical}
        if row["alias"]:
            names.add(str(row["alias"]))
        for name in names:
            key = normalize_key(name)
            if not key or key.isdigit():
                continue
            if row["entity_type"] == "hero":
                heroes.add(key)
                hero_names.setdefault(key, canonical)
            elif row["entity_type"] == "hero_internal":
                hero_internals.add(key)
                hero_internal_names.setdefault(key, canonical)
            elif row["entity_type"] == "item":
                items.add(key)
                item_names.setdefault(key, canonical)
            elif row["entity_type"] == "item_special":
                special_items.add(key)
                special_item_names.setdefault(key, canonical)
            elif row["entity_type"] == "ability":
                abilities.add(key)
                ability_names.setdefault(key, canonical)
            elif row["entity_type"] == "ability_internal":
                if key in GENERIC_INTERNAL_KEYS:
                    continue
                internal_abilities.add(key)
                internal_ability_names.setdefault(key, canonical)
            elif row["entity_type"] == "weapon_or_internal":
                if key in GENERIC_INTERNAL_KEYS:
                    continue
                internals.add(key)
                internal_names.setdefault(key, canonical)
    return EntityIndex(
        heroes=heroes,
        hero_names=hero_names,
        hero_internals=hero_internals,
        hero_internal_names=hero_internal_names,
        items=items,
        item_names=item_names,
        special_items=special_items,
        special_item_names=special_item_names,
        abilities=abilities,
        ability_names=ability_names,
        internal_abilities=internal_abilities,
        internal_ability_names=internal_ability_names,
        internals=internals,
        internal_names=internal_names,
    )


def classify_source_kind(url: str | None) -> str:
    lower = (url or "").lower()
    if "steamcommunity.com" in lower or "steampowered.com" in lower or "steamstore-a.akamaihd.net" in lower:
        return "steam"
    if "forums.playdeadlock.com" in lower:
        return "forum"
    return "other"


def detect_section(line: str) -> str | None:
    cleaned = line.strip()
    if cleaned.lower().startswith("deadlock patch notes"):
        return None
    match = SECTION_RE.match(cleaned)
    if not match:
        return None
    label = clean_subject(match.group(1))
    key = label.lower()
    return KNOWN_SECTIONS.get(key)


def bullet_body_from_line(line: str) -> str | None:
    match = BULLET_RE.match(line)
    if match:
        return match.group(1).strip()
    return None


def split_subject(text: str) -> tuple[str | None, str]:
    match = SUBJECT_RE.match(text)
    if not match:
        return None, text.strip()
    subject = clean_subject(match.group("subject"))
    body = match.group("body").strip()
    if subject.lower() in {"t1", "t2", "t3", "t4", "tier 1", "tier 2", "tier 3", "tier 4"}:
        return None, text.strip()
    return subject, body


def standalone_group_name(line: str) -> str | None:
    cleaned = clean_subject(line)
    if not cleaned or len(cleaned) > 70:
        return None
    if cleaned.startswith("#") or cleaned.startswith("-"):
        return None
    if any(word in cleaned.lower() for word in ("increased", "reduced", "fixed", "added", "removed")):
        return None
    return cleaned


def looks_like_group_heading(text: str) -> bool:
    cleaned = text.strip().rstrip(":")
    return bool(cleaned) and len(cleaned) < 50 and not any(
        word in cleaned.lower() for word in ("from", "to", "increased", "reduced", "fixed", "now", "no longer")
    )


def infer_entities_from_line(text: str, index: EntityIndex) -> list[tuple[str, str, float]]:
    found: list[tuple[str, str, float]] = []
    lower = normalize_scan_text(text)
    for key, name in index.hero_names.items():
        if key and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("hero", name, 0.65))
    for key, name in index.hero_internal_names.items():
        if key and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("hero_internal", name, 0.5))
    for key, name in index.item_names.items():
        if key and len(key) >= 4 and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("item", name, 0.6))
    for key, name in index.special_item_names.items():
        if key and len(key) >= 4 and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("item_special", name, 0.55))
    for key, name in index.ability_names.items():
        if key and len(key) >= 4 and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("ability", name, 0.58))
    for key, name in index.internal_ability_names.items():
        if key and len(key) >= 4 and re.search(rf"\b{re.escape(key)}\b", lower):
            found.append(("ability_internal", name, 0.45))
    return found[:5]


def classify_change_type(text: str) -> str:
    lower = text.lower()
    if "fixed" in lower or "bug" in lower or "crash" in lower:
        return "bugfix"
    if "added" in lower or "new item" in lower or "new hero" in lower:
        return "added"
    if "removed" in lower or "no longer" in lower:
        return "removed"
    if "reworked" in lower or "changed from" in lower or "rescaled" in lower or "moved from" in lower:
        return "rework"
    if "reduced" in lower or "decreased" in lower or "slower" in lower:
        return "nerf" if _mentions_positive_stat(lower) else "buff"
    if "increased" in lower or "faster" in lower or "improved" in lower:
        return "buff" if _mentions_positive_stat(lower) or not _mentions_negative_stat(lower) else "nerf"
    if "now" in lower or "upgrades from" in lower:
        return "changed"
    return "changed"


def _mentions_positive_stat(lower: str) -> bool:
    return any(stat in lower for stat in POSITIVE_STATS)


def _mentions_negative_stat(lower: str) -> bool:
    return any(stat in lower for stat in NEGATIVE_STATS)


def extract_old_new(text: str) -> tuple[str | None, str | None]:
    quoted = QUOTED_FROM_TO_RE.search(text)
    if quoted:
        return quoted.group(1).strip(), quoted.group(2).strip()
    match = FROM_TO_RE.search(text)
    if not match:
        return None, None
    old_value = match.group(1).strip()
    new_value = match.group(2).strip()
    return old_value[:160], new_value[:160]


def normalize_patch_line(text: str) -> str:
    return re.sub(r"\s+", " ", clean_subject(text)).strip()


def clean_subject(text: str | None) -> str:
    if not text:
        return ""
    cleaned = text.strip()
    cleaned = re.sub(r"^\*\*|\*\*$", "", cleaned)
    cleaned = cleaned.strip(" :-\t")
    return cleaned


def normalize_key(text: str) -> str:
    cleaned = clean_subject(text).lower()
    cleaned = cleaned.replace("&", " and ")
    cleaned = re.sub(r"[^a-z0-9]+", " ", cleaned)
    return re.sub(r"\s+", " ", cleaned).strip()


def normalize_scan_text(text: str) -> str:
    return f" {normalize_key(text)} "
