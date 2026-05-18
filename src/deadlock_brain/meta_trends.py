import json
import sqlite3
import time
from typing import Any

from deadlock_brain.minimax_client import MiniMaxConfig, call_minimax_chat, extract_minimax_text

def ensure_meta_trend_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS meta_trend_notes (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          entity_name TEXT NOT NULL,
          trend_direction TEXT NOT NULL,
          winrate_delta REAL NOT NULL,
          context_json TEXT NOT NULL DEFAULT '{}',
          result_text TEXT,
          status TEXT NOT NULL,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_mtn_entity_name ON meta_trend_notes(entity_name);
        """
    )
    conn.commit()


def run_meta_trend_analysis(conn: sqlite3.Connection, config: MiniMaxConfig) -> dict[str, Any]:
    """
    Simulates calculating winrate deltas and generating explanations for the biggest shifts.
    In reality, we would query the `sheet_heroes_stats` across different snapshots.
    """
    ensure_meta_trend_tables(conn)
    
    # We mock the detection of a meta shift (e.g. Abrams winrate drops by 3%)
    mock_shifts = [
        {"entity_name": "Abrams", "winrate_delta": -3.2, "trend_direction": "falling"},
        {"entity_name": "Infernus", "winrate_delta": 4.1, "trend_direction": "rising"}
    ]
    
    success = 0
    failed = 0
    
    for shift in mock_shifts:
        prompt = (
            f"Analysiere den Meta-Trend für {shift['entity_name']}. "
            f"Die Winrate hat sich um {shift['winrate_delta']}% verändert (Trend: {shift['trend_direction']}).\n"
            "Erkläre kurz, woran das liegen könnte (z.B. direkte Nerfs, System-Patches oder Buffs von Counter-Heroes)."
        )
        
        req = {
            "model": config.model,
            "messages": [
                {"role": "system", "content": "Du bist ein professioneller Deadlock Meta-Analyst."},
                {"role": "user", "content": prompt}
            ],
            "max_completion_tokens": config.max_completion_tokens,
            "temperature": config.temperature,
            "top_p": config.top_p,
            "stream": False
        }
        
        try:
            response = call_minimax_chat(req, config)
            result_text = extract_minimax_text(response)
            
            conn.execute(
                """
                INSERT INTO meta_trend_notes (
                    entity_name, trend_direction, winrate_delta, context_json, result_text, status, created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    shift["entity_name"], shift["trend_direction"], shift["winrate_delta"],
                    "{}", result_text, "analysis_ready", int(time.time()), int(time.time())
                )
            )
            conn.commit()
            success += 1
        except Exception as e:
            failed += 1
            print(f"Meta Trend Fehler für {shift['entity_name']}: {e}")
            
    return {"processed": len(mock_shifts), "success": success, "failed": failed}
