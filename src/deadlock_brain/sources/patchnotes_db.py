from __future__ import annotations

import json
import sqlite3
from pathlib import Path
from typing import Any

from deadlock_brain.storage import BrainStore


SOURCE = "deadlock_patchnotes_db"


def classify_source_kind(url: str | None) -> str:
    lower = (url or "").lower()
    if "steamcommunity.com" in lower or "steampowered.com" in lower or "steamstore-a.akamaihd.net" in lower:
        return "steam"
    if "forums.playdeadlock.com" in lower:
        return "forum"
    return "other"


def pull_patchnotes(store: BrainStore, *, central_db_path: Path) -> dict[str, Any]:
    if not central_db_path.exists():
        raise FileNotFoundError(f"Zentrale Deadlock-DB nicht gefunden: {central_db_path}")

    conn = sqlite3.connect(str(central_db_path))
    conn.row_factory = sqlite3.Row
    try:
        rows = conn.execute(
            """
            SELECT id, title, url, posted_at, raw_content, translated_content
            FROM changelog_posts
            WHERE raw_content IS NOT NULL AND raw_content != ''
            ORDER BY id ASC
            """
        ).fetchall()
    finally:
        conn.close()

    imported = 0
    source_kinds: dict[str, int] = {}
    for row in rows:
        external_id = str(row["url"] or row["id"])
        source_kind = classify_source_kind(row["url"])
        source_kinds[source_kind] = source_kinds.get(source_kind, 0) + 1
        payload = {
            "id": row["id"],
            "title": row["title"],
            "url": row["url"],
            "posted_at": row["posted_at"],
            "raw_content": row["raw_content"],
            "translated_content": row["translated_content"],
        }
        raw = json.dumps(payload, ensure_ascii=True, sort_keys=True).encode("utf-8")
        raw_path = store.write_raw(source=SOURCE, external_id=str(row["id"]), content=raw, suffix="json")
        document_id = store.upsert_source_document(
            source=SOURCE,
            external_id=external_id,
            title=row["title"],
            url=row["url"],
            content_type="application/json",
            raw_path=raw_path,
            content=raw,
            metadata={
                "central_db_path": str(central_db_path),
                "changelog_post_id": row["id"],
                "source_kind": source_kind,
            },
        )
        store.upsert_entity_snapshot(
            source=SOURCE,
            entity_type="patchnote",
            external_id=external_id,
            canonical_name=row["title"],
            payload=payload,
            source_document_id=document_id,
        )
        imported += 1
    return {"central_db_path": str(central_db_path), "patchnotes": imported, "source_kinds": source_kinds}
