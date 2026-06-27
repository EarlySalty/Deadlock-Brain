# Transcript Claims Pipeline

Scope: `deadlock-brain-yt transcript-claims` owns only the deterministic data plane. The LLM extraction and DB verification happen outside this binary.

## Flow

1. Run `deadlock-brain-yt transcript-claims prepare`.
2. Send the selected transcripts to an external LLM workflow that extracts and verifies claims against trusted DB data.
3. Save the LLM workflow output as the ingest JSON array.
4. Run `deadlock-brain-yt transcript-claims ingest --in <path> --write`.
5. For historical zero-yield leftovers, run `deadlock-brain-yt transcript-claims backfill-attempts --write`.

Dry-run is the default for ingest. Omit `--write` to compute the summary without inserts or backup.
Dry-run is also the default for `backfill-attempts`.

## Prepare Output

`prepare` selects verbal strategy videos that have transcripts, no `youtube_claims_de_transcript_v1` rows yet, and no matching row in `youtube_transcript_claim_attempts`. It never writes to the DB. Normal `prepare` caps transcript length at 150,000 characters by default, so Monster transcripts >150k stay out of regular batches and go through the separate Monster mode in a follow-up ticket. Use `--max-chars 0` to lift the length limit.

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

## Attempts Ledger

`ingest --write` records one `youtube_transcript_claim_attempts` row for every input video, even when the video has zero valid claims. Videos with at least one valid claim get `status='ok'`; videos with no valid claims get `status='zero_yield'`. Dry-runs do not write attempts.

`backfill-attempts` marks old verbal-strategy videos with non-empty transcripts, no claims for the prompt version, no existing attempt row, and transcript length within `--max-chars` as `zero_yield` without running LLM extraction. Use `--max-chars 0` to ignore transcript length. It returns `would_mark` and `marked` counts and is idempotent.

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

## Status (Stand 2026-06-27)

- `transcript_v1` corpus: ~2090 Claims über ~133 Videos (Status accepted/needs_review/unverified/rejected). Wellen 1+2 dieser Pipeline haben die ~169 verbal-Videos mit Transcript bis 150k Zeichen abgearbeitet.
- **Zero-Yield-Reselect (behoben 2026-06-27):** `youtube_transcript_claim_attempts` hält verarbeitete Videos getrennt von Claims fest. `prepare` schließt passende Attempt-Zeilen aus; `ingest --write` schreibt `ok`/`zero_yield`; `backfill-attempts` markiert historische <=150k-Zeichen-Leftovers ohne LLM-Lauf.
- **Monster-VODs (offen):** ~94 verbal-Videos > 150k Zeichen (bis ~1,3 Mio, mehrstündige Stream-/Coaching-VODs) brauchen eine Chunking-Variante (Transcript segmentieren → mehrere Extract-Agenten pro Video → Claims vereinen), bevor sie sinnvoll ausgewertet werden. Offensichtlich themenfremde Streams ausschließen.
- **needs_asr (offen):** ~109 verbal-Videos haben keine Captions (`transcript_status='unavailable'`) und brauchen Whisper-ASR (nicht installiert).

## Orchestrierung (wie die LLM-Stufe heute läuft)

`prepare`/`ingest` sind die deterministische Daten-Ebene; die Extraktion+Verifikation dazwischen läuft als Claude-Workflow (deutsche `claim_text` + abgeleitete-Mechanik-Verifikation, NIE MiniMax). Pro Welle: `prepare`/Selektion → Workflow (Extract→Verify, schreibt `verified_<id>.json` pro Video) → `verified_*.json` zu einem Array bündeln → `ingest --write`. Eine vollautonome In-Rust-Variante (Rust ruft GPT direkt) wäre ein Folgeschritt; dafür fehlt im Core noch ein OpenAI/Anthropic-Client (nur MiniMax vorhanden).
