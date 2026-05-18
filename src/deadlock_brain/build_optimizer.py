from __future__ import annotations

import html
import json
import re
import sqlite3
from collections import defaultdict
from typing import Any

from deadlock_brain.retrieval import build_entity_context
from deadlock_brain.review_context import build_review_context


PUBLIC_ITEM_SLOTS = {"weapon", "vitality", "spirit"}
SHOP_BONUS_SPIKE = 4800
SHOP_BONUS_CAP = 28800

WIKI_ECONOMY_RULES = {
    "souls_are_currency_and_xp": True,
    "item_tiers": [800, 1600, 3200, 6400],
    "shop_bonus_categories": {
        "weapon": "Weapon Damage",
        "vitality": "Health",
        "spirit": "Spirit Power",
    },
    "significant_investment_bonus": SHOP_BONUS_SPIKE,
    "shop_bonus_cap": SHOP_BONUS_CAP,
    "component_discount": True,
    "active_item_limit": 4,
    "ability_unlock_souls": [600, 1200, 2100, 3600],
    "ability_upgrade_costs": [1, 2, 5],
    "mechanic_notes": {
        "silence": "verhindert Abilities und Active Items",
        "stun": "verhindert Bewegung, Schiessen und Actions",
        "immobilize": "rootet Bewegung, verhindert aber nicht automatisch Abilities oder Waffen",
        "resistance": "Bullet/Spirit/Melee Resist reduzieren Schaden; mehrere Quellen stacken diminishing/multiplikativ",
        "resistance_reduction": "Resist-Reduction wird separat verrechnet und kann effektive Resistenz negativ machen",
    },
}

BUCKET_SCORE = {
    "Best": 45.0,
    "Good": 28.0,
    "Normal": 0.0,
    "Bad": -35.0,
    "Avoid": -50.0,
}

SLOT_LABELS = {
    "weapon": "Gun",
    "vitality": "HP",
    "spirit": "Spirit",
}

HIGH_VALUE_PROPERTY_HINTS = {
    "cooldown": ("Cooldown economy", 12.0),
    "duration": ("Duration/value uptime", 9.0),
    "range": ("Engage/ability reach", 8.0),
    "radius": ("AoE reliability", 7.0),
    "techpower": ("Spirit Power", 10.0),
    "spirit power": ("Spirit Power", 10.0),
    "tech power": ("Spirit Power", 10.0),
    "health": ("Frontline durability", 9.0),
    "resist": ("Frontline durability", 8.0),
    "armor": ("Frontline durability", 8.0),
    "move speed": ("Engage mobility", 11.0),
    "sprint": ("Map/engage mobility", 9.0),
    "stamina": ("Chase/escape economy", 8.0),
    "lifesteal": ("Fight sustain", 7.0),
    "healing": ("Fight sustain", 6.0),
    "weapon damage": ("Weapon scaling", 5.0),
    "fire rate": ("Weapon scaling", 5.0),
    "bullet": ("Weapon pressure", 4.0),
}

ENGAGE_ACTIVE_HINTS = ("teleport", "dash", "silence", "stun", "slow", "disarm", "root", "knock", "hex")
PURE_DEFENSE_HINTS = ("bonus health", "resist", "armor", "regen")

SITUATIONAL_ITEMS = (
    "Healbane",
    "Healing Booster",
    "Debuff Reducer",
    "Dispel Magic",
    "Reactive Barrier",
    "Metal Skin",
    "Return Fire",
    "Knockdown",
    "Slowing Hex",
    "Silence Wave",
    "Curse",
    "Cursed Relic",
    "Indomitable",
    "Unstoppable",
    "Spellbreaker",
    "Counterspell",
)

LATE_ITEMS = (
    "Boundless Spirit",
    "Phantom Strike",
    "Magic Carpet",
    "Arctic Blast",
    "Colossus",
    "Leech",
    "Diviner's Kevlar",
)

QUESTIONABLE_LUXURY_ITEMS = {
    "Refresher": "teurer Cooldown-Reset; nur kaufen, wenn der Hero konkret von einem zweiten vollen Ability-Zyklus gewinnt",
}


def build_hero_build_context(conn: sqlite3.Connection, query: str, *, vs_heroes: list[str] | None = None, limit_events: int = 80) -> dict[str, Any]:
    review_context = build_review_context(conn, query, limit_events=limit_events)
    entity = review_context.get("entity_summary") or {}
    if entity.get("entity_type") != "hero":
        raise ValueError(f"Builds sind aktuell nur fuer Heroes gedacht. Treffer: {entity.get('entity_type') or 'none'}")

    hero_payload = _load_entity_payload(conn, str(entity.get("name") or query), entity_type="hero")
    if not hero_payload:
        raise ValueError(f"Kein Hero-Payload fuer {query} gefunden.")

    abilities = _load_hero_abilities(conn, hero_payload)
    hero_needs = _infer_hero_needs(hero_payload, abilities)
    wiki_summary = _load_wiki_summary(conn, str(entity.get("name") or query))
    statlocker_signals = _load_statlocker_wpa_signals(conn, str(entity.get("name") or query))
    items = _load_public_items(conn)
    scored_items = [_score_item(hero_payload, item, review_context, hero_needs, statlocker_signals, vs_heroes) for item in items]
    scored_items.sort(key=lambda item: item["score"], reverse=True)

    build = _build_recommendation(hero_payload, scored_items, hero_needs, vs_heroes)
    return {
        "query": query,
        "vs_heroes": vs_heroes or [],
        "hero": _hero_summary(hero_payload, review_context, wiki_summary, abilities, hero_needs),
        "economy": _economy_summary(hero_payload),
        "build": build,
        "top_items": scored_items[:40],
        "rejected_expensive_items": _rejected_expensive_items(scored_items),
        "statlocker_signals": _compact_statlocker_signals(statlocker_signals),
        "review_signals": {
            "change_type_counts": (review_context.get("timeline_signals") or {}).get("change_type_counts"),
            "ability_mentions": (review_context.get("timeline_signals") or {}).get("ability_mentions"),
            "current_stat_hints": (review_context.get("current_stat_hints") or {}).get("hints"),
            "open_questions": review_context.get("open_questions"),
        },
    }


def build_item_context(conn: sqlite3.Connection, query: str) -> dict[str, Any]:
    payload = _load_entity_payload(conn, query, entity_type="item") or _load_entity_payload(conn, query, entity_type="item_special")
    if not payload:
        raise ValueError(f"Kein Item-Payload fuer {query} gefunden.")
    return _item_summary(payload)


def summarize_item_payload(payload: dict[str, Any]) -> dict[str, Any]:
    return _item_summary(payload)


def _build_recommendation(
    hero_payload: dict[str, Any],
    scored_items: list[dict[str, Any]],
    hero_needs: dict[str, Any],
    vs_heroes: list[str] | None = None,
) -> dict[str, Any]:
    needs = set(hero_needs.get("needs") or [])
    early_pool = [
        item for item in scored_items
        if item["item"]["cost"] <= 1600
        and not item["tags"].get("situational")
        and not item["tags"].get("late_only")
        and (not item["tags"].get("pure_defense") or "frontline_survival" in needs)
    ]
    early_pool.sort(key=lambda item: (item["item"]["cost"], 0 if item["hero_bucket"] == "Good" else 1, -item["score"]))
    core_pool = [
        item for item in scored_items
        if 1600 <= item["item"]["cost"] <= SHOP_BONUS_SPIKE
        and not item["tags"].get("situational")
        and not item["tags"].get("questionable_core")
        and (not item["tags"].get("pure_defense") or "frontline_survival" in needs)
    ]
    late_pool = [
        item for item in scored_items
        if item["item"]["cost"] >= 6400
        and item["score"] >= 42
        and not item["tags"].get("questionable_core")
    ]
    situational_pool = [
        item for item in scored_items
        if item["tags"].get("situational")
        or item["tags"].get("counter_item")
        or item["tags"].get("pure_defense")
    ]

    early = _diversified_pick(early_pool, max_items=7, max_per_slot=3)
    core = _diversified_pick(core_pool, max_items=8, max_per_slot=4, exclude_names={i["item"]["name"] for i in early})
    late = _diversified_pick(late_pool, max_items=5, max_per_slot=3, exclude_names={i["item"]["name"] for i in early + core})
    situational = _diversified_pick(
        situational_pool,
        max_items=10,
        max_per_slot=5,
        exclude_names={i["item"]["name"] for i in early + core + late},
    )

    return {
        "plan": _build_plan(hero_needs),
        "lane_decision_model": _lane_decision_model(),
        "early": _compact_recommendations(early),
        "core": _compact_recommendations(core),
        "late": _compact_recommendations(late),
        "situational": _compact_recommendations(situational),
        "shop_routes_to_4800": _shop_routes_to_4800(scored_items),
    }


def _score_item(
    hero_payload: dict[str, Any],
    item_payload: dict[str, Any],
    review_context: dict[str, Any],
    hero_needs: dict[str, Any],
    statlocker_signals: dict[str, dict[str, Any]] | None = None,
    vs_heroes: list[str] | None = None,
) -> dict[str, Any]:
    item = _item_summary(item_payload)
    archetypes = set(item.get("archetypes") or [])
    bucket = _hero_item_bucket(hero_payload, item_payload)
    reasons: list[str] = []
    warnings: list[str] = []
    tags: dict[str, bool] = {}
    score = BUCKET_SCORE.get(bucket, 0.0)
    if bucket != "Normal":
        reasons.append(f"Hero bucket: {bucket}")

    phase_hint = _phase_hint(str(item["name"]))
    if phase_hint:
        tags[phase_hint] = True
        if phase_hint == "situational":
            warnings.append("situational statt blindem Core-Kauf")

    cost = int(item["cost"] or 0)
    tier = int(item["tier"] or 0)
    slot = str(item["slot"] or "")
    if cost <= 1600:
        score += 9
        tags["early_economy"] = True
        reasons.append("hilft frueh beim Shop-Bonus-Aufbau")
        if "lane_farm" in archetypes:
            score += 10
            reasons.append("Lane/Farm-Item: verbessert Souls oder NPC/Orb-Druck")
        if "lane_trade" in archetypes:
            score += 6
            reasons.append("Lane-Trade-Item: verbessert fruehe Trades oder Druck")
    elif cost <= SHOP_BONUS_SPIKE:
        score += 5
        tags["core_window"] = True
        reasons.append("liegt vor/bei dem wichtigen 4800-Shop-Bonus-Fenster")
    elif cost >= 6400:
        score -= 10
        tags["late_only"] = True
        warnings.append("teuer; nur kaufen, wenn der Effekt wirklich der Gameplan ist")

    needs = set(hero_needs.get("needs") or [])
    
    # Adaptive Counter-Scoring against threats
    if vs_heroes and item["name"] in SITUATIONAL_ITEMS:
        # Define some basic counter heuristics
        # Ideally this would come from the database/vector store, but we start with simple hardcoded rules for demonstration
        if "Infernus" in vs_heroes and item["name"] == "Debuff Reducer":
            score += 30
            tags["counter_item"] = True
            reasons.append("ESSENTIELLER COUNTER: Reduziert Infernus' DoT stark.")
        if "Abrams" in vs_heroes and item["name"] in ("Healbane", "Toxic Bullets"):
            score += 30
            tags["counter_item"] = True
            reasons.append("ESSENTIELLER COUNTER: Stoppt Abrams' Heal.")
        if "Kelvin" in vs_heroes and item["name"] == "Unstoppable":
            score += 20
            tags["counter_item"] = True
            reasons.append("Guter Counter: Verhindert Kelvins Slows/Roots.")

    if slot == "vitality":
        score += 6
        reasons.append("Vitality-Slot gibt Health/Defense/Movement-Shop-Bonus")
        if "frontline_survival" in needs:
            score += 4
            reasons.append("passt zu Frontline-/Channel-Risiko")
    elif slot == "spirit":
        score += 5
        reasons.append("Spirit-Slot gibt Ability-/Spirit-Power-Shop-Bonus")
    elif slot == "weapon":
        score += 2
        reasons.append("Weapon-Slot gibt Gun-Damage-Shop-Bonus")
        if "weapon_damage" in needs:
            score += 8
            reasons.append("passt zu erkanntem Weapon-/Gun-Fenster")

    property_score, property_reasons, property_tags = _score_properties(item, hero_needs)
    score += property_score
    reasons.extend(property_reasons[:4])
    tags.update(property_tags)

    archetype_score, archetype_reasons, archetype_tags, archetype_warnings = _score_archetypes(item, hero_needs)
    score += archetype_score
    reasons.extend(archetype_reasons)
    warnings.extend(archetype_warnings)
    tags.update(archetype_tags)

    description = str(item.get("description") or "").casefold()
    if item["is_active"] and any(hint in description for hint in ENGAGE_ACTIVE_HINTS):
        score += 14
        tags["counter_item"] = True
        reasons.append("Active gibt Engage/CC/Counterplay statt nur Stats")
    if item["is_active"] and "reset the cooldown" in description:
        score -= 34
        tags["questionable_core"] = True
        warnings.append("Cooldown-Reset ist teuer und braucht einen klaren Hero-spezifischen Gameplan")

    if item["name"] in QUESTIONABLE_LUXURY_ITEMS:
        score -= 24
        tags["questionable_core"] = True
        warnings.append(QUESTIONABLE_LUXURY_ITEMS[str(item["name"])])

    if _is_pure_defense(item):
        score -= 7
        tags["pure_defense"] = True
        warnings.append("viel Sustain/Defense, aber wenig direkter Impact")

    if tier >= 4 and bucket == "Normal":
        score -= 12
        tags["questionable_core"] = True
        warnings.append("T4 ohne Hero-Good-Bucket braucht klare Begruendung")

    patch_bonus, patch_reasons = _patch_synergy_bonus(item, review_context)
    score += patch_bonus
    reasons.extend(patch_reasons)

    need_bonus, need_reasons = _hero_need_bonus(item, hero_needs)
    score += need_bonus
    reasons.extend(need_reasons)

    statlocker_bonus, statlocker_reasons, statlocker_tags, statlocker_warnings = _statlocker_wpa_bonus(
        item,
        statlocker_signals or {},
    )
    score += statlocker_bonus
    reasons.extend(statlocker_reasons)
    tags.update(statlocker_tags)
    warnings.extend(statlocker_warnings)

    return {
        "item": item,
        "hero_bucket": bucket,
        "score": round(score, 2),
        "reasons": _dedupe_keep_order(statlocker_reasons + reasons)[:7],
        "warnings": _dedupe_keep_order(warnings)[:4],
        "tags": tags,
    }


def _score_properties(item: dict[str, Any], hero_needs: dict[str, Any]) -> tuple[float, list[str], dict[str, bool]]:
    score = 0.0
    reasons: list[str] = []
    tags: dict[str, bool] = {}
    for prop in item.get("properties") or []:
        label_blob = " ".join(
            str(prop.get(key) or "")
            for key in ("name", "label", "provided_property_type")
        ).casefold()
        hint = _property_hint_score(label_blob, hero_needs)
        if hint:
            reason, value = hint
            score += value
            reasons.append(reason)
        provided = str(prop.get("provided_property_type") or "")
        scales = set(str(value) for value in (prop.get("scales_with") or []))
        matched_stats = set(hero_needs.get("priority_scaling_stats") or set()) & ({provided} | scales)
        if matched_stats:
            score += 7
            reasons.append("matched konkrete Ability-Scaling-Stats")
    if any("mobility" in reason.casefold() for reason in reasons):
        tags["mobility"] = True
    if any("cooldown" in reason.casefold() for reason in reasons):
        tags["cooldown"] = True
    return min(score, 36.0), _dedupe_keep_order(reasons), tags


def _property_hint_score(label_blob: str, hero_needs: dict[str, Any]) -> tuple[str, float] | None:
    needs = set(hero_needs.get("needs") or [])
    if "cooldown" in label_blob:
        return ("Cooldown economy", 12.0 if "cooldown_reliability" in needs else 4.0)
    if "duration" in label_blob:
        return ("Duration/value uptime", 9.0 if "ability_uptime" in needs else 3.0)
    if "range" in label_blob:
        return ("Engage/ability reach", 8.0 if "engage_reach" in needs else 3.0)
    if "radius" in label_blob:
        return ("AoE reliability", 7.0 if "engage_reach" in needs or "ability_uptime" in needs else 3.0)
    if "techpower" in label_blob or "spirit power" in label_blob or "tech power" in label_blob:
        return ("Spirit Power", 10.0 if "spirit_damage" in needs else 1.0)
    if "health" in label_blob:
        return ("Frontline durability", 9.0 if "frontline_survival" in needs else 3.0)
    if "resist" in label_blob or "armor" in label_blob:
        return ("Frontline durability", 8.0 if "frontline_survival" in needs else 3.0)
    if "move speed" in label_blob:
        return ("Engage mobility", 11.0 if "engage_reach" in needs else 7.0)
    if "sprint" in label_blob:
        return ("Map/engage mobility", 9.0 if "engage_reach" in needs else 5.0)
    if "stamina" in label_blob:
        return ("Chase/escape economy", 8.0 if "engage_reach" in needs or "weapon_damage" in needs else 4.0)
    if "lifesteal" in label_blob:
        return ("Fight sustain", 7.0 if "frontline_survival" in needs or "weapon_damage" in needs else 2.0)
    if "healing" in label_blob:
        return ("Fight sustain", 6.0 if "frontline_survival" in needs or "team_support" in needs else 2.0)
    if "weapon damage" in label_blob:
        return ("Weapon scaling", 8.0 if "weapon_damage" in needs else 2.0)
    if "fire rate" in label_blob:
        return ("Weapon scaling", 8.0 if "weapon_damage" in needs else 2.0)
    if "bullet" in label_blob and "resist" not in label_blob and "armor" not in label_blob:
        return ("Weapon pressure", 6.0 if "weapon_damage" in needs else 2.0)
    return None


def _hero_need_bonus(item: dict[str, Any], hero_needs: dict[str, Any]) -> tuple[float, list[str]]:
    reasons = []
    score = 0.0
    property_blob = " ".join(
        str(prop.get("label") or prop.get("name") or prop.get("provided_property_type") or "")
        for prop in item.get("properties") or []
    ).casefold()
    description = str(item.get("description") or "").casefold()
    needs = set(hero_needs.get("needs") or [])

    if "engage_reach" in needs and any(word in property_blob or word in description for word in ("range", "move speed", "teleport", "dash")):
        score += 9
        reasons.append("loest ein Hero-Reichweiten-/Engage-Problem")
    if "cooldown_reliability" in needs and "cooldown" in property_blob:
        score += 8
        reasons.append("senkt lange zentrale Cooldowns")
    if "ability_uptime" in needs and "duration" in property_blob:
        score += 6
        reasons.append("verlaengert wichtige CC-/Channel-Fenster")
    if "spirit_damage" in needs and any(word in property_blob for word in ("spirit power", "techpower", "tech power")):
        score += 7
        reasons.append("skaliert Ability-Spirit-Damage")
    if "weapon_damage" in needs and any(word in property_blob for word in ("weapon damage", "fire rate", "bullet", "ammo", "reload")):
        score += 8
        reasons.append("skaliert erkannte Weapon-/Gun-Fenster")
    if "frontline_survival" in needs and any(word in property_blob for word in ("health", "resist", "armor")):
        score += 5
        reasons.append("hilft beim Channel-/Frontline-Risiko")
    return score, reasons


def _score_archetypes(
    item: dict[str, Any],
    hero_needs: dict[str, Any],
) -> tuple[float, list[str], dict[str, bool], list[str]]:
    archetypes = set(item.get("archetypes") or [])
    needs = set(hero_needs.get("needs") or [])
    score = 0.0
    reasons: list[str] = []
    tags: dict[str, bool] = {}
    warnings: list[str] = []

    if "kill_setup" in archetypes and {"engage_reach", "cooldown_reliability", "ability_uptime"} & needs:
        score += 12
        reasons.append("Kill-Setup: hilft zentrale CC-/Pick-Fenster zu erzwingen")
    if "waveclear" in archetypes:
        score += 7
        reasons.append("Waveclear: stabilisiert Lane/Farm und Map-Tempo")
    if "orb_secure" in archetypes:
        score += 7
        reasons.append("Orb-Secure: hilft Souls in der Lane wirklich zu sichern")
    if "duel" in archetypes and {"weapon_damage", "frontline_survival"} & needs:
        score += 8
        reasons.append("Duel-Item: verbessert direkte 1v1-/Trade-Fenster")
    if "burst" in archetypes and {"spirit_damage", "weapon_damage"} & needs:
        score += 7
        reasons.append("Burst: verstaerkt kurze Kill-Fenster")
    if "sustained_dps" in archetypes and "weapon_damage" in needs:
        score += 8
        reasons.append("Sustained DPS: passt zu wiederholten Gun-Damage-Fenstern")
    if "escape" in archetypes:
        score += 5
        reasons.append("Escape/Reposition: senkt Risiko nach Engage oder Trade")
    if "teamfight_engage" in archetypes and {"engage_reach", "ability_uptime"} & needs:
        score += 9
        reasons.append("Teamfight-Engage: macht zentrale Fight-Eröffnung verlaesslicher")
    if "objective_damage" in archetypes:
        score += 4
        reasons.append("Objective-Druck: hilft Map-/Guardian-/Boss-Timings")
    if "core_scaling" in archetypes:
        score += 8
        reasons.append("Core-Scaling: verbessert wiederholbare Hero-Wincondition")
    if "counter" in archetypes:
        warnings.append("Counter-/Defense-Anteil pruefen; als Core nur kaufen, wenn der Haupt-Effekt zum Hero-Plan passt")
    if "support" in archetypes:
        if "team_support" in needs:
            score += 8
            reasons.append("Support-Value passt zur erkannten Team-/Ally-Rolle")
        else:
            tags["situational"] = True
            reasons.append("Team-/Support-Value statt reinem Solo-Scaling")
    if "anti_heal" in archetypes or "anti_carry" in archetypes:
        tags["situational"] = True
        warnings.append("Anti-Heal/Anti-Carry: nach Gegnerdraft und Problemziel kaufen")
    if "save" in archetypes and "team_support" not in needs:
        tags["situational"] = True
        warnings.append("Save-Item: stark, aber nur mit klarer Team-/Peel-Aufgabe")
    if "luxury" in archetypes:
        tags["late_only"] = True
        warnings.append("Luxury/T4: erst nach Core-Plan und Shop-Bonus-Route rechtfertigen")
    if "active_burden" in archetypes:
        warnings.append("Active-Slot beachten; maximal vier Active Items")
    return min(score, 34.0), reasons, tags, warnings


def _patch_synergy_bonus(item: dict[str, Any], review_context: dict[str, Any]) -> tuple[float, list[str]]:
    timeline = review_context.get("timeline_signals") or {}
    ability_mentions = timeline.get("ability_mentions") or {}
    top_abilities = {str(name).casefold() for name, count in ability_mentions.items() if int(count or 0) >= 2}
    reasons = []
    score = 0.0
    property_blob = " ".join(str(prop.get("label") or prop.get("name") or "") for prop in item.get("properties") or []).casefold()
    if top_abilities and ("cooldown" in property_blob or "duration" in property_blob or "range" in property_blob or "radius" in property_blob):
        score += 6
        reasons.append("passt zu oft gepatchten/zentralen Ability-Signalen")
    return score, reasons


def _statlocker_wpa_bonus(
    item: dict[str, Any],
    statlocker_signals: dict[str, dict[str, Any]],
) -> tuple[float, list[str], dict[str, bool], list[str]]:
    signal = statlocker_signals.get(_statlocker_key(str(item.get("name") or "")))
    if not signal:
        return 0.0, [], {}, []
    sample_size = _int_or_zero(signal.get("sampleSize") or signal.get("sample_size"))
    if sample_size <= 0:
        return 0.0, [], {}, []
    wpa = _numeric_value(signal.get("wpaValue"))
    cost_relative = _numeric_value(signal.get("costRelativeWpa") if signal.get("costRelativeWpa") is not None else wpa)
    value = cost_relative if cost_relative else wpa
    score = 0.0
    reasons: list[str] = []
    warnings: list[str] = []
    tags: dict[str, bool] = {"statlocker_wpa": True}

    if value >= 0.03:
        score += 12
        tags["statlocker_positive"] = True
    elif value >= 0.015:
        score += 8
        tags["statlocker_positive"] = True
    elif value >= 0.005:
        score += 4
        tags["statlocker_positive"] = True
    elif value <= -0.015:
        score -= 8
        tags["statlocker_negative"] = True
    elif value <= -0.005:
        score -= 4
        tags["statlocker_negative"] = True

    if score:
        reasons.append(f"Statlocker WPA-Signal {value:+.3f} bei n={sample_size} (korrelativ, nicht causal)")
    if sample_size < 100:
        warnings.append("Statlocker-Sample fuer dieses Hero/Item ist klein")
    return score, reasons, tags, warnings


def _slot_targets(hero_payload: dict[str, Any], picked: list[dict[str, Any]]) -> dict[str, Any]:
    spend = {slot: 0 for slot in PUBLIC_ITEM_SLOTS}
    for item in picked:
        slot = str(item["item"].get("slot") or "")
        if slot in spend:
            spend[slot] += int(item["item"].get("cost") or 0)
    return {
        slot: {
            "label": SLOT_LABELS.get(slot, slot),
            "planned_spend": spend[slot],
            "next_key_threshold": _next_threshold(hero_payload, slot, spend[slot]),
            "spike_threshold": SHOP_BONUS_SPIKE,
            "spike_reached": spend[slot] >= SHOP_BONUS_SPIKE,
        }
        for slot in ("vitality", "spirit", "weapon")
    }


def _shop_routes_to_4800(scored_items: list[dict[str, Any]]) -> dict[str, Any]:
    by_slot: dict[str, list[dict[str, Any]]] = defaultdict(list)
    for item in scored_items:
        slot = str(item["item"].get("slot") or "")
        if slot in PUBLIC_ITEM_SLOTS and item["item"]["cost"] <= 3200:
            by_slot[slot].append(item)

    routes: dict[str, Any] = {}
    for slot in ("vitality", "spirit", "weapon"):
        candidates = sorted(
            by_slot.get(slot, []),
            key=lambda item: (
                0 if item["tags"].get("early_economy") else 1,
                0 if item["hero_bucket"] == "Good" else 1,
                item["item"]["cost"],
                -item["score"],
            ),
        )
        route = []
        spend = 0
        for item in candidates:
            if item["item"]["name"] in {row["item"]["name"] for row in route}:
                continue
            route.append(item)
            spend += int(item["item"]["cost"] or 0)
            if spend >= SHOP_BONUS_SPIKE or len(route) >= 5:
                break
        routes[slot] = {
            "label": SLOT_LABELS.get(slot, slot),
            "spend": spend,
            "target": SHOP_BONUS_SPIKE,
            "items": _compact_recommendations(route),
            "target_reached": spend >= SHOP_BONUS_SPIKE,
        }
    return routes


def _economy_summary(hero_payload: dict[str, Any]) -> dict[str, Any]:
    cost_bonuses = hero_payload.get("cost_bonuses") if isinstance(hero_payload.get("cost_bonuses"), dict) else {}
    purchase_bonuses = hero_payload.get("purchase_bonuses") if isinstance(hero_payload.get("purchase_bonuses"), dict) else {}
    return {
        "wiki_rules": WIKI_ECONOMY_RULES,
        "important_shop_spike": SHOP_BONUS_SPIKE,
        "cost_bonuses": {
            slot: list(cost_bonuses.get(slot) or [])
            for slot in ("weapon", "vitality", "spirit")
        },
        "purchase_bonuses": {
            slot: list(purchase_bonuses.get(slot) or [])
            for slot in ("weapon", "vitality", "spirit")
        },
        "interpretation": "Souls sind Geld und XP. Ein Build muss Item-Effekt, Shop-Kategorie-Bonus, Boon-/Ability-Timing und Komponenten zusammen betrachten; 4800 Souls pro Kategorie ist ein wichtiger frueher Investment-Spike.",
    }


def _hero_summary(
    hero_payload: dict[str, Any],
    review_context: dict[str, Any],
    wiki_summary: dict[str, Any] | None,
    abilities: list[dict[str, Any]],
    hero_needs: dict[str, Any],
) -> dict[str, Any]:
    desc = hero_payload.get("description") if isinstance(hero_payload.get("description"), dict) else {}
    return {
        "name": hero_payload.get("name"),
        "hero_type": hero_payload.get("hero_type"),
        "role": desc.get("role"),
        "playstyle": desc.get("playstyle"),
        "gun_tag": hero_payload.get("gun_tag"),
        "starting_stats": _compact_starting_stats(hero_payload.get("starting_stats")),
        "level_up_stats": hero_payload.get("standard_level_up_upgrades") or {},
        "abilities": abilities,
        "inferred_gameplan": hero_needs,
        "wiki_summary": wiki_summary,
        "sheet_hints": (review_context.get("current_stat_hints") or {}).get("hints") or [],
    }


def _item_summary(payload: dict[str, Any]) -> dict[str, Any]:
    item = {
        "name": payload.get("name") or payload.get("class_name"),
        "class_name": payload.get("class_name"),
        "slot": payload.get("item_slot_type"),
        "tier": _int_or_zero(payload.get("item_tier")),
        "cost": _int_or_zero(payload.get("cost")),
        "is_active": bool(payload.get("is_active_item")),
        "activation": payload.get("activation"),
        "description": _clean_html_text(_nested_desc(payload.get("description"))),
        "component_items": payload.get("component_items") or [],
        "properties": _compact_properties(payload.get("properties")),
        "upgrades": payload.get("upgrades") or [],
    }
    item["archetypes"] = _classify_item_archetypes(item)
    return item


def _classify_item_archetypes(item: dict[str, Any]) -> list[str]:
    desc = str(item.get("description") or "")
    props = item.get("properties") or []
    blob = " ".join(
        [
            str(item.get("name") or ""),
            desc,
            *[
                " ".join(
                    str(prop.get(key) or "")
                    for key in ("name", "label", "provided_property_type")
                )
                for prop in props
            ],
        ]
    ).casefold()
    archetypes: set[str] = set()
    cost = int(item.get("cost") or 0)
    if cost >= 6400:
        archetypes.add("luxury")
    if item.get("is_active"):
        archetypes.add("active_burden")
    if any(word in blob for word in ("npc", "nonplayer", "non-player", "trooper", "bonus souls", "souls", "creep")):
        archetypes.add("lane_farm")
    if any(word in blob for word in ("npc damage", "trooper", "non-player", "nonplayer", "creep", "chain", "ricochet")):
        archetypes.add("waveclear")
    if any(word in blob for word in ("bonus souls", "secure", "claim", "confirm", "last hit", "orb")):
        archetypes.add("orb_secure")
    if any(word in blob for word in ("close range", "weapon damage", "fire rate", "max ammo", "reload", "bonus damage", "current health damage", "bullet lifesteal", "out of combat regen")):
        archetypes.add("lane_trade")
    if any(word in blob for word in ("close range", "bullet lifesteal", "melee", "duel", "weapon damage", "fire rate", "slow resist")):
        archetypes.add("duel")
    if any(word in blob for word in ("burst", "bonus damage", "damage amp", "amplification", "current health damage", "execute", "crit")):
        archetypes.add("burst")
    if any(word in blob for word in ("fire rate", "weapon damage", "max ammo", "reload", "bullet procs", "ricochet", "sustained")):
        archetypes.add("sustained_dps")
    if any(word in blob for word in ("slow", "stun", "silence", "disarm", "root", "immobil", "knock", "teleport", "dash distance", "gravity")):
        archetypes.add("kill_setup")
    if any(word in blob for word in ("teleport", "dash", "move speed", "sprint speed", "stamina", "escape", "barrier")):
        archetypes.add("escape")
    if any(word in blob for word in ("stun", "silence", "disarm", "knock", "hex", "curse", "area", "nearby enemies", "radius")):
        archetypes.add("teamfight_engage")
    if any(word in blob for word in ("debuff", "cleanse", "dispel", "unstoppable", "immune", "barrier", "shield", "return fire", "healing reduction", "metal skin")):
        archetypes.add("counter")
    if any(word in blob for word in ("healing reduction", "anti-heal", "healbane")):
        archetypes.add("anti_heal")
    if any(word in blob for word in ("disarm", "return fire", "metal skin", "bullet resist", "weapon damage reduction", "fire rate slow")):
        archetypes.add("anti_carry")
    if any(word in blob for word in ("rescue", "barrier", "shield", "cleanse", "dispel", "heal yourself and nearby allies")):
        archetypes.add("save")
    if any(word in blob for word in ("stack", "escalat", "amp", "cooldown", "duration", "ability range", "radius", "spirit power", "techpower", "charges", "ricochet", "bullet procs", "max weapon damage")):
        archetypes.add("core_scaling")
    if any(word in blob for word in ("nearby allies", "friendly", "ally", "aura", "rescue", "heal yourself and nearby allies", "healing output")):
        archetypes.add("support")
    if any(word in blob for word in ("guardian", "walker", "patron", "mid boss", "midboss", "objective", "non-player", "nonplayer", "npc damage")):
        archetypes.add("objective_damage")
    if any(word in blob for word in ("split push", "splitpush", "lane pressure", "trooper", "wave")):
        archetypes.add("splitpush")
    return sorted(archetypes)


def _compact_properties(properties: Any) -> list[dict[str, Any]]:
    if not isinstance(properties, dict):
        return []
    rows = []
    for name, prop in properties.items():
        if not isinstance(prop, dict):
            continue
        value = prop.get("value")
        disable = prop.get("disable_value")
        if str(value) in {"", "0", "0.0", "-1.0"} and str(disable) in {"0", "-1", "-2"}:
            continue
        rows.append(
            {
                "name": name,
                "label": prop.get("label") or prop.get("postvalue_label") or name,
                "value": value,
                "prefix": prop.get("prefix"),
                "postfix": prop.get("postfix"),
                "provided_property_type": prop.get("provided_property_type"),
                "tooltip_section": prop.get("tooltip_section"),
                "important": bool(prop.get("tooltip_is_important")),
                "elevated": bool(prop.get("tooltip_is_elevated")),
                "scales_with": _scale_hint(prop.get("scale_function")),
            }
        )
    return rows


def _load_public_items(conn: sqlite3.Connection) -> list[dict[str, Any]]:
    rows = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND entity_type='item_or_ability'
        """
    ).fetchall()
    items = []
    seen: set[str] = set()
    for row in rows:
        payload = _loads_json_object(row["payload_json"])
        class_name = str(payload.get("class_name") or "")
        if not class_name or class_name in seen:
            continue
        if payload.get("shopable") is not True:
            continue
        if payload.get("disabled") is True:
            continue
        if str(payload.get("item_slot_type") or "") not in PUBLIC_ITEM_SLOTS:
            continue
        if _int_or_zero(payload.get("cost")) <= 0:
            continue
        seen.add(class_name)
        items.append(payload)
    return items


def _load_hero_abilities(conn: sqlite3.Connection, hero_payload: dict[str, Any]) -> list[dict[str, Any]]:
    hero_items = hero_payload.get("items") if isinstance(hero_payload.get("items"), dict) else {}
    ability_classes = [
        str(hero_items.get(key) or "")
        for key in ("signature1", "signature2", "signature3", "signature4")
        if hero_items.get(key)
    ]
    abilities = []
    for class_name in ability_classes:
        row = conn.execute(
            """
            SELECT payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api'
              AND entity_type='item_or_ability'
              AND payload_json LIKE ?
            LIMIT 1
            """,
            (f'%"class_name": "{class_name}"%',),
        ).fetchone()
        if not row:
            continue
        payload = _loads_json_object(row["payload_json"])
        abilities.append(_ability_summary(payload))
    return abilities


def _ability_summary(payload: dict[str, Any]) -> dict[str, Any]:
    props = _compact_properties(payload.get("properties"))
    desc = payload.get("description") if isinstance(payload.get("description"), dict) else {}
    text = _clean_html_text(" ".join(str(desc.get(key) or "") for key in ("desc", "quip", "t1_desc", "t2_desc", "t3_desc")))
    return {
        "name": payload.get("name"),
        "class_name": payload.get("class_name"),
        "description": _clean_html_text(str(desc.get("desc") or "")),
        "role_tags": _infer_ability_role_tags(text, props),
        "key_properties": _key_ability_properties(props),
        "scaling_stats": sorted(_ability_scaling_stats(props)),
        "damage_profile": _ability_damage_profile(props),
        "upgrades": _compact_ability_upgrades(payload.get("upgrades")),
    }


def _infer_hero_needs(hero_payload: dict[str, Any], abilities: list[dict[str, Any]]) -> dict[str, Any]:
    role = str(hero_payload.get("hero_type") or "").casefold()
    gun_tag = str(hero_payload.get("gun_tag") or "").casefold()
    ability_tags = sorted({tag for ability in abilities for tag in (ability.get("role_tags") or [])})
    scaling_stats = sorted({stat for ability in abilities for stat in (ability.get("scaling_stats") or [])})
    damage_profile = _hero_damage_profile(abilities, ability_tags, role, gun_tag)
    needs = set()
    if "brawler" in role:
        needs.update({"frontline_survival", "engage_reach"})
    if any(tag in ability_tags for tag in ("engage", "pick", "disarm", "knockup", "stun", "silence", "immobilize", "sleep", "tether")):
        needs.update({"engage_reach", "cooldown_reliability"})
    if any(tag in ability_tags for tag in ("channel", "stun", "disarm", "slow", "silence", "immobilize", "sleep", "tether")):
        needs.add("ability_uptime")
    if damage_profile["damage_plan"] in {"spirit", "hybrid"}:
        needs.add("spirit_damage")
    if damage_profile["damage_plan"] in {"weapon", "hybrid"}:
        needs.add("weapon_damage")
    if any(tag in ability_tags for tag in ("self_heal", "health_stacking")):
        needs.add("frontline_survival")
    if "support_team" in ability_tags:
        needs.add("team_support")
    return {
        "summary": _gameplan_summary(hero_payload, ability_tags),
        "needs": sorted(needs),
        "ability_role_tags": ability_tags,
        "scaling_stats": scaling_stats,
        "priority_scaling_stats": _priority_scaling_stats(needs, damage_profile),
        "damage_profile": damage_profile,
        "lane_priorities": _lane_priorities(hero_payload, abilities),
        "build_implications": _build_implications(needs, scaling_stats, damage_profile),
        "decision_framework": _hero_decision_framework(),
    }


def _infer_ability_role_tags(text: str, props: list[dict[str, Any]]) -> list[str]:
    lower = text.casefold()
    tags = set()
    checks = {
        "self_heal": ("heal yourself", "heal", "lifesteal"),
        "engage": ("burrow", "moving faster", "teleport", "jump out"),
        "knockup": ("knockup", "knock up"),
        "disarm": ("disarm",),
        "silence": ("silence", "silenced"),
        "stun": ("stun", "stunning", "stunned"),
        "immobilize": ("immobilize", "immobilized", "root", "rooted"),
        "sleep": ("sleep", "drowsy"),
        "tether": ("tether", "binds", "binding"),
        "bleed": ("bleed",),
        "burn": ("burn", "burning"),
        "slow": ("slow", "dash distance"),
        "pick": ("hold the target", "stunning", "stun", "sleep", "immobilized"),
        "health_stacking": ("permanently gain max health", "bonus max health"),
        "channel": ("channel",),
        "damage_amp": ("damage to them", "+15% damage"),
        "support_team": ("ally", "allies", "friendly", "teammate", "nearby allies", "heal an ally", "targeted ally"),
        "weapon_scaling": ("weapon damage", "bullet damage", "fire rate", "ammo", "reload", "headshot"),
        "mobility": ("dash", "leap", "fly", "jump", "sprint", "move speed"),
        "burst": ("burst", "explode", "detonate", "nuke"),
    }
    for tag, needles in checks.items():
        if any(needle in lower for needle in needles):
            tags.add(tag)
    for prop in props:
        label = str(prop.get("label") or prop.get("name") or "").casefold()
        if "cooldown" in label:
            tags.add("cooldown_bound")
        if "cast range" in label or label == "radius":
            tags.add("range_sensitive")
        if (
            any(word in label for word in ("weapon damage", "fire rate", "ammo", "reload", "headshot"))
            or ("bullet" in label and not any(word in label for word in ("resist", "armor")))
        ):
            tags.add("weapon_scaling")
    return sorted(tags)


def _key_ability_properties(props: list[dict[str, Any]]) -> list[dict[str, Any]]:
    important_names = {
        "AbilityCooldown",
        "AbilityCastRange",
        "AbilityChannelTime",
        "AbilityDuration",
        "Radius",
        "Damage",
        "DPS",
        "BonusMoveSpeed",
        "BulletResist",
        "TechResist",
        "DamageHealMult",
        "DamageHealMultNonHero",
        "BonusHealthOnKill",
        "SlowPercent",
        "GroundDashReductionPercent",
    }
    return [prop for prop in props if prop.get("name") in important_names][:14]


def _ability_scaling_stats(props: list[dict[str, Any]]) -> set[str]:
    stats = set()
    for prop in props:
        provided = prop.get("provided_property_type")
        if provided:
            stats.add(str(provided))
        for scale in prop.get("scales_with") or []:
            stats.add(str(scale))
    return stats


def _ability_damage_profile(props: list[dict[str, Any]]) -> dict[str, int]:
    profile = {
        "damage_sources": 0,
        "spirit_scaling_damage_sources": 0,
        "weapon_stat_sources": 0,
        "control_sources": 0,
        "survival_sources": 0,
    }
    for prop in props:
        name = str(prop.get("name") or "")
        label = str(prop.get("label") or name).casefold()
        scales = {str(value) for value in (prop.get("scales_with") or [])}
        if name in {"Damage", "DPS"} and _numeric_value(prop.get("value")) > 0:
            profile["damage_sources"] += 1
            if "ETechPower" in scales:
                profile["spirit_scaling_damage_sources"] += 1
        if any(word in label for word in ("weapon damage", "fire rate", "ammo", "reload", "headshot")):
            profile["weapon_stat_sources"] += 1
        if name in {"SlowPercent", "GroundDashReductionPercent"} or any(word in label for word in ("stun", "slow", "silence", "disarm", "immobil")):
            profile["control_sources"] += 1
        if any(word in label for word in ("heal", "lifesteal", "resist", "barrier", "bonus health")):
            profile["survival_sources"] += 1
    return profile


def _hero_damage_profile(
    abilities: list[dict[str, Any]],
    ability_tags: list[str],
    role: str,
    gun_tag: str,
) -> dict[str, Any]:
    ability_damage_sources = sum(int((ability.get("damage_profile") or {}).get("damage_sources") or 0) for ability in abilities)
    spirit_scaling_damage_sources = sum(
        int((ability.get("damage_profile") or {}).get("spirit_scaling_damage_sources") or 0)
        for ability in abilities
    )
    weapon_stat_sources = sum(int((ability.get("damage_profile") or {}).get("weapon_stat_sources") or 0) for ability in abilities)
    if "weapon_scaling" in ability_tags:
        weapon_stat_sources += 2
    if "rapid" in gun_tag or "gun" in role or "marksman" in role or "assassin" in role:
        weapon_stat_sources += 1

    if ability_damage_sources >= 3 and weapon_stat_sources >= 2:
        damage_plan = "hybrid"
    elif weapon_stat_sources >= 3 and spirit_scaling_damage_sources <= 1:
        damage_plan = "weapon"
    elif spirit_scaling_damage_sources >= 3 and weapon_stat_sources <= 1:
        damage_plan = "spirit"
    elif spirit_scaling_damage_sources >= 2 and weapon_stat_sources >= 1:
        damage_plan = "hybrid"
    elif weapon_stat_sources >= 2:
        damage_plan = "weapon"
    elif spirit_scaling_damage_sources >= 2 or ability_damage_sources >= 3:
        damage_plan = "spirit"
    else:
        damage_plan = "utility"

    return {
        "damage_plan": damage_plan,
        "ability_damage_sources": ability_damage_sources,
        "spirit_scaling_damage_sources": spirit_scaling_damage_sources,
        "weapon_stat_sources": weapon_stat_sources,
    }


def _priority_scaling_stats(needs: set[str], damage_profile: dict[str, Any]) -> list[str]:
    stats = set()
    if "spirit_damage" in needs:
        stats.add("ETechPower")
    if "weapon_damage" in needs:
        stats.update(
            {
                "MODIFIER_VALUE_ATTACK_DAMAGE_ADDITIVE_ONLY",
                "MODIFIER_VALUE_BASEATTACK_DAMAGE_PERCENT",
                "MODIFIER_VALUE_FIRE_RATE",
                "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT",
            }
        )
    if "cooldown_reliability" in needs:
        stats.add("ETechCooldown")
    if "ability_uptime" in needs:
        stats.update({"ETechDuration", "EChannelDuration"})
    if "engage_reach" in needs:
        stats.update({"ETechRange", "ETechRadius"})
    if damage_profile.get("damage_plan") == "weapon":
        stats.discard("ETechPower")
    return sorted(stats)


def _compact_ability_upgrades(upgrades: Any) -> list[dict[str, Any]]:
    if not isinstance(upgrades, list):
        return []
    rows = []
    for index, upgrade in enumerate(upgrades, start=1):
        changes = []
        for change in (upgrade or {}).get("property_upgrades") or []:
            if isinstance(change, dict):
                changes.append({"name": change.get("name"), "bonus": change.get("bonus"), "scale_stat_filter": change.get("scale_stat_filter")})
        rows.append({"tier_index": index, "changes": changes})
    return rows


def _gameplan_summary(hero_payload: dict[str, Any], ability_tags: list[str]) -> str:
    if "pick" in ability_tags and "engage" in ability_tags:
        return "Engage-/Pick-Hero: Reichweitenfenster erzwingen, Control landen und Team-Follow-up auf ein priorisiertes Ziel ermoeglichen."
    if "weapon_scaling" in ability_tags and "mobility" in ability_tags:
        return "Mobile Weapon- oder Hybrid-Damage-Rolle: Farm/Positioning halten und Items kaufen, die Waffenfenster verlaengern oder sicherer machen."
    if "self_heal" in ability_tags or "frontline_survival" in ability_tags:
        return "Brawler/Sustain-Rolle: Trades und laengere Fights ueberleben, aber defensive Kaeufe muessen weiter Impact erzeugen."
    return str((hero_payload.get("description") or {}).get("playstyle") or "")


def _lane_priorities(hero_payload: dict[str, Any], abilities: list[dict[str, Any]]) -> list[str]:
    return [
        "Souls sind gleichzeitig Geld und XP/Boons; Lane-Items muessen Farm, Orb-Sicherung, Trades oder Kill-Setup verbessern.",
        "Fruehe 800/1600-Kaeufe sind nicht nur Item-Effekte, sondern auch Schritte zu Shop-Bonus-Schwellen pro Kategorie.",
        "Ein Lane-Kauf ist gut, wenn er ein konkretes Hero-Problem loest: Reichweite, Waveclear, Sustain, Ammo, Mobility, Cooldown, CC oder Kill-Druck.",
        "Reine Defensive ist nur dann frueh gut, wenn sie den Hero in seiner eigentlichen Aufgabe haelt; sonst verzögert sie Impact.",
    ]


def _build_implications(needs: set[str], scaling_stats: list[str], damage_profile: dict[str, Any]) -> list[str]:
    implications = []
    implications.append(
        "Damage-Plan: "
        f"{damage_profile.get('damage_plan')} "
        f"(ability_damage={damage_profile.get('ability_damage_sources')}, "
        f"spirit_scaling={damage_profile.get('spirit_scaling_damage_sources')}, "
        f"weapon_hooks={damage_profile.get('weapon_stat_sources')})."
    )
    if "engage_reach" in needs:
        implications.append("Range/Mobility ist wertvoll, wenn sie zentrale Control- oder Damage-Fenster erreichbar macht.")
    if "cooldown_reliability" in needs:
        implications.append("Cooldown-Items sind nur gut, wenn sie echte zweite Engage-/Control-Fenster schaffen.")
    if "spirit_damage" in needs:
        implications.append("Spirit Power hat Wert, weil mehrere Abilities mit ETechPower skalieren.")
    if "weapon_damage" in needs:
        implications.append("Weapon-Kaeufe brauchen entweder starke Gun-Skalierung, Orb-/Lane-Kontrolle oder ein klares Damage-Fenster.")
    if "frontline_survival" in needs:
        implications.append("Defensive Items brauchen einen Zweck: Channel ueberleben, Re-Engage schaffen oder Counter beantworten.")
    if "team_support" in needs:
        implications.append("Team-/Ally-Items koennen Core sein, wenn sie die erkannte Support-Aufgabe direkt staerken.")
    return implications


def _build_plan(hero_needs: dict[str, Any]) -> str:
    needs = set(hero_needs.get("needs") or [])
    if {"engage_reach", "cooldown_reliability"} <= needs:
        return "Erst Lane und Shop-Boni stabilisieren, dann Items kaufen, die zentrale Engage-/Control-Fenster verlaesslicher machen."
    if "weapon_damage" in needs and "spirit_damage" in needs:
        return "Hybrid-Plan: 4800-Shop-Boni bewusst balancen und nur Items kaufen, die entweder Gun-Fenster oder Ability-Fenster klar verbessern."
    if "weapon_damage" in needs:
        return "Weapon-Plan: Souls sichern, Gun-Shop-Bonus effizient aufbauen und defensive/aktive Items nur kaufen, wenn sie Damage-Fenster ermoeglichen."
    if "spirit_damage" in needs:
        return "Spirit-Plan: fruehe Economy mit Spirit-Shop-Bonus verbinden, danach Cooldown/Range/Duration passend zu den wichtigsten Abilities kaufen."
    return "Generischer Plan: Lane stabilisieren, 4800-Shop-Boni bewusst planen und jedes teure Item gegen den Hero-Gameplan begruenden."


def _lane_decision_model() -> list[str]:
    return [
        "1. Kann das Item Souls sichern, Trades gewinnen oder einen fruehen Shop-Bonus sinnvoll fuellen?",
        "2. Passt der Shop-Slot zum geplanten 4800-Fenster oder blockiert er bessere Core-Kaeufe?",
        "3. Loest es ein konkretes Hero-Problem: Reichweite, Cooldown, Channel-Sicherheit, Disarm/Pick-Follow-up?",
        "4. Wenn es nur Sustain/Defense gibt: nur kaufen, wenn der Gegnerdruck das erzwingt.",
    ]


def _hero_decision_framework() -> list[str]:
    return [
        "1. Hero-Aufgabe aus Abilities ableiten: poke, burst, pick, frontline, support, objective, gun-carry oder hybrid.",
        "2. Scaling lesen: welche Abilities skalieren mit Spirit Power, Weapon Damage, Duration, Range, Cooldown, Charges oder Boons?",
        "3. Fruehe Oekonomie planen: 800/1600-Items sollen Lane-Problem und Shop-Bonus-Fortschritt gleichzeitig bedienen.",
        "4. Core-Items muessen den Gameplan staerker machen, nicht nur gute generische Stats geben.",
        "5. T4/Luxury nur kaufen, wenn das Item einen konkreten Win-Condition-Schritt liefert.",
    ]


def _load_entity_payload(conn: sqlite3.Connection, query: str, *, entity_type: str) -> dict[str, Any] | None:
    context = build_entity_context(conn, query, limit_events=1)
    best = context.get("best_match") or {}
    if best.get("entity_type") != entity_type:
        if not (entity_type == "item" and best.get("entity_type") == "item_special"):
            return None
    entity_id = best.get("id")
    if entity_id is None:
        return None
    row = conn.execute(
        """
        SELECT s.payload_json
        FROM entity_aliases a
        JOIN entity_snapshots s ON s.id=a.snapshot_id
        WHERE a.entity_id=? AND s.source='deadlock_assets_api'
        ORDER BY CASE s.entity_type WHEN 'hero' THEN 0 WHEN 'item_or_ability' THEN 0 ELSE 1 END, s.id
        LIMIT 1
        """,
        (int(entity_id),),
    ).fetchone()
    return _loads_json_object(row["payload_json"]) if row else None


def _load_wiki_summary(conn: sqlite3.Connection, title: str) -> dict[str, Any] | None:
    row = conn.execute(
        """
        SELECT payload_json, fetched_at
        FROM entity_snapshots
        WHERE source='deadlock_wiki' AND external_id=?
        ORDER BY id DESC
        LIMIT 1
        """,
        (title,),
    ).fetchone()
    if not row:
        return None
    payload = _loads_json_object(row["payload_json"])
    pages = ((payload.get("query") or {}).get("pages") or {}) if isinstance(payload.get("query"), dict) else {}
    first_page = next(iter(pages.values()), {}) if isinstance(pages, dict) and pages else {}
    extract = str(first_page.get("extract") or "")
    paragraphs = [part.strip() for part in extract.split("\n") if part.strip()]
    gameplay = paragraphs[0] if paragraphs else ""
    trivia = next((part for part in paragraphs if "highest base health" in part.casefold()), "")
    return {
        "source": "deadlock_wiki",
        "fetched_at": row["fetched_at"],
        "gameplay_extract": gameplay[:700],
        "notable_trivia": trivia[:300] if trivia else None,
    }


def _load_statlocker_wpa_signals(conn: sqlite3.Connection, hero_name: str) -> dict[str, dict[str, Any]]:
    hero_key = _statlocker_key(hero_name)
    hero_web_key = _statlocker_key(_statlocker_hero_name(hero_name))
    rows = conn.execute(
        """
        SELECT payload_json, fetched_at
        FROM entity_snapshots
        WHERE source='statlocker'
          AND entity_type='statlocker_wpa_item'
        ORDER BY fetched_at DESC, id DESC
        """
    ).fetchall()
    signals: dict[str, dict[str, Any]] = {}
    fallback: dict[str, dict[str, Any]] = {}
    for row in rows:
        payload = _loads_json_object(row["payload_json"])
        item_name = str(payload.get("item") or "").strip()
        if not item_name:
            continue
        meta = payload.get("_deadlock_brain") if isinstance(payload.get("_deadlock_brain"), dict) else {}
        requested_hero = str(meta.get("requested_hero") or payload.get("heroName") or "all")
        signal = dict(payload)
        signal["fetched_at"] = row["fetched_at"]
        key = _statlocker_key(item_name)
        requested_key = _statlocker_key(requested_hero)
        if requested_key in {hero_key, hero_web_key}:
            signals.setdefault(key, signal)
        elif requested_hero == "all":
            fallback.setdefault(key, signal)
    return signals or fallback


def _compact_statlocker_signals(signals: dict[str, dict[str, Any]], *, limit: int = 12) -> list[dict[str, Any]]:
    rows = []
    for signal in signals.values():
        rows.append(
            {
                "item": signal.get("item"),
                "hero_name": signal.get("heroName"),
                "wpa": signal.get("wpaValue"),
                "cost_relative_wpa": signal.get("costRelativeWpa"),
                "sample_size": signal.get("sampleSize"),
                "mean_purchase_time_min": signal.get("mean_purchase_time_min"),
            }
        )
    rows.sort(
        key=lambda row: _numeric_value(
            row.get("cost_relative_wpa") if row.get("cost_relative_wpa") is not None else row.get("wpa")
        ),
        reverse=True,
    )
    return rows[:limit]


def _hero_item_bucket(hero_payload: dict[str, Any], item_payload: dict[str, Any]) -> str:
    buckets = hero_payload.get("item_draft_bucketing") if isinstance(hero_payload.get("item_draft_bucketing"), dict) else {}
    meta = buckets.get(str(item_payload.get("class_name") or ""))
    if isinstance(meta, dict):
        return str(meta.get("bucket") or "Normal")
    return "Normal"


def _diversified_pick(
    items: list[dict[str, Any]],
    *,
    max_items: int,
    max_per_slot: int,
    exclude_names: set[str] | None = None,
) -> list[dict[str, Any]]:
    exclude_names = exclude_names or set()
    picked = []
    by_slot: dict[str, int] = defaultdict(int)
    for item in items:
        name = str(item["item"]["name"])
        slot = str(item["item"].get("slot") or "")
        if name in exclude_names or by_slot[slot] >= max_per_slot:
            continue
        picked.append(item)
        by_slot[slot] += 1
        if len(picked) >= max_items:
            break
    return picked


def _pick_named(
    by_name: dict[str, dict[str, Any]],
    names: tuple[str, ...],
    *,
    max_items: int,
    exclude_names: set[str] | None = None,
) -> list[dict[str, Any]]:
    exclude_names = exclude_names or set()
    picked = []
    for name in names:
        item = by_name.get(name)
        if not item or name in exclude_names:
            continue
        picked.append(item)
        if len(picked) >= max_items:
            break
    return picked


def _phase_hint(name: str) -> str | None:
    if name in SITUATIONAL_ITEMS:
        return "situational"
    if name in LATE_ITEMS:
        return "late"
    return None


def _compact_recommendations(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    return [
        {
            "name": item["item"]["name"],
            "slot": item["item"]["slot"],
            "tier": item["item"]["tier"],
            "cost": item["item"]["cost"],
            "archetypes": item["item"].get("archetypes") or [],
            "score": item["score"],
            "why": item["reasons"][:3],
            "warnings": item["warnings"],
        }
        for item in items
    ]


def _rejected_expensive_items(items: list[dict[str, Any]]) -> list[dict[str, Any]]:
    rows = []
    for item in items:
        if item["item"]["cost"] >= 6400 and (
            item["score"] < 35
            or item["tags"].get("questionable_core")
            or item["item"]["name"] in QUESTIONABLE_LUXURY_ITEMS
        ):
            rows.append(
                {
                    "name": item["item"]["name"],
                    "cost": item["item"]["cost"],
                    "score": item["score"],
                    "warnings": item["warnings"],
                    "bucket": item["hero_bucket"],
                }
            )
    rows.sort(key=lambda row: (0 if row["name"] in QUESTIONABLE_LUXURY_ITEMS else 1, row["score"]))
    return rows[:12]


def _is_pure_defense(item: dict[str, Any]) -> bool:
    labels = [str(prop.get("label") or prop.get("name") or "").casefold() for prop in item.get("properties") or []]
    if not labels:
        return False
    defensive = sum(1 for label in labels if any(hint in label for hint in PURE_DEFENSE_HINTS))
    offensive = sum(1 for label in labels if any(hint in label for hint in ("damage", "cooldown", "duration", "range", "radius", "slow", "fire")))
    return defensive >= 2 and offensive == 0 and not item.get("is_active")


def _next_threshold(hero_payload: dict[str, Any], slot: str, spend: int) -> dict[str, Any] | None:
    cost_bonuses = hero_payload.get("cost_bonuses") if isinstance(hero_payload.get("cost_bonuses"), dict) else {}
    for row in cost_bonuses.get(slot) or []:
        threshold = _int_or_zero(row.get("gold_threshold"))
        if threshold > spend:
            return row
    return None


def _compact_starting_stats(stats: Any) -> dict[str, Any]:
    if not isinstance(stats, dict):
        return {}
    keys = ("max_health", "base_health_regen", "max_move_speed", "sprint_speed", "stamina", "heavy_melee_damage", "light_melee_damage")
    return {key: stats.get(key) for key in keys if key in stats}


def _scale_hint(scale_function: Any) -> list[str]:
    if not isinstance(scale_function, dict):
        return []
    stats = scale_function.get("scaling_stats")
    if isinstance(stats, list):
        return [str(stat) for stat in stats]
    stat = scale_function.get("specific_stat_scale_type")
    return [str(stat)] if stat else []


def _nested_desc(value: Any) -> str:
    if isinstance(value, dict):
        return str(value.get("desc") or "")
    return str(value or "")


def _clean_html_text(value: str) -> str:
    without_svg = re.sub(r"<svg\b.*?</svg>", " ", value, flags=re.IGNORECASE | re.DOTALL)
    without_tags = re.sub(r"<[^>]+>", " ", without_svg)
    return re.sub(r"\s+", " ", html.unescape(without_tags)).strip()


def _loads_json_object(value: Any) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    try:
        decoded = json.loads(str(value or "{}"))
    except json.JSONDecodeError:
        return {}
    return decoded if isinstance(decoded, dict) else {}


def _int_or_zero(value: Any) -> int:
    try:
        return int(float(str(value).replace(",", "").strip()))
    except (TypeError, ValueError):
        return 0


def _numeric_value(value: Any) -> float:
    try:
        return float(str(value).replace(",", "").strip())
    except (TypeError, ValueError):
        return 0.0


def _statlocker_hero_name(value: str) -> str:
    return str(value or "").strip().replace(" & ", "_and_").replace("&", "and").replace(" ", "_")


def _statlocker_key(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "", _statlocker_hero_name(str(value or "")).casefold())


def _dedupe_keep_order(values: list[str]) -> list[str]:
    seen = set()
    result = []
    for value in values:
        if not value or value in seen:
            continue
        seen.add(value)
        result.append(value)
    return result
