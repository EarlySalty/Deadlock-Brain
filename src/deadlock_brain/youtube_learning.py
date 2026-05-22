from __future__ import annotations

import json
import re
import shutil
import sqlite3
import subprocess
import sys
import time
import urllib.parse
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Any

from deadlock_brain.entity_normalizer import normalize_alias
from deadlock_brain.http import HttpClient
from deadlock_brain.minimax_client import MiniMaxConfig, call_minimax_chat, extract_minimax_text, minimax_usage_summary
from deadlock_brain.storage import BrainStore, stable_hash_text


SOURCE = "youtube"
PROMPT_VERSION = "youtube_claims_de_v1"
TRANSCRIPT_MAX_CHARS = 24000

DEFAULT_FEEDS: list[dict[str, str]] = [
    {"type": "channel", "url": "https://www.youtube.com/@Deathy", "handle": "Deathy"},
    {"type": "channel", "url": "https://www.youtube.com/@DLMYTHBUSTERS", "handle": "DLMYTHBUSTERS"},
    {"type": "channel", "url": "https://www.youtube.com/@Midknighttxt", "handle": "Midknighttxt"},
    {
        "type": "playlist",
        "url": "https://www.youtube.com/watch?v=fZXYW9qcy24&list=PLRaixQ0u4jjmv-q9g1XUgO3G2ppTaSWpS",
        "playlist_id": "PLRaixQ0u4jjmv-q9g1XUgO3G2ppTaSWpS",
    },
    {"type": "channel", "url": "https://www.youtube.com/@MetroDeadlock", "handle": "MetroDeadlock"},
    {"type": "channel", "url": "https://www.youtube.com/@Zerggyyyy", "handle": "Zerggyyyy"},
    {"type": "channel", "url": "https://www.youtube.com/@poshypop", "handle": "poshypop"},
    {"type": "channel", "url": "https://www.youtube.com/@crayon_fps", "handle": "crayon_fps"},
    {"type": "channel", "url": "https://www.youtube.com/@piggyxdd", "handle": "piggyxdd"},
    {"type": "channel", "url": "https://www.youtube.com/@NatanDeadlock", "handle": "NatanDeadlock"},
    {"type": "channel", "url": "https://www.youtube.com/@Shiere", "handle": "Shiere"},
    {"type": "channel", "url": "https://www.youtube.com/@xlvi4804", "handle": "xlvi4804"},
    {"type": "channel", "url": "https://www.youtube.com/@Shyerow", "handle": "Shyerow"},
]


def ensure_youtube_tables(conn: sqlite3.Connection) -> None:
    conn.executescript(
        """
        CREATE TABLE IF NOT EXISTS youtube_feed_sources (
          feed_key TEXT PRIMARY KEY,
          source_type TEXT NOT NULL,
          url TEXT NOT NULL,
          handle TEXT,
          playlist_id TEXT,
          channel_id TEXT,
          title TEXT,
          enabled INTEGER NOT NULL DEFAULT 1,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS youtube_videos (
          video_id TEXT PRIMARY KEY,
          feed_key TEXT NOT NULL,
          channel_id TEXT,
          channel_title TEXT,
          title TEXT NOT NULL,
          url TEXT NOT NULL,
          published_at TEXT,
          description TEXT,
          metadata_json TEXT NOT NULL DEFAULT '{}',
          transcript_status TEXT NOT NULL DEFAULT 'missing',
          learning_status TEXT NOT NULL DEFAULT 'queued',
          discovered_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(feed_key) REFERENCES youtube_feed_sources(feed_key)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_videos_learning
          ON youtube_videos(learning_status, transcript_status, published_at);

        CREATE TABLE IF NOT EXISTS youtube_transcripts (
          video_id TEXT PRIMARY KEY,
          language TEXT,
          source_kind TEXT NOT NULL,
          transcript_text TEXT NOT NULL,
          content_hash TEXT NOT NULL,
          source_document_id INTEGER,
          imported_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id),
          FOREIGN KEY(source_document_id) REFERENCES source_documents(id)
        );

        CREATE TABLE IF NOT EXISTS youtube_learning_claims (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          video_id TEXT NOT NULL,
          claim_hash TEXT NOT NULL UNIQUE,
          claim_index INTEGER NOT NULL,
          entity_type TEXT,
          entity_name TEXT,
          claim_type TEXT NOT NULL,
          claim_text TEXT NOT NULL,
          evidence_quote TEXT NOT NULL,
          timestamp_seconds REAL,
          model_confidence REAL NOT NULL,
          verifier_confidence REAL NOT NULL,
          status TEXT NOT NULL,
          model TEXT,
          prompt_version TEXT NOT NULL,
          prompt_text TEXT NOT NULL,
          model_response_text TEXT NOT NULL,
          provider_metadata_json TEXT NOT NULL DEFAULT '{}',
          verifier_json TEXT NOT NULL DEFAULT '{}',
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          FOREIGN KEY(video_id) REFERENCES youtube_videos(video_id)
        );

        CREATE INDEX IF NOT EXISTS idx_youtube_learning_claims_video
          ON youtube_learning_claims(video_id, status);
        """
    )
    conn.commit()


def load_feed_config(path: Path | None = None) -> list[dict[str, str]]:
    if path is None or not path.exists():
        return list(DEFAULT_FEEDS)
    raw = json.loads(path.read_text(encoding="utf-8"))
    feeds = raw.get("feeds") if isinstance(raw, dict) else raw
    if not isinstance(feeds, list):
        raise ValueError(f"Ungueltige YouTube-Feed-Konfiguration: {path}")
    return _dedupe_feeds([_normalize_feed(item) for item in feeds if isinstance(item, dict)])


def write_default_feed_config(path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps({"feeds": DEFAULT_FEEDS}, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def discover_youtube_videos(
    store: BrainStore,
    http: HttpClient,
    *,
    config_path: Path | None = None,
    max_videos_per_feed: int = 50,
    cache_ttl_seconds: int = 21600,
) -> dict[str, Any]:
    ensure_youtube_tables(store.conn)
    feeds = load_feed_config(config_path)
    summary: dict[str, Any] = {"feeds": len(feeds), "videos_seen": 0, "inserted": 0, "updated": 0, "errors": []}
    for feed in feeds:
        feed_key = _feed_key(feed)
        _upsert_feed_source(store.conn, feed_key, feed)
        try:
            channel_id = feed.get("channel_id")
            if feed.get("type") == "channel":
                channel_id = channel_id or _stored_channel_id(store.conn, feed_key)
                channel_id = channel_id or _resolve_channel_id(http, feed, cache_ttl_seconds)
                _set_feed_channel_id(store.conn, feed_key, channel_id)
                rss_url = f"https://www.youtube.com/feeds/videos.xml?channel_id={urllib.parse.quote(channel_id)}"
            elif feed.get("type") == "playlist":
                playlist_id = str(feed.get("playlist_id") or "")
                rss_url = f"https://www.youtube.com/feeds/videos.xml?playlist_id={urllib.parse.quote(playlist_id)}"
            else:
                raise ValueError(f"Unbekannter Feed-Typ: {feed.get('type')}")

            result = http.get(rss_url, cache_ttl_seconds=cache_ttl_seconds, timeout=30)
            document_id = store.upsert_source_document(
                source=SOURCE,
                external_id=f"feed:{feed_key}",
                title=feed.get("handle") or feed.get("playlist_id") or feed_key,
                url=rss_url,
                content_type=result.content_type or "application/atom+xml",
                raw_path=store.write_raw(source=SOURCE, external_id=f"feed:{feed_key}", content=result.content, suffix="xml"),
                content=result.content,
                metadata={"feed": feed, "from_cache": result.from_cache},
            )
            videos = _parse_atom_videos(result.text, feed_key, rss_url)[: max(1, int(max_videos_per_feed))]
            for video in videos:
                action = _upsert_video(store.conn, video)
                summary["videos_seen"] += 1
                summary[action] += 1
                store.upsert_entity_snapshot(
                    source=SOURCE,
                    entity_type="youtube_video",
                    external_id=video["video_id"],
                    canonical_name=video["title"],
                    payload=video,
                    source_document_id=document_id,
                )
        except Exception as exc:
            summary["errors"].append({"feed": feed_key, "error": str(exc)})
    return summary


def import_transcript_directory(
    store: BrainStore,
    *,
    transcript_dir: Path,
    limit: int | None = None,
) -> dict[str, Any]:
    ensure_youtube_tables(store.conn)
    transcript_dir.mkdir(parents=True, exist_ok=True)
    files = sorted(path for path in transcript_dir.rglob("*") if path.is_file() and path.suffix.lower() in {".txt", ".vtt", ".srt", ".json"})
    if limit is not None:
        files = files[: max(0, int(limit))]
    summary = {"directory": str(transcript_dir), "files": len(files), "imported": 0, "skipped": 0, "errors": []}
    for path in files:
        try:
            parsed = _read_transcript_file(path)
            video_id = parsed.get("video_id") or _video_id_from_name(path.stem)
            text = str(parsed.get("text") or "").strip()
            if not video_id or not text:
                summary["skipped"] += 1
                continue
            _ensure_placeholder_video(store.conn, video_id)
            content = path.read_bytes()
            source_doc_id = store.upsert_source_document(
                source=SOURCE,
                external_id=f"transcript:{video_id}",
                title=f"YouTube transcript {video_id}",
                url=f"https://www.youtube.com/watch?v={video_id}",
                content_type=_content_type_for_suffix(path.suffix),
                raw_path=store.write_raw(source=SOURCE, external_id=f"transcript:{video_id}", content=content, suffix=path.suffix),
                content=content,
                metadata={"source_file": str(path), "language": parsed.get("language")},
            )
            now = int(time.time())
            conn = store.conn
            conn.execute(
                """
                INSERT INTO youtube_transcripts(
                  video_id, language, source_kind, transcript_text, content_hash,
                  source_document_id, imported_at, updated_at
                )
                VALUES(?,?,?,?,?,?,?,?)
                ON CONFLICT(video_id) DO UPDATE SET
                  language=excluded.language,
                  source_kind=excluded.source_kind,
                  transcript_text=excluded.transcript_text,
                  content_hash=excluded.content_hash,
                  source_document_id=excluded.source_document_id,
                  updated_at=excluded.updated_at
                """,
                (
                    video_id,
                    parsed.get("language") or "unknown",
                    path.suffix.lower().lstrip("."),
                    text,
                    stable_hash_text(text),
                    source_doc_id,
                    now,
                    now,
                ),
            )
            conn.execute(
                """
                UPDATE youtube_videos
                SET transcript_status='ready',
                    learning_status=CASE WHEN learning_status IN ('missing_transcript', 'queued') THEN 'queued' ELSE learning_status END,
                    updated_at=?
                WHERE video_id=?
                """,
                (now, video_id),
            )
            conn.commit()
            summary["imported"] += 1
        except Exception as exc:
            summary["errors"].append({"file": str(path), "error": str(exc)})
    return summary


def fetch_missing_transcripts_with_ytdlp(
    conn: sqlite3.Connection,
    *,
    transcript_dir: Path,
    limit: int = 20,
    languages: str = "original",
    order: str = "oldest",
) -> dict[str, Any]:
    ensure_youtube_tables(conn)
    transcript_dir.mkdir(parents=True, exist_ok=True)
    command_prefix = _ytdlp_command_prefix()
    if command_prefix is None:
        return {
            "available": False,
            "selected": 0,
            "downloaded": 0,
            "errors": [{"error": "yt-dlp is not installed"}],
        }
    order_sql = "COALESCE(published_at, '') ASC, discovered_at ASC" if order == "oldest" else "COALESCE(published_at, '') DESC, discovered_at DESC"
    rows = conn.execute(
        f"""
        SELECT video_id, title, url
        FROM youtube_videos
        WHERE video_id NOT IN (SELECT video_id FROM youtube_transcripts)
          AND transcript_status NOT IN ('unavailable')
        ORDER BY {order_sql}
        LIMIT ?
        """,
        (max(1, int(limit)),),
    ).fetchall()
    summary: dict[str, Any] = {"available": True, "selected": len(rows), "downloaded": 0, "errors": []}
    before = {path.name for path in transcript_dir.iterdir() if path.is_file()}
    for row in rows:
        url = str(row["url"] or f"https://www.youtube.com/watch?v={row['video_id']}")
        subtitle_languages = _subtitle_languages_for_video(command_prefix, url, languages)
        output_template = str(transcript_dir.resolve() / "%(id)s.%(ext)s")
        cmd = [
            *command_prefix,
            "--skip-download",
            "--write-subs",
            "--write-auto-subs",
            "--sub-langs",
            subtitle_languages,
            "--sub-format",
            "vtt",
            "--no-playlist",
            "--output",
            output_template,
            url,
        ]
        result = subprocess.run(cmd, cwd=str(transcript_dir), text=True, capture_output=True, timeout=180)
        after = {path.name for path in transcript_dir.iterdir() if path.is_file()}
        created = sorted(after - before)
        before = after
        matching_created = [name for name in created if row["video_id"] in name]
        if result.returncode == 0 and matching_created:
            summary["downloaded"] += len(matching_created)
            continue
        if result.returncode == 0:
            summary["errors"].append({"video_id": row["video_id"], "title": row["title"], "error": "no subtitles found"})
            _set_video_transcript_status(conn, row["video_id"], "unavailable", "missing_transcript")
        else:
            stderr = (result.stderr or result.stdout or "").strip()
            summary["errors"].append({"video_id": row["video_id"], "title": row["title"], "error": stderr[-1000:]})
    return summary


def transcribe_missing_videos_locally(
    store: BrainStore,
    *,
    audio_dir: Path,
    limit: int = 3,
    model_size: str = "base",
    device: str = "auto",
    compute_type: str = "int8",
    order: str = "oldest",
    keep_audio: bool = False,
    download_timeout_seconds: int = 900,
) -> dict[str, Any]:
    ensure_youtube_tables(store.conn)
    audio_dir.mkdir(parents=True, exist_ok=True)
    command_prefix = _ytdlp_command_prefix()
    if command_prefix is None:
        return {"available": False, "selected": 0, "transcribed": 0, "errors": [{"error": "yt-dlp is not installed"}]}
    try:
        from faster_whisper import WhisperModel
    except Exception as exc:
        return {
            "available": False,
            "selected": 0,
            "transcribed": 0,
            "errors": [{"error": f"faster-whisper is not installed or not importable: {exc}"}],
        }

    order_sql = "COALESCE(published_at, '') ASC, discovered_at ASC" if order == "oldest" else "COALESCE(published_at, '') DESC, discovered_at DESC"
    rows = store.conn.execute(
        f"""
        SELECT video_id, title, url
        FROM youtube_videos
        WHERE video_id NOT IN (SELECT video_id FROM youtube_transcripts)
        ORDER BY {order_sql}
        LIMIT ?
        """,
        (max(1, int(limit)),),
    ).fetchall()
    summary: dict[str, Any] = {
        "available": True,
        "selected": len(rows),
        "transcribed": 0,
        "model_size": model_size,
        "device": device,
        "compute_type": compute_type,
        "errors": [],
    }
    if not rows:
        return summary

    whisper_device = "cpu" if device == "auto" else device
    model = WhisperModel(model_size, device=whisper_device, compute_type=compute_type)
    for row in rows:
        video_id = str(row["video_id"])
        url = str(row["url"] or f"https://www.youtube.com/watch?v={video_id}")
        try:
            audio_path = _download_youtube_audio(
                command_prefix,
                url=url,
                video_id=video_id,
                audio_dir=audio_dir,
                timeout_seconds=download_timeout_seconds,
            )
            segments, info = model.transcribe(str(audio_path), vad_filter=True)
            transcript_text = _segments_to_transcript_text(segments)
            if not transcript_text.strip():
                summary["errors"].append({"video_id": video_id, "title": row["title"], "error": "local ASR produced empty transcript"})
                _set_video_transcript_status(store.conn, video_id, "unavailable", "missing_transcript")
                continue
            _save_transcript_text(
                store,
                video_id=video_id,
                text=transcript_text,
                language=str(getattr(info, "language", "") or "unknown"),
                source_kind="local_asr",
                metadata={
                    "asr": "faster-whisper",
                    "model_size": model_size,
                    "device": whisper_device,
                    "compute_type": compute_type,
                    "duration": float(getattr(info, "duration", 0.0) or 0.0),
                    "audio_path": str(audio_path) if keep_audio else None,
                },
            )
            if not keep_audio:
                audio_path.unlink(missing_ok=True)
            summary["transcribed"] += 1
        except Exception as exc:
            summary["errors"].append({"video_id": video_id, "title": row["title"], "error": str(exc)[-1000:]})
    return summary


def analyze_next_youtube_videos(
    store: BrainStore,
    config: MiniMaxConfig,
    *,
    limit: int = 5,
    dry_run: bool = False,
    delay_seconds: float = 1.0,
) -> dict[str, Any]:
    ensure_youtube_tables(store.conn)
    rows = store.conn.execute(
        """
        SELECT v.*, t.transcript_text, t.language
        FROM youtube_videos v
        JOIN youtube_transcripts t ON t.video_id=v.video_id
        WHERE v.transcript_status='ready'
          AND v.video_id NOT IN (
            SELECT video_id FROM youtube_learning_claims
            WHERE prompt_version=? AND COALESCE(model, '')=COALESCE(?, '')
          )
        ORDER BY COALESCE(v.published_at, '') DESC, v.discovered_at DESC
        LIMIT ?
        """,
        (PROMPT_VERSION, config.model, max(1, int(limit))),
    ).fetchall()
    summary: dict[str, Any] = {"selected": len(rows), "processed": 0, "claims_saved": 0, "dry_run": dry_run, "videos": []}
    for index, row in enumerate(rows):
        video = dict(row)
        request_payload, prompt_text = build_minimax_youtube_claim_request(video, video["transcript_text"], config)
        if dry_run:
            summary["videos"].append(
                {
                    "video_id": video["video_id"],
                    "title": video["title"],
                    "model": config.model,
                    "api_key_present": bool(config.api_key),
                    "request": request_payload,
                }
            )
            continue
        response = call_minimax_chat(request_payload, config)
        response_text = extract_minimax_text(response)
        claims = _parse_model_claims(response_text)
        saved = _save_verified_claims(
            store.conn,
            video,
            claims,
            prompt_text=prompt_text,
            model_response_text=response_text,
            provider_metadata=minimax_usage_summary(response),
            model=config.model,
        )
        _set_video_learning_status(store.conn, video["video_id"], "claims_ready" if saved else "no_claims")
        summary["processed"] += 1
        summary["claims_saved"] += saved
        summary["videos"].append({"video_id": video["video_id"], "title": video["title"], "claims_saved": saved})
        if delay_seconds > 0 and index + 1 < len(rows):
            time.sleep(delay_seconds)
    return summary


def run_youtube_auto_learning(
    store: BrainStore,
    http: HttpClient,
    config: MiniMaxConfig,
    *,
    config_path: Path | None,
    transcript_dir: Path,
    discover_limit: int,
    analyze_limit: int,
    fetch_transcripts: bool,
    transcript_fetch_limit: int,
    transcript_languages: str,
    local_transcribe: bool,
    local_transcribe_limit: int,
    audio_dir: Path,
    asr_model_size: str,
    asr_device: str,
    asr_compute_type: str,
    keep_audio: bool,
    dry_run: bool,
    cache_ttl_seconds: int,
    delay_seconds: float = 1.0,
) -> dict[str, Any]:
    discover = discover_youtube_videos(
        store,
        http,
        config_path=config_path,
        max_videos_per_feed=discover_limit,
        cache_ttl_seconds=cache_ttl_seconds,
    )
    if fetch_transcripts and not dry_run:
        fetched = fetch_missing_transcripts_with_ytdlp(
            store.conn,
            transcript_dir=transcript_dir,
            limit=transcript_fetch_limit,
            languages=transcript_languages,
        )
    else:
        fetched = {"skipped": True, "reason": "disabled_or_dry_run"}
    imported = import_transcript_directory(store, transcript_dir=transcript_dir)
    if local_transcribe and not dry_run:
        local_asr = transcribe_missing_videos_locally(
            store,
            audio_dir=audio_dir,
            limit=local_transcribe_limit,
            model_size=asr_model_size,
            device=asr_device,
            compute_type=asr_compute_type,
            keep_audio=keep_audio,
        )
    else:
        local_asr = {"skipped": True, "reason": "disabled_or_dry_run"}
    marked = mark_missing_transcripts(store.conn)
    analyzed = analyze_next_youtube_videos(store, config, limit=analyze_limit, dry_run=dry_run, delay_seconds=delay_seconds)
    return {
        "discover": discover,
        "transcript_fetch": fetched,
        "transcripts": imported,
        "local_asr": local_asr,
        "marked_missing_transcripts": marked,
        "analysis": analyzed,
    }


def list_youtube_queue(conn: sqlite3.Connection, *, status: str | None = None, limit: int = 25) -> list[dict[str, Any]]:
    ensure_youtube_tables(conn)
    params: list[Any] = []
    where = ""
    if status:
        where = "WHERE learning_status=?"
        params.append(status)
    params.append(max(1, int(limit)))
    rows = conn.execute(
        f"""
        SELECT v.video_id, v.title, v.channel_title, v.published_at, v.transcript_status, v.learning_status, v.url,
               COUNT(c.id) AS claims
        FROM youtube_videos v
        LEFT JOIN youtube_learning_claims c ON c.video_id=v.video_id
        {where}
        GROUP BY v.video_id
        ORDER BY COALESCE(v.published_at, '') DESC, v.discovered_at DESC
        LIMIT ?
        """,
        params,
    ).fetchall()
    return [dict(row) for row in rows]


def mark_missing_transcripts(conn: sqlite3.Connection) -> int:
    ensure_youtube_tables(conn)
    now = int(time.time())
    cursor = conn.execute(
        """
        UPDATE youtube_videos
        SET transcript_status='missing', learning_status='missing_transcript', updated_at=?
        WHERE video_id NOT IN (SELECT video_id FROM youtube_transcripts)
          AND learning_status='queued'
        """,
        (now,),
    )
    conn.commit()
    return int(cursor.rowcount)


def build_minimax_youtube_claim_request(
    video: dict[str, Any],
    transcript_text: str,
    config: MiniMaxConfig,
) -> tuple[dict[str, Any], str]:
    transcript = _clip_text(transcript_text, TRANSCRIPT_MAX_CHARS)
    system = (
        "Du bist MiniMax im Deadlock-Brain-Lernprozess. Deine Aufgabe ist nur die erste Sichtung: "
        "erkenne, ob aus dem YouTube-Transkript wertvolle Deadlock-Lernsignale extrahierbar sind. "
        "Erfinde nichts. Nutze nur Video-Metadaten und Transkript. Gib ausschliesslich JSON zurueck."
    )
    prompt = (
        "Extrahiere konkrete, spaeter pruefbare Claims aus diesem Deadlock-Video. "
        "Ein Claim ist wertvoll, wenn er eine spielrelevante Aussage zu Hero, Item, Ability, Macro, Laning, Build, Timing, Counterplay "
        "oder Entscheidungslogik enthaelt. Ignoriere Smalltalk, Sponsor, reine Meinungen ohne Evidenz und Wiederholungen.\n\n"
        "JSON-Schema:\n"
        "{\n"
        '  "video_value": "high|medium|low|none",\n'
        '  "reason": "kurz",\n'
        '  "claims": [\n'
        "    {\n"
        '      "entity_type": "hero|item|ability|macro|build|matchup|general",\n'
        '      "entity_name": "Name oder null",\n'
        '      "claim_type": "build|mechanic|timing|counterplay|macro|matchup|mistake|general",\n'
        '      "claim_text": "konkrete Aussage auf Deutsch",\n'
        '      "evidence_quote": "kurzes wortgleiches Zitat aus dem Transkript",\n'
        '      "timestamp_seconds": null,\n'
        '      "confidence": 0.0,\n'
        '      "is_opinion": false,\n'
        '      "needs_patch_context": false,\n'
        '      "risk": "low|medium|high"\n'
        "    }\n"
        "  ]\n"
        "}\n\n"
        f"Video:\n{json.dumps(_compact_video(video), ensure_ascii=False, sort_keys=True)}\n\n"
        f"Transkript:\n{transcript}"
    )
    return (
        {
            "model": config.model,
            "messages": [{"role": "system", "content": system}, {"role": "user", "content": prompt}],
            "max_completion_tokens": config.max_completion_tokens,
            "temperature": min(config.temperature, 0.2),
            "top_p": config.top_p,
            "stream": False,
        },
        system + "\n\n" + prompt,
    )


def _normalize_feed(feed: dict[str, Any]) -> dict[str, str]:
    url = str(feed.get("url") or "").strip()
    parsed = urllib.parse.urlparse(url)
    query = urllib.parse.parse_qs(parsed.query)
    handle = str(feed.get("handle") or "").strip().lstrip("@")
    playlist_id = str(feed.get("playlist_id") or "").strip()
    if not playlist_id and query.get("list"):
        playlist_id = query["list"][0]
    if not handle and "/@" in parsed.path:
        handle = parsed.path.split("/@", 1)[1].split("/", 1)[0]
    source_type = str(feed.get("type") or ("playlist" if playlist_id else "channel")).strip()
    normalized: dict[str, str] = {"type": source_type, "url": url}
    if handle:
        normalized["handle"] = handle
    if playlist_id:
        normalized["playlist_id"] = playlist_id
    if feed.get("channel_id"):
        normalized["channel_id"] = str(feed["channel_id"])
    return normalized


def _dedupe_feeds(feeds: list[dict[str, str]]) -> list[dict[str, str]]:
    seen: set[str] = set()
    result = []
    for feed in feeds:
        key = _feed_key(feed)
        if key in seen:
            continue
        seen.add(key)
        result.append(feed)
    return result


def _feed_key(feed: dict[str, str]) -> str:
    if feed.get("type") == "playlist":
        return f"playlist:{feed.get('playlist_id')}"
    return f"channel:{(feed.get('handle') or feed.get('channel_id') or feed.get('url') or '').lower()}"


def _upsert_feed_source(conn: sqlite3.Connection, feed_key: str, feed: dict[str, str]) -> None:
    now = int(time.time())
    conn.execute(
        """
        INSERT INTO youtube_feed_sources(
          feed_key, source_type, url, handle, playlist_id, channel_id, enabled, metadata_json, created_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(feed_key) DO UPDATE SET
          source_type=excluded.source_type,
          url=excluded.url,
          handle=excluded.handle,
          playlist_id=excluded.playlist_id,
          channel_id=COALESCE(youtube_feed_sources.channel_id, excluded.channel_id),
          metadata_json=excluded.metadata_json,
          updated_at=excluded.updated_at
        """,
        (
            feed_key,
            feed.get("type") or "channel",
            feed.get("url") or "",
            feed.get("handle"),
            feed.get("playlist_id"),
            feed.get("channel_id"),
            1,
            json.dumps(feed, ensure_ascii=True, sort_keys=True),
            now,
            now,
        ),
    )
    conn.commit()


def _stored_channel_id(conn: sqlite3.Connection, feed_key: str) -> str | None:
    row = conn.execute("SELECT channel_id FROM youtube_feed_sources WHERE feed_key=?", (feed_key,)).fetchone()
    return str(row["channel_id"]) if row and row["channel_id"] else None


def _set_feed_channel_id(conn: sqlite3.Connection, feed_key: str, channel_id: str) -> None:
    conn.execute(
        "UPDATE youtube_feed_sources SET channel_id=?, updated_at=? WHERE feed_key=?",
        (channel_id, int(time.time()), feed_key),
    )
    conn.commit()


def _resolve_channel_id(http: HttpClient, feed: dict[str, str], cache_ttl_seconds: int) -> str:
    if feed.get("channel_id"):
        return str(feed["channel_id"])
    url = feed.get("url") or f"https://www.youtube.com/@{feed.get('handle')}"
    result = http.get(url, cache_ttl_seconds=cache_ttl_seconds, timeout=30)
    text = result.text
    for pattern in (
        r'"channelId"\s*:\s*"(UC[^"]+)"',
        r'"browseId"\s*:\s*"(UC[^"]+)"',
        r'<meta itemprop="channelId" content="(UC[^"]+)">',
    ):
        match = re.search(pattern, text)
        if match:
            return match.group(1)
    raise RuntimeError(f"Konnte YouTube channel_id fuer {url} nicht finden.")


def _ytdlp_command_prefix() -> list[str] | None:
    executable = shutil.which("yt-dlp")
    if executable:
        return [executable]
    try:
        result = subprocess.run(
            [sys.executable, "-m", "yt_dlp", "--version"],
            text=True,
            capture_output=True,
            timeout=10,
        )
    except Exception:
        return None
    if result.returncode == 0:
        return [sys.executable, "-m", "yt_dlp"]
    return None


def _download_youtube_audio(
    command_prefix: list[str],
    *,
    url: str,
    video_id: str,
    audio_dir: Path,
    timeout_seconds: int,
) -> Path:
    output_template = str(audio_dir.resolve() / "%(id)s.%(ext)s")
    before = {path.name for path in audio_dir.iterdir() if path.is_file()}
    cmd = [
        *command_prefix,
        "--no-playlist",
        "--extract-audio",
        "--audio-format",
        "wav",
        "--audio-quality",
        "0",
        "--output",
        output_template,
        url,
    ]
    result = subprocess.run(cmd, cwd=str(audio_dir), text=True, capture_output=True, timeout=timeout_seconds)
    after = {path.name for path in audio_dir.iterdir() if path.is_file()}
    candidates = [audio_dir / name for name in sorted(after - before) if video_id in name]
    existing = sorted(audio_dir.glob(f"{video_id}.*"))
    if not candidates and existing:
        candidates = existing
    if result.returncode != 0:
        stderr = (result.stderr or result.stdout or "").strip()
        raise RuntimeError(f"yt-dlp audio download failed: {stderr[-1000:]}")
    wav_candidates = [path for path in candidates if path.suffix.lower() == ".wav"]
    if wav_candidates:
        return wav_candidates[0]
    if candidates:
        return candidates[0]
    raise RuntimeError("yt-dlp audio download finished but no audio file was found")


def _segments_to_transcript_text(segments: Any) -> str:
    lines = []
    for segment in segments:
        start = float(getattr(segment, "start", 0.0) or 0.0)
        end = float(getattr(segment, "end", 0.0) or 0.0)
        text = str(getattr(segment, "text", "") or "").strip()
        if text:
            lines.append(f"[{start:.1f}-{end:.1f}] {text}")
    return "\n".join(lines)


def _save_transcript_text(
    store: BrainStore,
    *,
    video_id: str,
    text: str,
    language: str,
    source_kind: str,
    metadata: dict[str, Any],
) -> None:
    _ensure_placeholder_video(store.conn, video_id)
    payload = {
        "video_id": video_id,
        "language": language,
        "source_kind": source_kind,
        "text": text,
        "metadata": metadata,
    }
    content = json.dumps(payload, ensure_ascii=False, sort_keys=True).encode("utf-8")
    source_doc_id = store.upsert_source_document(
        source=SOURCE,
        external_id=f"transcript:{video_id}:{source_kind}",
        title=f"YouTube transcript {video_id}",
        url=f"https://www.youtube.com/watch?v={video_id}",
        content_type="application/json",
        raw_path=store.write_raw(source=SOURCE, external_id=f"transcript:{video_id}:{source_kind}", content=content, suffix="json"),
        content=content,
        metadata=metadata,
    )
    now = int(time.time())
    store.conn.execute(
        """
        INSERT INTO youtube_transcripts(
          video_id, language, source_kind, transcript_text, content_hash,
          source_document_id, imported_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?)
        ON CONFLICT(video_id) DO UPDATE SET
          language=excluded.language,
          source_kind=excluded.source_kind,
          transcript_text=excluded.transcript_text,
          content_hash=excluded.content_hash,
          source_document_id=excluded.source_document_id,
          updated_at=excluded.updated_at
        """,
        (video_id, language, source_kind, text, stable_hash_text(text), source_doc_id, now, now),
    )
    store.conn.execute(
        """
        UPDATE youtube_videos
        SET transcript_status='ready', learning_status='queued', updated_at=?
        WHERE video_id=?
        """,
        (now, video_id),
    )
    store.conn.commit()


def _subtitle_languages_for_video(command_prefix: list[str], url: str, requested: str) -> str:
    requested = requested.strip()
    if requested and requested != "original":
        return requested
    try:
        result = subprocess.run(
            [*command_prefix, "--skip-download", "--dump-single-json", "--no-playlist", url],
            text=True,
            capture_output=True,
            timeout=90,
        )
    except Exception:
        return "en.*,en"
    if result.returncode != 0:
        return "en.*,en"
    try:
        info = json.loads(result.stdout)
    except json.JSONDecodeError:
        return "en.*,en"
    subtitles = info.get("subtitles") if isinstance(info.get("subtitles"), dict) else {}
    automatic = info.get("automatic_captions") if isinstance(info.get("automatic_captions"), dict) else {}
    available = [key for key in list(subtitles.keys()) + list(automatic.keys()) if key and key != "live_chat"]
    language = str(info.get("language") or "").strip()
    if language and language in available:
        return language
    if language:
        matching = [key for key in available if key == language or key.startswith(language + "-") or key.startswith(language + ".")]
        if matching:
            return matching[0]
    manual = [key for key in subtitles.keys() if key and key != "live_chat"]
    if manual:
        return manual[0]
    for candidate in ("en", "en-US", "en.*"):
        if candidate in available or candidate.endswith(".*"):
            return candidate
    return available[0] if available else "en.*,en"


def _parse_atom_videos(text: str, feed_key: str, rss_url: str) -> list[dict[str, Any]]:
    root = ET.fromstring(text)
    ns = {"atom": "http://www.w3.org/2005/Atom", "yt": "http://www.youtube.com/xml/schemas/2015", "media": "http://search.yahoo.com/mrss/"}
    channel_title = _text(root.find("atom:title", ns))
    channel_id = _text(root.find("yt:channelId", ns))
    videos = []
    for entry in root.findall("atom:entry", ns):
        video_id = _text(entry.find("yt:videoId", ns))
        title = _text(entry.find("atom:title", ns)) or video_id
        url = ""
        link = entry.find("atom:link", ns)
        if link is not None:
            url = str(link.attrib.get("href") or "")
        if not url and video_id:
            url = f"https://www.youtube.com/watch?v={video_id}"
        group = entry.find("media:group", ns)
        description = _text(group.find("media:description", ns)) if group is not None else ""
        videos.append(
            {
                "video_id": video_id,
                "feed_key": feed_key,
                "channel_id": channel_id,
                "channel_title": channel_title,
                "title": title,
                "url": url,
                "published_at": _text(entry.find("atom:published", ns)),
                "description": description,
                "rss_url": rss_url,
            }
        )
    return [video for video in videos if video.get("video_id")]


def _text(element: ET.Element | None) -> str:
    return (element.text or "").strip() if element is not None else ""


def _upsert_video(conn: sqlite3.Connection, video: dict[str, Any]) -> str:
    now = int(time.time())
    existing = conn.execute("SELECT video_id FROM youtube_videos WHERE video_id=?", (video["video_id"],)).fetchone()
    conn.execute(
        """
        INSERT INTO youtube_videos(
          video_id, feed_key, channel_id, channel_title, title, url, published_at,
          description, metadata_json, discovered_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?,?,?)
        ON CONFLICT(video_id) DO UPDATE SET
          feed_key=excluded.feed_key,
          channel_id=COALESCE(excluded.channel_id, youtube_videos.channel_id),
          channel_title=COALESCE(excluded.channel_title, youtube_videos.channel_title),
          title=excluded.title,
          url=excluded.url,
          published_at=excluded.published_at,
          description=excluded.description,
          metadata_json=excluded.metadata_json,
          updated_at=excluded.updated_at
        """,
        (
            video["video_id"],
            video["feed_key"],
            video.get("channel_id"),
            video.get("channel_title"),
            video["title"],
            video["url"],
            video.get("published_at"),
            video.get("description"),
            json.dumps(video, ensure_ascii=True, sort_keys=True),
            now,
            now,
        ),
    )
    conn.commit()
    return "updated" if existing else "inserted"


def _read_transcript_file(path: Path) -> dict[str, Any]:
    if path.suffix.lower() == ".json":
        raw = json.loads(path.read_text(encoding="utf-8"))
        if isinstance(raw, dict):
            if raw.get("text"):
                return {"video_id": raw.get("video_id"), "text": str(raw["text"]), "language": raw.get("language")}
            segments = raw.get("segments")
            if isinstance(segments, list):
                text = "\n".join(str(item.get("text") or "") for item in segments if isinstance(item, dict))
                return {"video_id": raw.get("video_id"), "text": text, "language": raw.get("language")}
        if isinstance(raw, list):
            return {"text": "\n".join(str(item.get("text") or "") for item in raw if isinstance(item, dict))}
        return {"text": ""}
    text = path.read_text(encoding="utf-8", errors="replace")
    if path.suffix.lower() in {".vtt", ".srt"}:
        text = _clean_caption_text(text)
    return {"text": text}


def _clean_caption_text(text: str) -> str:
    lines = []
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped or stripped.upper() == "WEBVTT":
            continue
        if re.match(r"^\d+$", stripped):
            continue
        if "-->" in stripped:
            continue
        stripped = re.sub(r"<[^>]+>", "", stripped)
        if stripped:
            lines.append(stripped)
    return "\n".join(lines)


def _video_id_from_name(name: str) -> str | None:
    match = re.search(r"([A-Za-z0-9_-]{11})", name)
    return match.group(1) if match else None


def _content_type_for_suffix(suffix: str) -> str:
    return {".json": "application/json", ".vtt": "text/vtt", ".srt": "text/plain", ".txt": "text/plain"}.get(suffix.lower(), "text/plain")


def _ensure_placeholder_video(conn: sqlite3.Connection, video_id: str) -> None:
    now = int(time.time())
    conn.execute(
        """
        INSERT OR IGNORE INTO youtube_feed_sources(feed_key, source_type, url, enabled, metadata_json, created_at, updated_at)
        VALUES('manual:transcripts', 'manual', '', 1, '{}', ?, ?)
        """,
        (now, now),
    )
    conn.execute(
        """
        INSERT OR IGNORE INTO youtube_videos(
          video_id, feed_key, title, url, metadata_json, transcript_status, learning_status, discovered_at, updated_at
        )
        VALUES(?,?,?,?,?,?,?,?,?)
        """,
        (video_id, "manual:transcripts", f"YouTube {video_id}", f"https://www.youtube.com/watch?v={video_id}", "{}", "missing", "queued", now, now),
    )
    conn.commit()


def _parse_model_claims(response_text: str) -> list[dict[str, Any]]:
    text = response_text.strip()
    fenced = re.search(r"```(?:json)?\s*(.*?)```", text, flags=re.S | re.I)
    if fenced:
        text = fenced.group(1).strip()
    parsed: Any
    try:
        parsed = json.loads(text)
    except json.JSONDecodeError:
        start = text.find("{")
        end = text.rfind("}")
        if start == -1 or end == -1 or end <= start:
            return []
        try:
            parsed = json.loads(text[start : end + 1])
        except json.JSONDecodeError:
            return []
    claims = parsed.get("claims") if isinstance(parsed, dict) else parsed
    if not isinstance(claims, list):
        return []
    return [claim for claim in claims if isinstance(claim, dict)]


def _save_verified_claims(
    conn: sqlite3.Connection,
    video: dict[str, Any],
    claims: list[dict[str, Any]],
    *,
    prompt_text: str,
    model_response_text: str,
    provider_metadata: dict[str, Any],
    model: str,
) -> int:
    now = int(time.time())
    saved = 0
    transcript_text = str(video.get("transcript_text") or "")
    for index, claim in enumerate(claims):
        verifier = _verify_claim(conn, claim, transcript_text)
        status = str(verifier["status"])
        if status == "rejected":
            continue
        claim_text = str(claim.get("claim_text") or "").strip()
        evidence_quote = str(claim.get("evidence_quote") or "").strip()
        claim_hash = stable_hash_text(
            json.dumps(
                {"video_id": video["video_id"], "claim_text": claim_text, "evidence_quote": evidence_quote, "prompt_version": PROMPT_VERSION},
                ensure_ascii=True,
                sort_keys=True,
            )
        )
        conn.execute(
            """
            INSERT INTO youtube_learning_claims(
              video_id, claim_hash, claim_index, entity_type, entity_name, claim_type,
              claim_text, evidence_quote, timestamp_seconds, model_confidence,
              verifier_confidence, status, model, prompt_version, prompt_text,
              model_response_text, provider_metadata_json, verifier_json, created_at, updated_at
            )
            VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ON CONFLICT(claim_hash) DO UPDATE SET
              claim_index=excluded.claim_index,
              entity_type=excluded.entity_type,
              entity_name=excluded.entity_name,
              claim_type=excluded.claim_type,
              claim_text=excluded.claim_text,
              evidence_quote=excluded.evidence_quote,
              timestamp_seconds=excluded.timestamp_seconds,
              model_confidence=excluded.model_confidence,
              verifier_confidence=excluded.verifier_confidence,
              status=excluded.status,
              model_response_text=excluded.model_response_text,
              provider_metadata_json=excluded.provider_metadata_json,
              verifier_json=excluded.verifier_json,
              updated_at=excluded.updated_at
            """,
            (
                video["video_id"],
                claim_hash,
                index,
                _clean_optional(claim.get("entity_type")) or "general",
                _clean_optional(claim.get("entity_name")),
                _clean_optional(claim.get("claim_type")) or "general",
                claim_text,
                evidence_quote,
                _float_or_none(claim.get("timestamp_seconds")),
                _clamp_float(claim.get("confidence"), default=0.0),
                float(verifier["confidence"]),
                status,
                model,
                PROMPT_VERSION,
                prompt_text,
                model_response_text,
                json.dumps(provider_metadata, ensure_ascii=True, sort_keys=True),
                json.dumps(verifier, ensure_ascii=True, sort_keys=True),
                now,
                now,
            ),
        )
        saved += 1
    conn.commit()
    return saved


def _verify_claim(conn: sqlite3.Connection, claim: dict[str, Any], transcript_text: str) -> dict[str, Any]:
    reasons = []
    claim_text = str(claim.get("claim_text") or "").strip()
    evidence_quote = str(claim.get("evidence_quote") or "").strip()
    if len(claim_text) < 18:
        reasons.append("claim_text_too_short")
    quote_found = _quote_in_text(evidence_quote, transcript_text)
    if not quote_found:
        reasons.append("evidence_quote_not_found")
    confidence = _clamp_float(claim.get("confidence"), default=0.0)
    if confidence < 0.45:
        reasons.append("model_confidence_low")
    entity_name = str(claim.get("entity_name") or "").strip()
    entity_match = _find_entity(conn, entity_name) if entity_name else None
    if entity_name and entity_match is None and str(claim.get("entity_type") or "") not in {"macro", "build", "matchup", "general"}:
        reasons.append("entity_not_resolved")
    if bool(claim.get("is_opinion")) and confidence < 0.7:
        reasons.append("weak_opinion")
    if not quote_found or not claim_text:
        status = "rejected"
    elif reasons:
        status = "needs_review"
    else:
        status = "accepted"
    verifier_confidence = 0.25 + (0.45 if quote_found else 0.0) + (0.2 if entity_match or not entity_name else 0.0) + min(confidence, 1.0) * 0.1
    if status == "needs_review":
        verifier_confidence = min(verifier_confidence, 0.74)
    return {
        "status": status,
        "confidence": round(max(0.0, min(1.0, verifier_confidence)), 3),
        "reasons": reasons,
        "quote_found": quote_found,
        "entity_match": entity_match,
        "gpt_verifier": "local_transcript_entity_evidence_v1",
    }


def _find_entity(conn: sqlite3.Connection, name: str) -> dict[str, Any] | None:
    norm = normalize_alias(name)
    row = conn.execute(
        """
        SELECT e.id, e.entity_type, e.canonical_name
        FROM entities e
        LEFT JOIN entity_aliases a ON a.entity_id=e.id
        WHERE lower(e.canonical_name)=lower(?) OR a.alias_norm=?
        ORDER BY CASE WHEN lower(e.canonical_name)=lower(?) THEN 0 ELSE 1 END
        LIMIT 1
        """,
        (name, norm, name),
    ).fetchone()
    return dict(row) if row else None


def _quote_in_text(quote: str, text: str) -> bool:
    quote_norm = _space_norm(quote)
    if not quote_norm or len(quote_norm) < 8:
        return False
    text_norm = _space_norm(text)
    if quote_norm in text_norm:
        return True
    words = quote_norm.split()
    if len(words) >= 8:
        return " ".join(words[:8]) in text_norm
    return False


def _space_norm(value: str) -> str:
    return re.sub(r"\s+", " ", value.casefold()).strip()


def _set_video_learning_status(conn: sqlite3.Connection, video_id: str, status: str) -> None:
    conn.execute("UPDATE youtube_videos SET learning_status=?, updated_at=? WHERE video_id=?", (status, int(time.time()), video_id))
    conn.commit()


def _set_video_transcript_status(conn: sqlite3.Connection, video_id: str, transcript_status: str, learning_status: str) -> None:
    conn.execute(
        "UPDATE youtube_videos SET transcript_status=?, learning_status=?, updated_at=? WHERE video_id=?",
        (transcript_status, learning_status, int(time.time()), video_id),
    )
    conn.commit()


def _compact_video(video: dict[str, Any]) -> dict[str, Any]:
    return {
        "video_id": video.get("video_id"),
        "title": video.get("title"),
        "channel_title": video.get("channel_title"),
        "published_at": video.get("published_at"),
        "url": video.get("url"),
        "description": _clip_text(str(video.get("description") or ""), 1200),
    }


def _clip_text(text: str, max_chars: int) -> str:
    if len(text) <= max_chars:
        return text
    return text[:max_chars] + "\n[... gekuerzt ...]"


def _clean_optional(value: Any) -> str | None:
    if value is None:
        return None
    text = str(value).strip()
    if not text or text.lower() in {"null", "none", "n/a"}:
        return None
    return text


def _float_or_none(value: Any) -> float | None:
    if value is None:
        return None
    try:
        return float(value)
    except (TypeError, ValueError):
        return None


def _clamp_float(value: Any, *, default: float) -> float:
    try:
        number = float(value)
    except (TypeError, ValueError):
        number = default
    return max(0.0, min(1.0, number))
