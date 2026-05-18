from __future__ import annotations

import json
import sqlite3
import time
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias


_HUMAN_HERO_ALIAS_KINDS = {"canonical", "snapshot_name"}

# Tabs mit eigener dedizierter Tabelle — alle anderen gehen in sheet_tab_rows
_DEDICATED_TABS = {
    "Heroes",
    "Hero meta ranking",
    "Boons/AP",
    "raw_hero_data",
    "raw_items_and_abilities",
    "shopBonuses",
}

_RANKING_COLUMN_MAP: dict[str, str] = {
    "carry": "carry",
    "crowd control": "crowd_control",
    "disengage": "disengage",
    "early": "early",
    "engage": "engage",
    "frontline": "frontline",
    "late": "late",
    "mid": "mid",
    "mid contest": "mid_contest",
    "mobility": "mobility",
    "nuke (phys)": "nuke_phys",
    "nuke (spirit)": "nuke_spirit",
    "pick": "pick",
    "poke": "poke",
    "support": "support",
    "wave clear": "wave_clear",
    "sustain dps (physical)": "sustain_dps_phys",
    "sustain dps (spirit)": "sustain_dps_spirit",
    "average rank": "average_rank",
}

_RAW_HERO_COLUMN_MAP: dict[str, str] = {
    "move speed": "move_speed",
    "sprint speed": "sprint_speed",
    "crouch speed": "crouch_speed",
    "move accel": "move_accel",
    "light melee dmg": "light_melee_dmg",
    "heavy melee dmg": "heavy_melee_dmg",
    "max hp": "max_hp",
    "base stamina": "base_stamina",
    "stam regen per second": "stam_regen",
    "hp regen": "hp_regen",
    "base health": "base_health",
    "gun growth": "gun_growth",
    "alt gun growth": "alt_gun_growth",
    "hp per boon": "hp_per_boon",
    "melee gain": "melee_gain",
    "spirit per boon": "spirit_per_boon",
}

_HEROES_STATS_MAP: dict[str, str] = {
    "Base HP": "base_hp",
    "Base Move Speed": "base_move_speed",
    "Base Sprint": "base_sprint",
    "Base Stamina": "base_stamina",
    "Base Regen": "base_regen",
    "Base Ammo": "base_ammo",
    "Pellets": "pellets",
    "Alt Fire Pellets": "alt_fire_pellets",
    "Base Bullet Dmg": "base_bullet_dmg",
    "Base Fire Rate": "base_fire_rate",
    "Base DPS": "base_dps",
    "Max Gun DPS": "max_gun_dps",
    "Max Gun Damage": "max_gun_damage",
    "DPM": "dpm",
    "Max DPM": "max_dpm",
    "Falloff Range Min": "falloff_range_min",
    "Falloff Range Max": "falloff_range_max",
    "HP Gain": "hp_gain",
    "Dmg Gain": "dmg_gain",
    "Spirit Gain": "spirit_gain",
    "Spirit Bonus": "spirit_bonus",
    "Spirit Bonus 2": "spirit_bonus_2",
    "Spirit Ratio": "spirit_ratio",
    "Spirit Ratio 2": "spirit_ratio_2",
    "Spirit Scaling": "spirit_scaling",
    "Spirit Scaling 2": "spirit_scaling_2",
    "aggregate growth %": "aggregate_growth_pct",
    "dps % growth increase": "dps_growth_pct",
    "hp % growth increase": "hp_growth_pct",
    "melee ratio": "melee_ratio",
    "total bullet ratio": "total_bullet_ratio",
    "total spirit ratio": "total_spirit_ratio",
    "Max Level HP": "max_level_hp",
}


def normalize_sheet_tabs(conn: sqlite3.Connection, rebuild: bool = False) -> dict[str, Any]:
    """Normalisiert alle Google-Sheet-Tabs in dedizierte und generische Tabellen."""
    _ensure_tables(conn)
    if rebuild:
        _clear_tables(conn)

    hero_index = _build_hero_index(conn)

    result = {
        "sheet_hero_rankings": _normalize_hero_rankings(conn, hero_index),
        "sheet_boons_ap": _normalize_boons_ap(conn),
        "sheet_raw_heroes": _normalize_raw_heroes(conn, hero_index),
        "sheet_heroes_stats": _normalize_heroes_stats(conn, hero_index),
        "sheet_items": _normalize_items(conn),
        "sheet_shop_bonuses": _normalize_shop_bonuses(conn),
        "sheet_tab_rows": _normalize_freeform_tabs(conn),
    }
    conn.commit()
    return result


# ── Heroes-Tab (Waffe, DPS, HP, Wachstumsraten) ─────────────────────────────

def _normalize_heroes_stats(conn: sqlite3.Connection, hero_index: dict[str, int]) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_hash, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'Heroes'
        ORDER BY id
        """
    ).fetchall()

    inserted = 0
    float_cols = list(_HEROES_STATS_MAP.values())
    col_list = ", ".join(
        ["snapshot_id", "entity_id", "hero_name", "alt_fire_type", "hero_labs"]
        + float_cols
        + ["payload_hash", "created_at", "updated_at"]
    )
    placeholders = ", ".join("?" for _ in range(5 + len(float_cols) + 3))
    update_set = ", ".join(
        [f"{c}=excluded.{c}" for c in
         ["entity_id", "hero_name", "alt_fire_type", "hero_labs"]
         + float_cols
         + ["payload_hash", "updated_at"]]
    )

    for snapshot_id, payload_hash, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        hero_name = str(values.get("Hero Name") or "").strip()
        if not hero_name or hero_name.casefold() in {"hero name", "hero labs"}:
            continue

        entity_id = hero_index.get(normalize_alias(hero_name))
        alt_fire_type = str(values.get("Alt Fire Type/Dmg") or "").strip() or None
        hero_labs = str(values.get("Hero Labs") or "").strip() or None
        now = int(time.time())

        float_vals = [_parse_float(str(values.get(label) or "").strip()) for label in _HEROES_STATS_MAP]

        conn.execute(
            f"""
            INSERT INTO sheet_heroes_stats({col_list})
            VALUES({placeholders})
            ON CONFLICT(snapshot_id) DO UPDATE SET {update_set}
            """,
            (snapshot_id, entity_id, hero_name, alt_fire_type, hero_labs,
             *float_vals, payload_hash, now, now),
        )
        inserted += 1

    return {"snapshots": len(rows), "inserted": inserted}


# ── Raw Items & Abilities ────────────────────────────────────────────────────

def _normalize_items(conn: sqlite3.Connection) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'raw_items_and_abilities'
        ORDER BY id
        """
    ).fetchall()

    inserted = 0
    for snapshot_id, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        code_name = str(values.get("code name") or "").strip()
        game_name = str(values.get("game name") or "").strip()
        item_id = _parse_int(str(values.get("id") or values.get("hero") or "").strip())
        if not code_name and not game_name:
            continue
        canonical = game_name or code_name
        now = int(time.time())

        conn.execute(
            """
            INSERT INTO sheet_items(snapshot_id, item_id, code_name, game_name, canonical_name, created_at, updated_at)
            VALUES(?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              item_id=excluded.item_id, code_name=excluded.code_name,
              game_name=excluded.game_name, canonical_name=excluded.canonical_name,
              updated_at=excluded.updated_at
            """,
            (snapshot_id, item_id, code_name, game_name, canonical, now, now),
        )
        inserted += 1

    return {"snapshots": len(rows), "inserted": inserted}


# ── Shop Bonuses ─────────────────────────────────────────────────────────────

def _normalize_shop_bonuses(conn: sqlite3.Connection) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'shopBonuses'
        ORDER BY id
        """
    ).fetchall()

    inserted = 0
    for snapshot_id, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        souls = _parse_int(str(values.get("Column 1") or values.get("Column 7") or "").strip())
        if souls is None or souls <= 0:
            continue
        weapon = _parse_int(str(values.get("Weapon") or "").strip())
        spirit = _parse_int(str(values.get("Spirit") or "").strip())
        vitality = _parse_int(str(values.get("Vitality") or "").strip())
        if weapon is None and spirit is None and vitality is None:
            continue
        inc_pct = _parse_float(str(values.get("inc from previous") or "").strip())
        now = int(time.time())

        conn.execute(
            """
            INSERT INTO sheet_shop_bonuses(snapshot_id, souls_cost, weapon, spirit, vitality, inc_from_prev_pct, created_at, updated_at)
            VALUES(?,?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              souls_cost=excluded.souls_cost, weapon=excluded.weapon,
              spirit=excluded.spirit, vitality=excluded.vitality,
              inc_from_prev_pct=excluded.inc_from_prev_pct, updated_at=excluded.updated_at
            """,
            (snapshot_id, souls, weapon, spirit, vitality, inc_pct, now, now),
        )
        inserted += 1

    return {"snapshots": len(rows), "inserted": inserted}


# ── Freeform-Tabs (damage calculator, Damage Comparison, ttk, Hidden Mechanics, …) ──

def _normalize_freeform_tabs(conn: sqlite3.Connection) -> dict[str, Any]:
    rows = conn.execute(
        """
        SELECT id, canonical_name, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') NOT IN ({})
        ORDER BY id
        """.format(",".join("?" for _ in _DEDICATED_TABS)),
        tuple(_DEDICATED_TABS),
    ).fetchall()

    counts_by_tab: dict[str, int] = {}
    for snapshot_id, canonical_name_raw, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        tab_name = str(payload.get("sheet_name") or "").strip()
        if not tab_name:
            continue
        gid = str(payload.get("gid") or "").strip()
        row_number = payload.get("row_number")
        canonical_name = str(canonical_name_raw or "").strip() or None
        values = payload.get("values") or {}
        row_json = json.dumps(values, ensure_ascii=False)
        now = int(time.time())

        conn.execute(
            """
            INSERT INTO sheet_tab_rows(snapshot_id, tab_name, gid, row_number, canonical_name, row_json, created_at, updated_at)
            VALUES(?,?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              tab_name=excluded.tab_name, gid=excluded.gid, row_number=excluded.row_number,
              canonical_name=excluded.canonical_name, row_json=excluded.row_json,
              updated_at=excluded.updated_at
            """,
            (snapshot_id, tab_name, gid, row_number, canonical_name, row_json, now, now),
        )
        counts_by_tab[tab_name] = counts_by_tab.get(tab_name, 0) + 1

    return {"total": sum(counts_by_tab.values()), "by_tab": counts_by_tab}


# ── Bereits vorhandene Normalizer (unverändert) ──────────────────────────────

def _normalize_hero_rankings(conn: sqlite3.Connection, hero_index: dict[str, int]) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_hash, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'Hero meta ranking'
        ORDER BY id
        """
    ).fetchall()

    inserted = 0
    for snapshot_id, payload_hash, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        hero_name = str(values.get("hero name") or values.get("Hero Name") or "").strip()
        if not hero_name:
            continue
        entity_id = hero_index.get(normalize_alias(hero_name))
        now = int(time.time())
        cols = {col: _parse_float(str(values.get(label) or values.get(label.title()) or "").strip())
                for label, col in _RANKING_COLUMN_MAP.items()}
        conn.execute(
            """
            INSERT INTO sheet_hero_rankings(
              snapshot_id, entity_id, hero_name,
              carry, crowd_control, disengage, early, engage, frontline,
              late, mid, mid_contest, mobility, nuke_phys, nuke_spirit,
              pick, poke, support, wave_clear, sustain_dps_phys, sustain_dps_spirit,
              average_rank, payload_hash, created_at, updated_at
            ) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              entity_id=excluded.entity_id, hero_name=excluded.hero_name,
              carry=excluded.carry, crowd_control=excluded.crowd_control,
              disengage=excluded.disengage, early=excluded.early, engage=excluded.engage,
              frontline=excluded.frontline, late=excluded.late, mid=excluded.mid,
              mid_contest=excluded.mid_contest, mobility=excluded.mobility,
              nuke_phys=excluded.nuke_phys, nuke_spirit=excluded.nuke_spirit,
              pick=excluded.pick, poke=excluded.poke, support=excluded.support,
              wave_clear=excluded.wave_clear, sustain_dps_phys=excluded.sustain_dps_phys,
              sustain_dps_spirit=excluded.sustain_dps_spirit,
              average_rank=excluded.average_rank, payload_hash=excluded.payload_hash,
              updated_at=excluded.updated_at
            """,
            (snapshot_id, entity_id, hero_name,
             cols["carry"], cols["crowd_control"], cols["disengage"],
             cols["early"], cols["engage"], cols["frontline"],
             cols["late"], cols["mid"], cols["mid_contest"],
             cols["mobility"], cols["nuke_phys"], cols["nuke_spirit"],
             cols["pick"], cols["poke"], cols["support"],
             cols["wave_clear"], cols["sustain_dps_phys"], cols["sustain_dps_spirit"],
             cols["average_rank"], payload_hash, now, now),
        )
        inserted += 1
    return {"snapshots": len(rows), "inserted": inserted}


def _normalize_boons_ap(conn: sqlite3.Connection) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'Boons/AP'
        ORDER BY id
        """
    ).fetchall()
    inserted = 0
    for snapshot_id, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        souls = _parse_int(str(values.get("Souls") or "").strip())
        if souls is None or souls <= 0:
            continue
        boons = _parse_int(str(values.get("Boons") or "").strip())
        ap = _parse_int(str(values.get("AP") or "").strip())
        note = str(values.get("Column 4") or "").strip() or None
        now = int(time.time())
        conn.execute(
            """
            INSERT INTO sheet_boons_ap(snapshot_id, souls, boons, ap, note, created_at, updated_at)
            VALUES(?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              souls=excluded.souls, boons=excluded.boons, ap=excluded.ap,
              note=excluded.note, updated_at=excluded.updated_at
            """,
            (snapshot_id, souls, boons, ap, note, now, now),
        )
        inserted += 1
    return {"snapshots": len(rows), "inserted": inserted}


def _normalize_raw_heroes(conn: sqlite3.Connection, hero_index: dict[str, int]) -> dict[str, int]:
    rows = conn.execute(
        """
        SELECT id, payload_hash, payload_json
        FROM entity_snapshots
        WHERE source='deadlock_stats_sheet'
          AND json_extract(payload_json, '$.sheet_name') = 'raw_hero_data'
        ORDER BY id
        """
    ).fetchall()
    inserted = 0
    for snapshot_id, payload_hash, payload_json in rows:
        try:
            payload = json.loads(payload_json)
        except (json.JSONDecodeError, TypeError):
            continue
        values: dict[str, str] = payload.get("values") or {}
        hero_name = str(values.get("name") or "").strip()
        if not hero_name:
            continue
        entity_id = hero_index.get(normalize_alias(hero_name))
        now = int(time.time())
        hero_id = _parse_int(str(values.get("id") or "").strip())
        disabled = _parse_bool(str(values.get("disabled") or "FALSE").strip())
        floats = {col: _parse_float(str(values.get(label) or "").strip())
                  for label, col in _RAW_HERO_COLUMN_MAP.items()}
        conn.execute(
            """
            INSERT INTO sheet_raw_heroes(
              snapshot_id, entity_id, hero_name, hero_id, disabled,
              move_speed, sprint_speed, crouch_speed, move_accel,
              light_melee_dmg, heavy_melee_dmg, max_hp, base_stamina,
              stam_regen, hp_regen, base_health, gun_growth, alt_gun_growth,
              hp_per_boon, melee_gain, spirit_per_boon,
              payload_hash, created_at, updated_at
            ) VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ON CONFLICT(snapshot_id) DO UPDATE SET
              entity_id=excluded.entity_id, hero_name=excluded.hero_name,
              hero_id=excluded.hero_id, disabled=excluded.disabled,
              move_speed=excluded.move_speed, sprint_speed=excluded.sprint_speed,
              crouch_speed=excluded.crouch_speed, move_accel=excluded.move_accel,
              light_melee_dmg=excluded.light_melee_dmg, heavy_melee_dmg=excluded.heavy_melee_dmg,
              max_hp=excluded.max_hp, base_stamina=excluded.base_stamina,
              stam_regen=excluded.stam_regen, hp_regen=excluded.hp_regen,
              base_health=excluded.base_health, gun_growth=excluded.gun_growth,
              alt_gun_growth=excluded.alt_gun_growth, hp_per_boon=excluded.hp_per_boon,
              melee_gain=excluded.melee_gain, spirit_per_boon=excluded.spirit_per_boon,
              payload_hash=excluded.payload_hash, updated_at=excluded.updated_at
            """,
            (snapshot_id, entity_id, hero_name, hero_id, disabled,
             floats.get("move_speed"), floats.get("sprint_speed"), floats.get("crouch_speed"),
             floats.get("move_accel"), floats.get("light_melee_dmg"), floats.get("heavy_melee_dmg"),
             floats.get("max_hp"), floats.get("base_stamina"), floats.get("stam_regen"),
             floats.get("hp_regen"), floats.get("base_health"), floats.get("gun_growth"),
             floats.get("alt_gun_growth"), floats.get("hp_per_boon"), floats.get("melee_gain"),
             floats.get("spirit_per_boon"), payload_hash, now, now),
        )
        inserted += 1
    return {"snapshots": len(rows), "inserted": inserted}


# ── Schema ───────────────────────────────────────────────────────────────────

def _ensure_tables(conn: sqlite3.Connection) -> None:
    float_col_defs = "\n".join(f"          {col} REAL," for col in _HEROES_STATS_MAP.values())
    conn.executescript(
        f"""
        CREATE TABLE IF NOT EXISTS sheet_hero_rankings (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          entity_id INTEGER,
          hero_name TEXT NOT NULL,
          carry REAL, crowd_control REAL, disengage REAL, early REAL,
          engage REAL, frontline REAL, late REAL, mid REAL, mid_contest REAL,
          mobility REAL, nuke_phys REAL, nuke_spirit REAL, pick REAL,
          poke REAL, support REAL, wave_clear REAL,
          sustain_dps_phys REAL, sustain_dps_spirit REAL,
          average_rank REAL,
          payload_hash TEXT NOT NULL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );
        CREATE INDEX IF NOT EXISTS idx_shr_entity ON sheet_hero_rankings(entity_id);
        CREATE INDEX IF NOT EXISTS idx_shr_name ON sheet_hero_rankings(hero_name);

        CREATE TABLE IF NOT EXISTS sheet_boons_ap (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          souls INTEGER NOT NULL, boons INTEGER, ap INTEGER, note TEXT,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_sba_souls ON sheet_boons_ap(souls);

        CREATE TABLE IF NOT EXISTS sheet_raw_heroes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          entity_id INTEGER,
          hero_name TEXT NOT NULL, hero_id INTEGER, disabled INTEGER NOT NULL DEFAULT 0,
          move_speed REAL, sprint_speed REAL, crouch_speed REAL, move_accel REAL,
          light_melee_dmg REAL, heavy_melee_dmg REAL, max_hp REAL,
          base_stamina REAL, stam_regen REAL, hp_regen REAL, base_health REAL,
          gun_growth REAL, alt_gun_growth REAL, hp_per_boon REAL,
          melee_gain REAL, spirit_per_boon REAL,
          payload_hash TEXT NOT NULL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );
        CREATE INDEX IF NOT EXISTS idx_srh_entity ON sheet_raw_heroes(entity_id);
        CREATE INDEX IF NOT EXISTS idx_srh_name ON sheet_raw_heroes(hero_name);

        CREATE TABLE IF NOT EXISTS sheet_heroes_stats (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          entity_id INTEGER,
          hero_name TEXT NOT NULL,
          alt_fire_type TEXT,
          hero_labs TEXT,
          {float_col_defs}
          payload_hash TEXT NOT NULL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );
        CREATE INDEX IF NOT EXISTS idx_shs_entity ON sheet_heroes_stats(entity_id);
        CREATE INDEX IF NOT EXISTS idx_shs_name ON sheet_heroes_stats(hero_name);

        CREATE TABLE IF NOT EXISTS sheet_items (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          item_id INTEGER,
          code_name TEXT NOT NULL,
          game_name TEXT NOT NULL,
          canonical_name TEXT NOT NULL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_si_code ON sheet_items(code_name);
        CREATE INDEX IF NOT EXISTS idx_si_game ON sheet_items(game_name);
        CREATE INDEX IF NOT EXISTS idx_si_canonical ON sheet_items(canonical_name);

        CREATE TABLE IF NOT EXISTS sheet_shop_bonuses (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          souls_cost INTEGER NOT NULL,
          weapon INTEGER, spirit INTEGER, vitality INTEGER,
          inc_from_prev_pct REAL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_ssb_souls ON sheet_shop_bonuses(souls_cost);

        CREATE TABLE IF NOT EXISTS sheet_tab_rows (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          snapshot_id INTEGER NOT NULL,
          tab_name TEXT NOT NULL,
          gid TEXT NOT NULL,
          row_number INTEGER,
          canonical_name TEXT,
          row_json TEXT NOT NULL,
          created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL,
          UNIQUE(snapshot_id),
          FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id) ON DELETE CASCADE
        );
        CREATE INDEX IF NOT EXISTS idx_str_tab ON sheet_tab_rows(tab_name);
        CREATE INDEX IF NOT EXISTS idx_str_canonical ON sheet_tab_rows(canonical_name);
        """
    )


def _clear_tables(conn: sqlite3.Connection) -> None:
    for table in (
        "sheet_hero_rankings", "sheet_boons_ap", "sheet_raw_heroes",
        "sheet_heroes_stats", "sheet_items", "sheet_shop_bonuses", "sheet_tab_rows",
    ):
        conn.execute(f"DELETE FROM {table}")


# ── Hilfsfunktionen ──────────────────────────────────────────────────────────

def _build_hero_index(conn: sqlite3.Connection) -> dict[str, int]:
    candidates: dict[str, set[int]] = {}
    for entity_id, canonical_name in conn.execute(
        "SELECT id, canonical_name FROM entities WHERE entity_type='hero'"
    ).fetchall():
        key = normalize_alias(str(canonical_name))
        if key:
            candidates.setdefault(key, set()).add(int(entity_id))
    for entity_id, _alias, alias_norm, alias_kind in conn.execute(
        """
        SELECT entity_id, alias, alias_norm, alias_kind
        FROM entity_aliases
        WHERE entity_id IN (SELECT id FROM entities WHERE entity_type='hero')
        """
    ).fetchall():
        if alias_kind not in _HUMAN_HERO_ALIAS_KINDS:
            continue
        key = str(alias_norm)
        if key:
            candidates.setdefault(key, set()).add(int(entity_id))
    return {k: next(iter(v)) for k, v in candidates.items() if len(v) == 1}


def _parse_float(value: str) -> float | None:
    cleaned = value.replace(",", "").replace("%", "").strip()
    if cleaned.startswith("+"):
        cleaned = cleaned[1:]
    try:
        return float(cleaned)
    except (ValueError, TypeError):
        return None


def _parse_int(value: str) -> int | None:
    f = _parse_float(value)
    return None if f is None else int(f)


def _parse_bool(value: str) -> bool:
    return value.strip().casefold() in {"true", "1", "yes"}
