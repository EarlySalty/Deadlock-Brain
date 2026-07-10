#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import shlex
import subprocess
from datetime import date
from pathlib import Path
from typing import Any
from urllib.parse import parse_qsl, unquote, urlparse

from mcp.server import FastMCP


SECRET_LOADER = Path("/home/naniadm/Documents/Infisical/export_claude_secret.py")
SECRET_NAME = "DEADLOCK_CENTRAL_DSN"
COLUMNS = """
    patch_title,
    patch_date,
    entity_type,
    entity_name,
    ability_name,
    stat_name,
    old_value,
    new_value,
    change_type,
    numeric_direction,
    raw_line,
    patch_url,
    confidence
"""

mcp = FastMCP(
    "dl-brain",
    instructions=(
        "Stellt die zentrale Postgres-Patch-Historie aus brain.patch_changes "
        "als read-only Tools bereit."
    ),
)


def _load_dsn_once() -> tuple[str | None, str | None]:
    try:
        result = subprocess.run(
            ["python3", str(SECRET_LOADER), "--secret", SECRET_NAME],
            capture_output=True,
            text=True,
            timeout=10,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None, f"{SECRET_NAME} konnte nicht geladen werden."

    if result.returncode != 0:
        return None, f"{SECRET_NAME} konnte nicht geladen werden."

    for line in result.stdout.splitlines():
        try:
            parts = shlex.split(line)
        except ValueError:
            continue
        if len(parts) == 2 and parts[0] == "export" and parts[1].startswith(f"{SECRET_NAME}="):
            value = parts[1].split("=", 1)[1]
            if value:
                return value, None
    return None, f"{SECRET_NAME} konnte nicht geladen werden."


_DSN, _DSN_ERROR = _load_dsn_once()


def _limit(value: int, default: int, cap: int) -> int:
    try:
        limit = int(value)
    except (TypeError, ValueError):
        return default
    return max(1, min(limit, cap))


def _require_text(value: str, name: str) -> str:
    text = (value or "").strip()
    if not text:
        raise ValueError(f"{name} darf nicht leer sein.")
    return text


def _require_date(value: str) -> str:
    text = _require_text(value, "since")
    try:
        date.fromisoformat(text)
    except ValueError as exc:
        raise ValueError("since muss ein ISO-Datum im Format YYYY-MM-DD sein.") from exc
    return text


def _query_rows(sql: str, variables: dict[str, Any] | None = None) -> list[dict[str, Any]]:
    if not _DSN:
        raise RuntimeError(f"Datenbank-Verbindung nicht verfuegbar: {_DSN_ERROR}")

    command = ["psql", "-X", "-q", "-tA", "-v", "ON_ERROR_STOP=1"]
    for key, value in (variables or {}).items():
        command.extend(["-v", f"{key}={value}"])
    input_sql = f"SELECT coalesce(json_agg(t), '[]'::json) FROM ({sql}) AS t;\n"

    env = _psql_env(_DSN, os.environ.copy())

    try:
        result = subprocess.run(
            command,
            input=input_sql,
            capture_output=True,
            text=True,
            timeout=12,
            env=env,
            check=False,
        )
    except FileNotFoundError as exc:
        raise RuntimeError("psql ist nicht installiert oder nicht im PATH.") from exc
    except subprocess.TimeoutExpired as exc:
        raise RuntimeError("Postgres-Abfrage hat das Zeitlimit ueberschritten.") from exc

    if result.returncode != 0:
        raise RuntimeError(f"Postgres-Abfrage fehlgeschlagen (psql exit {result.returncode}).")

    try:
        rows = json.loads(result.stdout.strip() or "[]")
    except json.JSONDecodeError as exc:
        raise RuntimeError("Postgres-Antwort war kein gueltiges JSON.") from exc
    if not isinstance(rows, list):
        raise RuntimeError("Postgres-Antwort hatte ein unerwartetes Format.")
    return rows


def _psql_env(dsn: str, base_env: dict[str, str]) -> dict[str, str]:
    env = dict(base_env)
    env["PGOPTIONS"] = "-c statement_timeout=8s -c default_transaction_read_only=on"

    parsed = urlparse(dsn)
    if parsed.scheme in {"postgres", "postgresql"}:
        if parsed.hostname:
            env["PGHOST"] = parsed.hostname
        if parsed.port:
            env["PGPORT"] = str(parsed.port)
        if parsed.username:
            env["PGUSER"] = unquote(parsed.username)
        if parsed.password:
            env["PGPASSWORD"] = unquote(parsed.password)
        if parsed.path and parsed.path != "/":
            env["PGDATABASE"] = unquote(parsed.path.lstrip("/"))
        for key, value in parse_qsl(parsed.query, keep_blank_values=True):
            if key == "sslmode":
                env["PGSSLMODE"] = value
            elif key == "connect_timeout":
                env["PGCONNECT_TIMEOUT"] = value
            elif key == "application_name":
                env["PGAPPNAME"] = value
        return env

    try:
        items = dict(part.split("=", 1) for part in shlex.split(dsn) if "=" in part)
    except ValueError:
        items = {}
    mapping = {
        "host": "PGHOST",
        "port": "PGPORT",
        "user": "PGUSER",
        "password": "PGPASSWORD",
        "dbname": "PGDATABASE",
        "sslmode": "PGSSLMODE",
        "connect_timeout": "PGCONNECT_TIMEOUT",
        "application_name": "PGAPPNAME",
    }
    for key, env_key in mapping.items():
        if items.get(key):
            env[env_key] = items[key]
    if not items:
        env["PGDATABASE"] = dsn
    return env


@mcp.tool(
    name="patch_history",
    description=(
        "Liest die Patch-Historie fuer eine Entity aus brain.patch_changes. "
        "entity ist ein exakter Name; ability/stat sind optionale ILIKE-Filter."
    ),
)
def patch_history(
    entity: str,
    ability: str | None = None,
    stat: str | None = None,
    since: str | None = None,
    limit: int = 100,
) -> list[dict[str, Any]]:
    """Patch-Historie fuer eine Entity, optional nach Ability, Stat und Datum gefiltert."""
    where = ["lower(entity_name) = lower(:'entity')"]
    variables: dict[str, Any] = {
        "entity": _require_text(entity, "entity"),
        "limit": _limit(limit, 100, 500),
    }
    if ability and ability.strip():
        where.append("ability_name ILIKE :'ability'")
        variables["ability"] = f"%{ability.strip()}%"
    if stat and stat.strip():
        where.append("stat_name ILIKE :'stat'")
        variables["stat"] = f"%{stat.strip()}%"
    if since:
        where.append("patch_date >= :'since'::date")
        variables["since"] = _require_date(since)

    sql = f"""
        SELECT *
        FROM (
            SELECT {COLUMNS}
            FROM brain.patch_changes
            WHERE {" AND ".join(where)}
            ORDER BY patch_date DESC, stat_name DESC
            LIMIT :limit
        ) AS recent_patch_changes
        ORDER BY patch_date, stat_name
    """
    return _query_rows(sql, variables)


@mcp.tool(
    name="patch_search",
    description="Sucht in raw_line und entity_name der zentralen Patch-Historie.",
)
def patch_search(text: str, limit: int = 50) -> list[dict[str, Any]]:
    """Freitextsuche in Patch-Zeilen und Entity-Namen."""
    variables = {
        "text": f"%{_require_text(text, 'text')}%",
        "limit": _limit(limit, 50, 500),
    }
    sql = f"""
        SELECT {COLUMNS}
        FROM brain.patch_changes
        WHERE raw_line ILIKE :'text' OR entity_name ILIKE :'text'
        ORDER BY patch_date DESC
        LIMIT :limit
    """
    return _query_rows(sql, variables)


@mcp.tool(
    name="list_patches",
    description="Listet bekannte Patches mit Datum und Titel, neueste zuerst.",
)
def list_patches(limit: int = 100) -> list[dict[str, Any]]:
    """Distinct Patch-Datum und Patch-Titel, absteigend sortiert."""
    sql = """
        SELECT DISTINCT patch_date, patch_title
        FROM brain.patch_changes
        ORDER BY patch_date DESC
        LIMIT :limit
    """
    return _query_rows(sql, {"limit": _limit(limit, 100, 500)})


@mcp.tool(
    name="entity_summary",
    description="Aggregiert Anzahl, ersten/letzten Patch sowie betroffene Abilities und Stats einer Entity.",
)
def entity_summary(entity: str) -> dict[str, Any]:
    """Kompakte Zusammenfassung der Patch-Aenderungen fuer eine Entity."""
    variables = {"entity": _require_text(entity, "entity")}
    sql = """
        SELECT
            :'entity' AS entity_name,
            count(*)::int AS change_count,
            min(patch_date) AS first_patch_date,
            max(patch_date) AS last_patch_date,
            coalesce(
                array_agg(DISTINCT ability_name ORDER BY ability_name)
                    FILTER (WHERE ability_name IS NOT NULL),
                ARRAY[]::text[]
            ) AS abilities,
            coalesce(
                array_agg(DISTINCT stat_name ORDER BY stat_name)
                    FILTER (WHERE stat_name IS NOT NULL),
                ARRAY[]::text[]
            ) AS stats
        FROM brain.patch_changes
        WHERE lower(entity_name) = lower(:'entity')
        LIMIT 1
    """
    rows = _query_rows(sql, variables)
    return rows[0]


if __name__ == "__main__":
    mcp.run()
