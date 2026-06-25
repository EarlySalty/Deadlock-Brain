# Transcript Claims Pipeline

Scope: `deadlock-brain-yt transcript-claims` owns only the deterministic data plane. The LLM extraction and DB verification happen outside this binary.

## Flow

1. Run `deadlock-brain-yt transcript-claims prepare`.
2. Send the selected transcripts to an external LLM workflow that extracts and verifies claims against trusted DB data.
3. Save the LLM workflow output as the ingest JSON array.
4. Run `deadlock-brain-yt transcript-claims ingest --in <path> --write`.

Dry-run is the default for ingest. Omit `--write` to compute the summary without inserts or backup.

## Prepare Output

`prepare` selects verbal strategy videos that have transcripts and no `youtube_claims_de_transcript_v1` rows yet. It never writes to the DB.

Output shape:

```json
{
  "generated_at": 0,
  "prompt_version": "youtube_claims_de_transcript_v1",
  "order": "recent",
  "count": 1,
  "videos": [
    {
      "video_id": "abc",
      "title": "Video title",
      "url": "https://youtube.com/watch?v=abc",
      "channel_title": "Channel",
      "char_len": 1234,
      "transcript_text": "full transcript text"
    }
  ]
}
```

Use `--order recent` for newest first, or `--order shortest` for shortest transcript first. Use `--out <path>` to write the JSON to a file.

## Ingest Input

Canonical input is a JSON array of video objects:

```json
[
  {
    "video_id": "abc",
    "claims": [
      {
        "entity_type": "hero",
        "entity_name": "Lash",
        "claim_type": "build",
        "claim_text": "verified claim text",
        "evidence_quote": "source quote",
        "timestamp_seconds": 12.5,
        "verdict": "supported",
        "confidence": 0.9,
        "model_confidence": null,
        "db_evidence": null,
        "db_value": null,
        "reasoning": null
      }
    ]
  }
]
```

`verified` is accepted as an alias for `claims`.

Verdict mapping:

| Verdict | Status |
| --- | --- |
| `supported` | `accepted` |
| `uncertain` | `needs_review` |
| `no_trusted_data` | `unverified` |
| `contradicted` | `rejected` |

Unknown verdicts are collected in `errors` and skipped.

## Idempotency

`claim_hash` is SHA-256 lower hex over:

```text
{video_id}|{claim_text}|{evidence_quote}
```

The pipe order and empty-string fallback for missing `evidence_quote` must not change. Existing hashes are skipped, so re-running the same ingest file inserts zero new rows.

## Backup

When `ingest --write` is used without `--no-backup`, the binary copies the opened SQLite main DB file before the first insert. The backup path is:

```text
<dbpath>.bak-<YYYY-MM-DD>-transcript-claims-scale
```

If that file already exists, a `-HHMMSS` suffix is added, with an extra numeric suffix if needed to avoid overwrite. The summary returns the backup path.

The DB runs in WAL mode, so before copying, `ingest` forces `PRAGMA wal_checkpoint(TRUNCATE)` to flush committed pages into the main file; if a checkpoint cannot fully truncate (busy), the non-empty `-wal`/`-shm` sidecars are copied alongside the backup. This makes the copied file self-consistent.

## Status (Stand 2026-06-26)

- `transcript_v1` corpus: ~2090 Claims über ~133 Videos (Status accepted/needs_review/unverified/rejected). Wellen 1+2 dieser Pipeline haben die ~169 verbal-Videos mit Transcript bis 150k Zeichen abgearbeitet.
- **Zero-Yield-Reselect (offen):** Videos, deren Transcript keine prüfbaren Claims hergibt (dünne Shorts), bekommen keine `transcript_v1`-Zeile und werden vom `NOT EXISTS`-Filter in `prepare` bei jedem Lauf erneut ausgewählt. Folgepunkt: einen "attempted"-Marker (z.B. `youtube_videos.learning_status` oder ein metadata-Flag) setzen und in der Selektion ausschließen, damit Null-Ertrag-Videos nicht wiederholt verarbeitet werden.
- **Monster-VODs (offen):** ~94 verbal-Videos > 150k Zeichen (bis ~1,3 Mio, mehrstündige Stream-/Coaching-VODs) brauchen eine Chunking-Variante (Transcript segmentieren → mehrere Extract-Agenten pro Video → Claims vereinen), bevor sie sinnvoll ausgewertet werden. Offensichtlich themenfremde Streams ausschließen.
- **needs_asr (offen):** ~109 verbal-Videos haben keine Captions (`transcript_status='unavailable'`) und brauchen Whisper-ASR (nicht installiert).

## Orchestrierung (wie die LLM-Stufe heute läuft)

`prepare`/`ingest` sind die deterministische Daten-Ebene; die Extraktion+Verifikation dazwischen läuft als Claude-Workflow (deutsche `claim_text` + abgeleitete-Mechanik-Verifikation, NIE MiniMax). Pro Welle: `prepare`/Selektion → Workflow (Extract→Verify, schreibt `verified_<id>.json` pro Video) → `verified_*.json` zu einem Array bündeln → `ingest --write`. Eine vollautonome In-Rust-Variante (Rust ruft GPT direkt) wäre ein Folgeschritt; dafür fehlt im Core noch ein OpenAI/Anthropic-Client (nur MiniMax vorhanden).
