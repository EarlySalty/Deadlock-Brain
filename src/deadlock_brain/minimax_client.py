from __future__ import annotations

import json
import re
import time
import urllib.error
import urllib.request
from dataclasses import dataclass
from typing import Any


DEFAULT_SYSTEM_PROMPT = (
    "Du bist ein Deadlock-Analyseassistent fuer einen deutschen Discord. "
    "Nutze ausschliesslich den bereitgestellten Kontext. Behalte Namen von "
    "Items, Heroes, Abilities, Stats und Quellen exakt auf Englisch. Schreibe "
    "die Analyse auf Deutsch. Markiere Unsicherheiten und fehlende Daten klar. "
    "Erfinde keine Winrates, Pickrates oder Patchdetails."
)


@dataclass(frozen=True)
class MiniMaxConfig:
    api_key: str | None
    base_url: str
    model: str
    timeout_seconds: int
    max_completion_tokens: int
    temperature: float
    top_p: float
    use_token_plan: bool = False


def build_minimax_review_request(review_context: dict[str, Any], config: MiniMaxConfig) -> dict[str, Any]:
    """Build an OpenAI-compatible MiniMax chat completion request."""
    compact_context = _compact_context_for_model(review_context)
    user_content = (
        f"{review_context.get('prompt_de') or ''}\n\n"
        "Erstelle die Antwort mit dieser Struktur:\n"
        "1. Kurzfazit\n"
        "2. Patch-Verlauf und relevante Reworks/Renames\n"
        "3. Aktuelle Einordnung anhand der Daten\n"
        "4. Build-/Gameplay-Implikationen\n"
        "5. Unsicherheiten / offene Punkte\n\n"
        "Review-Kontext als JSON:\n"
        f"{json.dumps(compact_context, ensure_ascii=False, sort_keys=True)}"
    )
    return {
        "model": config.model,
        "messages": [
            {"role": "system", "content": DEFAULT_SYSTEM_PROMPT},
            {"role": "user", "content": user_content},
        ],
        "max_completion_tokens": config.max_completion_tokens,
        "temperature": config.temperature,
        "top_p": config.top_p,
        "stream": False,
    }


def call_minimax_chat(request_payload: dict[str, Any], config: MiniMaxConfig) -> dict[str, Any]:
    if not config.api_key:
        raise RuntimeError("MiniMax API-Key fehlt. Setze MINIMAX_API_KEY oder MINIMAX_TOKEN_PLAN_KEY.")
    if config.use_token_plan:
        return _call_minimax_token_plan(request_payload, config)
    url = f"{config.base_url.rstrip('/')}/chat/completions"
    body = json.dumps(request_payload, ensure_ascii=False).encode("utf-8")
    request = urllib.request.Request(
        url,
        data=body,
        method="POST",
        headers={
            "Authorization": f"Bearer {config.api_key}",
            "Content-Type": "application/json",
            "User-Agent": "DeadlockBrain/0.1",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=config.timeout_seconds) as response:
            response_body = response.read().decode("utf-8", errors="replace")
            status = int(response.status)
    except urllib.error.HTTPError as exc:
        error_body = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"MiniMax HTTP {exc.code}: {error_body[:1000]}") from exc
    except urllib.error.URLError as exc:
        raise RuntimeError(f"MiniMax Request fehlgeschlagen: {exc}") from exc
    parsed = json.loads(response_body)
    parsed["_http_status"] = status
    return parsed


def _call_minimax_token_plan(request_payload: dict[str, Any], config: MiniMaxConfig) -> dict[str, Any]:
    url = f"{config.base_url.rstrip('/')}/messages"
    system_prompt, user_text = _messages_to_anthropic_parts(request_payload.get("messages"))
    body = json.dumps(
        {
            "model": request_payload.get("model") or config.model,
            "system": system_prompt,
            "messages": [{"role": "user", "content": [{"type": "text", "text": user_text}]}],
            "max_tokens": request_payload.get("max_completion_tokens") or config.max_completion_tokens,
            "temperature": request_payload.get("temperature", config.temperature),
        },
        ensure_ascii=False,
    ).encode("utf-8")
    request = urllib.request.Request(
        url,
        data=body,
        method="POST",
        headers={
            "x-api-key": config.api_key,
            "anthropic-version": "2023-06-01",
            "Content-Type": "application/json",
            "User-Agent": "DeadlockBrain/0.1",
        },
    )
    try:
        with urllib.request.urlopen(request, timeout=config.timeout_seconds) as response:
            response_body = response.read().decode("utf-8", errors="replace")
            status = int(response.status)
    except urllib.error.HTTPError as exc:
        error_body = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"MiniMax token-plan HTTP {exc.code}: {error_body[:1000]}") from exc
    except urllib.error.URLError as exc:
        raise RuntimeError(f"MiniMax token-plan request fehlgeschlagen: {exc}") from exc
    parsed = json.loads(response_body)
    parsed["_http_status"] = status
    parsed["_minimax_mode"] = "token_plan"
    return parsed


def extract_minimax_text(response: dict[str, Any]) -> str:
    content_items = response.get("content")
    if isinstance(content_items, list):
        fragments = []
        for item in content_items:
            if isinstance(item, dict) and item.get("type") == "text" and item.get("text"):
                fragments.append(str(item["text"]))
        if fragments:
            return strip_thinking("".join(fragments)).strip()
        if any(isinstance(item, dict) and item.get("type") == "thinking" for item in content_items):
            return ""
    choices = response.get("choices")
    if not isinstance(choices, list) or not choices:
        return ""
    message = choices[0].get("message") if isinstance(choices[0], dict) else None
    if not isinstance(message, dict):
        return ""
    content = str(message.get("content") or "")
    return strip_thinking(content).strip()


def strip_thinking(text: str) -> str:
    return re.sub(r"<think>.*?</think>", "", text, flags=re.IGNORECASE | re.DOTALL).strip()


def minimax_usage_summary(response: dict[str, Any]) -> dict[str, Any]:
    usage = response.get("usage") if isinstance(response.get("usage"), dict) else {}
    return {
        "id": response.get("id"),
        "model": response.get("model"),
        "http_status": response.get("_http_status"),
        "created": response.get("created") or int(time.time()),
        "usage": usage,
        "input_sensitive": response.get("input_sensitive"),
        "output_sensitive": response.get("output_sensitive"),
        "base_resp": response.get("base_resp"),
        "mode": response.get("_minimax_mode") or "openai_compatible",
    }


def _compact_context_for_model(review_context: dict[str, Any]) -> dict[str, Any]:
    timeline = dict(review_context.get("timeline_signals") or {})
    timeline["recent_events"] = list(timeline.get("recent_events") or [])[:18]
    timeline["stat_changes"] = list(timeline.get("stat_changes") or [])[:18]
    return {
        "query": review_context.get("query"),
        "entity_summary": review_context.get("entity_summary"),
        "lineage": review_context.get("lineage"),
        "current_stat_hints": review_context.get("current_stat_hints"),
        "timeline_signals": timeline,
        "open_questions": review_context.get("open_questions"),
        "source_references": list(review_context.get("source_references") or [])[:30],
        "retrieval_meta": review_context.get("retrieval_meta"),
    }


def _messages_to_anthropic_parts(messages: Any) -> tuple[str, str]:
    if not isinstance(messages, list):
        return "", ""
    system_parts = []
    user_parts = []
    for message in messages:
        if not isinstance(message, dict):
            continue
        role = str(message.get("role") or "")
        content = _stringify_message_content(message.get("content"))
        if not content:
            continue
        if role == "system":
            system_parts.append(content)
        else:
            user_parts.append(content)
    return "\n\n".join(system_parts), "\n\n".join(user_parts)


def _stringify_message_content(content: Any) -> str:
    if isinstance(content, str):
        return content
    if isinstance(content, list):
        parts = []
        for item in content:
            if isinstance(item, dict) and item.get("type") == "text":
                parts.append(str(item.get("text") or ""))
            elif isinstance(item, str):
                parts.append(item)
        return "\n".join(part for part in parts if part)
    return str(content or "")
