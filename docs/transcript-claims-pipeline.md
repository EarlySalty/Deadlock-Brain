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
