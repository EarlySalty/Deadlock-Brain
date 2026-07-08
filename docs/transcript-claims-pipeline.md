# Transcript Claims Pipeline

Scope: `deadlock-brain-yt transcript-claims` owns only the deterministic data plane. The LLM extraction and DB verification happen outside this binary.

## Flow

1. Run `deadlock-brain-yt transcript-claims prepare`.
2. Send the selected transcripts to an external LLM workflow that extracts and verifies claims against trusted DB data.
3. Save the LLM workflow output as the ingest JSON array.
4. Run `deadlock-brain-yt transcript-claims ingest --in <path> --write`.
5. For historical zero-yield leftovers, run `deadlock-brain-yt transcript-claims backfill-attempts --write`.
6. For oversized off-topic VODs, run `deadlock-brain-yt transcript-claims mark-offtopic` before Monster extraction.

Dry-run is the default for ingest. Omit `--write` to compute the summary without inserts.
Dry-run is also the default for `backfill-attempts`.

## Prepare Output

`prepare` selects verbal strategy videos that have transcripts, no `youtube_claims_de_transcript_v1` rows yet, and no matching row in `youtube_transcript_claim_attempts`. It never writes to the DB. Normal `prepare` caps transcript length at 150,000 characters by default, so Monster transcripts >150k stay out of regular batches. Use `--max-chars 0` to lift the normal-mode length limit.

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

## Monster Prepare Output

`prepare --mode monster` selects verbal strategy videos with non-empty transcripts longer than `--min-chars` (default 150,000, exclusive), no claims for the prompt version, and no matching attempt row. It outputs overlapping transcript chunks instead of `transcript_text`. Chunking is by Unicode character offset, not byte offset.

Defaults:

- `--chunk-chars 60000`
- `--overlap-chars 4000`
- `--min-chars 150000`

Output shape:

```json
{
  "generated_at": 0,
  "prompt_version": "youtube_claims_de_transcript_v1",
  "mode": "monster",
  "chunk_chars": 60000,
  "overlap_chars": 4000,
  "min_chars": 150000,
  "count": 1,
  "videos": [
    {
      "video_id": "abc",
      "title": "Long VOD",
      "url": "https://youtube.com/watch?v=abc",
      "channel_title": "Channel",
      "char_len": 250000,
      "chunk_count": 5,
      "chunks": [
        {
          "chunk_index": 0,
          "char_start": 0,
          "char_end": 60000,
          "char_len": 60000,
          "text": "chunk text"
        }
      ]
    }
  ]
}
```

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

## Attempts Ledger

`ingest --write` records one `youtube_transcript_claim_attempts` row for every input video, even when the video has zero valid claims. Videos with at least one valid claim get `status='ok'`; videos with no valid claims get `status='zero_yield'`. Attempt `mode` is derived from transcript length: `monster` when `char_len > 150000`, otherwise `normal`. Dry-runs do not write attempts.

`backfill-attempts` marks old verbal-strategy videos with non-empty transcripts, no claims for the prompt version, no existing attempt row, and transcript length within `--max-chars` as `zero_yield` without running LLM extraction. Use `--max-chars 0` to ignore transcript length. It returns `would_mark` and `marked` counts and is idempotent.

`mark-offtopic` writes `youtube_transcript_claim_attempts` rows with `mode='monster'` and `status='offtopic'` for Monster candidates that should not go through the external workflow. It requires at least one source:

- `--title-contains <STR>` can be repeated and matches case-insensitive substrings on title.
- `--video-ids <PATH>` reads one `video_id` per line.

Dry-run is the default. With `--write`, attempts are recorded in Postgres.

## Idempotency

`claim_hash` is SHA-256 lower hex over:

```text
{video_id}|{claim_text}|{evidence_quote}
```

The pipe order and empty-string fallback for missing `evidence_quote` must not change. Existing hashes are skipped, so re-running the same ingest file inserts zero new rows.

## Backup

`ingest --write` writes directly to the central Postgres `brain` schema. The old
local file backup path is gone; rollback is handled at the database/backups
layer.

## Status (Stand 2026-06-27)

- `transcript_v1` corpus: ~2090 Claims über ~133 Videos (Status accepted/needs_review/unverified/rejected). Wellen 1+2 dieser Pipeline haben die ~169 verbal-Videos mit Transcript bis 150k Zeichen abgearbeitet.
- **Zero-Yield-Reselect (behoben 2026-06-27):** `youtube_transcript_claim_attempts` hält verarbeitete Videos getrennt von Claims fest. `prepare` schließt passende Attempt-Zeilen aus; `ingest --write` schreibt `ok`/`zero_yield`; `backfill-attempts` markiert historische <=150k-Zeichen-Leftovers ohne LLM-Lauf.
- **Monster-VODs:** `prepare --mode monster` chunked >150k-Zeichen-Transkripte für den externen Workflow. Off-topic-VODs können vorher per `mark-offtopic` dauerhaft aus der Monster-Selektion genommen werden.
- **needs_asr (offen):** ~109 verbal-Videos haben keine Captions (`transcript_status='unavailable'`) und brauchen Whisper-ASR (nicht installiert).

## Orchestrierung (wie die LLM-Stufe heute läuft)

`prepare`/`ingest` sind die deterministische Daten-Ebene; die Extraktion+Verifikation dazwischen läuft als Claude-Workflow (deutsche `claim_text` + abgeleitete-Mechanik-Verifikation, nicht ueber das Brain-Textmodell). Pro Welle: `prepare`/Selektion → Workflow (Extract→Verify, schreibt `verified_<id>.json` pro Video) → `verified_*.json` zu einem Array bündeln → `ingest --write`. Eine vollautonome In-Rust-Variante (Rust ruft GPT direkt) wäre ein Folgeschritt; dafür fehlt im Core noch ein OpenAI/Anthropic-Client.
