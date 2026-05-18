import json
import sqlite3
import time
from typing import Any

from deadlock_brain.minimax_client import MiniMaxConfig, call_minimax_chat, extract_minimax_text
from deadlock_brain.review_context import build_review_context
from deadlock_brain.storage import stable_hash_text

PROMPT_VERSION = "patch_impact_de_v1"

def ensure_patch_impact_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS patch_impact_notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          entity_type TEXT NOT NULL,
          entity_name TEXT NOT NULL,
          entity_id INTEGER,
          context_hash TEXT NOT NULL,
          prompt_version TEXT NOT NULL DEFAULT 'patch_impact_de_v1',
          prompt_text TEXT NOT NULL,
          result_text TEXT,
          insights_json TEXT NOT NULL DEFAULT '{}',
          model TEXT,
          status TEXT NOT NULL,
          patch_range_start TEXT,
          patch_range_end TEXT,
          event_count INTEGER,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE SET NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS idx_pin_unique 
          ON patch_impact_notes(entity_name, context_hash, prompt_version, COALESCE(model, ''));
        CREATE INDEX IF NOT EXISTS idx_pin_entity_name ON patch_impact_notes(entity_name);
        CREATE INDEX IF NOT EXISTS idx_pin_status ON patch_impact_notes(status);
        """
    )
    conn.commit()


def list_pending_patch_impact_targets(conn: sqlite3.Connection, *, limit: int = 10, model: str | None = None) -> list[dict[str, Any]]:
    ensure_patch_impact_tables(conn)
    cursor = conn.cursor()
    cursor.execute(
        """
        SELECT DISTINCT entity_name, entity_type FROM patch_events
        WHERE entity_name NOT IN (
            SELECT entity_name FROM patch_impact_notes WHERE status='analysis_ready'
        )
        LIMIT ?
        """,
        (limit,)
    )
    return [{"entity_name": row[0], "entity_type": row[1]} for row in cursor.fetchall()]


def build_patch_impact_context(conn: sqlite3.Connection, entity_name: str, entity_type: str) -> dict[str, Any]:
    context = build_review_context(conn, entity_name, limit_events=80)
    
    timeline_signals = context.get("timeline_signals", {})
    recent_events = timeline_signals.get("recent_events", [])
    
    # Filter to last 30 events
    timeline_signals["recent_events"] = recent_events[:30]
    
    patch_range = timeline_signals.get("patch_range", {})
    
    return {
        "entity_summary": context.get("entity_summary", {}),
        "timeline_signals": timeline_signals,
        "stat_changes": context.get("current_stat_hints", {}).get("stat_changes", []),
        "patch_range": patch_range
    }


def build_patch_impact_request(context: dict[str, Any], config: MiniMaxConfig) -> dict[str, Any]:
    entity_name = context.get("entity_summary", {}).get("canonical_name", "Unknown")
    
    prompt = f"Analysiere die Patch-Entwicklung für {entity_name} basierend auf den folgenden Daten.\n"
    prompt += "Gib NUR JSON aus mit folgendem Schema: {\"trend\": \"buffs_dominant\"|\"nerfs_dominant\"|\"mixed\"|\"stable\", \"affected_areas\": [\"...\"], \"momentum\": \"rising\"|\"falling\"|\"stable\", \"key_changes\": [\"...\"], \"confidence\": 0-1}\n\n"
    prompt += json.dumps(context, indent=2, ensure_ascii=False)
    
    req = {
        "model": config.model,
        "messages": [
            {"role": "user", "content": prompt}
        ],
        "max_completion_tokens": config.max_completion_tokens,
        "temperature": config.temperature,
        "top_p": config.top_p,
        "stream": False
    }
    return {"request": req, "prompt_text": prompt}


def save_patch_impact_note(conn: sqlite3.Connection, context: dict[str, Any], *, prompt_text: str, result_text: str | None, model: str, status: str) -> dict[str, Any]:
    ensure_patch_impact_tables(conn)
    
    entity_name = context.get("entity_summary", {}).get("canonical_name", "")
    entity_type = context.get("entity_summary", {}).get("entity_type", "")
    entity_id = context.get("entity_summary", {}).get("entity_id")
    
    patch_range_start = context.get("patch_range", {}).get("oldest")
    patch_range_end = context.get("patch_range", {}).get("newest")
    event_count = context.get("patch_range", {}).get("event_count", 0)
    
    context_hash = stable_hash_text(json.dumps(context, sort_keys=True))
    
    insights_json = "{}"
    if result_text and status == "analysis_ready":
        try:
            # Try to extract JSON if it's wrapped in markdown
            text = result_text
            if "```json" in text:
                text = text.split("```json")[1].split("```")[0].strip()
            elif "```" in text:
                text = text.split("```")[1].split("```")[0].strip()
            # Test parse
            json.loads(text)
            insights_json = text
        except json.JSONDecodeError:
            status = "analysis_failed"
    
    now = int(time.time())
    
    cursor = conn.cursor()
    
    cursor.execute(
        """
        INSERT INTO patch_impact_notes (
            entity_type, entity_name, entity_id, context_hash, prompt_version,
            prompt_text, result_text, insights_json, model, status,
            patch_range_start, patch_range_end, event_count, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(entity_name, context_hash, prompt_version, COALESCE(model, ''))
        DO UPDATE SET
            result_text = excluded.result_text,
            insights_json = excluded.insights_json,
            status = excluded.status,
            updated_at = excluded.updated_at
        """,
        (
            entity_type, entity_name, entity_id, context_hash, PROMPT_VERSION,
            prompt_text, result_text, insights_json, model, status,
            patch_range_start, patch_range_end, event_count, now, now
        )
    )
    conn.commit()
    
    return {
        "id": cursor.lastrowid,
        "entity_name": entity_name,
        "status": status,
        "hash": context_hash
    }


def run_patch_impact_batch(conn: sqlite3.Connection, config: MiniMaxConfig, *, limit: int = 10) -> dict[str, Any]:
    targets = list_pending_patch_impact_targets(conn, limit=limit, model=config.model)
    if not targets:
        return {"processed": 0, "success": 0, "failed": 0, "message": "no targets pending"}
        
    success = 0
    failed = 0
    
    from deadlock_brain.minimax_client import call_minimax_chat, extract_minimax_text
    
    for target in targets:
        entity_name = target["entity_name"]
        entity_type = target["entity_type"]
        
        context = build_patch_impact_context(conn, entity_name, entity_type)
        req_info = build_patch_impact_request(context, config)
        
        prompt_text = req_info["prompt_text"]
        request = req_info["request"]
        
        try:
            response = call_minimax_chat(request, config)
            result_text = extract_minimax_text(response)
            save_patch_impact_note(
                conn, context,
                prompt_text=prompt_text,
                result_text=result_text,
                model=config.model,
                status="analysis_ready"
            )
            success += 1
        except Exception as e:
            save_patch_impact_note(
                conn, context,
                prompt_text=prompt_text,
                result_text=str(e),
                model=config.model,
                status="analysis_failed"
            )
            failed += 1
            
    return {"processed": len(targets), "success": success, "failed": failed}
