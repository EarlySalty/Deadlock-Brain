from __future__ import annotations

import json
import sqlite3
import time
from typing import Any

from deadlock_brain.storage import stable_hash_text


PROMPT_VERSION = "review_context_de_v1"


def ensure_analysis_notes_table(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS analysis_notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          query TEXT NOT NULL,
          entity_type TEXT,
          entity_name TEXT,
          context_kind TEXT NOT NULL,
          context_hash TEXT NOT NULL,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          result_text TEXT,
          model TEXT,
          confidence REAL,
          status TEXT NOT NULL,
          source_references_json TEXT NOT NULL DEFAULT '[]',
          context_json TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE UNIQUE INDEX IF NOT EXISTS idx_analysis_notes_unique
          ON analysis_notes(query, context_hash, prompt_version, COALESCE(model, ''), status);

        CREATE INDEX IF NOT EXISTS idx_analysis_notes_entity
          ON analysis_notes(entity_type, entity_name);

        CREATE INDEX IF NOT EXISTS idx_analysis_notes_query
          ON analysis_notes(query, created_at);
        """
    )
    conn.commit()


def save_review_analysis_note(
    conn: sqlite3.Connection,
    review_context: dict[str, Any],
    *,
    result_text: str | None = None,
    model: str | None = None,
    confidence: float | None = None,
    status: str = "context_ready",
    provider_metadata: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Persist a review context and optional model result for later audit."""
    ensure_analysis_notes_table(conn)
    query = str(review_context.get("query") or "").strip()
    entity_summary = review_context.get("entity_summary") if isinstance(review_context.get("entity_summary"), dict) else {}
    context_json = json.dumps(review_context, ensure_ascii=True, sort_keys=True)
    context_hash = stable_hash_text(context_json)
    now = int(time.time())
    conn.execute(
        """
        INSERT INTO analysis_notes(
          query, entity_type, entity_name, context_kind, context_hash,
          prompt_version, prompt_text, result_text, model, confidence, status,
          source_references_json, context_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT DO UPDATE SET
          prompt_text=excluded.prompt_text,
          result_text=excluded.result_text,
          confidence=excluded.confidence,
          source_references_json=excluded.source_references_json,
          context_json=excluded.context_json,
          updated_at=excluded.updated_at
        """,
        (
            query,
            entity_summary.get("entity_type"),
            entity_summary.get("name"),
            str(review_context.get("context_kind") or "analysis_review"),
            context_hash,
            PROMPT_VERSION,
            str(review_context.get("prompt_de") or ""),
            result_text,
            model,
            confidence,
            status,
            json.dumps(_source_references(review_context, provider_metadata), ensure_ascii=True, sort_keys=True),
            context_json,
            now,
            now,
        ),
    )
    row = conn.execute(
        """
        SELECT id, query, entity_type, entity_name, context_hash, prompt_version,
               model, status, created_at, updated_at
        FROM analysis_notes
        WHERE query=? AND context_hash=? AND prompt_version=? AND COALESCE(model, '')=COALESCE(?, '') AND status=?
        ORDER BY id DESC
        LIMIT 1
        """,
        (query, context_hash, PROMPT_VERSION, model, status),
    ).fetchone()
    conn.commit()
    return _row_to_dict(row) if row else {"query": query, "context_hash": context_hash, "status": status}


def list_analysis_notes(conn: sqlite3.Connection, *, query: str | None = None, limit: int = 25) -> list[dict[str, Any]]:
    ensure_analysis_notes_table(conn)
    max_rows = max(1, min(int(limit), 500))
    params: list[Any] = []
    sql = """
        SELECT id, query, entity_type, entity_name, context_kind, context_hash,
               prompt_version, model, confidence, status, created_at, updated_at
        FROM analysis_notes
    """
    if query:
        sql += " WHERE query LIKE ? OR entity_name LIKE ?"
        params.extend([f"%{query}%", f"%{query}%"])
    sql += " ORDER BY updated_at DESC, id DESC LIMIT ?"
    params.append(max_rows)
    return [_row_to_dict(row) for row in conn.execute(sql, tuple(params)).fetchall()]


def _row_to_dict(row: Any) -> dict[str, Any]:
    if row is None:
        return {}
    if hasattr(row, "keys"):
        return {key: row[key] for key in row.keys()}
    return dict(row)


def _source_references(review_context: dict[str, Any], provider_metadata: dict[str, Any] | None) -> list[Any]:
    references = list(review_context.get("source_references") or [])
    if provider_metadata:
        references.append({"kind": "model_call", **provider_metadata})
    return references
