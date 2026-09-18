from __future__ import annotations

from datetime import date
from typing import Any, Callable

Query = Callable[[str, dict[str, Any]], list[dict[str, Any]]]


def iso_date(value: str | None, name: str) -> str | None:
    if value is None:
        return None
    value = value.strip()
    try:
        parsed = date.fromisoformat(value)
    except (TypeError, ValueError) as exc:
        raise ValueError(f"{name} muss ein ISO-Datum YYYY-MM-DD sein.") from exc
    if parsed.isoformat() != value:
        raise ValueError(f"{name} muss ein ISO-Datum YYYY-MM-DD sein.")
    return value


def literal_pattern(value: str) -> str:
    return "%" + value.replace("!", "!!").replace("%", "!%").replace("_", "!_") + "%"


def lookup_changes(
    query_rows: Query,
    text: str,
    entity: str | None = None,
    since: str | None = None,
    until: str | None = None,
    limit: int = 50,
    offset: int = 0,
) -> dict[str, Any]:
    text = text.strip()
    terms = list(dict.fromkeys(text.split()))
    if not terms or len(text) > 256 or len(terms) > 12:
        raise ValueError("text muss 1 bis 12 Suchwörter und höchstens 256 Zeichen enthalten.")
    since, until = iso_date(since, "since"), iso_date(until, "until")
    if since and until and since > until:
        raise ValueError("since darf nicht nach until liegen.")
    limit, offset = max(1, min(int(limit), 200)), max(0, min(int(offset), 100000))
    variables: dict[str, Any] = {"limit": limit, "offset": offset}
    predicates: list[str] = []
    for number, term in enumerate(terms):
        key = f"term{number}"
        variables[key] = literal_pattern(term)
        predicates.append(
            f"concat_ws(' ', c.raw_line, c.entity_name, c.ability_name, c.stat_name) "
            f"ILIKE :'{key}' ESCAPE '!'"
        )
    names_cte = ""
    if entity is not None:
        entity = entity.strip()
        if not entity or len(entity) > 160:
            raise ValueError("entity muss einen Namen mit höchstens 160 Zeichen enthalten.")
        variables["entity"] = entity
        names_cte = """
            requested_entities AS (
                SELECT DISTINCT e.id, e.canonical_name
                FROM brain.entities e
                LEFT JOIN brain.entity_aliases a ON a.entity_id = e.id
                WHERE lower(e.canonical_name) = lower(:'entity')
                   OR lower(a.alias) = lower(:'entity')
            ), requested_names AS (
                SELECT lower(:'entity') AS name
                UNION SELECT lower(canonical_name) FROM requested_entities
                UNION SELECT lower(a.alias) FROM brain.entity_aliases a
                      JOIN requested_entities e ON e.id = a.entity_id
            ),
        """
        predicates.append("lower(c.entity_name) IN (SELECT name FROM requested_names)")
    if since:
        variables["since"] = since
        predicates.append("c.patch_date >= :'since'::date")
    if until:
        variables["until"] = until
        predicates.append("c.patch_date <= :'until'::date")
    sql = f"""
        WITH {names_cte} matched AS (
            SELECT DISTINCT c.patch_title, c.patch_date, c.entity_type, c.entity_name,
                   c.ability_name, c.stat_name, c.old_value, c.new_value,
                   c.change_type, c.numeric_direction, c.raw_line, c.patch_url, c.confidence
            FROM brain.patch_changes c
            WHERE {' AND '.join(predicates)}
        ), page AS (
            SELECT * FROM matched
            ORDER BY patch_date ASC NULLS LAST, patch_url, entity_name, raw_line
            LIMIT :limit OFFSET :offset
        ), indexed_page AS (
            SELECT p.*, i.first_indexed_at
            FROM page p
            LEFT JOIN LATERAL (
                SELECT min(e.created_at)::text AS first_indexed_at
                FROM brain.patch_events e
                WHERE e.patch_url = p.patch_url AND e.raw_line = p.raw_line
            ) i ON true
        )
        SELECT (SELECT count(*) FROM matched) AS match_count,
               (SELECT min(patch_date) FROM matched) AS earliest_matching_patch,
               (SELECT max(patch_date) FROM matched) AS latest_matching_patch,
               coalesce((SELECT json_agg(p ORDER BY p.patch_date ASC NULLS LAST,
                                        p.patch_url, p.entity_name, p.raw_line)
                         FROM indexed_page p), '[]'::json) AS changes
    """
    rows = query_rows(sql, variables)
    if len(rows) != 1 or not isinstance(rows[0].get("changes"), list):
        raise RuntimeError("Die Patchsuche lieferte kein gültiges Ergebnisformat.")
    result = dict(rows[0])
    count = int(result["match_count"])
    next_offset = offset + len(result["changes"])
    result.update({
        "query": text,
        "entity": entity,
        "since": since,
        "until": until,
        "next_offset": next_offset if next_offset < count else None,
        "date_semantics": {
            "patch_date": "Datum des Patch-Beitrags, nicht unabhängig bestätigter Live-Zeitpunkt.",
            "first_indexed_at": "Früheste erhaltene Indexierungszeit dieser exakten Patchzeile; kann nach Rebuilds später sein.",
        },
        "coverage": "Treffer im vorhandenen Index und Suchfenster, keine Garantie vollständiger Spielhistorie.",
        "authority": "Patchquellen; keine Creator-Bewertung und keine abgeleitete Meta-Prognose.",
    })
    return result


def read_insight(query_rows: Query, patch_url: str) -> dict[str, Any]:
    patch_url = patch_url.strip()
    if not patch_url or len(patch_url) > 2048:
        raise ValueError("patch_url fehlt oder ist zu lang.")
    rows = query_rows(
        """
        SELECT r.id, r.patch_url, r.patch_date, r.created_at, r.context_hash,
               CASE WHEN r.result#>>'{context,patch_source_hash}' = p.current_hash
                    THEN r.status ELSE 'stale' END AS status,
               (r.result#>>'{context,patch_source_hash}' = p.current_hash) AS source_revision_current,
               EXISTS(SELECT 1 FROM brain.patch_changes c WHERE c.patch_date > r.patch_date) AS newer_indexed_patch_exists,
               r.result - 'context' AS result, r.warnings
        FROM brain.patch_insight_runs r
        CROSS JOIN LATERAL (
            SELECT md5(coalesce(string_agg(to_jsonb(c)::text, chr(10) ORDER BY to_jsonb(c)::text),'')) AS current_hash
            FROM brain.patch_changes c WHERE c.patch_url=r.patch_url
        ) p
        WHERE r.patch_url = :'patch_url'
        ORDER BY r.created_at DESC, r.id DESC LIMIT 1
        """,
        {"patch_url": patch_url},
    )
    return {
        "found": bool(rows),
        "analysis": rows[0] if rows else None,
        "authority": "Eigene, quellengebundene Analyse zum gespeicherten Patchstand. Hypothesen sind keine bestätigten Meta-Fakten.",
        "current_patch_verified": False,
    }


def read_video_evidence(query_rows: Query, video_id: str, text: str = "", limit: int = 20) -> dict[str, Any]:
    if not video_id or len(video_id) > 128 or not all(c.isascii() and (c.isalnum() or c in "_-") for c in video_id):
        raise ValueError("Ungültige Video-ID.")
    if len(text) > 512:
        raise ValueError("text darf höchstens 512 Zeichen enthalten.")
    rows = query_rows(
        """
        WITH current_evidence AS (
            SELECT e.*, v.url AS source_url
            FROM brain.youtube_transcript_evidence e
            JOIN brain.youtube_transcripts t
              ON t.video_id=e.video_id AND t.content_hash=e.transcript_hash
            JOIN brain.youtube_videos v ON v.video_id=e.video_id
            WHERE e.video_id=:'video_id'
            ORDER BY e.imported_at DESC,e.evidence_hash LIMIT 1
        ), matching_segments AS (
            SELECT s.value AS segment, s.ordinality
            FROM current_evidence e,
                 jsonb_array_elements(e.segments) WITH ORDINALITY AS s(value,ordinality)
            WHERE s.value->>'text' ILIKE :'text' ESCAPE '!'
        ), page AS (
            SELECT segment,ordinality FROM matching_segments
            ORDER BY ordinality LIMIT :limit
        )
        SELECT e.video_id,e.source_url,e.language,e.source_kind,e.timing_status,
               e.evidence_hash,e.transcript_hash,e.parser_version,
               (SELECT count(*) FROM matching_segments) AS match_count,
               coalesce((SELECT json_agg(p.segment ORDER BY p.ordinality) FROM page p), '[]'::json) AS segments
        FROM current_evidence e
        """,
        {"video_id": video_id, "text": literal_pattern(text.strip()), "limit": max(1, min(int(limit), 100))},
    )
    return {
        "found": bool(rows),
        "evidence": rows[0] if rows else None,
        "evidence_type": "caption_text_not_visual_observation",
        "search_scope": "Wörtliche Suche innerhalb einzelner Caption-Segmente; fehlende Zeitmarken bleiben null.",
        "authority": "Belegt, was in den Untertiteln steht, nicht die Richtigkeit der Spielaussage.",
    }


def register_tools(mcp: Any, query_rows: Query) -> None:
    @mcp.tool(
        name="change_lookup",
        description="Wann wurde eine Änderung dokumentiert? Durchsucht Original-Patchzeilen mit Aliasauflösung, inklusiven Datumsgrenzen, Quellen und Pagination. Suchwörter werden wörtlich UND-verknüpft.",
    )
    def change_lookup(
        text: str,
        entity: str | None = None,
        since: str | None = None,
        until: str | None = None,
        limit: int = 50,
        offset: int = 0,
    ) -> dict[str, Any]:
        return lookup_changes(query_rows, text, entity, since, until, limit, offset)

    @mcp.tool(
        name="video_evidence",
        description="Liest gespeicherte Caption-Segmente mit echten Zeitmarken, Rohquellen-Hash und Sprache. Kein Bildbeleg und kein Nachweis der sachlichen Richtigkeit einer Videoaussage.",
    )
    def video_evidence(video_id: str, text: str = "", limit: int = 20) -> dict[str, Any]:
        return read_video_evidence(query_rows, video_id, text, limit)

    @mcp.tool(
        name="patch_insight",
        description="Liest die jüngste gespeicherte eigenständige Patchanalyse. Belegte Änderungen und strategische Hypothesen bleiben getrennt; keine automatische Aktualitätsgarantie.",
    )
    def patch_insight(patch_url: str) -> dict[str, Any]:
        return read_insight(query_rows, patch_url)

