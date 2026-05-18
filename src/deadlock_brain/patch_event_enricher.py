from __future__ import annotations

import json
import re
import time
from dataclasses import dataclass
from typing import Any


FROM_TO_RE = re.compile(
    r"^(?P<subject>.+?)\s+"
    r"(?P<verb>increased|reduced|decreased|lowered|raised|changed|rescaled|adjusted)\s+"
    r"from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
BARE_FROM_TO_RE = re.compile(
    r"^(?P<subject>.+?)\s+from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
BY_AMOUNT_RE = re.compile(
    r"^(?P<subject>.+?)\s+"
    r"(?P<verb>increased|reduced|decreased|lowered|raised|improved)\s+by\s+"
    r"(?P<amount>[+-]?\d+(?:\.\d+)?%?)(?:[;,)]|$)",
    re.IGNORECASE,
)
MOVED_RE = re.compile(r"^Moved\s+from\s+(?P<old>.+?)\s+to\s+(?P<new>.+?)(?:[;,)]|$)", re.IGNORECASE)
UPGRADES_FROM_RE = re.compile(
    r"^Now\s+upgrades\s+from\s+(?P<entity>.+?)(?:\s*\([^)]*\))?(?:[;,)]|$)",
    re.IGNORECASE,
)
NO_LONGER_GRANTS_RE = re.compile(
    r"^No\s+longer\s+grants\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
NO_LONGER_HAS_RE = re.compile(
    r"^No\s+longer\s+(?:has|scales\s+with|targets|procs|affects)\s+(?P<body>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
NOW_GRANTS_RE = re.compile(
    r"^Now\s+(?:grants|provides)\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
NOW_VALUE_STAT_RE = re.compile(
    r"^(?P<subject>.+?)\s+now\s+(?:has|costs|scales\s+with|scales\s+per)\s+(?P<value>[+-]?\d+(?:\.\d+)?%?)\s+(?P<stat>.+?)(?:[;,)]|$)",
    re.IGNORECASE,
)
NOW_TIER_RE = re.compile(r"^Now\s+a\s+(?P<tier>T\d+)\s+item(?:[.;,)]|$)", re.IGNORECASE)
VALUE_RE = re.compile(r"^(?P<number>[+-]?\d+(?:\.\d+)?)(?P<unit>%|s|m|ms|x)?$", re.IGNORECASE)

STAT_SUFFIXES = (
    "spirit scaling",
    "bullet scaling",
    "weapon scaling",
    "melee scaling",
    "boon scaling",
    "base damage",
    "bonus damage",
    "bullet resist",
    "spirit resist",
    "melee resistance",
    "spirit resistance",
    "fire rate",
    "souls per minute",
    "bonus health",
    "max health",
    "health regen",
    "cooldown",
    "duration",
    "damage",
    "barrier",
    "range",
    "radius",
    "speed",
    "slow",
    "ammo",
    "cost",
    "charges",
    "stacks",
    "scaling",
    "health",
)


@dataclass(frozen=True)
class PatchEventEnrichment:
    patch_event_id: int | None
    stat_name: str | None
    old_value: str | None
    new_value: str | None
    unit: str | None
    ability_name: str | None
    secondary_entity_name: str | None
    confidence: float
    flags: tuple[str, ...]

    def as_db_values(self) -> dict[str, Any]:
        return {
            "patch_event_id": self.patch_event_id,
            "stat_name": self.stat_name,
            "old_value": self.old_value,
            "new_value": self.new_value,
            "unit": self.unit,
            "ability_name": self.ability_name,
            "secondary_entity_name": self.secondary_entity_name,
            "confidence": self.confidence,
            "flags_json": json.dumps(list(self.flags), ensure_ascii=True, sort_keys=True),
        }


def build_patch_event_enrichments(conn, rebuild: bool = False) -> dict[str, Any]:
    """Build deterministic enrichments for rows in patch_events using a sqlite connection."""
    ensure_patch_event_enrichments_table(conn)
    before_total = _count_enrichments(conn)
    deleted = 0
    if rebuild:
        cursor = conn.execute("DELETE FROM patch_event_enrichments")
        deleted = int(cursor.rowcount if cursor.rowcount is not None else 0)

    rows = conn.execute(
        """
        SELECT pe.id, pe.normalized_line, pe.entity_name, pe.section, pe.raw_line
        FROM patch_events pe
        LEFT JOIN patch_event_enrichments pee ON pee.patch_event_id = pe.id
        WHERE (? OR pee.patch_event_id IS NULL)
        ORDER BY pe.id ASC
        """,
        (1 if rebuild else 0,),
    ).fetchall()

    processed = 0
    inserted = 0
    matched = 0
    for row in rows:
        event = _row_to_dict(row)
        enrichment = enrich_patch_event(event)
        processed += 1
        if enrichment.confidence > 0:
            matched += 1
        if _insert_enrichment(conn, enrichment):
            inserted += 1

    conn.commit()
    after_total = _count_enrichments(conn)
    return {
        "events_processed": processed,
        "enrichments_inserted": inserted,
        "enrichments_matched": matched,
        "enrichments_before": before_total,
        "enrichments_total": after_total,
        "deleted_before_build": deleted,
        "rebuild": rebuild,
    }


def ensure_patch_event_enrichments_table(conn) -> None:
    conn.execute(
        """
        CREATE TABLE IF NOT EXISTS patch_event_enrichments (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          patch_event_id INTEGER NOT NULL,
          stat_name TEXT,
          old_value TEXT,
          new_value TEXT,
          unit TEXT,
          ability_name TEXT,
          secondary_entity_name TEXT,
          confidence REAL NOT NULL,
          flags_json TEXT NOT NULL DEFAULT '[]',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(patch_event_id),
          FOREIGN KEY(patch_event_id) REFERENCES patch_events(id) ON DELETE CASCADE
        )
        """
    )
    conn.execute(
        """
        CREATE INDEX IF NOT EXISTS idx_patch_event_enrichments_stat
          ON patch_event_enrichments(stat_name)
        """
    )
    conn.execute(
        """
        CREATE INDEX IF NOT EXISTS idx_patch_event_enrichments_secondary_entity
          ON patch_event_enrichments(secondary_entity_name)
        """
    )
    conn.commit()


def enrich_patch_event(event: dict[str, Any]) -> PatchEventEnrichment:
    text = _clean_line(event.get("normalized_line") or event.get("raw_line") or "")
    patch_event_id = _optional_int(event.get("id") or event.get("patch_event_id"))
    flags: list[str] = []

    match = FROM_TO_RE.match(text)
    if match:
        return _from_to_enrichment(
            patch_event_id=patch_event_id,
            subject=match.group("subject"),
            old_raw=match.group("old"),
            new_raw=match.group("new"),
            direction=match.group("verb").lower(),
            base_confidence=0.95,
            flags=flags,
        )

    match = BARE_FROM_TO_RE.match(text)
    if match:
        return _from_to_enrichment(
            patch_event_id=patch_event_id,
            subject=match.group("subject"),
            old_raw=match.group("old"),
            new_raw=match.group("new"),
            direction="changed",
            base_confidence=0.88,
            flags=[*flags, "bare_from_to"],
        )

    match = BY_AMOUNT_RE.match(text)
    if match:
        value, unit, value_flags = _split_value(match.group("amount"))
        flags.extend(value_flags)
        stat_name, ability_name, subject_flags = _split_stat_subject(match.group("subject"))
        flags.extend(subject_flags)
        direction = match.group("verb").lower()
        flags.append(f"direction:{direction}")
        if direction in {"reduced", "decreased", "lowered"} and not value.startswith("-"):
            value = f"-{value}"
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name=stat_name,
            old_value=None,
            new_value=value,
            unit=unit,
            ability_name=ability_name,
            secondary_entity_name=None,
            confidence=_confidence(0.82, flags),
            flags=tuple(sorted(set([*flags, "relative_delta"]))),
        )

    match = MOVED_RE.match(text)
    if match:
        old_value, new_value, unit, value_flags = _split_values(match.group("old"), match.group("new"))
        flags.extend(value_flags)
        flags.append("movement")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="item_tier",
            old_value=old_value,
            new_value=new_value,
            unit=unit,
            ability_name=None,
            secondary_entity_name=None,
            confidence=_confidence(0.9, flags),
            flags=tuple(sorted(set(flags))),
        )

    match = UPGRADES_FROM_RE.match(text)
    if match:
        secondary = _clean_entity(match.group("entity"))
        flags.append("upgrade_source")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="upgrades_from",
            old_value=None,
            new_value=secondary,
            unit=None,
            ability_name=None,
            secondary_entity_name=secondary,
            confidence=0.92,
            flags=tuple(sorted(set(flags))),
        )

    match = NO_LONGER_GRANTS_RE.match(text)
    if match:
        stat_name = _clean_stat(match.group("stat"))
        old_value, _, unit, value_flags = _split_values(match.group("value"), "0")
        flags.extend(value_flags)
        flags.append("removed_grant")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name=stat_name,
            old_value=old_value,
            new_value="0",
            unit=unit,
            ability_name=None,
            secondary_entity_name=None,
            confidence=_confidence(0.88, flags),
            flags=tuple(sorted(set(flags))),
        )

    match = NOW_GRANTS_RE.match(text)
    if match:
        _, new_value, unit, value_flags = _split_values("0", match.group("value"))
        flags.extend(value_flags)
        flags.append("added_grant")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name=_clean_stat(match.group("stat")),
            old_value=None,
            new_value=new_value,
            unit=unit,
            ability_name=None,
            secondary_entity_name=None,
            confidence=_confidence(0.82, flags),
            flags=tuple(sorted(set(flags))),
        )

    match = NOW_VALUE_STAT_RE.match(text)
    if match:
        new_value, unit, value_flags = _split_value(match.group("value"))
        flags.extend(value_flags)
        stat_name, ability_name, subject_flags = _split_stat_subject(f"{match.group('subject')} {match.group('stat')}")
        flags.extend(subject_flags)
        flags.append("now_value")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name=stat_name,
            old_value=None,
            new_value=new_value,
            unit=unit,
            ability_name=ability_name,
            secondary_entity_name=None,
            confidence=_confidence(0.74, flags),
            flags=tuple(sorted(set(flags))),
        )

    match = NOW_TIER_RE.match(text)
    if match:
        flags.append("tier_change")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="item_tier",
            old_value=None,
            new_value=match.group("tier"),
            unit=None,
            ability_name=None,
            secondary_entity_name=None,
            confidence=0.78,
            flags=tuple(sorted(set(flags))),
        )

    match = NO_LONGER_HAS_RE.match(text)
    if match:
        flags.append("removed_property")
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name=_clean_stat(match.group("body")),
            old_value="present",
            new_value="removed",
            unit=None,
            ability_name=None,
            secondary_entity_name=None,
            confidence=0.68,
            flags=tuple(sorted(set(flags))),
        )

    generic = _generic_event_enrichment(patch_event_id, text)
    if generic:
        return generic

    flags.append("unparsed")
    return PatchEventEnrichment(
        patch_event_id=patch_event_id,
        stat_name=None,
        old_value=None,
        new_value=None,
        unit=None,
        ability_name=None,
        secondary_entity_name=None,
        confidence=0.0,
        flags=tuple(flags),
    )


def _from_to_enrichment(
    *,
    patch_event_id: int | None,
    subject: str,
    old_raw: str,
    new_raw: str,
    direction: str,
    base_confidence: float,
    flags: list[str],
) -> PatchEventEnrichment:
    old_value, new_value, unit, value_flags = _split_values(old_raw, new_raw)
    flags.extend(value_flags)
    stat_name, ability_name, subject_flags = _split_stat_subject(subject)
    flags.extend(subject_flags)
    flags.append(f"direction:{direction}")
    return PatchEventEnrichment(
        patch_event_id=patch_event_id,
        stat_name=stat_name,
        old_value=old_value,
        new_value=new_value,
        unit=unit,
        ability_name=ability_name,
        secondary_entity_name=None,
        confidence=_confidence(base_confidence, flags),
        flags=tuple(sorted(set(flags))),
    )


def _generic_event_enrichment(patch_event_id: int | None, text: str) -> PatchEventEnrichment | None:
    lower = text.casefold()
    if not lower:
        return None
    if lower.startswith("fixed ") or " bug " in f" {lower} " or "crash" in lower:
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="bugfix",
            old_value=None,
            new_value="fixed",
            unit=None,
            ability_name=None,
            secondary_entity_name=None,
            confidence=0.62,
            flags=("generic_bugfix",),
        )
    if lower.startswith("added "):
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="added_content",
            old_value=None,
            new_value="added",
            unit=None,
            ability_name=None,
            secondary_entity_name=None,
            confidence=0.6,
            flags=("generic_added",),
        )
    if " now " in f" {lower} " or lower.startswith("now ") or " swapped" in lower or "adjusted " in lower:
        return PatchEventEnrichment(
            patch_event_id=patch_event_id,
            stat_name="functional_change",
            old_value=None,
            new_value="changed",
            unit=None,
            ability_name=None,
            secondary_entity_name=None,
            confidence=0.58,
            flags=("generic_functional",),
        )
    return None


def _insert_enrichment(conn, enrichment: PatchEventEnrichment) -> bool:
    values = enrichment.as_db_values()
    now = int(time.time())
    cursor = conn.execute(
        """
        INSERT OR IGNORE INTO patch_event_enrichments(
          patch_event_id, stat_name, old_value, new_value, unit, ability_name,
          secondary_entity_name, confidence, flags_json, created_at, updated_at
        )
        VALUES(:patch_event_id, :stat_name, :old_value, :new_value, :unit, :ability_name,
               :secondary_entity_name, :confidence, :flags_json, :created_at, :updated_at)
        """,
        {**values, "created_at": now, "updated_at": now},
    )
    return cursor.rowcount > 0


def _split_stat_subject(subject: str) -> tuple[str, str | None, list[str]]:
    cleaned = _clean_stat(subject)
    lower = cleaned.lower()
    for suffix in STAT_SUFFIXES:
        if lower == suffix:
            return cleaned, None, []
        if lower.endswith(f" {suffix}"):
            ability = cleaned[: -len(suffix)].strip()
            if ability:
                return _title_like_suffix(cleaned[-len(suffix) :]), ability, ["ability_prefix"]
    return cleaned, None, ["unknown_stat_shape"]


def _split_values(old_raw: str, new_raw: str) -> tuple[str, str, str | None, list[str]]:
    old_value, old_unit, old_flags = _split_value(old_raw)
    new_value, new_unit, new_flags = _split_value(new_raw)
    flags = old_flags + new_flags
    unit = old_unit if old_unit == new_unit else None
    if old_unit and new_unit and old_unit != new_unit:
        flags.append("mixed_units")
    elif old_unit or new_unit:
        unit = old_unit or new_unit
    return old_value, new_value, unit, flags


def _split_value(raw: str) -> tuple[str, str | None, list[str]]:
    cleaned = _clean_value(raw)
    match = VALUE_RE.match(cleaned)
    if not match:
        return cleaned, None, ["non_numeric_value"]
    return match.group("number"), match.group("unit"), []


def _confidence(base: float, flags: list[str]) -> float:
    penalty = 0.0
    if "unknown_stat_shape" in flags:
        penalty += 0.12
    if "non_numeric_value" in flags:
        penalty += 0.08
    if "mixed_units" in flags:
        penalty += 0.05
    return max(0.0, round(base - penalty, 3))


def _clean_line(text: str) -> str:
    return re.sub(r"\s+", " ", str(text)).strip()


def _clean_stat(text: str) -> str:
    cleaned = _clean_line(text).strip(" :-")
    return cleaned[:1].upper() + cleaned[1:] if cleaned else cleaned


def _clean_value(text: str) -> str:
    cleaned = _clean_line(text).strip(" .;,)")
    return cleaned[:160]


def _clean_entity(text: str) -> str:
    return _clean_value(text).strip("\"'")


def _title_like_suffix(text: str) -> str:
    if not text:
        return text
    return " ".join(part[:1].upper() + part[1:] for part in text.split())


def _row_to_dict(row: Any) -> dict[str, Any]:
    if hasattr(row, "keys"):
        return {key: row[key] for key in row.keys()}
    keys = ("id", "normalized_line", "entity_name", "section", "raw_line")
    return dict(zip(keys, row, strict=False))


def _optional_int(value: Any) -> int | None:
    if value is None:
        return None
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def _count_enrichments(conn) -> int:
    row = conn.execute("SELECT COUNT(*) AS count FROM patch_event_enrichments").fetchone()
    if hasattr(row, "keys"):
        return int(row["count"])
    return int(row[0])
