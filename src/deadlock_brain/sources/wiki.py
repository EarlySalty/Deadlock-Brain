from __future__ import annotations

import json
import time
import urllib.parse
from pathlib import Path
from typing import Any

from deadlock_brain.http import HttpClient
from deadlock_brain.storage import BrainStore


SOURCE = "deadlock_wiki"
DEFAULT_API_URL = "https://deadlock.wiki/api.php"


class WikiRateLimiter:
    def __init__(self, state_path: Path, *, min_delay_seconds: float) -> None:
        self.state_path = state_path
        self.min_delay_seconds = min_delay_seconds
        self.state_path.parent.mkdir(parents=True, exist_ok=True)

    def wait(self) -> None:
        last = 0.0
        if self.state_path.exists():
            try:
                last = float(self.state_path.read_text(encoding="utf-8").strip() or "0")
            except ValueError:
                last = 0.0
        wait_for = self.min_delay_seconds - (time.time() - last)
        if wait_for > 0:
            time.sleep(wait_for)
        self.state_path.write_text(str(time.time()), encoding="utf-8")


def pull_wiki_page(
    store: BrainStore,
    http: HttpClient,
    *,
    title: str,
    enabled: bool,
    cache_ttl_seconds: int,
    min_delay_seconds: float,
    api_url: str = DEFAULT_API_URL,
) -> dict[str, Any]:
    if not enabled:
        raise RuntimeError(
            "Wiki-Netzwerkzugriff ist deaktiviert. Setze DEADLOCK_BRAIN_WIKI_ENABLED=1 "
            "oder nutze die CLI-Option --allow-wiki-network."
        )

    limiter = WikiRateLimiter(http.cache_dir / "wiki_last_request.txt", min_delay_seconds=min_delay_seconds)
    limiter.wait()
    params = urllib.parse.urlencode(
        {
            "action": "query",
            "format": "json",
            "prop": "extracts|revisions",
            "explaintext": "1",
            "rvprop": "ids|timestamp|content",
            "rvslots": "main",
            "titles": title,
            "redirects": "1",
        }
    )
    url = f"{api_url}?{params}"
    result = http.get(url, cache_ttl_seconds=cache_ttl_seconds, timeout=30)
    payload = json.loads(result.text)
    raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
    raw_path = store.write_raw(source=SOURCE, external_id=title, content=raw, suffix="json")
    document_id = store.upsert_source_document(
        source=SOURCE,
        external_id=title,
        title=title,
        url=url,
        content_type="application/json",
        raw_path=raw_path,
        content=raw,
        metadata={"from_cache": result.from_cache, "cache_ttl_seconds": cache_ttl_seconds},
    )
    store.upsert_entity_snapshot(
        source=SOURCE,
        entity_type="wiki_page",
        external_id=title,
        canonical_name=title,
        payload=payload,
        source_document_id=document_id,
    )
    pages = payload.get("query", {}).get("pages", {})
    return {
        "title": title,
        "from_cache": result.from_cache,
        "pages": len(pages) if isinstance(pages, dict) else 0,
    }

