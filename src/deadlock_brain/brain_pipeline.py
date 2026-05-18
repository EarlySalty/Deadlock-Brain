import json
import sqlite3
from typing import Any

from deadlock_brain.analysis_notes import save_review_analysis_note
from deadlock_brain.build_optimizer import build_hero_build_context, build_item_context
from deadlock_brain.minimax_client import MiniMaxConfig, call_minimax_chat, extract_minimax_text
from deadlock_brain.query_planner import QueryPlan, analyze_query
from deadlock_brain.review_context import build_review_context

INTENT_PROMPT_SUFFIX = {
    "build_recommendation": "Fokus: konkrete Item-Empfehlung mit Begründung.",
    "mechanics_question":   "Fokus: präzise Regelerklärung aus den Daten.",
    "item_question":        "Fokus: Item-Stats, Kauf-Timing und welche Heroes davon profitieren.",
    "patch_changes":        "Fokus: Eine Zusammenfassung der wichtigsten Patch-Änderungen und deren Einfluss auf die Meta.",
    "hero_overview":        "Fokus: Stärken, Schwächen und grundlegende Spielweise des Heroes.",
    "meta_question":        "Fokus: Aktuelle Stellung des Heroes in der Meta basierend auf Stats und Rankings.",
    "hero_comparison":      "Fokus: Direkter Vergleich der Stats, Stärken und Spielphasen beider Heroes.",
    "match_coaching":       "Fokus: Analyse von Match-Daten zur Verbesserung des Gameplays."
}

def ask(query: str, conn: sqlite3.Connection, config: MiniMaxConfig) -> dict[str, Any]:
    plan = analyze_query(query, conn, config)
    context = _build_context(plan, conn)
    result_text = _generate_answer(context, plan, config)
    
    # Save review analysis note expects a query parameter, context requires a query key
    if "query" not in context:
        context["query"] = query
        
    try:
        save_review_analysis_note(conn, context, result_text=result_text, model=config.model, status="analysis_ready")
    except Exception as e:
        print(f"Warnung: save_review_analysis_note fehlgeschlagen: {e}")
        
    return {
        "plan": {
            "intent": plan.intent,
            "entities": plan.entities,
            "fetch": plan.fetch
        },
        "context_summary": {k: type(v).__name__ for k, v in context.items()},
        "answer": result_text,
        "model": config.model,
        "usage": {}
    }


def _build_context(plan: QueryPlan, conn: sqlite3.Connection) -> dict[str, Any]:
    intent = plan.intent
    entities = plan.entities
    
    context = {"query": plan.raw_query}
    
    if not entities:
        # Fallback to general context or mechanics
        context["data"] = "Keine spezifischen Entitäten erkannt."
    elif intent == "hero_overview":
        entity_name = entities[0]["name"]
        context.update(build_review_context(conn, entity_name))
    elif intent == "build_recommendation":
        entity_name = entities[0]["name"]
        try:
            context.update(build_hero_build_context(conn, entity_name, vs_heroes=plan.threats))
        except ValueError:
            context.update(build_review_context(conn, entity_name))
    elif intent == "item_question":
        entity_name = entities[0]["name"]
        try:
            context.update(build_item_context(conn, entity_name))
        except ValueError:
            context.update(build_review_context(conn, entity_name))
    elif intent == "mechanics_question":
        try:
            from deadlock_brain.retrieval import search_mechanic_notes
            notes = search_mechanic_notes(conn, plan.raw_query, limit=5)
            context["mechanics"] = notes
            if entities:
                context.update(build_review_context(conn, entities[0]["name"]))
        except Exception:
            pass
    elif intent == "patch_changes":
        entity_name = entities[0]["name"]
        context.update(build_review_context(conn, entity_name))
        # Keep mainly timeline_signals
    elif intent == "match_coaching":
        try:
            from deadlock_brain.match_coaching import build_match_coaching_context
            match_id = plan.filters.get("match_id", "simulated_123")
            context.update(build_match_coaching_context(conn, match_id, plan.raw_query))
        except Exception:
            pass
    elif intent == "hero_comparison" and len(entities) >= 2:
        context = {
            "query": plan.raw_query,
            "hero1": build_review_context(conn, entities[0]["name"]),
            "hero2": build_review_context(conn, entities[1]["name"])
        }
    else:
        entity_name = entities[0]["name"] if entities else "General"
        context.update(build_review_context(conn, entity_name))
        
    # Inject offline insights
    if entities:
        _inject_offline_insights(context, entities[0]["name"], conn)
        
    return context


def _inject_offline_insights(context: dict[str, Any], entity_name: str, conn: sqlite3.Connection) -> None:
    cursor = conn.cursor()
    insights = {}
    
    # Patch Impact Notes
    try:
        cursor.execute(
            "SELECT insights_json FROM patch_impact_notes WHERE entity_name = ? AND status = 'analysis_ready' ORDER BY updated_at DESC LIMIT 1",
            (entity_name,)
        )
        row = cursor.fetchone()
        if row:
            insights["patch_impact"] = json.loads(row[0])
    except Exception:
        pass
        
    # Build Learning Notes
    try:
        cursor.execute(
            "SELECT insights_json FROM build_learning_notes WHERE hero_name = ? AND status = 'analysis_ready' ORDER BY updated_at DESC LIMIT 1",
            (entity_name,)
        )
        row = cursor.fetchone()
        if row:
            insights["build_learning"] = json.loads(row[0])
    except Exception:
        pass
        
    # Meta Trend Notes
    try:
        cursor.execute(
            "SELECT result_text FROM meta_trend_notes WHERE entity_name = ? AND status = 'analysis_ready' ORDER BY updated_at DESC LIMIT 1",
            (entity_name,)
        )
        row = cursor.fetchone()
        if row:
            insights["meta_trend_analysis"] = row[0]
    except Exception:
        pass
        
    context["offline_insights"] = insights


def _generate_answer(context: dict[str, Any], plan: QueryPlan, config: MiniMaxConfig) -> str:
    compact_context = context
    try:
        from deadlock_brain.minimax_client import _compact_context_for_model
        if "hero1" in context and "hero2" in context:
            compact_context = {
                "query": context.get("query"),
                "hero1": _compact_context_for_model(context["hero1"]),
                "hero2": _compact_context_for_model(context["hero2"]),
                "offline_insights": context.get("offline_insights")
            }
        elif "build" in context or "top_items" in context:
            # It's a build_hero_build_context output
            compact_context = context.copy()
            if "top_items" in compact_context:
                compact_context["top_items"] = compact_context["top_items"][:10]
        elif "timeline_signals" in context:
            compact_context = _compact_context_for_model(context)
            compact_context["offline_insights"] = context.get("offline_insights")
    except Exception:
        pass
        
    prompt = f"Beantworte die folgende Frage basierend auf den bereitgestellten Daten.\n"
    suffix = INTENT_PROMPT_SUFFIX.get(plan.intent)
    if suffix:
        prompt += f"{suffix}\n"
    prompt += f"\nFrage: {plan.raw_query}\n\n"
    prompt += "Daten:\n"
    prompt += json.dumps(compact_context, indent=2, ensure_ascii=False)
    
    req = {
        "model": config.model,
        "messages": [
            {"role": "system", "content": "Du bist das Deadlock-Brain, ein AI-Assistent für das Spiel Deadlock. Antworte kompetent, präzise und hilfsbereit auf Deutsch."},
            {"role": "user", "content": prompt}
        ],
        "max_completion_tokens": config.max_completion_tokens,
        "temperature": config.temperature,
        "top_p": config.top_p,
        "stream": False
    }
    
    response = call_minimax_chat(req, config)
    result_text = extract_minimax_text(response).strip()
    return result_text
