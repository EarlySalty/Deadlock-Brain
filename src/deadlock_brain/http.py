from __future__ import annotations

import hashlib
import json
import time
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class HttpResult:
    url: str
    content: bytes
    from_cache: bool
    content_type: str

    @property
    def text(self) -> str:
        return self.content.decode("utf-8", errors="replace")

    def json(self) -> Any:
        return json.loads(self.text)


class HttpClient:
    def __init__(self, *, user_agent: str, cache_dir: Path) -> None:
        self.user_agent = user_agent
        self.cache_dir = cache_dir
        self.cache_dir.mkdir(parents=True, exist_ok=True)

    def get(
        self,
        url: str,
        *,
        cache_ttl_seconds: int | None = None,
        timeout: int = 30,
        headers: dict[str, str] | None = None,
    ) -> HttpResult:
        cache_path = self._cache_path(url)
        meta_path = cache_path.with_suffix(cache_path.suffix + ".json")
        if cache_ttl_seconds is not None and cache_path.exists() and meta_path.exists():
            age = time.time() - cache_path.stat().st_mtime
            if age <= cache_ttl_seconds:
                meta = json.loads(meta_path.read_text(encoding="utf-8"))
                return HttpResult(
                    url=url,
                    content=cache_path.read_bytes(),
                    from_cache=True,
                    content_type=str(meta.get("content_type") or ""),
                )

        request_headers = {"User-Agent": self.user_agent}
        request_headers.update(headers or {})
        request = urllib.request.Request(url, headers=request_headers)
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                content = response.read()
                content_type = response.headers.get("content-type", "")
        except urllib.error.HTTPError as exc:
            raise RuntimeError(f"HTTP {exc.code} beim Laden von {url}") from exc

        cache_path.write_bytes(content)
        meta_path.write_text(
            json.dumps(
                {
                    "url": url,
                    "content_type": content_type,
                    "fetched_at": int(time.time()),
                },
                ensure_ascii=True,
                indent=2,
            ),
            encoding="utf-8",
        )
        return HttpResult(url=url, content=content, from_cache=False, content_type=content_type)

    def get_json(
        self,
        url: str,
        *,
        cache_ttl_seconds: int | None = None,
        headers: dict[str, str] | None = None,
    ) -> Any:
        return self.get(url, cache_ttl_seconds=cache_ttl_seconds, headers=headers).json()

    def _cache_path(self, url: str) -> Path:
        digest = hashlib.sha256(url.encode("utf-8")).hexdigest()
        return self.cache_dir / f"{digest}.bin"
