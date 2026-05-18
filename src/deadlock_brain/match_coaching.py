import sqlite3
from typing import Any

from deadlock_brain.review_context import build_review_context

def build_match_coaching_context(conn: sqlite3.Connection, match_id: str, query: str) -> dict[str, Any]:
    """
    Builds context for analyzing a specific match to provide coaching feedback.
    In a real scenario, this would query match timelines from the database.
    Since we don't have the full match timeline DB schema yet, we mock a structural template.
    """
    
    # We pretend we have a match object with player performance, enemy team, and timeline
    # We can try to extract entity (hero) from the query to assume who the player was.
    context = {"query": query, "match_id": match_id}
    
    # Fallback to basic review context for the query if no specific match data is available yet
    # This ensures the AI still gets the basic hero rules if asked.
    base_context = build_review_context(conn, query, limit_events=20)
    
    # Mocked Match Data for illustration of Phase 3 capability
    context["match_data"] = {
        "status": "simulated",
        "timeline_events": [
            {"time_min": 5, "event": "First Blood (Died)", "enemy": "Infernus"},
            {"time_min": 10, "event": "Networth Difference: -1500", "cause": "Missed farm"},
            {"time_min": 15, "event": "Enemy Infernus bought Toxic Bullets"},
            {"time_min": 25, "event": "Player bought Debuff Reducer (Late)"}
        ],
        "heuristics": [
            "Warning: Debuff Reducer was bought 10 minutes after Infernus started snowballing.",
            "Warning: High death rate in early laning phase against Infernus."
        ]
    }
    
    context["entity_knowledge"] = base_context
    return context
