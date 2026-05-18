from __future__ import annotations

import hashlib
import json
import shutil
import sqlite3
import time
from collections.abc import Iterable
from pathlib import Path
from typing import Any


def stable_hash_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def stable_hash_text(content: str) -> str:
    return stable_hash_bytes(content.encode("utf-8"))


class BrainStore:
    def __init__(self, db_path: Path, raw_dir: Path) -> None:
        self.db_path = db_path
        self.raw_dir = raw_dir
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self.raw_dir.mkdir(parents=True, exist_ok=True)
        self.conn = sqlite3.connect(str(self.db_path))
        self.conn.row_factory = sqlite3.Row
        self.conn.execute("PRAGMA journal_mode=WAL")
        self.conn.execute("PRAGMA foreign_keys=ON")
        self.conn.execute("PRAGMA busy_timeout=15000")
        self.migrate()

    def close(self) -> None:
        self.conn.close()

    def migrate(self) -> None:
        self.conn.executescript(
            """
            CREATE TABLE IF NOT EXISTS source_documents (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              source TEXT NOT NULL,
              external_id TEXT NOT NULL,
              title TEXT,
              url TEXT,
              content_type TEXT NOT NULL,
              raw_path TEXT NOT NULL,
              content_hash TEXT NOT NULL,
              fetched_at INTEGER NOT NULL,
              metadata_json TEXT NOT NULL DEFAULT '{}',
              UNIQUE(source, external_id, content_hash)
            );

            CREATE INDEX IF NOT EXISTS idx_source_documents_source
              ON source_documents(source, external_id);

            CREATE TABLE IF NOT EXISTS entity_snapshots (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              source TEXT NOT NULL,
              entity_type TEXT NOT NULL,
              external_id TEXT NOT NULL,
              canonical_name TEXT,
              payload_hash TEXT NOT NULL,
              payload_json TEXT NOT NULL,
              fetched_at INTEGER NOT NULL,
              source_document_id INTEGER,
              UNIQUE(source, entity_type, external_id, payload_hash),
              FOREIGN KEY(source_document_id) REFERENCES source_documents(id)
            );

            CREATE INDEX IF NOT EXISTS idx_entity_snapshots_entity
              ON entity_snapshots(entity_type, canonical_name);

            CREATE TABLE IF NOT EXISTS source_runs (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              source TEXT NOT NULL,
              status TEXT NOT NULL,
              started_at INTEGER NOT NULL,
              finished_at INTEGER,
              summary_json TEXT NOT NULL DEFAULT '{}'
            );

            CREATE TABLE IF NOT EXISTS patch_events (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              patch_snapshot_id INTEGER NOT NULL,
              patch_external_id TEXT NOT NULL,
              patch_title TEXT,
              patch_url TEXT,
              source_kind TEXT NOT NULL,
              posted_at TEXT,
              line_index INTEGER NOT NULL,
              section TEXT,
              entity_type TEXT NOT NULL,
              entity_name TEXT,
              subject TEXT,
              change_type TEXT NOT NULL,
              raw_line TEXT NOT NULL,
              normalized_line TEXT NOT NULL,
              old_value TEXT,
              new_value TEXT,
              confidence REAL NOT NULL,
              metadata_json TEXT NOT NULL DEFAULT '{}',
              event_hash TEXT NOT NULL,
              created_at INTEGER NOT NULL,
              UNIQUE(event_hash),
              FOREIGN KEY(patch_snapshot_id) REFERENCES entity_snapshots(id)
            );

            CREATE INDEX IF NOT EXISTS idx_patch_events_entity
              ON patch_events(entity_type, entity_name);

            CREATE INDEX IF NOT EXISTS idx_patch_events_patch
              ON patch_events(patch_snapshot_id, line_index);

            CREATE INDEX IF NOT EXISTS idx_patch_events_source_kind
              ON patch_events(source_kind);

            CREATE TABLE IF NOT EXISTS entities (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              entity_type TEXT NOT NULL,
              canonical_name TEXT NOT NULL,
              primary_external_id TEXT,
              source TEXT NOT NULL,
              first_snapshot_id INTEGER,
              metadata_json TEXT NOT NULL DEFAULT '{}',
              created_at INTEGER NOT NULL,
              updated_at INTEGER NOT NULL,
              UNIQUE(entity_type, canonical_name),
              FOREIGN KEY(first_snapshot_id) REFERENCES entity_snapshots(id)
            );

            CREATE INDEX IF NOT EXISTS idx_entities_type_name
              ON entities(entity_type, canonical_name);

            CREATE TABLE IF NOT EXISTS entity_aliases (
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              entity_id INTEGER NOT NULL,
              alias TEXT NOT NULL,
              alias_norm TEXT NOT NULL,
              alias_kind TEXT NOT NULL,
              source TEXT NOT NULL,
              external_id TEXT,
              snapshot_id INTEGER,
              created_at INTEGER NOT NULL,
              UNIQUE(entity_id, alias_norm, alias_kind),
              FOREIGN KEY(entity_id) REFERENCES entities(id) ON DELETE CASCADE,
              FOREIGN KEY(snapshot_id) REFERENCES entity_snapshots(id)
            );

            CREATE INDEX IF NOT EXISTS idx_entity_aliases_norm
              ON entity_aliases(alias_norm);
            """
        )

        # Load sqlite-vec extension and create vector table
        try:
            import sqlite_vec
            self.conn.enable_load_extension(True)
            sqlite_vec.load(self.conn)
            self.conn.enable_load_extension(False)
            self.conn.executescript(
                """
                CREATE VIRTUAL TABLE IF NOT EXISTS vector_embeddings USING vec0(
                    embedding float[384]
                );

                CREATE TABLE IF NOT EXISTS mechanic_notes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    source TEXT NOT NULL,
                    category TEXT NOT NULL,
                    rowid INTEGER NOT NULL,
                    created_at INTEGER NOT NULL,
                    UNIQUE(title)
                );
                """
            )
        except Exception as e:
            print(f"Warnung: Konnte sqlite-vec nicht laden: {e}")
            self.conn.executescript(
                """
                CREATE TABLE IF NOT EXISTS mechanic_notes (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    source TEXT NOT NULL,
                    category TEXT NOT NULL,
                    rowid INTEGER NOT NULL,
                    created_at INTEGER NOT NULL,
                    UNIQUE(title)
                );
                """
            )
        self.conn.commit()

    def begin_run(self, source: str) -> int:
        cursor = self.conn.execute(
            "INSERT INTO source_runs(source, status, started_at) VALUES(?,?,?)",
            (source, "running", int(time.time())),
        )
        self.conn.commit()
        return int(cursor.lastrowid)

    def finish_run(self, run_id: int, *, status: str, summary: dict[str, Any]) -> None:
        self.conn.execute(
            """
            UPDATE source_runs
            SET status=?, finished_at=?, summary_json=?
            WHERE id=?
            """,
            (status, int(time.time()), json.dumps(summary, ensure_ascii=True), run_id),
        )
        self.conn.commit()

    def write_raw(
        self,
        *,
        source: str,
        external_id: str,
        content: bytes,
        suffix: str,
    ) -> Path:
        source_dir = self.raw_dir / source
        source_dir.mkdir(parents=True, exist_ok=True)
        digest = stable_hash_bytes(content)[:16]
        safe_external = "".join(ch if ch.isalnum() or ch in "-_." else "_" for ch in external_id)
        path = source_dir / f"{safe_external}.{digest}.{suffix.lstrip('.')}"
        if not path.exists():
            path.write_bytes(content)
        return path

    def copy_raw_file(
        self,
        *,
        source: str,
        external_id: str,
        path: Path,
        suffix: str,
    ) -> Path:
        content = path.read_bytes()
        target = self.write_raw(source=source, external_id=external_id, content=content, suffix=suffix)
        if target != path and not target.exists():
            shutil.copy2(path, target)
        return target

    def upsert_source_document(
        self,
        *,
        source: str,
        external_id: str,
        title: str | None,
        url: str | None,
        content_type: str,
        raw_path: Path,
        content: bytes,
        metadata: dict[str, Any] | None = None,
    ) -> int:
        content_hash = stable_hash_bytes(content)
        fetched_at = int(time.time())
        self.conn.execute(
            """
            INSERT OR IGNORE INTO source_documents(
              source, external_id, title, url, content_type, raw_path, content_hash, fetched_at, metadata_json
            )
            VALUES(?,?,?,?,?,?,?,?,?)
            """,
            (
                source,
                external_id,
                title,
                url,
                content_type,
                str(raw_path),
                content_hash,
                fetched_at,
                json.dumps(metadata or {}, ensure_ascii=True),
            ),
        )
        row = self.conn.execute(
            """
            SELECT id FROM source_documents
            WHERE source=? AND external_id=? AND content_hash=?
            ORDER BY id DESC LIMIT 1
            """,
            (source, external_id, content_hash),
        ).fetchone()
        self.conn.commit()
        return int(row["id"])

    def upsert_entity_snapshot(
        self,
        *,
        source: str,
        entity_type: str,
        external_id: str,
        canonical_name: str | None,
        payload: dict[str, Any],
        source_document_id: int | None = None,
    ) -> None:
        payload_json = json.dumps(payload, ensure_ascii=True, sort_keys=True)
        self.conn.execute(
            """
            INSERT OR IGNORE INTO entity_snapshots(
              source, entity_type, external_id, canonical_name, payload_hash,
              payload_json, fetched_at, source_document_id
            )
            VALUES(?,?,?,?,?,?,?,?)
            """,
            (
                source,
                entity_type,
                external_id,
                canonical_name,
                stable_hash_text(payload_json),
                payload_json,
                int(time.time()),
                source_document_id,
            ),
        )
        self.conn.commit()

    def latest_counts(self) -> list[sqlite3.Row]:
        return self.conn.execute(
            """
            SELECT source, COUNT(*) AS documents
            FROM source_documents
            GROUP BY source
            ORDER BY source
            """
        ).fetchall()

    def snapshot_counts(self) -> list[sqlite3.Row]:
        return self.conn.execute(
            """
            SELECT source, entity_type, COUNT(*) AS snapshots
            FROM entity_snapshots
            GROUP BY source, entity_type
            ORDER BY source, entity_type
            """
        ).fetchall()

    def patch_event_counts(self) -> list[sqlite3.Row]:
        return self.conn.execute(
            """
            SELECT source_kind, entity_type, COUNT(*) AS events
            FROM patch_events
            GROUP BY source_kind, entity_type
            ORDER BY source_kind, entity_type
            """
        ).fetchall()

    def clear_patch_events(self) -> int:
        if self.conn.execute(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='legacy_entities'"
        ).fetchone():
            self.conn.execute("DELETE FROM legacy_entities")
        if self.conn.execute(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='entity_lineage'"
        ).fetchone():
            self.conn.execute("DELETE FROM entity_lineage")
        if self.conn.execute(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='patch_event_enrichments'"
        ).fetchone():
            self.conn.execute("DELETE FROM patch_event_enrichments")
        cursor = self.conn.execute("DELETE FROM patch_events")
        self.conn.commit()
        return int(cursor.rowcount)

    def clear_entities(self) -> dict[str, int]:
        alias_cursor = self.conn.execute("DELETE FROM entity_aliases")
        entity_cursor = self.conn.execute("DELETE FROM entities")
        self.conn.commit()
        return {"aliases": int(alias_cursor.rowcount), "entities": int(entity_cursor.rowcount)}

    def upsert_entity(
        self,
        *,
        entity_type: str,
        canonical_name: str,
        primary_external_id: str | None,
        source: str,
        first_snapshot_id: int | None,
        metadata: dict[str, Any] | None = None,
    ) -> int:
        now = int(time.time())
        metadata_json = json.dumps(metadata or {}, ensure_ascii=True, sort_keys=True)
        self.conn.execute(
            """
            INSERT INTO entities(
              entity_type, canonical_name, primary_external_id, source,
              first_snapshot_id, metadata_json, created_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?)
            ON CONFLICT(entity_type, canonical_name) DO UPDATE SET
              primary_external_id=COALESCE(excluded.primary_external_id, entities.primary_external_id),
              first_snapshot_id=COALESCE(entities.first_snapshot_id, excluded.first_snapshot_id),
              metadata_json=excluded.metadata_json,
              updated_at=excluded.updated_at
            """,
            (
                entity_type,
                canonical_name,
                primary_external_id,
                source,
                first_snapshot_id,
                metadata_json,
                now,
                now,
            ),
        )
        row = self.conn.execute(
            """
            SELECT id FROM entities
            WHERE entity_type=? AND canonical_name=?
            """,
            (entity_type, canonical_name),
        ).fetchone()
        self.conn.commit()
        return int(row["id"])

    def upsert_entity_alias(
        self,
        *,
        entity_id: int,
        alias: str,
        alias_norm: str,
        alias_kind: str,
        source: str,
        external_id: str | None = None,
        snapshot_id: int | None = None,
    ) -> bool:
        cursor = self.conn.execute(
            """
            INSERT OR IGNORE INTO entity_aliases(
              entity_id, alias, alias_norm, alias_kind, source,
              external_id, snapshot_id, created_at
            )
            VALUES(?,?,?,?,?,?,?,?)
            """,
            (
                entity_id,
                alias,
                alias_norm,
                alias_kind,
                source,
                external_id,
                snapshot_id,
                int(time.time()),
            ),
        )
        self.conn.commit()
        return cursor.rowcount > 0

    def entity_counts(self) -> list[sqlite3.Row]:
        return self.conn.execute(
            """
            SELECT entity_type, COUNT(*) AS entities
            FROM entities
            GROUP BY entity_type
            ORDER BY entity_type
            """
        ).fetchall()

    def find_entities(
        self,
        *,
        entity_type: str | None = None,
        query: str | None = None,
        limit: int = 25,
    ) -> list[sqlite3.Row]:
        where = []
        params: list[Any] = []
        if entity_type:
            where.append("e.entity_type=?")
            params.append(entity_type)
        if query:
            like = f"%{query}%"
            where.append("(e.canonical_name LIKE ? OR a.alias LIKE ?)")
            params.extend([like, like])
        sql = """
            SELECT e.id, e.entity_type, e.canonical_name, e.primary_external_id,
                   GROUP_CONCAT(DISTINCT a.alias) AS aliases
            FROM entities e
            LEFT JOIN entity_aliases a ON a.entity_id=e.id
        """
        if where:
            sql += " WHERE " + " AND ".join(where)
        sql += """
            GROUP BY e.id
            ORDER BY e.entity_type, e.canonical_name
            LIMIT ?
        """
        params.append(max(1, min(limit, 500)))
        return self.conn.execute(sql, params).fetchall()

    def insert_patch_event(self, event: dict[str, Any]) -> bool:
        event_hash = event.get("event_hash")
        if not event_hash:
            hash_input = "|".join(
                str(event.get(key) or "")
                for key in (
                    "patch_snapshot_id",
                    "line_index",
                    "section",
                    "entity_type",
                    "entity_name",
                    "raw_line",
                )
            )
            event_hash = stable_hash_text(hash_input)
        cursor = self.conn.execute(
            """
            INSERT OR IGNORE INTO patch_events(
              patch_snapshot_id, patch_external_id, patch_title, patch_url,
              source_kind, posted_at, line_index, section, entity_type,
              entity_name, subject, change_type, raw_line, normalized_line,
              old_value, new_value, confidence, metadata_json, event_hash, created_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            """,
            (
                event["patch_snapshot_id"],
                event["patch_external_id"],
                event.get("patch_title"),
                event.get("patch_url"),
                event["source_kind"],
                event.get("posted_at"),
                event["line_index"],
                event.get("section"),
                event["entity_type"],
                event.get("entity_name"),
                event.get("subject"),
                event["change_type"],
                event["raw_line"],
                event["normalized_line"],
                event.get("old_value"),
                event.get("new_value"),
                float(event["confidence"]),
                json.dumps(event.get("metadata") or {}, ensure_ascii=True, sort_keys=True),
                event_hash,
                int(time.time()),
            ),
        )
        self.conn.commit()
        return cursor.rowcount > 0

    def insert_many_snapshots(
        self,
        snapshots: Iterable[dict[str, Any]],
        *,
        source_document_id: int | None = None,
    ) -> int:
        count = 0
        for snapshot in snapshots:
            self.upsert_entity_snapshot(
                source=snapshot["source"],
                entity_type=snapshot["entity_type"],
                external_id=snapshot["external_id"],
                canonical_name=snapshot.get("canonical_name"),
                payload=snapshot["payload"],
                source_document_id=source_document_id,
            )
            count += 1
        return count
