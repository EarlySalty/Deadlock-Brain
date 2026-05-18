from __future__ import annotations

import json
import sqlite3
import time
from collections import defaultdict
from typing import Any

from deadlock_brain.build_optimizer import build_hero_build_context, summarize_item_payload
from deadlock_brain.storage import stable_hash_text


PROMPT_VERSION = "player_match_decision_de_v2"


def ensure_player_decision_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS player_match_decision_notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          account_id TEXT NOT NULL,
          match_id TEXT NOT NULL,
          hero_id TEXT,
          hero_name TEXT,
          context_hash TEXT NOT NULL,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          result_text TEXT,
          insights_json TEXT NOT NULL DEFAULT '{}',
          model TEXT,
          status TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_player_match_decision_notes_unique
          ON player_match_decision_notes(account_id, match_id, context_hash, prompt_version, COALESCE(model, ''), status);

        CREATE INDEX IF NOT EXISTS idx_player_match_decision_notes_lookup
          ON player_match_decision_notes(account_id, match_id, updated_at DESC);
        """
    )
    conn.commit()


def list_player_matches(conn: sqlite3.Connection, *, account_id: str | None = None, limit: int = 25) -> list[dict[str, Any]]:
    params: list[Any] = []
    sql = """
        SELECT external_id, canonical_name, payload_json, fetched_at
        FROM entity_snapshots
        WHERE source='statlocker' AND entity_type='statlocker_player_match'
    """
    if account_id:
        sql += " AND external_id LIKE ?"
        params.append(f"{account_id}:%")
    sql += " ORDER BY fetched_at DESC, id DESC LIMIT ?"
    params.append(max(1, min(int(limit), 500)))
    rows = []
    for row in conn.execute(sql, params).fetchall():
        payload = _loads_json(row["payload_json"], fallback={})
        source = payload.get("_deadlock_brain") if isinstance(payload, dict) else {}
        source = source if isinstance(source, dict) else {}
        external = str(row["external_id"] or "")
        account, _, match = external.partition(":")
        rows.append(
            {
                "account_id": source.get("account_id") or account,
                "match_id": source.get("match_id") or match,
                "hero_id": source.get("hero_id") or _hero_id_from_payload(payload),
                "fetched_at": row["fetched_at"],
                "summary": _compact_match_row(payload),
            }
        )
    return rows


def list_pending_player_match_decision_targets(
    conn: sqlite3.Connection,
    *,
    account_id: str | None = None,
    limit: int = 5,
    model: str | None = None,
) -> list[dict[str, Any]]:
    ensure_player_decision_tables(conn)
    params: list[Any] = [PROMPT_VERSION, model]
    sql = """
        SELECT pm.external_id, pm.payload_json, pm.fetched_at
        FROM entity_snapshots pm
        WHERE pm.source='statlocker'
          AND pm.entity_type='statlocker_player_match'
          AND NOT EXISTS (
            SELECT 1
            FROM player_match_decision_notes n
            WHERE n.account_id = substr(pm.external_id, 1, instr(pm.external_id, ':') - 1)
              AND n.match_id = substr(pm.external_id, instr(pm.external_id, ':') + 1)
              AND n.status = 'analysis_ready'
              AND n.prompt_version = ?
              AND COALESCE(n.model, '') = COALESCE(?, '')
          )
    """
    if account_id:
        sql += " AND pm.external_id LIKE ?"
        params.append(f"{account_id}:%")
    sql += " ORDER BY pm.fetched_at DESC, pm.id DESC LIMIT ?"
    params.append(max(1, min(int(limit), 100)))
    targets = []
    for row in conn.execute(sql, params).fetchall():
        payload = _loads_json(row["payload_json"], fallback={})
        source = payload.get("_deadlock_brain") if isinstance(payload, dict) else {}
        source = source if isinstance(source, dict) else {}
        account, _, match = str(row["external_id"] or "").partition(":")
        account_id_value = str(source.get("account_id") or account).strip()
        match_id_value = str(source.get("match_id") or match).strip()
        if not account_id_value or not match_id_value:
            continue
        hero_id = source.get("hero_id") or _hero_id_from_payload(payload)
        targets.append(
            {
                "account_id": account_id_value,
                "match_id": match_id_value,
                "hero_id": str(hero_id) if hero_id is not None else None,
                "fetched_at": row["fetched_at"],
                "summary": _compact_match_row(payload),
            }
        )
    return targets


def build_player_match_decision_context(conn: sqlite3.Connection, *, account_id: str, match_id: str) -> dict[str, Any]:
    safe_account_id = str(account_id).strip()
    safe_match_id = str(match_id).strip()
    if not safe_account_id or not safe_match_id:
        raise ValueError("account_id und match_id sind erforderlich.")

    player_match = _latest_snapshot_payload(conn, "statlocker_player_match", f"{safe_account_id}:{safe_match_id}")
    if not player_match:
        raise ValueError(f"Kein Statlocker Player-Match fuer {safe_account_id}:{safe_match_id} gefunden.")
    profile = _latest_snapshot_payload(conn, "statlocker_player_profile", safe_account_id)
    match_detail = _latest_snapshot_payload(conn, "statlocker_match_detail", safe_match_id)
    deadlock_api_match = _latest_deadlock_api_match_metadata(conn, safe_match_id)

    hero_id = _hero_id_from_payload(player_match) or _hero_id_from_match_detail(match_detail, safe_account_id)
    hero_id = hero_id or _hero_id_from_deadlock_api_match(deadlock_api_match, safe_account_id)
    hero_name = _hero_name_from_id(conn, hero_id)
    build_analysis = _latest_snapshot_payload(conn, "statlocker_player_build_analysis", f"{safe_account_id}:{hero_id}") if hero_id else {}
    deterministic_context: dict[str, Any] = {}
    if hero_name:
        try:
            deterministic_context = build_hero_build_context(conn, hero_name, limit_events=60)
        except Exception as exc:
            deterministic_context = {"error": str(exc), "hero_name": hero_name}

    return {
        "context_kind": "player_match_decision_learning",
        "prompt_version": PROMPT_VERSION,
        "account_id": safe_account_id,
        "match_id": safe_match_id,
        "hero_id": hero_id,
        "hero_name": hero_name,
        "player_profile": _compact_profile(profile),
        "player_match": _compact_match_row(player_match),
        "match_detail": _compact_match_detail(match_detail, safe_account_id),
        "deadlock_api_match": _compact_deadlock_api_match(conn, deadlock_api_match, safe_account_id),
        "player_build_analysis": _compact_json(build_analysis, max_depth=4, max_list=25),
        "deterministic_build_brain": _compact_hero_context(deterministic_context),
        "learning_goal": {
            "source_assumption": "Deadlock API match metadata is treated as actual match evidence. Statlocker player_build_analysis is aggregate behavior across matches and must not be described as this exact match unless also present in the Deadlock API item timeline.",
            "focus": [
                "item order and timing",
                "specific item purpose for this hero and this game state",
                "hero ability function and scaling",
                "shop-bonus routes by Weapon/Vitality/Spirit spend",
                "standard core versus adaptation",
                "hero job and build variant",
                "game-state reasons for buys",
                "reusable coaching/build rules",
            ],
            "caution": "Do not invent winrates or hidden match data. Do not infer item sales unless sold_time_s is explicitly present and > 0. Mark facts as belegt, wahrscheinlich or unsicher.",
        },
    }


def build_minimax_player_match_decision_request(context: dict[str, Any], config: Any) -> dict[str, Any]:
    compact = _compact_json(context, max_depth=6, max_list=40)
    prompt = (
        "Analysiere dieses Deadlock Player-Match als Trainingsbeispiel fuer ein Coaching- und Build-Brain.\n"
        "Schreibe Deutsch, aber lasse alle Hero-, Item-, Ability-, Stat- und Map-/Mode-Namen exakt auf Englisch.\n"
        "Nutze nur den Kontext. Trenne harte Match-Fakten strikt von Aggregatmustern und Vermutungen.\n"
        "Prioritaet der Evidenz: 1) deadlock_api_match.actual_item_timeline und Match-Stats, "
        "2) Statlocker Player-/Build-Aggregate, 3) deterministic_build_brain als Regel-/Mechanik-Hilfe.\n"
        "Wenn sich Quellen widersprechen, zaehlt die Deadlock API fuer dieses konkrete Match.\n"
        "Item-ID-Mapping aus der Assets API ist verbindlich. Verwechsle z.B. Extra Stamina nicht mit Warp Stone.\n"
        "Beschreibe einen Sale nur, wenn `sold_time_s` im echten Match-Itemevent > 0 ist.\n\n"
        "Aufgaben:\n"
        "1. Hero-Verstaendnis: Was macht der Hero, welche Abilities tragen den Gameplan, welche Stats/Cooldowns/Actives skalieren ihn?\n"
        "2. Item-Verstaendnis: Fuer jedes wichtige Core-/T4-/Active-/verkaufte Item: welches konkrete Problem loest es hier fuer diesen Hero?\n"
        "3. Erklaere besonders Cooldown-Items sauber: Superior Cooldown = Ability-Fokus, Transcendent Cooldown = Abilities und Items; "
        "bewerte, ob der Hero und die gekauften Actives genug Cooldown-gebundenen Wert haben.\n"
        "4. Shop-Oekonomie: Welche Weapon/Vitality/Spirit Route entsteht, wann wird ungefaehr der 4800-Shop-Bonus erreicht, "
        "und ob die Route Lane/Core/Spike sinnvoll verbindet.\n"
        "5. Was ist belegt, was wahrscheinlich, was unsicher? Nutze diese Woerter explizit.\n"
        "6. Was wirkt wie Standard-Core, was wie situative Adaptation gegen Teamcomp/Game-State?\n"
        "7. Welche Build-Regeln soll das Brain daraus lernen, ohne sie blind auf jeden Hero zu kopieren?\n"
        "8. Gib am Ende ein kompaktes JSON-Feld `insights` mit keys: "
        "actual_match_facts, hero_job, hero_ability_scaling_rules, build_variant, item_purpose_rules, "
        "shop_bonus_rules, item_timing_rules, adaptation_rules, uncertain_inferences, coaching_takeaways, data_gaps.\n\n"
        f"Kontext JSON:\n{json.dumps(compact, ensure_ascii=False, sort_keys=True)}"
    )
    return {
        "model": config.model,
        "messages": [
            {
                "role": "system",
                "content": (
                    "Du bist ein strenger Deadlock Match- und Build-Analyst fuer einen deutschen Discord. "
                    "Du wandelst beobachtete High-Skill-Matches in wiederverwendbare, datenbasierte Regeln um. "
                    "Keine erfundenen Zahlen, keine ungekennzeichneten Vermutungen."
                ),
            },
            {"role": "user", "content": prompt},
        ],
        "max_completion_tokens": config.max_completion_tokens,
        "temperature": config.temperature,
        "top_p": config.top_p,
        "stream": False,
    }


def save_player_match_decision_note(
    conn: sqlite3.Connection,
    context: dict[str, Any],
    *,
    prompt_text: str,
    result_text: str | None,
    model: str | None,
    status: str,
) -> dict[str, Any]:
    ensure_player_decision_tables(conn)
    context_json = json.dumps(context, ensure_ascii=True, sort_keys=True)
    context_hash = stable_hash_text(context_json)
    now = int(time.time())
    conn.execute(
        """
        INSERT INTO player_match_decision_notes(
          account_id, match_id, hero_id, hero_name, context_hash, prompt_version,
          prompt_text, result_text, insights_json, model, status, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          insights_json=excluded.insights_json,
          hero_id=excluded.hero_id,
          hero_name=excluded.hero_name,
          updated_at=excluded.updated_at
        """,
        (
            str(context.get("account_id") or ""),
            str(context.get("match_id") or ""),
            context.get("hero_id"),
            context.get("hero_name"),
            context_hash,
            PROMPT_VERSION,
            prompt_text,
            result_text,
            json.dumps(_extract_insights(result_text or ""), ensure_ascii=True, sort_keys=True),
            model,
            status,
            now,
            now,
        ),
    )
    row = conn.execute(
        """
        SELECT id, account_id, match_id, hero_id, hero_name, model, status, updated_at
        FROM player_match_decision_notes
        WHERE account_id=? AND match_id=? AND context_hash=? AND prompt_version=?
          AND COALESCE(model, '')=COALESCE(?, '') AND status=?
        ORDER BY id DESC LIMIT 1
        """,
        (context.get("account_id"), context.get("match_id"), context_hash, PROMPT_VERSION, model, status),
    ).fetchone()
    conn.commit()
    return _row_to_dict(row) if row else {"context_hash": context_hash, "status": status}


def _latest_snapshot_payload(conn: sqlite3.Connection, entity_type: str, external_id: str) -> dict[str, Any]:
    row = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='statlocker' AND entity_type=? AND external_id=?
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        """,
        (entity_type, external_id),
    ).fetchone()
    return _loads_json(row["payload_json"], fallback={}) if row else {}


def _latest_deadlock_api_match_metadata(conn: sqlite3.Connection, match_id: str) -> dict[str, Any]:
    row = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_api' AND entity_type='deadlock_api_match_metadata' AND external_id=?
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        """,
        (str(match_id),),
    ).fetchone()
    return _loads_json(row["payload_json"], fallback={}) if row else {}


def _hero_id_from_deadlock_api_match(payload: Any, account_id: str) -> str | None:
    player = _deadlock_api_player(payload, account_id)
    value = player.get("hero_id") if isinstance(player, dict) else None
    return str(value).strip() if value is not None and str(value).strip() else None


def _compact_deadlock_api_match(conn: sqlite3.Connection, payload: Any, account_id: str) -> dict[str, Any]:
    if not isinstance(payload, dict):
        return {}
    player = _deadlock_api_player(payload, account_id)
    players = payload.get("players") if isinstance(payload.get("players"), list) else []
    team_roster = [_compact_deadlock_api_player_roster_row(conn, row) for row in players if isinstance(row, dict)]
    compact = {
        "evidence_layer": "actual_match_facts_from_deadlock_api",
        "match_id": payload.get("match_id") or payload.get("matchId"),
        "start_time": payload.get("start_time") or payload.get("startTime"),
        "duration_s": payload.get("duration_s") or payload.get("duration"),
        "winning_team": payload.get("winning_team") or payload.get("winningTeam"),
        "match_outcome": payload.get("match_outcome") or payload.get("matchOutcome"),
        "match_mode": payload.get("match_mode") or payload.get("matchMode"),
        "game_mode": payload.get("game_mode") or payload.get("gameMode"),
        "average_badges": {
            "team0": payload.get("average_badge_team0"),
            "team1": payload.get("average_badge_team1"),
        },
        "teams": team_roster,
        "data_quality_notes": [
            "actual_item_timeline kommt aus Deadlock API match metadata und ist fuer dieses Match priorisiert",
            "player_build_analysis ist Statlocker-Aggregat ueber mehrere Matches und darf nicht als exakte Timeline behandelt werden",
            "upgrade_id/imbued_ability_id werden soweit moeglich ueber Assets API aufgeloest; unbekannte IDs bleiben unsicher",
        ],
    }
    if player:
        compact["player"] = _compact_deadlock_api_player(conn, player, payload)
    return _compact_json(compact, max_depth=6, max_list=80)


def _deadlock_api_player(payload: Any, account_id: str) -> dict[str, Any]:
    if not isinstance(payload, dict):
        return {}
    for row in payload.get("players") or []:
        if not isinstance(row, dict):
            continue
        row_account = row.get("account_id") or row.get("accountId")
        if str(row_account or "") == str(account_id):
            return row
    return {}


def _compact_deadlock_api_player_roster_row(conn: sqlite3.Connection, row: dict[str, Any]) -> dict[str, Any]:
    hero_id = row.get("hero_id") or row.get("heroId")
    return {
        "account_id": row.get("account_id") or row.get("accountId"),
        "team": row.get("team"),
        "player_slot": row.get("player_slot") or row.get("playerSlot"),
        "assigned_lane": row.get("assigned_lane") or row.get("assignedLane"),
        "hero_id": hero_id,
        "hero_name": _hero_name_from_id(conn, str(hero_id)) if hero_id is not None else None,
        "kills": row.get("kills"),
        "deaths": row.get("deaths"),
        "assists": row.get("assists"),
        "net_worth": row.get("net_worth") or row.get("netWorth"),
    }


def _compact_deadlock_api_player(conn: sqlite3.Connection, player: dict[str, Any], match_payload: dict[str, Any]) -> dict[str, Any]:
    hero_id = player.get("hero_id") or player.get("heroId")
    raw_events = player.get("items") if isinstance(player.get("items"), list) else []
    all_events = [_compact_actual_item_event(conn, item) for item in raw_events if isinstance(item, dict)]
    item_timeline = [event for event in all_events if _is_public_shop_item_event(event)]
    ability_events = [event for event in all_events if not _is_public_shop_item_event(event)]
    item_timeline.sort(key=lambda row: _num(row.get("game_time_s"), default=0))
    ability_events.sort(key=lambda row: _num(row.get("game_time_s"), default=0))
    return {
        "account_id": player.get("account_id") or player.get("accountId"),
        "hero_id": hero_id,
        "hero_name": _hero_name_from_id(conn, str(hero_id)) if hero_id is not None else None,
        "team": player.get("team"),
        "player_slot": player.get("player_slot") or player.get("playerSlot"),
        "assigned_lane": player.get("assigned_lane") or player.get("assignedLane"),
        "scoreline": {
            "kills": player.get("kills"),
            "deaths": player.get("deaths"),
            "assists": player.get("assists"),
            "net_worth": player.get("net_worth") or player.get("netWorth"),
        },
        "enemy_heroes": _enemy_heroes(conn, match_payload, player.get("team")),
        "ally_heroes": _ally_heroes(conn, match_payload, player.get("team"), player.get("account_id") or player.get("accountId")),
        "actual_item_timeline": item_timeline,
        "ability_or_non_shop_events": ability_events[:30],
        "shop_bonus_actual_spend_by_slot_approx": _shop_bonus_progress(item_timeline),
        "stat_checkpoints": _stat_checkpoints(player.get("stats")),
        "death_details": _compact_death_details(conn, player.get("death_details") or player.get("deathDetails"), match_payload),
        "objective_events": _compact_objectives(match_payload.get("objectives")),
    }


def _compact_actual_item_event(conn: sqlite3.Connection, row: dict[str, Any]) -> dict[str, Any]:
    item_id = row.get("item_id") or row.get("itemId")
    upgrade_id = row.get("upgrade_id") or row.get("upgradeId")
    imbued_id = row.get("imbued_ability_id") or row.get("imbuedAbilityId")
    payload = _asset_payload_by_id(conn, item_id)
    upgrade_payload = _asset_payload_by_id(conn, upgrade_id)
    item = _item_mechanics(payload, item_id)
    upgrade = _item_mechanics(upgrade_payload, upgrade_id) if upgrade_payload else {}
    event = {
        "evidence": "deadlock_api_match_metadata.players.items",
        "game_time_s": row.get("game_time_s") or row.get("gameTimeS"),
        "item_id": str(item_id) if item_id is not None else None,
        "item_name": item.get("name"),
        "slot": item.get("slot"),
        "tier": item.get("tier"),
        "cost": item.get("cost"),
        "is_active": item.get("is_active"),
        "archetypes": item.get("archetypes"),
        "specific_purpose": _specific_item_purpose(item),
        "description": item.get("description"),
        "key_properties": item.get("key_properties"),
        "upgrade_id": str(upgrade_id) if upgrade_id is not None else None,
        "upgrade_name": upgrade.get("name"),
        "imbued_ability_id": str(imbued_id) if imbued_id is not None else None,
        "imbued_ability_name": _ability_name_from_id(conn, imbued_id),
        "sold_time_s": row.get("sold_time_s") or row.get("soldTimeS"),
        "flags": row.get("flags"),
    }
    if _num(event.get("sold_time_s"), default=0) <= 0:
        event.pop("sold_time_s", None)
    return {key: value for key, value in event.items() if value not in (None, "", [], {})}


def _asset_payload_by_id(conn: sqlite3.Connection, asset_id: Any) -> dict[str, Any]:
    if asset_id is None or str(asset_id).strip() == "":
        return {}
    safe_id = str(asset_id).strip()
    row = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api'
          AND entity_type='item_or_ability'
          AND external_id=?
        ORDER BY fetched_at DESC, id DESC
        LIMIT 1
        """,
        (safe_id,),
    ).fetchone()
    if not row:
        row = conn.execute(
            """
            SELECT payload_json
            FROM entity_snapshots
            WHERE source='deadlock_assets_api'
              AND entity_type='item_or_ability'
              AND payload_json LIKE ?
            ORDER BY fetched_at DESC, id DESC
            LIMIT 1
            """,
            (f'%"id": {safe_id}%',),
        ).fetchone()
    return _loads_json(row["payload_json"], fallback={}) if row else {}


def _is_public_shop_item_event(event: dict[str, Any]) -> bool:
    slot = str(event.get("slot") or "").strip()
    cost = _num(event.get("cost"), default=0)
    return slot in {"weapon", "vitality", "spirit"} and cost > 0


def _item_mechanics(payload: dict[str, Any], item_id: Any) -> dict[str, Any]:
    if not payload:
        return {"name": f"unknown:{item_id}" if item_id is not None else None}
    try:
        item = summarize_item_payload(payload)
    except Exception:
        item = {
            "name": payload.get("name") or payload.get("class_name"),
            "class_name": payload.get("class_name"),
            "slot": payload.get("item_slot_type"),
            "tier": payload.get("item_tier"),
            "cost": payload.get("cost"),
            "is_active": bool(payload.get("is_active_item")),
            "description": "",
            "properties": [],
            "archetypes": [],
        }
    return {
        "name": item.get("name"),
        "class_name": item.get("class_name"),
        "slot": item.get("slot"),
        "tier": item.get("tier"),
        "cost": item.get("cost"),
        "is_active": item.get("is_active"),
        "activation": item.get("activation"),
        "description": item.get("description"),
        "archetypes": item.get("archetypes"),
        "key_properties": _key_item_properties(item.get("properties")),
    }


def _specific_item_purpose(item: dict[str, Any]) -> str | None:
    name = str(item.get("name") or "")
    archetypes = set(item.get("archetypes") or [])
    if name == "Transcendent Cooldown":
        return "T4 Spirit cooldown economy fuer Abilities und Active Items; stark, wenn Hero/Build ueber wiederholte Ability- und Active-Zyklen gewinnt."
    if name == "Superior Cooldown":
        return "Ability-spezifische Cooldown-Verkuerzung; stark fuer eine zentrale, cooldown-limitierte Ability, aber nicht fuer Active-Item-CDs."
    if name == "Warp Stone":
        return "Active-Reposition fuer Engage, Escape, Angle-Wechsel und Pick-Setup; kein reines Damage-Item."
    if name == "Ethereal Shift":
        return "Defensives Active fuer Dodge/Survive/Reset gegen Burst oder Catch; kann riskante Engage-Fenster absichern."
    if name == "Extra Stamina":
        return "Fruehe Stamina-Oekonomie fuer Chase, Escape, Lane-Dodges und Map-Bewegung."
    if name == "Healing Rite":
        return "Fruehes Lane-/Rotation-Sustain-Active; stabilisiert HP und Tempo, wird spaeter oft verkauft, wenn Slots/Core wichtiger werden."
    if name == "Cold Front":
        return "Fruehes/mittleres Spirit-Active fuer AoE-Slow, Wave/Fight-Setup und sichere Trefferfenster."
    if name == "Mystic Burst":
        return "Frueher Spirit-Burst-Verstaerker fuer Abilities mit verlaesslichen Schadensfenstern; oft Komponente fuer spaetere Spirit-Burst-Linie."
    if name == "Compress Cooldown":
        return "Fruehe Cooldown-Oekonomie fuer einen cooldown-limitierten Ability-Plan; Vorbereitung auf Superior/Transcendent Cooldown-Linien."
    if name == "Mystic Expansion":
        return "Fruehes Spirit-Scaling ueber Range/Radius/Flaechen-Zuverlaessigkeit; gut, wenn Abilities mehrere Ziele oder sichere Treffer brauchen."
    if name == "Restorative Locket":
        return "Defensives Sustain-/Burst-Heal-Item gegen Poke und Fight-Schaden; kauft Zeit fuer riskantere Engage- oder Poke-Heroes."
    if name == "Swift Striker":
        return "Weapon-DPS und Bewegungsdruck; sinnvoll, wenn der Hero zwischen Ability-Zyklen mit Gun-Trades weiter Impact erzeugen soll."
    if name == "Close Quarters":
        return "Frueher Nahdistanz-Weapon-Druck; stark bei Heroes, die ohnehin in kurze Distanzen gehen."
    if name == "Point Blank":
        return "Weapon-Core fuer Nahdistanz-Burst und Chase; passt, wenn der Hero regelmaessig in kurze Reichweite kommt."
    if name == "Tankbuster":
        return "Spirit-Damage-Upgrade gegen hohe HP/Frontline-Ziele; kein generisches Burst-Item, sondern Anti-Health-Skalierung."
    if name == "Improved Spirit":
        return "Effizientes Spirit-Stat-Upgrade fuer Ability-Schaden und Spirit-Shop-Bonus-Fortschritt."
    if name == "Stamina Mastery":
        return "Midgame-Stamina-Core fuer Chase, Escape, Dodge-Frequenz und laengere Fight-Pattern."
    if name == "Weakening Headshot":
        return "Situativer Gun-Debuff gegen Ziele, die durch Resist/Frontline-Wert schwer sterben."
    if name == "Crippling Headshot":
        return "T4 Weapon-Debuff fuer Resist-Break und Ziel-Fokus; stark, wenn der Hero genug Headshot-/Gun-Fenster bekommt."
    if name == "Majestic Leap":
        return "Makro-/Engage-Mobility fuer Winkel, Initiation und schnelle Rotations; besonders wertvoll fuer Heroes mit starkem Eintritt in Fights."
    if "counter" in archetypes:
        return "Situatives Counter-Item gegen konkrete gegnerische Mechaniken, nicht automatisch Core."
    if "kill_setup" in archetypes:
        return "Setup-Item fuer Pick-Potenzial, Catch oder verlaessliche Ability-Treffer."
    if "core_scaling" in archetypes:
        return "Core-Scaling-Item; muss zum Hero-Schadensprofil oder Cooldown-/Uptime-Plan passen."
    if "escape" in archetypes:
        return "Mobilitaets-/Survival-Item zur Fehlervermeidung, Jagd oder Reposition."
    return None


def _key_item_properties(properties: Any) -> list[dict[str, Any]]:
    rows = properties if isinstance(properties, list) else []
    important = [row for row in rows if isinstance(row, dict) and (row.get("important") or row.get("elevated"))]
    selected = important or [row for row in rows if isinstance(row, dict)]
    return selected[:10]


def _ability_name_from_id(conn: sqlite3.Connection, ability_id: Any) -> str | None:
    payload = _asset_payload_by_id(conn, ability_id)
    name = payload.get("name") if isinstance(payload, dict) else None
    return str(name).strip() if name else None


def _shop_bonus_progress(item_timeline: list[dict[str, Any]]) -> dict[str, Any]:
    gross_spend_by_slot: defaultdict[str, int] = defaultdict(int)
    purchases_by_slot: defaultdict[str, list[dict[str, Any]]] = defaultdict(list)
    first_4800_gross: dict[str, Any] = {}
    for event in item_timeline:
        slot = str(event.get("slot") or "").strip()
        cost = int(_num(event.get("cost"), default=0))
        if slot not in {"weapon", "vitality", "spirit"} or cost <= 0:
            continue
        gross_spend_by_slot[slot] += cost
        purchases_by_slot[slot].append(
            {
                "time_s": event.get("game_time_s"),
                "item": event.get("item_name"),
                "cost": cost,
                "sold_time_s": event.get("sold_time_s"),
                "gross_running_spend": gross_spend_by_slot[slot],
            }
        )
        if slot not in first_4800_gross and gross_spend_by_slot[slot] >= 4800:
            first_4800_gross[slot] = event.get("game_time_s")
    current_value = _shop_current_value_progress(item_timeline)
    return {
        "note": "gross spend is useful for purchase tempo; current value subtracts explicit sold_time_s events and is safer for shop-bonus state. Upgrade/component interpretation can still be uncertain.",
        "target_spike_per_slot": 4800,
        "gross_spend_by_slot": dict(gross_spend_by_slot),
        "first_4800_time_s_gross": first_4800_gross,
        "current_value_by_slot_after_sales_approx": current_value.get("current_value_by_slot"),
        "first_4800_time_s_current_value_approx": current_value.get("first_4800_time_s"),
        "current_value_events": current_value.get("events"),
        "purchases_by_slot": {slot: rows for slot, rows in purchases_by_slot.items()},
    }


def _shop_current_value_progress(item_timeline: list[dict[str, Any]]) -> dict[str, Any]:
    events = []
    for item in item_timeline:
        slot = str(item.get("slot") or "").strip()
        cost = int(_num(item.get("cost"), default=0))
        if slot not in {"weapon", "vitality", "spirit"} or cost <= 0:
            continue
        events.append(
            {
                "time_s": _num(item.get("game_time_s"), default=0),
                "sort": 1,
                "slot": slot,
                "delta": cost,
                "item": item.get("item_name"),
                "event": "buy",
            }
        )
        sold_time = _num(item.get("sold_time_s"), default=0)
        if sold_time > 0:
            events.append(
                {
                    "time_s": sold_time,
                    "sort": 0,
                    "slot": slot,
                    "delta": -cost,
                    "item": item.get("item_name"),
                    "event": "sold",
                }
            )
    events.sort(key=lambda row: (row["time_s"], row["sort"]))
    current_by_slot: defaultdict[str, int] = defaultdict(int)
    first_4800: dict[str, Any] = {}
    compact_events = []
    for event in events:
        slot = str(event["slot"])
        current_by_slot[slot] = max(0, current_by_slot[slot] + int(event["delta"]))
        if slot not in first_4800 and current_by_slot[slot] >= 4800:
            first_4800[slot] = event["time_s"]
        compact_events.append(
            {
                "time_s": event["time_s"],
                "slot": slot,
                "event": event["event"],
                "item": event["item"],
                "delta": event["delta"],
                "current_value": current_by_slot[slot],
            }
        )
    return {
        "current_value_by_slot": dict(current_by_slot),
        "first_4800_time_s": first_4800,
        "events": compact_events[:80],
    }


def _enemy_heroes(conn: sqlite3.Connection, match_payload: dict[str, Any], team: Any) -> list[dict[str, Any]]:
    return [
        _compact_deadlock_api_player_roster_row(conn, row)
        for row in (match_payload.get("players") or [])
        if isinstance(row, dict) and row.get("team") != team
    ]


def _ally_heroes(conn: sqlite3.Connection, match_payload: dict[str, Any], team: Any, account_id: Any) -> list[dict[str, Any]]:
    return [
        _compact_deadlock_api_player_roster_row(conn, row)
        for row in (match_payload.get("players") or [])
        if isinstance(row, dict) and row.get("team") == team and str(row.get("account_id") or row.get("accountId") or "") != str(account_id or "")
    ]


def _stat_checkpoints(stats: Any) -> list[dict[str, Any]]:
    if not isinstance(stats, list):
        return []
    rows = [row for row in stats if isinstance(row, dict)]
    if not rows:
        return []
    indexes = sorted({0, len(rows) // 4, len(rows) // 2, (len(rows) * 3) // 4, len(rows) - 1})
    keys = (
        "time_stamp_s",
        "timeStampS",
        "net_worth",
        "netWorth",
        "player_damage",
        "playerDamage",
        "player_damage_taken",
        "playerDamageTaken",
        "tech_power",
        "techPower",
        "weapon_power",
        "weaponPower",
        "max_health",
        "maxHealth",
        "kills",
        "deaths",
        "assists",
    )
    return [{key: rows[index].get(key) for key in keys if key in rows[index]} for index in indexes]


def _compact_death_details(conn: sqlite3.Connection, death_details: Any, match_payload: dict[str, Any]) -> list[dict[str, Any]]:
    if not isinstance(death_details, list):
        return []
    players_by_slot = {
        str(row.get("player_slot") or row.get("playerSlot") or ""): row
        for row in match_payload.get("players") or []
        if isinstance(row, dict)
    }
    result = []
    for row in death_details[:12]:
        if not isinstance(row, dict):
            continue
        killer_slot = row.get("killer_player_slot") or row.get("killerPlayerSlot") or row.get("killer_slot")
        killer = players_by_slot.get(str(killer_slot or ""))
        killer_hero_id = killer.get("hero_id") if isinstance(killer, dict) else None
        result.append(
            {
                "game_time_s": row.get("game_time_s") or row.get("gameTimeS"),
                "killer_player_slot": killer_slot,
                "killer_hero_name": _hero_name_from_id(conn, str(killer_hero_id)) if killer_hero_id is not None else None,
                "time_to_kill_s": row.get("time_to_kill_s") or row.get("timeToKillS"),
                "damage_taken": row.get("damage_taken") or row.get("damageTaken"),
                "raw": _compact_json(row, max_depth=2, max_list=8),
            }
        )
    return result


def _compact_objectives(objectives: Any) -> list[dict[str, Any]]:
    if not isinstance(objectives, list):
        return []
    keys = (
        "game_time_s",
        "gameTimeS",
        "team",
        "objective_id",
        "objectiveId",
        "destroyed",
        "damage",
        "player_slot",
        "playerSlot",
    )
    rows = []
    for row in objectives[:20]:
        if isinstance(row, dict):
            rows.append({key: row.get(key) for key in keys if key in row})
    return rows


def _num(value: Any, *, default: float = 0.0) -> float:
    try:
        return float(value)
    except (TypeError, ValueError):
        return default


def _hero_name_from_id(conn: sqlite3.Connection, hero_id: str | None) -> str | None:
    if not hero_id:
        return None
    row = conn.execute(
        """
        SELECT canonical_name, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api' AND entity_type='hero'
          AND (external_id=? OR json_extract(payload_json, '$.id')=?)
        ORDER BY
          CASE
            WHEN json_extract(payload_json, '$.name') IS NOT NULL
                 AND json_extract(payload_json, '$.name') NOT LIKE 'hero_%'
            THEN 0 ELSE 1
          END,
          fetched_at DESC,
          id DESC
        LIMIT 1
        """,
        (str(hero_id), int(hero_id) if str(hero_id).isdigit() else str(hero_id)),
    ).fetchone()
    if not row:
        return None
    payload = _loads_json(row["payload_json"], fallback={})
    return str(payload.get("name") or row["canonical_name"] or "").strip() or None


def _hero_id_from_payload(payload: Any) -> str | None:
    if not isinstance(payload, dict):
        return None
    for key in ("hero_id", "heroId", "player_hero_id", "playerHeroId", "hero"):
        value = payload.get(key)
        if value is not None and str(value).strip():
            return str(value).strip()
    source = payload.get("_deadlock_brain") if isinstance(payload.get("_deadlock_brain"), dict) else {}
    value = source.get("hero_id")
    return str(value).strip() if value is not None and str(value).strip() else None


def _hero_id_from_match_detail(payload: Any, account_id: str) -> str | None:
    if not isinstance(payload, dict):
        return None
    for row in _walk_dicts(payload):
        row_account = row.get("accountId") or row.get("account_id") or row.get("playerAccountId")
        if str(row_account or "") != str(account_id):
            continue
        hero_id = _hero_id_from_payload(row)
        if hero_id:
            return hero_id
    return None


def _compact_profile(payload: Any) -> dict[str, Any]:
    if not isinstance(payload, dict):
        return {}
    keys = ("accountId", "account_id", "name", "personaName", "rank", "rankedRank", "badgeLevel", "leaderboardRank")
    return {key: payload.get(key) for key in keys if key in payload}


def _compact_match_row(payload: Any) -> dict[str, Any]:
    if not isinstance(payload, dict):
        return {}
    wanted = (
        "match_id",
        "matchId",
        "startTime",
        "start_time",
        "duration",
        "duration_s",
        "hero_id",
        "heroId",
        "netWorth",
        "net_worth",
        "kills",
        "deaths",
        "assists",
        "playerScore",
        "player_score",
        "matchResult",
        "match_result",
        "won",
        "items",
        "itemBuild",
        "item_build",
        "abilityOrder",
        "ability_order",
    )
    compact = {key: payload.get(key) for key in wanted if key in payload}
    source = payload.get("_deadlock_brain") if isinstance(payload.get("_deadlock_brain"), dict) else {}
    if source:
        compact["_deadlock_brain"] = source
    return _compact_json(compact, max_depth=4, max_list=30)


def _compact_match_detail(payload: Any, account_id: str) -> dict[str, Any]:
    if not isinstance(payload, dict):
        return {}
    compact = {}
    for key in ("match_id", "matchId", "duration", "duration_s", "startTime", "start_time", "winningTeam", "winning_team"):
        if key in payload:
            compact[key] = payload.get(key)
    player_rows = []
    for row in _walk_dicts(payload):
        row_account = row.get("accountId") or row.get("account_id") or row.get("playerAccountId")
        if str(row_account or "") == str(account_id):
            player_rows.append(_compact_json(row, max_depth=3, max_list=30))
    if player_rows:
        compact["matched_player_rows"] = player_rows[:4]
    return compact


def _compact_hero_context(context: dict[str, Any]) -> dict[str, Any]:
    hero = context.get("hero") if isinstance(context.get("hero"), dict) else {}
    build = context.get("build") if isinstance(context.get("build"), dict) else {}
    return {
        "hero": {
            "name": hero.get("name"),
            "hero_type": hero.get("hero_type"),
            "role": hero.get("role"),
            "playstyle": hero.get("playstyle"),
            "inferred_gameplan": hero.get("inferred_gameplan"),
            "abilities": hero.get("abilities"),
            "sheet_hints": hero.get("sheet_hints"),
        },
        "build": {
            "plan": build.get("plan"),
            "lane_decision_model": build.get("lane_decision_model"),
            "early": build.get("early"),
            "core": build.get("core"),
            "late": build.get("late"),
            "situational": build.get("situational"),
            "shop_routes_to_4800": build.get("shop_routes_to_4800"),
        },
        "statlocker_signals": context.get("statlocker_signals"),
        "review_signals": context.get("review_signals"),
        "error": context.get("error"),
    }


def _compact_json(value: Any, *, max_depth: int, max_list: int) -> Any:
    if max_depth <= 0:
        if isinstance(value, dict):
            return {"_truncated": "object", "keys": list(value.keys())[:20]}
        if isinstance(value, list):
            return {"_truncated": "list", "count": len(value)}
        return value
    if isinstance(value, dict):
        return {str(key): _compact_json(item, max_depth=max_depth - 1, max_list=max_list) for key, item in list(value.items())[:80]}
    if isinstance(value, list):
        return [_compact_json(item, max_depth=max_depth - 1, max_list=max_list) for item in value[:max_list]]
    return value


def _walk_dicts(value: Any):
    if isinstance(value, dict):
        yield value
        for item in value.values():
            yield from _walk_dicts(item)
    elif isinstance(value, list):
        for item in value:
            yield from _walk_dicts(item)


def _extract_insights(text: str) -> dict[str, Any]:
    marker = '"insights"'
    if marker not in text:
        return {}
    start = text.find("{", max(0, text.find(marker) - 50))
    end = text.rfind("}")
    if start < 0 or end <= start:
        return {}
    try:
        parsed = json.loads(text[start : end + 1])
    except json.JSONDecodeError:
        return {}
    return parsed.get("insights") if isinstance(parsed.get("insights"), dict) else parsed if isinstance(parsed, dict) else {}


def _loads_json(value: Any, *, fallback: Any) -> Any:
    if isinstance(value, (dict, list)):
        return value
    try:
        return json.loads(str(value or ""))
    except json.JSONDecodeError:
        return fallback


def _row_to_dict(row: Any) -> dict[str, Any]:
    if row is None:
        return {}
    return {key: row[key] for key in row.keys()}
