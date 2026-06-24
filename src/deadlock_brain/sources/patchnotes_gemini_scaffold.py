from __future__ import annotations

import html
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any


PROJECT_ROOT = Path(__file__).resolve().parents[3]
DEFAULT_WORKER_PATH = PROJECT_ROOT / "rust/python_worker/gemini_browser.py"
MAX_CHANGELOG_CHARS = 80_000

PATCHNOTE_ANALYSIS_PROMPT = """Review this Deadlock changelog text.

Extract concrete gameplay-relevant changes only: heroes, items, abilities, map,
objectives, economy, matchmaking, modes, fixes that affect gameplay, and meta-impact.
Ignore forum chrome, comments, signatures, reactions, unrelated links, and duplicate text.
Use concise bullet points and keep entity names exactly as written in the changelog.

Changelog metadata:
{metadata}

Changelog text:
{changelog_text}
"""

PATCHNOTE_JSON_CONVERT_PROMPT = (
    "Convert the analysis above into exactly this JSON object. Output only JSON, "
    "without Markdown or commentary:\n"
    '{"events":[{"entity":"name or null","entity_type":"hero|item|ability|map|objective|economy|mode|system|other",'
    '"change_type":"buff|nerf|rework|fix|addition|removal|tuning|other",'
    '"summary":"one concise sentence","old_value":null,"new_value":null,"confidence":0.0}]}\n'
    'If there are no gameplay-relevant changes, output {"events":[]}.'
)


def analyze_changelog_text(
    changelog_text: str,
    *,
    title: str | None = None,
    url: str | None = None,
    worker_path: Path | None = None,
    python_executable: str | None = None,
    timeout_seconds: int = 900,
) -> dict[str, Any]:
    """Minimal scaffold; TODO: wire only after review of the existing patchnotes path."""
    prompt = build_patchnote_prompt(changelog_text, title=title, url=url)
    worker_response = run_gemini_text_worker(
        prompt,
        convert_prompt=PATCHNOTE_JSON_CONVERT_PROMPT,
        worker_path=worker_path,
        python_executable=python_executable,
        timeout_seconds=timeout_seconds,
    )
    if worker_response.get("status") != "ok":
        return worker_response

    raw_text = str(worker_response.get("text") or "")
    parsed = extract_json_object(raw_text)
    if parsed is None:
        return {
            "status": "error",
            "kind": "invalid_json",
            "message": "worker returned no parseable JSON",
            "raw_text": raw_text,
        }
    return {"status": "ok", "analysis": parsed, "raw_text": raw_text}


def build_patchnote_prompt(changelog_text: str, *, title: str | None = None, url: str | None = None) -> str:
    metadata = json.dumps(
        {"title": title, "url": url},
        ensure_ascii=False,
        sort_keys=True,
    )
    return PATCHNOTE_ANALYSIS_PROMPT.format(
        metadata=metadata,
        changelog_text=clean_changelog_text(changelog_text),
    )


def run_gemini_text_worker(
    prompt: str,
    *,
    convert_prompt: str,
    worker_path: Path | None = None,
    python_executable: str | None = None,
    timeout_seconds: int = 900,
) -> dict[str, Any]:
    worker = worker_path or DEFAULT_WORKER_PATH
    python = python_executable or os.environ.get("GEMINI_PYTHON") or sys.executable
    payload = json.dumps(
        {"prompt": prompt, "convert_prompt": convert_prompt},
        ensure_ascii=False,
    ).encode("utf-8")
    result = subprocess.run(
        [python, str(worker), "analyze-text"],
        input=payload,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=timeout_seconds,
        check=False,
    )
    stdout = result.stdout.decode("utf-8", errors="replace").strip()
    stderr = result.stderr.decode("utf-8", errors="replace").strip()
    line = next((part for part in stdout.splitlines() if part.strip()), "")
    if not line:
        return {"status": "error", "kind": "unknown", "message": stderr or "empty worker output"}
    try:
        response = json.loads(line)
    except json.JSONDecodeError as exc:
        return {"status": "error", "kind": "unknown", "message": f"invalid worker json: {exc}"}
    if result.returncode != 0 and response.get("status") == "ok":
        return {
            "status": "error",
            "kind": "unknown",
            "message": f"worker exited {result.returncode}",
            "text": response.get("text"),
        }
    return response


def clean_changelog_text(changelog_text: str) -> str:
    text = html.unescape(str(changelog_text or ""))
    text = re.sub(r"<br\s*/?>", "\n", text, flags=re.IGNORECASE)
    text = re.sub(r"</(?:p|div|li|h[1-6])>", "\n", text, flags=re.IGNORECASE)
    text = re.sub(r"<[^>]+>", " ", text)
    text = re.sub(r"[ \t\r\f\v]+", " ", text)
    text = re.sub(r"\n\s+", "\n", text)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.strip()[:MAX_CHANGELOG_CHARS]


def extract_json_object(text: str) -> dict[str, Any] | None:
    stripped = str(text or "").strip()
    fenced = re.search(r"(?is)```(?:json)?\s*(.*?)```", stripped)
    if fenced:
        stripped = fenced.group(1).strip()
    for candidate in (stripped, _outer_json_slice(stripped)):
        if not candidate:
            continue
        try:
            parsed = json.loads(candidate)
        except json.JSONDecodeError:
            continue
        if isinstance(parsed, dict):
            return parsed
    return None


def _outer_json_slice(text: str) -> str | None:
    start = text.find("{")
    end = text.rfind("}")
    if start < 0 or end <= start:
        return None
    return text[start : end + 1]
