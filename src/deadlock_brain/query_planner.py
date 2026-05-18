import json
import sqlite3
from dataclasses import dataclass, field
from typing import Any

from deadlock_brain.minimax_client import MiniMaxConfig, call_minimax_chat, extract_minimax_text

@dataclass
class QueryPlan:
    entities: list[dict[str, str]]
    threats: list[str]
    intent: str
    fetch: list[str]
    filters: dict[str, Any]
    raw_query: str
    language: str

INTENT_FETCH_MAP = {
    "hero_overview":        ["hero_stats", "hero_rankings", "patch_events", "patch_impact_notes"],
    "build_recommendation": ["builds", "build_notes", "item_wpa", "hero_stats", "patch_impact_notes"],
    "item_question":        ["item_data", "item_wpa", "patch_events"],
    "patch_changes":        ["patch_events", "enrichments", "patch_impact_notes"],
    "mechanics_question":   ["hidden_mechanics", "boons_ap", "shop_bonuses", "damage_calc"],
    "meta_question":        ["hero_rankings", "hero_stats", "patch_impact_notes"],
    "hero_comparison":      ["hero_stats", "hero_rankings", "patch_impact_notes"],
    "match_coaching":       ["match_data", "build_notes"],
}

def analyze_query(query: str, conn: sqlite3.Connection, config: MiniMaxConfig) -> QueryPlan:
    cursor = conn.cursor()
    
    # Get known heroes
    try:
        cursor.execute("SELECT canonical_name FROM entities WHERE entity_type='hero'")
        known_heroes = [row[0] for row in cursor.fetchall()]
    except Exception:
        known_heroes = []
        
    # Get known items
    try:
        cursor.execute("SELECT canonical_name FROM entities WHERE entity_type='item' LIMIT 60")
        known_items = [row[0] for row in cursor.fetchall()]
    except Exception:
        known_items = []
        
    system_prompt = f"""Du bist ein Query-Router für ein Deadlock-Analyse-System.
Analysiere die Anfrage und gib NUR gültiges JSON zurück, kein Text.

Bekannte Heroes: {", ".join(known_heroes)}
Bekannte Items: {", ".join(known_items)}
Bekannte Intents: {" | ".join(INTENT_FETCH_MAP.keys())}

JSON-Schema:
{{"entities":[{{"name":"...","type":"hero|item|general","raw":"..."}}],
 "threats": ["Gegner-Hero-Name", "..."],
 "intent":"...", "filters":{{}}, "language":"de|en"}}
"""

    # We enforce specific config overrides
    call_config = MiniMaxConfig(
        api_key=config.api_key,
        base_url=config.base_url,
        model=config.model,
        timeout_seconds=config.timeout_seconds,
        max_completion_tokens=400,
        temperature=0.1,
        top_p=config.top_p,
        use_token_plan=config.use_token_plan
    )
    
    req = {
        "model": call_config.model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": query}
        ],
        "max_completion_tokens": call_config.max_completion_tokens,
        "temperature": call_config.temperature,
        "top_p": call_config.top_p,
        "stream": False
    }
    
    try:
        response = call_minimax_chat(req, call_config)
        content = extract_minimax_text(response).strip()
        
        if "```json" in content:
            content = content.split("```json")[1].split("```")[0].strip()
        elif "```" in content:
            content = content.split("```")[1].split("```")[0].strip()
            
        data = json.loads(content)
        intent = data.get("intent", "hero_overview")
        
        # Validate intent
        if intent not in INTENT_FETCH_MAP:
            intent = "hero_overview"
            
        return QueryPlan(
            entities=data.get("entities", []),
            threats=data.get("threats", []),
            intent=intent,
            fetch=INTENT_FETCH_MAP[intent],
            filters=data.get("filters", {}),
            raw_query=query,
            language=data.get("language", "de")
        )
        
    except Exception:
        # Fallback simplistic deterministic routing
        intent = "hero_overview"
        entities = []
        threats = []
        
        query_lower = query.lower()
        for hero in known_heroes:
            if hero.lower() in query_lower:
                if len(entities) == 0:
                    entities.append({"name": hero, "type": "hero", "raw": hero})
                else:
                    threats.append(hero)
                
        if not entities:
            for item in known_items:
                if item.lower() in query_lower:
                    entities.append({"name": item, "type": "item", "raw": item})
                    intent = "item_question"
                    
        return QueryPlan(
            entities=entities,
            threats=threats,
            intent=intent,
            fetch=INTENT_FETCH_MAP[intent],
            filters={},
            raw_query=query,
            language="de"
        )
