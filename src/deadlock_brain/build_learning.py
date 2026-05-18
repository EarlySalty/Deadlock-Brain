from __future__ import annotations

import json
import sqlite3
import time
from pathlib import Path
from typing import Any

from deadlock_brain.build_optimizer import build_hero_build_context
from deadlock_brain.storage import stable_hash_text


PROMPT_VERSION = "build_learning_de_v1"


def ensure_build_learning_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS learned_builds (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          source TEXT NOT NULL,
          source_build_id TEXT NOT NULL,
          hero_id INTEGER,
          hero_name TEXT,
          language INTEGER,
          source_rank INTEGER,
          quality_tier TEXT NOT NULL,
          quality_score REAL NOT NULL,
          name TEXT,
          author_account_id TEXT,
          description TEXT,
          tags_json TEXT NOT NULL DEFAULT '[]',
          details_json TEXT NOT NULL DEFAULT '{}',
          item_names_json TEXT NOT NULL DEFAULT '[]',
          ability_order_json TEXT NOT NULL DEFAULT '[]',
          source_metadata_json TEXT NOT NULL DEFAULT '{}',
          imported_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          UNIQUE(source, source_build_id)
        );

        CREATE INDEX IF NOT EXISTS idx_learned_builds_hero
          ON learned_builds(hero_name, quality_score DESC);

        CREATE INDEX IF NOT EXISTS idx_learned_builds_quality
          ON learned_builds(quality_tier, quality_score DESC);

        CREATE TABLE IF NOT EXISTS build_learning_notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          learned_build_id INTEGER,
          hero_name TEXT,
          source TEXT NOT NULL,
          context_hash TEXT NOT NULL,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          result_text TEXT,
          insights_json TEXT NOT NULL DEFAULT '{}',
          model TEXT,
          status TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(learned_build_id) REFERENCES learned_builds(id) ON DELETE CASCADE
        );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_build_learning_notes_unique
          ON build_learning_notes(learned_build_id, context_hash, prompt_version, COALESCE(model, ''), status);

        CREATE INDEX IF NOT EXISTS idx_build_learning_notes_hero
          ON build_learning_notes(hero_name, updated_at DESC);
        """
    )
    conn.commit()


def import_steam_builds(
    conn: sqlite3.Connection,
    *,
    steam_db_path: Path,
    hero: str | None = None,
    language: int = 0,
    limit_per_hero: int = 10,
) -> dict[str, Any]:
    ensure_build_learning_tables(conn)
    if not steam_db_path.exists():
        return {"steam_db_path": str(steam_db_path), "imported": 0, "updated": 0, "skipped": 0, "error": "db_not_found"}
    source_conn = sqlite3.connect(str(steam_db_path))
    source_conn.row_factory = sqlite3.Row
    try:
        if not _table_exists(source_conn, "hero_build_sources"):
            return {"steam_db_path": str(steam_db_path), "imported": 0, "updated": 0, "skipped": 0, "error": "hero_build_sources_missing"}
        hero_map = _hero_id_name_map(conn)
        item_map = _item_id_name_map(conn)
        rows = _load_steam_build_rows(source_conn, hero_map, hero=hero, language=language, limit_per_hero=limit_per_hero)
        now = int(time.time())
        imported = 0
        updated = 0
        skipped = 0
        for row in rows:
            source_build_id = str(row["hero_build_id"] or "").strip()
            if not source_build_id:
                skipped += 1
                continue
            hero_id = _int_or_none(row["hero_id"])
            hero_name = hero_map.get(hero_id) or f"hero_id:{hero_id}" if hero_id is not None else None
            details = _loads_json(row["details_json"], fallback={})
            tags = _loads_json(row["tags_json"], fallback=[])
            item_names = _extract_item_names(details, item_map)
            ability_order = _extract_ability_order(details, item_map)
            source_rank = int(row["source_rank"] or 0)
            quality_tier, quality_score = _quality_from_rank(source_rank)
            before = conn.execute(
                "SELECT id FROM learned_builds WHERE source=? AND source_build_id=?",
                ("steam_gc", source_build_id),
            ).fetchone()
            conn.execute(
                """
                INSERT INTO learned_builds(
                  source, source_build_id, hero_id, hero_name, language, source_rank,
                  quality_tier, quality_score, name, author_account_id, description,
                  tags_json, details_json, item_names_json, ability_order_json,
                  source_metadata_json, imported_at, updated_at
                )
                VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
                ON CONFLICT(source, source_build_id) DO UPDATE SET
                  hero_id=excluded.hero_id,
                  hero_name=excluded.hero_name,
                  language=excluded.language,
                  source_rank=excluded.source_rank,
                  quality_tier=excluded.quality_tier,
                  quality_score=excluded.quality_score,
                  name=excluded.name,
                  author_account_id=excluded.author_account_id,
                  description=excluded.description,
                  tags_json=excluded.tags_json,
                  details_json=excluded.details_json,
                  item_names_json=excluded.item_names_json,
                  ability_order_json=excluded.ability_order_json,
                  source_metadata_json=excluded.source_metadata_json,
                  updated_at=excluded.updated_at
                """,
                (
                    "steam_gc",
                    source_build_id,
                    hero_id,
                    hero_name,
                    _int_or_none(row["language"]),
                    source_rank,
                    quality_tier,
                    quality_score,
                    row["name"],
                    str(row["author_account_id"] or "") if row["author_account_id"] is not None else None,
                    row["description"],
                    json.dumps(tags if isinstance(tags, list) else [], ensure_ascii=True, sort_keys=True),
                    row["details_json"] or "{}",
                    json.dumps(item_names, ensure_ascii=True, sort_keys=True),
                    json.dumps(ability_order, ensure_ascii=True, sort_keys=True),
                    json.dumps(
                        {
                            "origin_build_id": row["origin_build_id"],
                            "version": row["version"],
                            "publish_ts": row["publish_ts"],
                            "last_updated_ts": row["last_updated_ts"],
                            "fetched_at": row["fetched_at"],
                            "last_seen_at": row["last_seen_at"],
                        },
                        ensure_ascii=True,
                        sort_keys=True,
                    ),
                    now,
                    now,
                ),
            )
            if before:
                updated += 1
            else:
                imported += 1
        conn.commit()
        return {
            "steam_db_path": str(steam_db_path),
            "language": language,
            "limit_per_hero": limit_per_hero,
            "rows_seen": len(rows),
            "imported": imported,
            "updated": updated,
            "skipped": skipped,
        }
    finally:
        source_conn.close()


def list_learned_builds(conn: sqlite3.Connection, *, hero: str | None = None, limit: int = 25) -> list[dict[str, Any]]:
    ensure_build_learning_tables(conn)
    params: list[Any] = []
    sql = """
        SELECT id, source, source_build_id, hero_name, language, source_rank,
               quality_tier, quality_score, name, author_account_id, updated_at,
               item_names_json
        FROM learned_builds
    """
    if hero:
        sql += " WHERE hero_name LIKE ?"
        params.append(f"%{hero}%")
    sql += " ORDER BY hero_name, quality_score DESC, source_rank ASC, updated_at DESC LIMIT ?"
    params.append(max(1, min(int(limit), 500)))
    return [_row_to_dict(row) | {"item_names": _loads_json(row["item_names_json"], fallback=[])} for row in conn.execute(sql, params).fetchall()]


def list_pending_build_learning_targets(
    conn: sqlite3.Connection,
    *,
    hero: str | None = None,
    limit: int = 5,
    model: str | None = None,
) -> list[dict[str, Any]]:
    ensure_build_learning_tables(conn)
    params: list[Any] = [PROMPT_VERSION, model]
    sql = """
        SELECT id, source, source_build_id, hero_name, source_rank,
               quality_tier, quality_score, name, item_names_json
        FROM learned_builds lb
        WHERE NOT EXISTS (
          SELECT 1
          FROM build_learning_notes n
          WHERE n.learned_build_id = lb.id
            AND n.status = 'analysis_ready'
            AND n.prompt_version = ?
            AND COALESCE(n.model, '') = COALESCE(?, '')
        )
    """
    if hero:
        sql += " AND lb.hero_name LIKE ?"
        params.append(f"%{hero}%")
    sql += """
        ORDER BY lb.quality_score DESC, lb.source_rank ASC, lb.updated_at DESC, lb.id ASC
        LIMIT ?
    """
    params.append(max(1, min(int(limit), 100)))
    return [_row_to_dict(row) | {"item_names": _loads_json(row["item_names_json"], fallback=[])} for row in conn.execute(sql, params).fetchall()]


def build_learning_context(conn: sqlite3.Connection, learned_build_id: int) -> dict[str, Any]:
    ensure_build_learning_tables(conn)
    row = conn.execute("SELECT * FROM learned_builds WHERE id=?", (int(learned_build_id),)).fetchone()
    if not row:
        raise ValueError(f"Kein learned_build mit id={learned_build_id} gefunden.")
    build = _row_to_dict(row)
    build["tags"] = _loads_json(build.pop("tags_json"), fallback=[])
    build["details"] = _loads_json(build.pop("details_json"), fallback={})
    build["item_names"] = _loads_json(build.pop("item_names_json"), fallback=[])
    build["ability_order"] = _loads_json(build.pop("ability_order_json"), fallback=[])
    build["source_metadata"] = _loads_json(build.pop("source_metadata_json"), fallback={})
    build["item_categories"] = _extract_item_categories(build["details"], _item_id_name_map(conn))
    hero_name = str(build.get("hero_name") or "")
    hero_context = build_hero_build_context(conn, hero_name, limit_events=80) if hero_name and not hero_name.startswith("hero_id:") else {}
    sibling_builds = _sibling_build_summaries(conn, hero_name, exclude_id=int(learned_build_id))
    return {
        "context_kind": "build_learning",
        "prompt_version": PROMPT_VERSION,
        "build": build,
        "hero_context": hero_context,
        "nearby_top_builds": sibling_builds,
        "learning_goal": {
            "top_build_assumption": "Steam/GC top builds are treated as weak positive labels: top 1-3 usually good, top 4-10 often usable, lower ranks noisy.",
            "output_goal": "Extract reusable build principles, item timing, hero job, build variants and suspicious choices.",
        },
    }


def build_minimax_build_learning_request(context: dict[str, Any], config: Any) -> dict[str, Any]:
    compact = _compact_learning_context(context)
    prompt = (
        "Analysiere diesen Deadlock Hero-Build als Trainingsbeispiel fuer ein Build-Brain.\n"
        "Schreibe Deutsch, aber lasse alle Hero-, Item-, Ability- und Stat-Namen exakt auf Englisch.\n"
        "Bewerte nicht nach erfundenen Winrates. Nutze nur den Kontext.\n\n"
        "Aufgaben:\n"
        "1. Ist der Build wahrscheinlich gut, situativ gut, mittel oder schlecht? Warum?\n"
        "2. Welche Hero-Aufgabe und Build-Variante erkennt man?\n"
        "3. Welche Items sind Lane/Core/Late/Situational und warum genau?\n"
        "4. Welche Kaufreihenfolge/Timing-Logik laesst sich ableiten?\n"
        "5. Welche allgemeinen Regeln soll das Brain fuer diesen Hero lernen?\n"
        "6. Welche Teile sind unsicher oder koennten nur Popularitaetsrauschen sein?\n"
        "7. Gib am Ende ein kompaktes JSON-Feld `insights` mit keys: "
        "hero_job, build_variant, core_items, situational_items, avoid_or_question, timing_rules, scoring_hints.\n\n"
        f"Kontext JSON:\n{json.dumps(compact, ensure_ascii=False, sort_keys=True)}"
    )
    return {
        "model": config.model,
        "messages": [
            {
                "role": "system",
                "content": (
                    "Du bist ein strenger Deadlock Build-Analyst fuer ein deutsches Coaching-Brain. "
                    "Du arbeitest datenbasiert, markierst Unsicherheit und verwandelst einzelne Builds in wiederverwendbare Regeln."
                ),
            },
            {"role": "user", "content": prompt},
        ],
        "max_completion_tokens": config.max_completion_tokens,
        "temperature": config.temperature,
        "top_p": config.top_p,
        "stream": False,
    }


def save_build_learning_note(
    conn: sqlite3.Connection,
    context: dict[str, Any],
    *,
    prompt_text: str,
    result_text: str | None,
    model: str | None,
    status: str,
) -> dict[str, Any]:
    ensure_build_learning_tables(conn)
    context_json = json.dumps(context, ensure_ascii=True, sort_keys=True)
    context_hash = stable_hash_text(context_json)
    build = context.get("build") if isinstance(context.get("build"), dict) else {}
    now = int(time.time())
    conn.execute(
        """
        INSERT INTO build_learning_notes(
          learned_build_id, hero_name, source, context_hash, prompt_version,
          prompt_text, result_text, insights_json, model, status, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          insights_json=excluded.insights_json,
          updated_at=excluded.updated_at
        """,
        (
            int(build.get("id")) if build.get("id") is not None else None,
            build.get("hero_name"),
            str(build.get("source") or "unknown"),
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
        SELECT id, learned_build_id, hero_name, source, model, status, updated_at
        FROM build_learning_notes
        WHERE context_hash=? AND prompt_version=? AND COALESCE(model, '')=COALESCE(?, '') AND status=?
        ORDER BY id DESC LIMIT 1
        """,
        (context_hash, PROMPT_VERSION, model, status),
    ).fetchone()
    conn.commit()
    return _row_to_dict(row) if row else {"context_hash": context_hash, "status": status}


def _load_steam_build_rows(
    conn: sqlite3.Connection,
    hero_map: dict[int, str],
    *,
    hero: str | None,
    language: int,
    limit_per_hero: int,
) -> list[sqlite3.Row]:
    hero_ids = None
    if hero:
        hero_key = hero.casefold()
        hero_ids = [hero_id for hero_id, name in hero_map.items() if hero_key in name.casefold()]
        if not hero_ids:
            return []
    params: list[Any] = [int(language)]
    sql = """
        SELECT *
        FROM (
          SELECT hbs.*,
                 ROW_NUMBER() OVER (
                   PARTITION BY hbs.hero_id, hbs.language
                   ORDER BY COALESCE(hbs.last_seen_at, hbs.fetched_at, hbs.last_updated_ts, hbs.publish_ts, 0) DESC,
                            COALESCE(hbs.publish_ts, 0) DESC,
                            hbs.hero_build_id ASC
                 ) AS source_rank
          FROM hero_build_sources hbs
          WHERE hbs.language = ?
    """
    if hero_ids:
        placeholders = ",".join("?" for _ in hero_ids)
        sql += f" AND hbs.hero_id IN ({placeholders})"
        params.extend(hero_ids)
    sql += """
        )
        WHERE source_rank <= ?
        ORDER BY hero_id ASC, source_rank ASC
    """
    params.append(max(1, min(int(limit_per_hero), 50)))
    return list(conn.execute(sql, params).fetchall())


def _hero_id_name_map(conn: sqlite3.Connection) -> dict[int, str]:
    rows = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api' AND entity_type='hero'
        """
    ).fetchall()
    result = {}
    for row in rows:
        payload = _loads_json(row["payload_json"], fallback={})
        hero_id = _int_or_none(payload.get("id"))
        name = str(payload.get("name") or "").strip()
        if hero_id is not None and name and payload.get("disabled") is not True:
            result[hero_id] = name
    return result


def _item_id_name_map(conn: sqlite3.Connection) -> dict[int, str]:
    rows = conn.execute(
        """
        SELECT payload_json
        FROM entity_snapshots
        WHERE source='deadlock_assets_api' AND entity_type='item_or_ability'
        """
    ).fetchall()
    result = {}
    for row in rows:
        payload = _loads_json(row["payload_json"], fallback={})
        item_id = _int_or_none(payload.get("id"))
        name = str(payload.get("name") or "").strip()
        if item_id is not None and name:
            result[item_id] = name
    return result


def _extract_item_names(details: Any, item_map: dict[int, str]) -> list[str]:
    names = []
    if not isinstance(details, dict):
        return names
    for category in details.get("mod_categories") or details.get("modCategories") or []:
        if not isinstance(category, dict):
            continue
        for mod in category.get("mods") or []:
            if not isinstance(mod, dict):
                continue
            ability_id = _int_or_none(mod.get("ability_id") or mod.get("abilityId"))
            name = item_map.get(ability_id) if ability_id is not None else None
            if name and name not in names:
                names.append(name)
    return names


def _extract_ability_order(details: Any, item_map: dict[int, str]) -> list[dict[str, Any]]:
    if not isinstance(details, dict):
        return []
    ability_order = details.get("ability_order") or details.get("abilityOrder") or {}
    changes = ability_order.get("currency_changes") or ability_order.get("currencyChanges") or []
    result = []
    for change in changes:
        if not isinstance(change, dict):
            continue
        ability_id = _int_or_none(change.get("ability_id") or change.get("abilityId"))
        result.append(
            {
                "ability_id": ability_id,
                "ability_name": item_map.get(ability_id) if ability_id is not None else None,
                "currency_type": change.get("currency_type") or change.get("currencyType"),
                "delta": change.get("delta"),
                "annotation": change.get("annotation"),
            }
        )
    return result


def _quality_from_rank(rank: int) -> tuple[str, float]:
    if rank <= 0:
        return "unknown", 0.5
    if rank <= 3:
        return "likely_good", 0.9
    if rank <= 10:
        return "usable_noisy", 0.7
    return "low_confidence", 0.35


def _sibling_build_summaries(conn: sqlite3.Connection, hero_name: str, *, exclude_id: int) -> list[dict[str, Any]]:
    if not hero_name:
        return []
    rows = conn.execute(
        """
        SELECT id, source_build_id, source_rank, quality_tier, quality_score, name, item_names_json
        FROM learned_builds
        WHERE hero_name=? AND id != ?
        ORDER BY quality_score DESC, source_rank ASC
        LIMIT 8
        """,
        (hero_name, exclude_id),
    ).fetchall()
    return [_row_to_dict(row) | {"item_names": _loads_json(row["item_names_json"], fallback=[])} for row in rows]


def _compact_learning_context(context: dict[str, Any]) -> dict[str, Any]:
    hero_context = context.get("hero_context") if isinstance(context.get("hero_context"), dict) else {}
    hero = hero_context.get("hero") if isinstance(hero_context.get("hero"), dict) else {}
    build = hero_context.get("build") if isinstance(hero_context.get("build"), dict) else {}
    return {
        "build": _compact_build_for_model(context.get("build")),
        "nearby_top_builds": context.get("nearby_top_builds"),
        "learning_goal": context.get("learning_goal"),
        "hero_understanding": {
            "name": hero.get("name"),
            "hero_type": hero.get("hero_type"),
            "role": hero.get("role"),
            "playstyle": hero.get("playstyle"),
            "inferred_gameplan": hero.get("inferred_gameplan"),
            "abilities": hero.get("abilities"),
            "sheet_hints": hero.get("sheet_hints"),
        },
        "deterministic_build_brain": {
            "plan": build.get("plan"),
            "early": build.get("early"),
            "core": build.get("core"),
            "late": build.get("late"),
            "situational": build.get("situational"),
            "shop_routes_to_4800": build.get("shop_routes_to_4800"),
        },
        "review_signals": hero_context.get("review_signals"),
    }


def _compact_build_for_model(build: Any) -> dict[str, Any]:
    if not isinstance(build, dict):
        return {}
    return {
        "id": build.get("id"),
        "source": build.get("source"),
        "source_build_id": build.get("source_build_id"),
        "hero_name": build.get("hero_name"),
        "language": build.get("language"),
        "source_rank": build.get("source_rank"),
        "quality_tier": build.get("quality_tier"),
        "quality_score": build.get("quality_score"),
        "name": build.get("name"),
        "description": build.get("description"),
        "tags": build.get("tags"),
        "item_names": build.get("item_names"),
        "item_categories": build.get("item_categories"),
        "ability_order": build.get("ability_order"),
        "source_metadata": build.get("source_metadata"),
    }


def _extract_item_categories(details: Any, item_map: dict[int, str]) -> list[dict[str, Any]]:
    if not isinstance(details, dict):
        return []
    rows = []
    for category in details.get("mod_categories") or details.get("modCategories") or []:
        if not isinstance(category, dict):
            continue
        mods = []
        for mod in category.get("mods") or []:
            if not isinstance(mod, dict):
                continue
            ability_id = _int_or_none(mod.get("ability_id") or mod.get("abilityId"))
            item_name = item_map.get(ability_id) if ability_id is not None else None
            if item_name:
                mods.append(
                    {
                        "item": item_name,
                        "annotation": mod.get("annotation"),
                        "sell_priority": mod.get("sell_priority") or mod.get("sellPriority"),
                    }
                )
        if mods:
            rows.append(
                {
                    "name": category.get("name"),
                    "description": category.get("description"),
                    "optional": bool(category.get("optional")),
                    "items": mods,
                }
            )
    return rows


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


def _table_exists(conn: sqlite3.Connection, name: str) -> bool:
    row = conn.execute("SELECT 1 FROM sqlite_master WHERE type='table' AND name=?", (name,)).fetchone()
    return row is not None


def _loads_json(value: Any, *, fallback: Any) -> Any:
    if isinstance(value, (dict, list)):
        return value
    try:
        return json.loads(str(value or ""))
    except json.JSONDecodeError:
        return fallback


def _int_or_none(value: Any) -> int | None:
    try:
        return int(float(str(value).strip()))
    except (TypeError, ValueError):
        return None


def _row_to_dict(row: Any) -> dict[str, Any]:
    if row is None:
        return {}
    if hasattr(row, "keys"):
        return {key: row[key] for key in row.keys()}
    return dict(row)
