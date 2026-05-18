from __future__ import annotations

from collections import Counter, defaultdict
from typing import Any

from deadlock_brain.retrieval import build_entity_context


MAX_CURRENT_STAT_HINTS = 18
MAX_RECENT_EVENTS = 24
MAX_STAT_CHANGES = 24
MAX_SOURCE_REFERENCES = 40

PRIMARY_STAT_KEYS = (
    "base_hp",
    "max_level_hp",
    "hp_gain",
    "base_regen",
    "base_move_speed",
    "base_sprint",
    "base_stamina",
    "base_ammo",
    "base_bullet_dmg",
    "base_fire_rate",
    "base_dps",
    "max_gun_dps",
    "max_gun_damage",
    "dmg_gain",
    "spirit_gain",
    "total_bullet_ratio",
    "total_spirit_ratio",
    "aggregate_growth",
    "hp_growth_increase",
    "dps_growth_increase",
)


def build_review_context(conn, query: str, limit_events: int = 80) -> dict[str, Any]:
    """Build a compact, AI-ready review context bundle without calling AI services.

    The function is intentionally a thin deterministic layer on top of
    ``retrieval.build_entity_context``. It summarizes the matched entity, current
    sheet hints, patch-timeline signals, gaps and source references, plus a
    German prompt draft for a later model call.
    """

    retrieval_context = build_entity_context(conn, query, limit_events=limit_events)
    best_match = retrieval_context.get("best_match") or {}
    events = list(retrieval_context.get("patch_events") or [])
    enrichments = list((retrieval_context.get("enrichments") or {}).get("rows") or [])

    entity_summary = _build_entity_summary(retrieval_context)
    current_stat_hints = _build_current_stat_hints(retrieval_context)
    timeline_signals = _build_timeline_signals(events, enrichments)
    source_references = _build_source_references(retrieval_context, current_stat_hints)
    open_questions = _build_open_questions(
        retrieval_context,
        entity_summary=entity_summary,
        current_stat_hints=current_stat_hints,
        timeline_signals=timeline_signals,
    )

    return {
        "query": retrieval_context.get("query") or query,
        "context_kind": "analysis_review",
        "entity_summary": entity_summary,
        "lineage": _build_lineage_summary(retrieval_context),
        "current_stat_hints": current_stat_hints,
        "timeline_signals": timeline_signals,
        "open_questions": open_questions,
        "source_references": source_references,
        "prompt_de": _build_prompt_de(best_match, query),
        "retrieval_meta": {
            "limit_events_requested": limit_events,
            "events_loaded": len(events),
            "enrichments_loaded": len(enrichments),
            "fallback_used": bool((retrieval_context.get("fallback") or {}).get("used")),
            "sheet_stats_available": bool((retrieval_context.get("sheet_stats") or {}).get("available")),
        },
    }


def _build_lineage_summary(ctx: dict[str, Any]) -> dict[str, Any]:
    rows = list(ctx.get("lineage") or [])
    names: set[str] = set()
    relation_counts: dict[str, int] = {}
    for row in rows:
        relation = str(row.get("relation_type") or "unknown")
        relation_counts[relation] = relation_counts.get(relation, 0) + 1
        for key in ("source_name", "target_name", "owner_name"):
            value = str(row.get(key) or "").strip()
            if value:
                names.add(value)
    return {
        "available": bool(rows),
        "relations": rows[:12],
        "related_names": sorted(names),
        "relation_counts": dict(sorted(relation_counts.items())),
        "omitted_relation_count": max(0, len(rows) - 12),
    }


def _build_entity_summary(ctx: dict[str, Any]) -> dict[str, Any]:
    best_match = ctx.get("best_match") or {}
    aliases = list(ctx.get("aliases") or [])
    metadata = best_match.get("metadata") if isinstance(best_match.get("metadata"), dict) else {}
    human_aliases = []
    for alias in aliases:
        kind = str(alias.get("alias_kind") or "")
        value = str(alias.get("alias") or "").strip()
        if value and kind in {"canonical", "snapshot_name", "class_name_short"} and value not in human_aliases:
            human_aliases.append(value)

    return {
        "matched": bool(best_match),
        "name": best_match.get("canonical_name") or ctx.get("query"),
        "entity_type": best_match.get("entity_type"),
        "source": best_match.get("source"),
        "external_id": best_match.get("primary_external_id"),
        "match_score": best_match.get("score"),
        "matched_alias_kinds": best_match.get("matched_alias_kinds") or [],
        "aliases": human_aliases[:12],
        "metadata_hints": {
            "disabled": metadata.get("disabled"),
            "document_kinds": metadata.get("document_kinds") or [],
        },
    }


def _build_current_stat_hints(ctx: dict[str, Any]) -> dict[str, Any]:
    sheet_stats = ctx.get("sheet_stats") or {}
    sources = list(sheet_stats.get("sources") or [])
    value_rows = _sheet_rows_for_source(sources, "hero_stat_values")
    snapshot_rows = _sheet_rows_for_source(sources, "entity_snapshots")
    profile_rows = _sheet_rows_for_source(sources, "hero_stat_profiles")

    hints = _merged_sheet_stat_hints(value_rows, snapshot_rows)

    return {
        "available": bool(sheet_stats.get("available")),
        "source_count": len(sources),
        "profile": _compact_profile(profile_rows[0]) if profile_rows else None,
        "hints": hints[:MAX_CURRENT_STAT_HINTS],
        "omitted_hint_count": max(0, len(hints) - MAX_CURRENT_STAT_HINTS),
        "source_tables": [str(source.get("source")) for source in sources],
    }


def _build_timeline_signals(events: list[dict[str, Any]], enrichments: list[dict[str, Any]]) -> dict[str, Any]:
    enrichments_by_event_id = _index_enrichments(enrichments)
    change_type_counts = Counter(str(event.get("change_type") or "unknown") for event in events)
    source_counts = Counter(str(event.get("source_kind") or "unknown") for event in events)
    section_counts = Counter(str(event.get("section") or "Unsectioned") for event in events)
    dates = [str(event.get("posted_at")) for event in events if event.get("posted_at")]

    recent_events = []
    stat_changes = []
    ability_mentions: dict[str, int] = defaultdict(int)
    low_confidence_events = 0

    for event in events:
        event_enrichments = enrichments_by_event_id.get(_int_or_none(event.get("id")), [])
        if not event_enrichments:
            event_enrichments = [{}]

        if any(float(enrichment.get("confidence") or 0) < 0.5 for enrichment in event_enrichments):
            low_confidence_events += 1

        recent_events.append(_compact_event(event, event_enrichments))
        for enrichment in event_enrichments:
            ability = enrichment.get("ability_name")
            if ability:
                ability_mentions[str(ability)] += 1
            if _has_structured_stat_change(enrichment):
                stat_changes.append(_compact_stat_change(event, enrichment))

    return {
        "event_count": len(events),
        "date_range": {
            "newest": max(dates) if dates else None,
            "oldest": min(dates) if dates else None,
        },
        "latest_patch": _latest_patch(events),
        "change_type_counts": dict(change_type_counts.most_common()),
        "source_counts": dict(source_counts.most_common()),
        "top_sections": dict(section_counts.most_common(8)),
        "recent_events": recent_events[:MAX_RECENT_EVENTS],
        "stat_changes": stat_changes[:MAX_STAT_CHANGES],
        "omitted_recent_event_count": max(0, len(recent_events) - MAX_RECENT_EVENTS),
        "omitted_stat_change_count": max(0, len(stat_changes) - MAX_STAT_CHANGES),
        "ability_mentions": dict(sorted(ability_mentions.items(), key=lambda item: (-item[1], item[0]))[:12]),
        "low_confidence_event_count": low_confidence_events,
    }


def _build_source_references(ctx: dict[str, Any], current_stat_hints: dict[str, Any]) -> list[dict[str, Any]]:
    references = []
    seen: set[tuple[str, str]] = set()

    best_match = ctx.get("best_match") or {}
    if best_match:
        _append_reference(
            references,
            seen,
            {
                "kind": "entity",
                "source": best_match.get("source"),
                "label": best_match.get("canonical_name"),
                "external_id": best_match.get("primary_external_id"),
            },
        )

    for event in ctx.get("patch_events") or []:
        _append_reference(
            references,
            seen,
            {
                "kind": "patch_event",
                "source": event.get("source_kind"),
                "label": event.get("patch_title"),
                "url": event.get("patch_url"),
                "posted_at": event.get("posted_at"),
                "patch_event_id": event.get("id"),
                "line_index": event.get("line_index"),
            },
        )
        if len(references) >= MAX_SOURCE_REFERENCES:
            break

    profile = current_stat_hints.get("profile") or {}
    if profile:
        _append_reference(
            references,
            seen,
            {
                "kind": "sheet_stats",
                "source": profile.get("source"),
                "label": profile.get("hero_name"),
                "external_id": profile.get("external_id"),
                "snapshot_id": profile.get("snapshot_id"),
                "row_number": profile.get("row_number"),
            },
        )

    return references[:MAX_SOURCE_REFERENCES]


def _build_open_questions(
    ctx: dict[str, Any],
    *,
    entity_summary: dict[str, Any],
    current_stat_hints: dict[str, Any],
    timeline_signals: dict[str, Any],
) -> list[str]:
    questions = []
    if not entity_summary.get("matched"):
        questions.append("Entity konnte nicht kanonisch gematcht werden; Patch-Treffer sind nur Fallback-Suche.")
    if (ctx.get("fallback") or {}).get("used"):
        questions.append("Retrieval nutzt Fallback-Matching; Namen und Aliase vor einer Review-Aussage pruefen.")
    if timeline_signals.get("event_count") == 0:
        questions.append("Keine Patch-Events gefunden; Trend- oder Balance-Aussagen waeren spekulativ.")
    if not (ctx.get("enrichments") or {}).get("available"):
        questions.append("Patch-Event-Enrichments fehlen; Stat-Aenderungen sind nur aus Rohzeilen ableitbar.")
    elif timeline_signals.get("low_confidence_event_count"):
        questions.append("Ein Teil der Patchzeilen ist unstrukturiert oder low-confidence; Rohzeilen gegenlesen.")
    if entity_summary.get("entity_type") == "hero" and not current_stat_hints.get("hints"):
        questions.append("Keine aktuellen Sheet-Stat-Hints fuer den Hero gefunden.")
    if current_stat_hints.get("omitted_hint_count"):
        questions.append("Sheet-Stats wurden gekuerzt; fuer Detailanalyse ggf. Rohkontext nachladen.")
    if timeline_signals.get("omitted_recent_event_count") or timeline_signals.get("omitted_stat_change_count"):
        questions.append("Timeline wurde gekuerzt; fuer historische Vollanalyse hoeheres limit_events nutzen.")
    return questions


def _build_prompt_de(best_match: dict[str, Any], query: str) -> str:
    name = str(best_match.get("canonical_name") or query).strip()
    entity_type = str(best_match.get("entity_type") or "Entity").strip()
    return (
        "Du bist ein Deadlock-Analyseassistent. Nutze ausschliesslich den bereitgestellten "
        "Review-Kontext und kennzeichne Unsicherheiten klar. Behalte Namen von Items, Heroes "
        "und Abilities exakt auf Englisch; erklaere Bewertung, Patch-Interpretation und offene "
        f"Fragen auf Deutsch. Ziel: Erstelle eine kompakte Review fuer {name} ({entity_type}) "
        "mit aktueller Stat-Einordnung, relevanten Timeline-Signalen, moeglichen Balance- oder "
        "Build-Implikationen und Quellenhinweisen. Erfinde keine Zahlen oder Patchdetails, die "
        "nicht im Kontext stehen."
    )


def _sheet_rows_for_source(sources: list[dict[str, Any]], source_name: str) -> list[dict[str, Any]]:
    for source in sources:
        if source.get("source") == source_name:
            return list(source.get("rows") or [])
    return []


def _merged_sheet_stat_hints(value_rows: list[dict[str, Any]], snapshot_rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    snapshot_hints = _stat_hints_from_snapshot_rows(snapshot_rows)
    normalized_hints = _stat_hints_from_normalized_rows(value_rows)
    by_key = {str(hint.get("stat_key") or ""): hint for hint in snapshot_hints if hint.get("stat_key")}
    for hint in normalized_hints:
        key = str(hint.get("stat_key") or "")
        if not key:
            continue
        existing = by_key.get(key)
        if existing:
            existing["numeric_value"] = hint.get("numeric_value")
            existing["value"] = existing.get("value") or hint.get("value")
        else:
            by_key[key] = hint

    ordered_keys = [key for key in PRIMARY_STAT_KEYS if key in by_key]
    ordered_keys.extend(sorted(key for key in by_key if key not in ordered_keys))
    return [by_key[key] for key in ordered_keys]


def _stat_hints_from_normalized_rows(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    by_key = {str(row.get("stat_key") or ""): row for row in rows if row.get("stat_key")}
    ordered_keys = [key for key in PRIMARY_STAT_KEYS if key in by_key]
    ordered_keys.extend(sorted(key for key in by_key if key not in ordered_keys))
    return [_compact_stat_value(by_key[key]) for key in ordered_keys]


def _stat_hints_from_snapshot_rows(rows: list[dict[str, Any]]) -> list[dict[str, Any]]:
    if not rows:
        return []
    values = rows[0].get("values") if isinstance(rows[0].get("values"), dict) else {}
    hints = []
    for label, raw_value in values.items():
        value = str(raw_value).strip()
        stat_key = _stat_key(str(label))
        if not value or not stat_key:
            continue
        hints.append({"stat_key": stat_key, "stat_label": str(label), "value": value, "numeric_value": _float_or_none(value)})
    return hints


def _compact_profile(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "hero_name": row.get("hero_name"),
        "source": row.get("source"),
        "external_id": row.get("external_id"),
        "snapshot_id": row.get("snapshot_id"),
        "row_number": row.get("row_number"),
    }


def _compact_stat_value(row: dict[str, Any]) -> dict[str, Any]:
    return {
        "stat_key": row.get("stat_key"),
        "stat_label": row.get("stat_label"),
        "value": row.get("raw_value"),
        "numeric_value": row.get("numeric_value"),
    }


def _stat_key(label: str) -> str:
    lowered = label.strip().casefold()
    if not lowered or lowered.startswith("column "):
        return ""
    if set(lowered) <= {"|", "l", "i", " "}:
        return ""
    return "".join(character if character.isalnum() else "_" for character in lowered).strip("_")


def _index_enrichments(enrichments: list[dict[str, Any]]) -> dict[int | None, list[dict[str, Any]]]:
    indexed: dict[int | None, list[dict[str, Any]]] = defaultdict(list)
    for enrichment in enrichments:
        indexed[_int_or_none(enrichment.get("patch_event_id") or enrichment.get("event_id"))].append(enrichment)
    return indexed


def _compact_event(event: dict[str, Any], enrichments: list[dict[str, Any]]) -> dict[str, Any]:
    return {
        "patch_event_id": event.get("id"),
        "posted_at": event.get("posted_at"),
        "patch_title": event.get("patch_title"),
        "source_kind": event.get("source_kind"),
        "section": event.get("section"),
        "change_type": event.get("change_type"),
        "line": event.get("normalized_line") or event.get("raw_line"),
        "structured_changes": [_compact_enrichment(enrichment) for enrichment in enrichments if enrichment],
    }


def _compact_enrichment(enrichment: dict[str, Any]) -> dict[str, Any]:
    return {
        "stat_name": enrichment.get("stat_name"),
        "old_value": enrichment.get("old_value"),
        "new_value": enrichment.get("new_value"),
        "unit": enrichment.get("unit"),
        "ability_name": enrichment.get("ability_name"),
        "secondary_entity_name": enrichment.get("secondary_entity_name"),
        "confidence": enrichment.get("confidence"),
    }


def _compact_stat_change(event: dict[str, Any], enrichment: dict[str, Any]) -> dict[str, Any]:
    compact = _compact_enrichment(enrichment)
    compact.update(
        {
            "patch_event_id": event.get("id"),
            "posted_at": event.get("posted_at"),
            "patch_title": event.get("patch_title"),
            "change_type": event.get("change_type"),
            "line": event.get("normalized_line") or event.get("raw_line"),
        }
    )
    return compact


def _has_structured_stat_change(enrichment: dict[str, Any]) -> bool:
    return bool(enrichment.get("stat_name") and (enrichment.get("old_value") is not None or enrichment.get("new_value") is not None))


def _latest_patch(events: list[dict[str, Any]]) -> dict[str, Any] | None:
    if not events:
        return None
    event = events[0]
    return {
        "posted_at": event.get("posted_at"),
        "patch_title": event.get("patch_title"),
        "patch_url": event.get("patch_url"),
        "source_kind": event.get("source_kind"),
    }


def _append_reference(
    references: list[dict[str, Any]],
    seen: set[tuple[str, str]],
    reference: dict[str, Any],
) -> None:
    identity = (str(reference.get("kind") or ""), str(reference.get("url") or reference.get("external_id") or reference.get("patch_event_id") or ""))
    if identity in seen:
        return
    seen.add(identity)
    references.append({key: value for key, value in reference.items() if value is not None})


def _int_or_none(value: Any) -> int | None:
    if value is None:
        return None
    try:
        return int(value)
    except (TypeError, ValueError):
        return None


def _float_or_none(value: Any) -> float | None:
    if value is None:
        return None
    cleaned = str(value).strip().removesuffix("%")
    try:
        return float(cleaned)
    except ValueError:
        return None
